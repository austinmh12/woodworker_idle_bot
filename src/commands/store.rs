use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Player, Axe, Kiln, Hammer, Tree, BPUnlock};
use crate::utils;

const LOGGER_BASE: f64 = 25.0;
const LOGGER_EXP: f64 = 1.01;
const LUMBERER_BASE: f64 = 150.0;
const LUMBERER_EXP: f64 = 1.03;
const CNC_BASE: f64 = 1000.0;
const CNC_EXP: f64 = 1.07;

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
			desc.push_str(&format!("**1:** Loggers - ${:.2}\n", utils::get_price(1, LOGGER_BASE, LOGGER_EXP, player.loggers)));
			desc.push_str(&format!("**2:** Lumberers - ${:.2}\n", utils::get_price(1, LUMBERER_BASE, LUMBERER_EXP, player.lumberers)));
			desc.push_str(&format!("**3:** CNCs - ${:.2}\n", utils::get_price(1, CNC_BASE, CNC_EXP, player.cncs)));
			if player.axe != Axe::Rune {
				desc.push_str(&format!("**4:** {} Axe - ${:.2}\n", get_next_axe(&player), get_axe_price(get_next_axe(&player))));
			}
			if player.kiln != Kiln::WorldWide {
				desc.push_str(&format!("**5:** {} Kiln - ${:.2}\n", get_next_kiln(&player), get_kiln_price(get_next_kiln(&player))));
			}
			if player.hammer != Hammer::Rune {
				desc.push_str(&format!("**6:** {} Hammer - ${:.2}\n", get_next_hammer(&player), get_hammer_price(get_next_hammer(&player))));
			}
			if player.blueprints.next_unlock() != None {
				desc.push_str(&format!("**7:** {} - ${:.2}\n", get_next_blueprint(&player).unwrap(), get_blueprint_price(get_next_blueprint(&player).unwrap())))
			}
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
				1 => utils::get_max_buyable_amount_and_price(&player, amount, LOGGER_BASE, LOGGER_EXP, player.loggers),
				2 => utils::get_max_buyable_amount_and_price(&player, amount, LUMBERER_BASE, LUMBERER_EXP, player.lumberers),
				3 => utils::get_max_buyable_amount_and_price(&player, amount, CNC_BASE, CNC_EXP, player.cncs),
				4 => (1, get_axe_price(get_next_axe(&player))),
				5 => (1, get_kiln_price(get_next_kiln(&player))),
				6 => (1, get_hammer_price(get_next_hammer(&player))),
				7 => (1, get_blueprint_price(get_next_blueprint(&player).unwrap())),
				_ => (0, 0.0) // Can't get here
			};
			if player.cash < total_price || count == 0 {
				return (format!("You need **${:.2}** more to buy that", total_price - player.cash), None);
			}
			player.cash -= total_price;
			let ret = match slot {
				1 => {
					player.loggers += count;
					
					(format!("You bought **{}** loggers!", count), None)
				},
				2 => {
					player.lumberers += count;
					
					(format!("You bought **{}** lumberers!", count), None)
				},
				3 => {
					player.cncs += count;
					
					(format!("You bought **{}** CNCs!", count), None)
				},
				4 => {
					player.axe = get_next_axe(&player);

					(format!("You bought the **{}** axe!", &player.axe), None)
				},
				5 => {
					player.kiln = get_next_kiln(&player);

					(format!("You bought the **{}** kiln!", player.kiln), None)
				},
				6 => {
					player.hammer = get_next_hammer(&player);

					(format!("You bought the **{}** hammer!", &player.hammer), None)
				},
				7 => {
					let bp = player.unlock_next_blueprint();
					match bp {
						Some(t) => (format!("You bought the **{}** blueprint!", t), None),
						None => ("How'd you get here?".to_string(), None)
					}
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
						.max_int_value(7)
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
		Axe::Iron => 650.0,
		Axe::Steel => 7500.0,
		Axe::Mithril => 67000.0,
		Axe::Adamant => 318500.0,
		Axe::Rune => 1111500.0,
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
		Kiln::SteelBucket => 5.0,
		Kiln::Firebrick => 710.0,
		Kiln::Hobby => 7900.0,
		Kiln::LabGrade => 69690.0,
		Kiln::Industrial => 330000.0,
		Kiln::WorldWide => 1140000.0,
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
		Hammer::Stone => 15.0,
		Hammer::Iron => 770.0,
		Hammer::Steel => 8300.0,
		Hammer::Mithril => 72360.0,
		Hammer::Adamant => 339000.0,
		Hammer::Rune => 1168500.0,
	}
}

fn get_next_blueprint(player: &Player) -> Option<Tree> {
	player.blueprints.next_unlock()
}

fn get_blueprint_price(tree: Tree) -> f64 {
	match tree {
		Tree::Pine(b) => match b {
			BPUnlock::BirdHouse => 0.0,
			BPUnlock::Shelf => 45.5,
			BPUnlock::SideTable => 128.0,
			BPUnlock::CoffeeTable => 247.5,
			BPUnlock::DiningSet => 420.0,
			_ => 0.0
		},
		Tree::Oak(b) => match b {
			BPUnlock::BirdHouse => 767.0,
			BPUnlock::Shelf => 826.0,
			BPUnlock::SideTable => 1552.5,
			BPUnlock::CoffeeTable => 2992.0,
			BPUnlock::DiningSet => 4930.0,
			_ => 0.0
		},
		Tree::Maple(b) => match b {
			BPUnlock::BirdHouse => 8300.0,
			BPUnlock::Shelf => 8715.0,
			BPUnlock::SideTable => 13200.0,
			BPUnlock::CoffeeTable => 26105.0,
			BPUnlock::DiningSet => 43812.0,
			_ => 0.0
		},
		Tree::Walnut(b) => match b {
			BPUnlock::BirdHouse => 72360.0,
			BPUnlock::Shelf => 75040.0,
			BPUnlock::SideTable => 77720.0,
			BPUnlock::CoffeeTable => 122700.0,
			BPUnlock::DiningSet => 207746.5,
			_ => 0.0
		},
		Tree::Cherry(b) => match b {
			BPUnlock::BirdHouse => 338470.0,
			BPUnlock::Shelf => 348425.0,
			BPUnlock::SideTable => 358380.0,
			BPUnlock::CoffeeTable => 420875.0,
			BPUnlock::DiningSet => 722000.0,
			_ => 0.0
		},
		Tree::PurpleHeart(b) => match b {
			BPUnlock::BirdHouse => 1168500.0,
			BPUnlock::Shelf => 1197000.0,
			BPUnlock::SideTable => 1225500.0,
			BPUnlock::CoffeeTable => 1862256.0,
			BPUnlock::DiningSet => 3226050.0,
			_ => 0.0
		},
	}
}