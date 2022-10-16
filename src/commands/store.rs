use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::get_player;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> (String, Option<CreateEmbed>) {
	// store.view
	// store.buy.slot amount
	let action = &options
		.get(0)
		.expect("Expected a SubcommandGroup");
	let mut player = get_player(player_id).await;
	match action.name.as_str() {
		"view" => ("todo".to_string(), None),
		"buy" => ("todo".to_string(), None),
		_ => ("todo".to_string(), None),
	}
	
	// ("todo".to_string(), None)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("store").description("Buy axes, kilns, employees, and more!")
		.create_option(|option| {
			option
				.name("view")
				.description("view the store!")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("buy")
				.description("buy from the store!")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("slot")
						.description("slot to purchase")
						.kind(CommandOptionType::Integer)
						.min_int_value(1)
						.max_int_value(9)
						.required(true)
				})
				.create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to purchase")
						.kind(CommandOptionType::Integer)
						.min_int_value(1)
						.required(false)
				})
		})
}