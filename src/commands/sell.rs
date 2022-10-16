use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::get_player;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> String {
	// sell.logs.pine.amount
	// sell.lumber.pine.amount
	// sell.furniture.pine.birdhouse.amount
	let action = &options
		.get(0)
		.expect("Expected a SubcommandGroup");
	let mut player = get_player(player_id).await;
	match action.name.as_str() {
		"logs" => {
			let log_type = &action.options.get(0).expect("Expected a subcommand");
			let amount = if &log_type.options.len() == &0usize {
				1
			} else {
				match log_type.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
					CommandDataOptionValue::Integer(i) => i.to_owned(),
					_ => 1
				}
			};
			match log_type.name.as_str() {
				"pine" => {
					let amounts = vec![amount, player.logs.pine];
					let amount = *amounts.iter().min().unwrap();
					player.logs.pine -= amount;
					player.cash += amount as f64;
					player.update().await;

					format!("You sold **{}** pine logs for **${:.2}**", amount, amount as f64)
				},
				"oak" => {
					let amounts = vec![amount, player.logs.oak];
					let amount = *amounts.iter().min().unwrap();
					player.logs.oak -= amount;
					player.cash += amount as f64;
					player.update().await;

					format!("You sold **{}** oak logs for **${:.2}**", amount, amount as f64)
				},
				"maple" => {
					let amounts = vec![amount, player.logs.maple];
					let amount = *amounts.iter().min().unwrap();
					player.logs.maple -= amount;
					player.cash += amount as f64;
					player.update().await;

					format!("You sold **{}** maple logs for **${:.2}**", amount, amount as f64)
				},
				"walnut" => {
					let amounts = vec![amount, player.logs.walnut];
					let amount = *amounts.iter().min().unwrap();
					player.logs.walnut -= amount;
					player.cash += amount as f64;
					player.update().await;

					format!("You sold **{}** walnut logs for **${:.2}**", amount, amount as f64)
				},
				"cherry" => {
					let amounts = vec![amount, player.logs.cherry];
					let amount = *amounts.iter().min().unwrap();
					player.logs.cherry -= amount;
					player.cash += amount as f64;
					player.update().await;

					format!("You sold **{}** cherry logs for **${:.2}**", amount, amount as f64)
				},
				"purpleheart" => {
					let amounts = vec![amount, player.logs.purpleheart];
					let amount = *amounts.iter().min().unwrap();
					player.logs.purpleheart -= amount;
					player.cash += amount as f64;
					player.update().await;

					format!("You sold **{}** purpleheart logs for **${:.2}**", amount, amount as f64)
				},
				_ => "No log exists".to_string()
			}
		},
		"lumber" => {
			"todo".to_string()
		},
		"furniture" => {
			"todo".to_string()
		},
		_ => "No such action".to_string()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("sell").description("Sell your logs, lumber, or furniture!")
		.create_option(|option| {
			option
				.name("logs")
				.description("sell your logs!")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("Sell pine lumber")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("The amount to sell")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(0)
						})
				})	
		})
		.create_option(|option| {
			option
				.name("lumber")
				.description("sell your lumber!")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("Sell pine lumber")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("The amount to sell")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})	
		})
}