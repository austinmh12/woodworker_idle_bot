use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::get_player;
use crate::utils;

const CASH_SHARPER_AXES_BASE: f64 = 100.0;
const CASH_SHARPER_AXES_EXP: f64 = 1.025;
const CASH_WIDER_AXES_BASE: f64 = 250.0;
const CASH_WIDER_AXES_EXP: f64 = 1.03;
const CASH_HOTTER_KILNS_BASE: f64 = 500.0;
const CASH_HOTTER_KILNS_EXP: f64 = 1.0275;
const CASH_BETTER_TEMPERATURES_BASE: f64 = 750.0;
const CASH_BETTER_TEMPERATURES_EXP: f64 = 1.05;
const CASH_FAST_DRYING_GLUE_BASE: f64 = 1250.0;
const CASH_FAST_DRYING_GLUE_EXP: f64 = 1.075;
const CASH_INDUSTRIAL_NAILS_BASE: f64 = 1800.0;
const CASH_INDUSTRIAL_NAILS_EXP: f64 = 1.095;

const SAWDUST_SHARPER_AXES_BASE: f64 = 100.0;
const SAWDUST_SHARPER_AXES_EXP: f64 = 1.025;
const SAWDUST_WIDER_AXES_BASE: f64 = 250.0;
const SAWDUST_WIDER_AXES_EXP: f64 = 1.03;
const SAWDUST_HOTTER_KILNS_BASE: f64 = 500.0;
const SAWDUST_HOTTER_KILNS_EXP: f64 = 1.0275;
const SAWDUST_BETTER_TEMPERATURES_BASE: f64 = 750.0;
const SAWDUST_BETTER_TEMPERATURES_EXP: f64 = 1.05;
const SAWDUST_FAST_DRYING_GLUE_BASE: f64 = 1250.0;
const SAWDUST_FAST_DRYING_GLUE_EXP: f64 = 1.075;
const SAWDUST_INDUSTRIAL_NAILS_BASE: f64 = 1800.0;
const SAWDUST_INDUSTRIAL_NAILS_EXP: f64 = 1.095;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> (String, Option<CreateEmbed>) {
	// store.view
	// store.buy.slot amount
	let action = &options
		.get(0)
		.expect("Expected a SubcommandGroup");
	let subaction = &action
		.options
		.get(0)
		.expect("Subcommand");
	let mut player = get_player(player_id).await;
	match action.name.as_str() {
		"normal" => match subaction.name.as_str() {
			"view" => {
				let mut ret = CreateEmbed::default();
				let mut desc = "Welcome to the normal upgrades! See something you like, buy it with **/upgrade normal buy**\n".to_string();
				desc.push_str(&format!("You have **${:.2}**\n\n", player.cash));
				desc.push_str(&format!("**1:** Sharper Axes - ${:.2}\n", utils::get_price(1, CASH_SHARPER_AXES_BASE, CASH_SHARPER_AXES_EXP, player.upgrades.sharper_axes)));
				desc.push_str(&format!("**2:** Wider Axes - ${:.2}\n", utils::get_price(1, CASH_WIDER_AXES_BASE, CASH_WIDER_AXES_EXP, player.upgrades.wider_axes)));
				desc.push_str(&format!("**3:** Hotter Kilns - ${:.2}\n", utils::get_price(1, CASH_HOTTER_KILNS_BASE, CASH_HOTTER_KILNS_EXP, player.upgrades.hotter_kilns)));
				desc.push_str(&format!("**4:** Better Temperatures - ${:.2}\n", utils::get_price(1, CASH_BETTER_TEMPERATURES_BASE, CASH_BETTER_TEMPERATURES_EXP, player.upgrades.better_temperatures)));
				desc.push_str(&format!("**5:** Fast Drying Glue - ${:.2}\n", utils::get_price(1, CASH_FAST_DRYING_GLUE_BASE, CASH_FAST_DRYING_GLUE_EXP, player.upgrades.fast_drying_glue)));
				desc.push_str(&format!("**6:** Industrial Nails - ${:.2}\n", utils::get_price(1, CASH_INDUSTRIAL_NAILS_BASE, CASH_INDUSTRIAL_NAILS_EXP, player.upgrades.industrial_nails)));
				ret
					.title("Normal Upgrades")
					.description(&desc)
					.colour(utils::default_colour());

				("".to_string(), Some(ret))
			},
			"buy" => {
				let slot = match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
					CommandDataOptionValue::Integer(i) => i.to_owned(),
					_ => 0,
				};
				if !(1..=6).contains(&slot) {
					return ("Invalid slot".to_string(), None);
				}
				let amount = if &subaction.options.len() == &1usize {
					1
				} else {
					match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
						CommandDataOptionValue::Integer(i) => i.to_owned(),
						_ => 1
					}
				};
				let (count, total_price) = match slot {
					1 => utils::get_max_buyable_amount_and_price(&player, amount, CASH_SHARPER_AXES_BASE, CASH_SHARPER_AXES_EXP, player.upgrades.sharper_axes),
					2 => utils::get_max_buyable_amount_and_price(&player, amount, CASH_WIDER_AXES_BASE, CASH_WIDER_AXES_EXP, player.upgrades.wider_axes),
					3 => utils::get_max_buyable_amount_and_price(&player, amount, CASH_HOTTER_KILNS_BASE, CASH_HOTTER_KILNS_EXP, player.upgrades.hotter_kilns),
					4 => utils::get_max_buyable_amount_and_price(&player, amount, CASH_BETTER_TEMPERATURES_BASE, CASH_BETTER_TEMPERATURES_EXP, player.upgrades.better_temperatures),
					5 => utils::get_max_buyable_amount_and_price(&player, amount, CASH_FAST_DRYING_GLUE_BASE, CASH_FAST_DRYING_GLUE_EXP, player.upgrades.fast_drying_glue),
					6 => utils::get_max_buyable_amount_and_price(&player, amount, CASH_INDUSTRIAL_NAILS_BASE, CASH_INDUSTRIAL_NAILS_EXP, player.upgrades.industrial_nails),
					_ => (0, 0.0) // Can't get here
				};
				if player.cash < total_price || count == 0 {
					return (format!("You need **${:.2}** more to buy that", total_price - player.cash), None);
				}
				player.cash -= total_price;
				let ret = match slot {
					1 => {
						player.upgrades.sharper_axes += count;
	
						(format!("You bought **{}** sharper axes!", &player.axe), None)
					},
					2 => {
						player.upgrades.wider_axes += count;
	
						(format!("You bought **{}** wider axes!", player.kiln), None)
					},
					3 => {
						player.upgrades.hotter_kilns += count;
	
						(format!("You bought **{}** hotter kilns!", &player.hammer), None)
					},
					4 => {
						player.upgrades.better_temperatures += count;
	
						(format!("You bought **{}** better temperatures!", count), None)
					},
					5 => {
						player.upgrades.fast_drying_glue += count;
	
						(format!("You bought **{}** fast drying glues!", count), None)
					},
					6 => {
						player.upgrades.industrial_nails += count;
	
						(format!("You bought **{}** industrial nails!", count), None)
					},
					_ => ("How'd you get here?".to_string(), None)
				};
				// Stats maybe?
				player.update().await;
	
				ret
			},
			_ => ("Wtf?".to_string(), None)
		},
		"sawdust" => match subaction.name.as_str() {
			"view" => {
				("todo".to_string(), None)
			},
			"buy" => {
				let slot = match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
					CommandDataOptionValue::Integer(i) => i.to_owned(),
					_ => 0,
				};
				if !(1..=7).contains(&slot) {
					return ("Invalid slot".to_string(), None);
				}
				let amount = if &subaction.options.len() == &1usize {
					1
				} else {
					match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
						CommandDataOptionValue::Integer(i) => i.to_owned(),
						_ => 1
					}
				};

				("todo".to_string(), None)
			},
			_ => ("Wtf?".to_string(), None)
		},
		"seed" => match subaction.name.as_str() {
			"view" => {
				("todo".to_string(), None)
			},
			"buy" => {
				let slot = match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
					CommandDataOptionValue::Integer(i) => i.to_owned(),
					_ => 0,
				};
				if !(1..=6).contains(&slot) {
					return ("Invalid slot".to_string(), None);
				}
				let amount = if &subaction.options.len() == &1usize {
					1
				} else {
					match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
						CommandDataOptionValue::Integer(i) => i.to_owned(),
						_ => 1
					}
				};

				("todo".to_string(), None)
			},
			_ => ("Wtf?".to_string(), None)
		},
		_ => ("How?".to_string(), None)
	}	
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("upgrade").description("Buy upgrades to improve yourself!")
		.create_option(|option| {
			option
				.name("normal")
				.description("buy normal upgrades!")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("view")
						.description("view the normal upgrades")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("buy")
						.description("buy normal upgrades")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("slot")
								.description("slot to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.max_int_value(9)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("amount to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.required(false)
						})
				})
		})
		.create_option(|option| {
			option
				.name("sawdust")
				.description("buy sawdust upgrades!")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("view")
						.description("view the sawdust upgrades")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("buy")
						.description("buy sawdust upgrades")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("slot")
								.description("slot to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.max_int_value(9)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("amount to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.required(false)
						})
				})
		})
		.create_option(|option| {
			option
				.name("seed")
				.description("buy seed upgrades")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("view")
						.description("view the seed upgrades")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("buy")
						.description("buy seed upgrades")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("slot")
								.description("slot to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.max_int_value(9)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("amount to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.required(false)
						})
				})
		})
}