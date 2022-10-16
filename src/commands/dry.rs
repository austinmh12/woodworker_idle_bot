use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
};

use crate::player::{get_player, Axe, Player, Action, ActionEnum};
use crate::utils;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> String {
	let tree = &options
		.get(0)
		.expect("Expected a Subcommand")
		.name;

	let mut player = get_player(player_id).await;
	match player.current_action.action {
		ActionEnum::None => (),
		_ => return format!("You're busy for another **{}s**!", player.current_action.time_to_complete()),
	}
	// TODO: Check if the player has available kilns
	match tree.as_str() {
		"pine" => {
			dry_player_update(&mut player, "pine").await
		},
		"oak" => {
			if player.axe < Axe::Iron {
				return "You need an **Iron** axe to chop oak logs!".to_string();
			}
			dry_player_update(&mut player, "oak").await
		},
		"maple" => {
			if player.axe < Axe::Steel {
				return "You need an **Steel** axe to chop maple logs!".to_string();
			}
			dry_player_update(&mut player, "maple").await
		},
		"walnut" => {
			if player.axe < Axe::Mithril {
				return "You need an **Mithril** axe to chop walnut logs!".to_string();
			}
			dry_player_update(&mut player, "walnut").await
		},
		"cherry" => {
			if player.axe < Axe::Adamant {
				return "You need an **Adamant** axe to chop cherry logs!".to_string();
			}
			dry_player_update(&mut player, "cherry").await
		},
		"purpleheart" => {
			if player.axe < Axe::Rune {
				return "You need an **Rune** axe to chop purpleheart logs!".to_string();
			}
			dry_player_update(&mut player, "purpleheart").await
		},
		_ => "No such tree".to_string()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("dry").description("Dry logs for lumber!")
		.create_option(|option| {
			option
				.name("pine")
				.description("Dry a pine log")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("oak")
				.description("Dry an oak log")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("maple")
				.description("Dry a maple log")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("walnut")
				.description("Dry a walnut log")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("cherry")
				.description("Dry a cherry log")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("purpleheart")
				.description("Dry a purpleheart log")
				.kind(CommandOptionType::SubCommand)	
		})
}

fn dry_log(player: &Player, tree: &str) -> Option<Action> {
	// returns None if insta-dried.
	let dry_time = utils::get_dry_time(player, tree);
	if dry_time == 0 {
		return None;
	} else {
		return Some(Action::drying(dry_time, tree));
	}
}

pub fn determine_lumber_earned(player: &Player) -> i64 {
	let base_lumber = 1;
	// TODO: Update to the kiln upgrades
	let upgrade = player.upgrades.wider_axes;
	let sawdust_upgrade = player.sawdust_upgrades.wider_axes;
	
	base_lumber + upgrade + sawdust_upgrade
}

pub async fn dry_player_update(player: &mut Player, tree: &str) -> String {
	let action = dry_log(&player, tree);
	match action {
		Some(a) => {
			player.current_action = a.clone();
			player.update().await;
			
			format!("You started drying a **pine** log! You'll be done in **{}s**", a.time_to_complete())
		}
		None => {
			let amount = determine_lumber_earned(&player);
			update_player_dry(player, amount, tree);
			player.update().await;
			format!("You dried **{} {}** lumber!", amount, tree)
		}
	}
}

pub fn update_player_dry(player: &mut Player, amount: i64, tree: &str) {
	player.current_action = Action::none();
	match tree {
		"pine" => {
			player.lumber.pine += amount;
			player.stats.pine_logs_dried += 1;
			player.stats.pine_lumber_earned += amount;
		},
		"oak" => {
			player.lumber.oak += amount;
			player.stats.oak_logs_dried += 1;
			player.stats.oak_lumber_earned += amount;
		},
		"maple" => {
			player.lumber.maple += amount;
			player.stats.maple_logs_dried += 1;
			player.stats.maple_lumber_earned += amount;
		},
		"walnut" => {
			player.lumber.walnut += amount;
			player.stats.walnut_logs_dried += 1;
			player.stats.walnut_lumber_earned += amount;
		},
		"cherry" => {
			player.lumber.cherry += amount;
			player.stats.cherry_logs_dried += 1;
			player.stats.cherry_lumber_earned += amount;
		},
		"purpleheart" => {
			player.lumber.purpleheart += amount;
			player.stats.purpleheart_logs_dried += 1;
			player.stats.purpleheart_lumber_earned += amount;
		},
		_ => ()
	}
}