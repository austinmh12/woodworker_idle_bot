use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Player, Axe, Kiln, Hammer};
use crate::utils;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> (String, Option<CreateEmbed>) {
	// store.view
	// store.buy.slot amount
	let action = &options
		.get(0)
		.expect("Expected a SubcommandGroup");
	let mut player = get_player(player_id).await;
	match action.name.as_str() {
		"view" => {
			let mut ret = CreateEmbed::default();
			let mut desc = "Welcome to the store! See something you like, buy it with **/store buy**\n".to_string();
			desc.push_str(&format!("You have **${:.2}**\n\n", player.cash));
			if player.axe != Axe::Rune {
				desc.push_str(&format!("**1:** {} Axe - ${:.2}\n", get_next_axe(&player), get_axe_price(get_next_axe(&player))));
			}
			if player.kiln != Kiln::WorldWide {
				desc.push_str(&format!("**2:** {} Kiln - ${:.2}\n", get_next_kiln(&player), get_kiln_price(get_next_kiln(&player))));
			}
			if player.hammer != Hammer::Rune {
				desc.push_str(&format!("**3:** {} Hammer - ${:.2}\n", get_next_hammer(&player), get_hammer_price(get_next_hammer(&player))));
			}
			desc.push_str(&format!("**4:** Loggers - ${:.2}\n", get_logger_price(&player)));
			desc.push_str(&format!("**5:** Lumberers - ${:.2}\n", get_lumberer_price(&player)));
			desc.push_str(&format!("**6:** CNCs - ${:.2}\n", get_cnc_price(&player)));
			ret
				.title("Store")
				.description(&desc)
				.colour(utils::default_colour());

			("".to_string(), Some(ret))
		},
		"buy" => {
			let slot = match action.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
				CommandDataOptionValue::Integer(i) => i.to_owned(),
				_ => 0,
			};
			if !(1..=6).contains(&slot) {
				return ("Invalid slot".to_string(), None);
			}
			let amount = if &action.options.len() == &1usize {
				1
			} else {
				match action.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
					CommandDataOptionValue::Integer(i) => i.to_owned(),
					_ => 1
				}
			};
			let (count, total_price) = match slot {
				1 => (1, get_axe_price(get_next_axe(&player))),
				2 => (1, get_kiln_price(get_next_kiln(&player))),
				3 => (1, get_hammer_price(get_next_hammer(&player))),
				4 => get_total_price(amount, &player, &get_logger_price),
				5 => get_total_price(amount, &player, &get_lumberer_price),
				6 => get_total_price(amount, &player, &get_cnc_price),
				_ => (0, 0.0) // Can't get here
			};
			if player.cash < total_price || count == 0 {
				return (format!("You need **${:.2}** more to buy that", total_price - player.cash), None);
			}
			player.cash -= total_price;
			let ret = match slot {
				1 => {
					player.axe = get_next_axe(&player);

					(format!("You bought the **{}** axe!", &player.axe), None)
				},
				2 => {
					player.kiln = get_next_kiln(&player);

					(format!("You bought the **{}** kiln!", player.kiln), None)
				},
				3 => {
					player.hammer = get_next_hammer(&player);

					(format!("You bought the **{}** hammer!", &player.hammer), None)
				},
				4 => {
					player.loggers += count;

					(format!("You bought **{}** loggers!", count), None)
				},
				5 => {
					player.lumberers += count;

					(format!("You bought **{}** lumberers!", count), None)
				},
				6 => {
					player.cncs += count;

					(format!("You bought **{}** CNCs!", count), None)
				},
				_ => ("How'd you get here?".to_string(), None)
			};
			// Stats maybe?
			player.update().await;

			ret
		},
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

fn get_logger_price(player: &Player) -> f64 {
	25.0 * f64::powi(1.01, player.loggers as i32)
}

fn get_lumberer_price(player: &Player) -> f64 {
	50.0 * f64::powi(1.03, player.lumberers as i32)
}

fn get_cnc_price(player: &Player) -> f64 {
	100.0 * f64::powi(1.07, player.cncs as i32)
}

fn get_next_axe(player: &Player) -> Axe {
	match player.axe {
		Axe::Stone => Axe::Iron,
		Axe::Iron => Axe::Steel,
		Axe::Steel => Axe::Mithril,
		Axe::Mithril => Axe::Adamant,
		Axe::Adamant => Axe::Rune,
		Axe::Rune => Axe::Rune,
	}
}

fn get_axe_price(axe: Axe) -> f64 {
	match axe {
		Axe::Stone => 0.0,
		Axe::Iron => 100.0,
		Axe::Steel => 1000.0,
		Axe::Mithril => 10000.0,
		Axe::Adamant => 100000.0,
		Axe::Rune => 1000000.0,
	}
}

fn get_next_kiln(player: &Player) -> Kiln {
	match player.kiln {
		Kiln::None => Kiln::SteelBucket,
		Kiln::SteelBucket => Kiln::Firebrick,
		Kiln::Firebrick => Kiln::Hobby,
		Kiln::Hobby => Kiln::LabGrade,
		Kiln::LabGrade => Kiln::Industrial,
		Kiln::Industrial => Kiln::WorldWide,
		Kiln::WorldWide => Kiln::WorldWide,
	}
}

fn get_kiln_price(kiln: Kiln) -> f64 {
	match kiln {
		Kiln::None => 0.0,
		Kiln::SteelBucket => 10.0,
		Kiln::Firebrick => 500.0,
		Kiln::Hobby => 5000.0,
		Kiln::LabGrade => 50000.0,
		Kiln::Industrial => 500000.0,
		Kiln::WorldWide => 5000000.0,
	}
}

fn get_next_hammer(player: &Player) -> Hammer {
	match player.hammer {
		Hammer::None => Hammer::Stone,
		Hammer::Stone => Hammer::Iron,
		Hammer::Iron => Hammer::Steel,
		Hammer::Steel => Hammer::Mithril,
		Hammer::Mithril => Hammer::Adamant,
		Hammer::Adamant => Hammer::Rune,
		Hammer::Rune => Hammer::Rune,
	}
}

fn get_hammer_price(hammer: Hammer) -> f64 {
	match hammer {
		Hammer::None => 0.0,
		Hammer::Stone => 25.0,
		Hammer::Iron => 250.0,
		Hammer::Steel => 2500.0,
		Hammer::Mithril => 25000.0,
		Hammer::Adamant => 250000.0,
		Hammer::Rune => 2500000.0,
	}
}

fn get_total_price(amount: i64, player: &Player, f: &dyn Fn(&Player) -> f64) -> (i64, f64) {
	let mut total_price = 0.0; // There's probably a better way to calculate this
	let mut count = 0;
	let mut player_clone = player.clone();
	for i in 0..amount {
		let next_cost = f(&player_clone);
		if total_price + next_cost > player.cash {
			if i == 0 {
				// So we know how much the player needs
				return (i, total_price + next_cost);
			}
			return (i, total_price);
		}
		total_price += next_cost;
		count = i;

		// Just add one to all of them so we can get the next price
		player_clone.loggers += 1;
		player_clone.lumberers += 1;
		player_clone.cncs += 1;
	}

	(count, total_price)
}