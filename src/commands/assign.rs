use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Axe, Kiln, Hammer};
use crate::utils::Message;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> Message {
	let unit = &options
		.get(0)
		.expect("Expected a Subcommand");
	let tree = &unit
		.options
		.get(0)
		.expect("Subcommand");
	let amount = match &tree
		.options
		.get(0)
		.expect("int")
		.resolved
		.as_ref()
		.expect("int") 
	{
		CommandDataOptionValue::Integer(i) => i.to_owned(),
		_ => 0
	};
	let mut player = get_player(player_id).await;

	match unit.name.as_str() {
		"loggers" => match tree.name.as_str() {
			"pine" => {
				let amounts = vec![amount, player.available_loggers()];
				let amount = *amounts.iter().min().unwrap();
				player.loggers_active.pine += amount;
				player.update().await;

				Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
			},
			"oak" => {
				if player.axe < Axe::Iron {
					return Message::Content("You need an **Iron** axe to assign loggers to this tree!".to_string());
				}
				let amounts = vec![amount, player.available_loggers()];
				let amount = *amounts.iter().min().unwrap();
				player.loggers_active.oak += amount;
				player.update().await;

				Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
			},
			"maple" => {
				if player.axe < Axe::Steel {
					return Message::Content("You need a **Steel** axe to assign loggers to this tree!".to_string());
				}
				let amounts = vec![amount, player.available_loggers()];
				let amount = *amounts.iter().min().unwrap();
				player.loggers_active.maple += amount;
				player.update().await;

				Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
			},
			"walnut" => {
				if player.axe < Axe::Mithril {
					return Message::Content("You need a **Mithril** axe to assign loggers to this tree!".to_string());
				}
				let amounts = vec![amount, player.available_loggers()];
				let amount = *amounts.iter().min().unwrap();
				player.loggers_active.walnut += amount;
				player.update().await;

				Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
			},
			"cherry" => {
				if player.axe < Axe::Adamant {
					return Message::Content("You need an **Adamant** axe to assign loggers to this tree!".to_string());
				}
				let amounts = vec![amount, player.available_loggers()];
				let amount = *amounts.iter().min().unwrap();
				player.loggers_active.cherry += amount;
				player.update().await;

				Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
			},
			"purpleheart" => {
				if player.axe < Axe::Rune {
					return Message::Content("You need a **Rune** axe to assign loggers to this tree!".to_string());
				}
				let amounts = vec![amount, player.available_loggers()];
				let amount = *amounts.iter().min().unwrap();
				player.loggers_active.purpleheart += amount;
				player.update().await;

				Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
			},
			_ => Message::how()
		},
		"lumberers" => match tree.name.as_str() {
			"pine" => {
				Message::under_construction()
			},
			"oak" => {
				Message::under_construction()
			},
			"maple" => {
				Message::under_construction()
			},
			"walnut" => {
				Message::under_construction()
			},
			"cherry" => {
				Message::under_construction()
			},
			"purpleheart" => {
				Message::under_construction()
			},
			_ => Message::how()
		},
		"cncs" => match tree.name.as_str() {
			"pine" => {
				Message::under_construction()
			},
			"oak" => {
				Message::under_construction()
			},
			"maple" => {
				Message::under_construction()
			},
			"walnut" => {
				Message::under_construction()
			},
			"cherry" => {
				Message::under_construction()
			},
			"purpleheart" => {
				Message::under_construction()
			},
			_ => Message::how()
		},
		_ => Message::how()
	}	
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("assign").description("assign loggers, lumberers, and cncs to automate things!")
		.create_option(|option| {
			option
				.name("loggers")
				.description("assign loggers")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("assign loggers to pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("assign loggers to oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("assign loggers to maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("assign loggers to walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("assign loggers to cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("assign loggers to purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
		.create_option(|option| {
			option
				.name("lumberers")
				.description("assign lumberers")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("assign lumberers to pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("assign lumberers to oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("assign lumberers to maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("assign lumberers to walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("assign lumberers to cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("assign lumberers to purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
		.create_option(|option| {
			option
				.name("cncs")
				.description("assign cncs")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("assign cncs to pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("assign cncs to oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("assign cncs to maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("assign cncs to walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("assign cncs to cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("assign cncs to purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
}