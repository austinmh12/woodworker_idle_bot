use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player};

pub async fn run(player_id: u64, nickname: String, avatar: String, options: &[CommandDataOption]) -> (String, Option<CreateEmbed>) {
	let option = &options
		.get(0)
		.expect("Expected a Subcommand");
	
	let mut player = get_player(player_id).await;
	match option.name.as_str() {
		"profile" => ("".to_string(), Some(player.embed(nickname, avatar))),
		"stats" => ("".to_string(), Some(player.stats.embed(nickname, avatar))),
		"inventory" => ("todo".to_string(), None),
		"blueprints" => ("todo".to_string(), None),
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

			(format!("You updated your profile color to **{}, {}, {}**", red, green, blue), None)
		}
		_ => ("".to_string(), Some(player.embed(nickname, avatar)))
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