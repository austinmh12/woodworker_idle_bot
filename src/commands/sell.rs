use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player};

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> String {
	println!("{:#?}", &options);
	let sub_options = &options
		.get(0)
		.expect("Expected a SubcommandGroup")
		.options
		.get(0)
		.expect("Expected SubCommand")
		.options;
	let log = sub_options
		.get(0)
		.expect("Expexted String")
		.resolved
		.as_ref()
		.expect("Expected str");
	println!("{:#?}", &log);
	let amount = if sub_options.len() == 2usize {
		match sub_options
			.get(1)
			.expect("Expected an Integer")
			.resolved
			.as_ref()
			.expect("Expected int") {
				CommandDataOptionValue::Integer(i) => i.to_owned(),
				_ => 1
			}
	} else {
		1
	};

	println!("{}", player_id);
	let mut player = get_player(player_id).await;
	if let CommandDataOptionValue::String(tree) = log {
		match tree.as_str() {
			"pine" => {
				player.logs.pine -= amount;
				format!("You sold **{} pine** logs!", amount)
			},
			_ => "No such tree".to_string()
		}
	} else {
		"No such tree".to_string()
	}

	// "".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("sell").description("Sell your logs, lumber, or furniture!").create_option(|option| {
		option
			.name("type")
			.description("The type of item you want to sell")
			.kind(CommandOptionType::SubCommandGroup)
			.create_sub_option(|sub_option| {
				sub_option
					.name("logs")
					.description("sell your logs!")
					.kind(CommandOptionType::SubCommand)
					.create_sub_option(|sub_choice| {
						sub_choice
							.name("log")
							.description("the log to sell")	
							.kind(CommandOptionType::String)
							.required(true)
					})
					.create_sub_option(|sub_choice| {
						sub_choice
							.name("amount")
							.description("the amount to sell")	
							.kind(CommandOptionType::Integer)
							.required(false)
					})
			})
			.create_sub_option(|sub_option| {
				sub_option
					.name("lumber")
					.description("sell your lumber!")
					.kind(CommandOptionType::SubCommand)
					.create_sub_option(|sub_choice| {
						sub_choice
							.name("lumber")
							.description("the lumber to sell")	
							.kind(CommandOptionType::String)
							.required(true)
					})
					.create_sub_option(|sub_choice| {
						sub_choice
							.name("amount")
							.description("the amount to sell")	
							.kind(CommandOptionType::Integer)
							.required(false)
					})
			})
	})
}