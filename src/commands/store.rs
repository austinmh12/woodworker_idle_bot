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