use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Duration as StdDuration,
};
use dotenv;

use serenity::{
	async_trait, 
	model::{
		prelude::{
			GuildId, 
			interaction::{
				Interaction, 
				InteractionResponseType
			}
		}
	}, 
};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
	gateway::{
		Ready,
		GatewayIntents
	},
};
use serenity::framework::standard::{
    StandardFramework,
};

mod commands;
mod player;
mod utils;
mod loops;

struct Handler {
	is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
	// Allows the bot to interact with slash commands
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			// println!("Received command interaction: {:#?}", command);

			let player_id = command.member.as_ref().unwrap().user.id.0.clone();
			let (content, embed) = match command.data.name.as_str() {
				"chop" => {
					(commands::chop::run(player_id, &command.data.options).await, None)
				},
				"sell" => {
					(commands::sell::run(player_id, &command.data.options).await, None)
				},
				"my" => {
					let p = command.member.as_ref().unwrap();
					let player_nick = match p.user.nick_in(&ctx.http, command.guild_id.unwrap()).await {
						Some(n) => n,
						None => p.user.name.clone()
					}.to_string();
					let player_avatar = p.user.avatar_url().unwrap();
					commands::player::run(player_id, player_nick, player_avatar, &command.data.options).await
				}
				_ => ("not implemented :(".to_string(), None)
			};

			if let Err(why) = command
				.create_interaction_response(&ctx.http, |response| {
					response
						.kind(InteractionResponseType::ChannelMessageWithSource)
						.interaction_response_data(|message| {
							message.content(content);
							match embed {
								Some(e) => {
									message.set_embed(e);
								},
								None => ()
							};

							message
						})
				})
				.await
			{
				println!("Cannot respond to slash command: {}", why);
			}
		}
	}
	
	// Set the handler to be called on the `ready` event. This is called when a shard is booted, and a READY payload is sent by Discord.
	// This payload contains a bunch of data.
	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected and ready!", ready.user.name);

		let guild_id = dotenv::var("GUILD_ID").expect("Expected a token in the environment");
		let guild_id = GuildId(
			guild_id.parse().expect("GUILD_ID must be an integer")
		);

		let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
			commands
				.create_application_command(|command| commands::chop::register(command))
				.create_application_command(|command| commands::sell::register(command))
				.create_application_command(|command| commands::player::register(command))
		})
		.await;

		let ctx = Arc::new(ctx);
		
		if !self.is_loop_running.load(Ordering::Relaxed) {
			let ctx1 = Arc::clone(&ctx);
			tokio::spawn(async move {
				loop {
					loops::check_actions(Arc::clone(&ctx1)).await;
					tokio::time::sleep(StdDuration::from_secs(1)).await;
				}
			});
			// Keeping here for reference on how to do multiple bg tasks
			// let ctx2 = Arc::clone(&ctx);
			// tokio::spawn(async move {
			// 	loop {
			// 		commands::poketcg::refresh_card_prices(Arc::clone(&ctx2)).await;
			// 		tokio::time::sleep(StdDuration::from_secs(3600)).await;
			// 	}
			// });
			// let ctx3 = Arc::clone(&ctx);
			// tokio::spawn(async move {
			// 	loop {
			// 		commands::poketcg::check_daily_streaks(Arc::clone(&ctx3)).await;
			// 		tokio::time::sleep(StdDuration::from_secs(1800)).await;
			// 	}
			// });
		}

		// println!("I now have the following guild slash commands: {:#?}", _commands);
	}

	// Here for getting custom emoji IDs
	// async fn reaction_add(&self, _ctx: Context, reaction: serenity::model::channel::Reaction) {
	// 	match reaction.emoji {
	// 		serenity::model::channel::ReactionType::Custom {animated: _, id: y, name: Some(_)} => println!("{}", y.0),
	// 		serenity::model::channel::ReactionType::Unicode(s) => println!("{}", s),
	// 		_ => ()
	// 	}
	// }
}

#[tokio::main]
async fn main() {
	let framework = StandardFramework::new()
		.configure(|c| c.prefix("."));

	dotenv::dotenv().ok();
	// Configure the client with the discord token. Make sure one is commented out.
	let token = dotenv::var("BOTTOKEN").expect("Expected a token in the environment");
	let intents = GatewayIntents::GUILD_MESSAGES
		| GatewayIntents::GUILD_MESSAGE_REACTIONS
		| GatewayIntents::MESSAGE_CONTENT;

	let handler = Handler {
		is_loop_running: AtomicBool::new(false),
	};

	// Create a new instance of the client logging in as the bot. This will automatically
	// prepend your bot token with "Bot ", which is required by discord.
	let mut client = Client::builder(&token, intents)
		.framework(framework)
		.event_handler(handler)
		.await
		.expect("Error creating client");

	// Finally start a shard and listen for events.
	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}