use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::get_player;

const PINE_LOG_PRICE: f64 = 1.0;
const PINE_LUMBER_PRICE: f64 = 2.5;
const PINE_BIRDHOUSE_PRICE: f64 = 6.5;
const PINE_SHELF_PRICE: f64 = 16.0;
const PINE_SIDE_TABLE_PRICE: f64 = 27.5;
const PINE_COFFEE_TABLE_PRICE: f64 = 42.0;
const PINE_DINING_SET_PRICE: f64 = 59.0;

const OAK_LOG_PRICE: f64 = 5.0;
const OAK_LUMBER_PRICE: f64 = 13.5;
const OAK_BIRDHOUSE_PRICE: f64 = 40.0;
const OAK_SHELF_PRICE: f64 = 103.5;
const OAK_SIDE_TABLE_PRICE: f64 = 187.0;
const OAK_COFFEE_TABLE_PRICE: f64 = 290.0;
const OAK_DINING_SET_PRICE: f64 = 415.0;

const MAPLE_LOG_PRICE: f64 = 20.0;
const MAPLE_LUMBER_PRICE: f64 = 61.5;
const MAPLE_BIRDHOUSE_PRICE: f64 = 220.5;
const MAPLE_SHELF_PRICE: f64 = 600.0;
const MAPLE_SIDE_TABLE_PRICE: f64 = 1135.0;
const MAPLE_COFFEE_TABLE_PRICE: f64 = 1825.5;
const MAPLE_DINING_SET_PRICE: f64 = 2680.0;

const WALNUT_LOG_PRICE: f64 = 50.0;
const WALNUT_LUMBER_PRICE: f64 = 175.0;
const WALNUT_BIRDHOUSE_PRICE: f64 = 735.0;
const WALNUT_SHELF_PRICE: f64 = 2095.0;
const WALNUT_SIDE_TABLE_PRICE: f64 = 4090.0;
const WALNUT_COFFEE_TABLE_PRICE: f64 = 6701.5;
const WALNUT_DINING_SET_PRICE: f64 = 9955.0;

const CHERRY_LOG_PRICE: f64 = 90.0;
const CHERRY_LUMBER_PRICE: f64 = 370.0;
const CHERRY_BIRDHOUSE_PRICE: f64 = 1900.0;
const CHERRY_SHELF_PRICE: f64 = 5690.0;
const CHERRY_SIDE_TABLE_PRICE: f64 = 11375.0;
const CHERRY_COFFEE_TABLE_PRICE: f64 = 19000.0;
const CHERRY_DINING_SET_PRICE: f64 = 28500.0;

const PURPLEHEART_LOG_PRICE: f64 = 175.0;
const PURPLEHEART_LUMBER_PRICE: f64 = 930.0;
const PURPLEHEART_BIRDHOUSE_PRICE: f64 = 6480.0;
const PURPLEHEART_SHELF_PRICE: f64 = 20600.0;
const PURPLEHEART_SIDE_TABLE_PRICE: f64 = 42324.0;
const PURPLEHEART_COFFEE_TABLE_PRICE: f64 = 71690.0;
const PURPLEHEART_DINING_SET_PRICE: f64 = 109000.0;

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
					let money = PINE_LOG_PRICE * amount as f64;
					player.logs.pine -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** pine logs for **${:.2}**", amount, money)
				},
				"oak" => {
					let amounts = vec![amount, player.logs.oak];
					let amount = *amounts.iter().min().unwrap();
					let money = OAK_LOG_PRICE * amount as f64;
					player.logs.oak -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** oak logs for **${:.2}**", amount, money)
				},
				"maple" => {
					let amounts = vec![amount, player.logs.maple];
					let amount = *amounts.iter().min().unwrap();
					let money = MAPLE_LOG_PRICE * amount as f64;
					player.logs.maple -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** maple logs for **${:.2}**", amount, money)
				},
				"walnut" => {
					let amounts = vec![amount, player.logs.walnut];
					let amount = *amounts.iter().min().unwrap();
					let money = WALNUT_LOG_PRICE * amount as f64;
					player.logs.walnut -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** walnut logs for **${:.2}**", amount, money)
				},
				"cherry" => {
					let amounts = vec![amount, player.logs.cherry];
					let amount = *amounts.iter().min().unwrap();
					let money = CHERRY_LOG_PRICE * amount as f64;
					player.logs.cherry -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** cherry logs for **${:.2}**", amount, money)
				},
				"purpleheart" => {
					let amounts = vec![amount, player.logs.purpleheart];
					let amount = *amounts.iter().min().unwrap();
					let money = PURPLEHEART_LOG_PRICE * amount as f64;
					player.logs.purpleheart -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** purpleheart logs for **${:.2}**", amount, money)
				},
				_ => "No log exists".to_string()
			}
		},
		"lumber" => {
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
					let amounts = vec![amount, player.lumber.pine];
					let amount = *amounts.iter().min().unwrap();
					let money = PINE_LUMBER_PRICE * amount as f64;
					player.lumber.pine -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** pine lumber for **${:.2}**", amount, money)
				},
				"oak" => {
					let amounts = vec![amount, player.lumber.oak];
					let amount = *amounts.iter().min().unwrap();
					let money = OAK_LUMBER_PRICE * amount as f64;
					player.lumber.oak -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** oak lumber for **${:.2}**", amount, money)
				},
				"maple" => {
					let amounts = vec![amount, player.lumber.maple];
					let amount = *amounts.iter().min().unwrap();
					let money = MAPLE_LUMBER_PRICE * amount as f64;
					player.lumber.maple -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** maple lumber for **${:.2}**", amount, money)
				},
				"walnut" => {
					let amounts = vec![amount, player.lumber.walnut];
					let amount = *amounts.iter().min().unwrap();
					let money = WALNUT_LUMBER_PRICE * amount as f64;
					player.lumber.walnut -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** walnut lumber for **${:.2}**", amount, money)
				},
				"cherry" => {
					let amounts = vec![amount, player.lumber.cherry];
					let amount = *amounts.iter().min().unwrap();
					let money = CHERRY_LUMBER_PRICE * amount as f64;
					player.lumber.cherry -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** cherry lumber for **${:.2}**", amount, money)
				},
				"purpleheart" => {
					let amounts = vec![amount, player.lumber.purpleheart];
					let amount = *amounts.iter().min().unwrap();
					let money = PURPLEHEART_LUMBER_PRICE * amount as f64;
					player.lumber.purpleheart -= amount;
					player.cash += money;
					player.update().await;

					format!("You sold **{}** purpleheart lumber for **${:.2}**", amount, money)
				},
				_ => "No log exists".to_string()
			}
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