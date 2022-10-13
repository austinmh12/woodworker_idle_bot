use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Duration as StdDuration,
	collections::HashMap,
};
use dotenv;

use serenity::{async_trait, model::{channel::{Message}, prelude::{GuildId, interaction::{Interaction, InteractionResponseType}}}, framework::standard::{CommandOptions, Reason}, prelude::*};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
	gateway::{
		Ready,
		GatewayIntents
	},
};
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group,
		check
    },
	Args,
};
use chrono::{DateTime, Utc, Duration};
use mongodb::{
	Client as MongoClient,
	options::{
		ClientOptions,
	},
};
use std::error::Error;

mod commands;
mod player;

struct Handler {
	is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
	// Allows the bot to interact with slash commands
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Received command interaction: {:#?}", command);

			let content = match command.data.name.as_str() {
				"ping" => commands::ping::run(&command.data.options),
				"chop" => {
					let player_id = command.member.as_ref().unwrap().user.id.0.clone();
					commands::chop::run(player_id, &command.data.options)
				}
				_ => "not implemented :(".to_string()
			};

			if let Err(why) = command
				.create_interaction_response(&ctx.http, |response| {
					response
						.kind(InteractionResponseType::ChannelMessageWithSource)
						.interaction_response_data(|message| message.content(content))
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

		let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
			commands
				.create_application_command(|command| commands::ping::register(command))
				.create_application_command(|command| commands::chop::register(command))
		})
		.await;

		println!("I now have the following guild slash commands: {:#?}", commands);
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

async fn get_client() -> Result<MongoClient, Box<dyn Error>> {
	let mon_client_uri = dotenv::var("MONGODB_URI").expect("No mongodb uri");
	let options = ClientOptions::parse(&mon_client_uri).await?;
	let client = MongoClient::with_options(options)?;
	
	Ok(client)
}