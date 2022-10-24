use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Tree, BPUnlock};
use crate::utils::{Message, PaginatedEmbed};

pub async fn run(player_id: u64, nickname: String, avatar: String, options: &[CommandDataOption]) -> Message {
	let option = &options
		.get(0)
		.expect("Expected a Subcommand");
	
	let mut player = get_player(player_id).await;
	match option.name.as_str() {
		"profile" => Message::Embed(player.embed(nickname, avatar)),
		"stats" => Message::Embed(player.stats.embed(nickname, avatar)),
		"inventory" => {
			let p = PaginatedEmbed::new(
				vec![
					player.inventory(nickname.clone(), avatar.clone(), Tree::Pine(BPUnlock::None)),
					player.inventory(nickname.clone(), avatar.clone(), Tree::Oak(BPUnlock::None)),
					player.inventory(nickname.clone(), avatar.clone(), Tree::Maple(BPUnlock::None)),
					player.inventory(nickname.clone(), avatar.clone(), Tree::Walnut(BPUnlock::None)),
					player.inventory(nickname.clone(), avatar.clone(), Tree::Cherry(BPUnlock::None)),
					player.inventory(nickname.clone(), avatar.clone(), Tree::PurpleHeart(BPUnlock::None)),
				]
			);

			Message::Pages(p)
		},
		"blueprints" => {
			let p = PaginatedEmbed::new(
				vec![
					player.blueprint_embed(nickname.clone(), avatar.clone(), Tree::Pine(BPUnlock::None)),
					player.blueprint_embed(nickname.clone(), avatar.clone(), Tree::Oak(BPUnlock::None)),
					player.blueprint_embed(nickname.clone(), avatar.clone(), Tree::Maple(BPUnlock::None)),
					player.blueprint_embed(nickname.clone(), avatar.clone(), Tree::Walnut(BPUnlock::None)),
					player.blueprint_embed(nickname.clone(), avatar.clone(), Tree::Cherry(BPUnlock::None)),
					player.blueprint_embed(nickname.clone(), avatar.clone(), Tree::PurpleHeart(BPUnlock::None)),
				]
			);

			Message::Pages(p)
		},
		"colour" => {
			let red = match option.options.get(0).expect("Expected an integer").resolved.as_ref().expect("Expected an integer") {
				CommandDataOptionValue::Integer(r) => *r as u8,
				_ => 0u8
			};
			let green = match option.options.get(1).expect("Expected an integer").resolved.as_ref().expect("Expected an integer") {
				CommandDataOptionValue::Integer(g) => *g as u8,
				_ => 0u8
			};
			let blue = match option.options.get(2).expect("Expected an integer").resolved.as_ref().expect("Expected an integer") {
				CommandDataOptionValue::Integer(b) => *b as u8,
				_ => 0u8
			};
			player.color.red = red;
			player.color.green = green;
			player.color.blue = blue;
			player.update().await;

			Message::Content(format!("You updated your profile color to **{}, {}, {}**", red, green, blue))
		}
		_ => Message::how()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("my").description("See your profile")
		.create_option(|option| {
			option
				.name("profile")
				.description("Your profile")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("stats")
				.description("Your stats")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("inventory")
				.description("Your inventory")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("blueprints")
				.description("Your blueprints")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("colour")
				.description("Set your profile colour")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("red")
						.description("Red value")
						.kind(CommandOptionType::Integer)
						.required(true)
						.min_int_value(0)
						.max_int_value(255)
				})
				.create_sub_option(|sub| {
					sub
						.name("green")
						.description("Green value")
						.kind(CommandOptionType::Integer)
						.required(true)
						.min_int_value(0)
						.max_int_value(255)
				})
				.create_sub_option(|sub| {
					sub
						.name("blue")
						.description("Blue value")
						.kind(CommandOptionType::Integer)
						.required(true)
						.min_int_value(0)
						.max_int_value(255)
				})
		})
}