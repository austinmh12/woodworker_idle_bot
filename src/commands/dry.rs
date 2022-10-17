use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
};

use crate::player::{get_player, Kiln, Player, Action, ActionEnum};
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
	if player.kiln == Kiln::None {
		return "You don't have a kiln! Buy one from the store!".to_string();
	}
	match tree.as_str() {
		// Don't need to check kiln::none since we do it above.
		"pine" => {
			dry_player_update(&mut player, "pine").await
		},
		"oak" => {
			if player.kiln < Kiln::Firebrick {
				return "You need a **Firebrick** kiln to dry oak logs!".to_string();
			}
			dry_player_update(&mut player, "oak").await
		},
		"maple" => {
			if player.kiln < Kiln::Hobby {
				return "You need a **Hobby** kiln to dry maple logs!".to_string();
			}
			dry_player_update(&mut player, "maple").await
		},
		"walnut" => {
			if player.kiln < Kiln::LabGrade {
				return "You need a **Lab Grade** kiln to dry walnut logs!".to_string();
			}
			dry_player_update(&mut player, "walnut").await
		},
		"cherry" => {
			if player.kiln < Kiln::Industrial {
				return "You need an **Industrial** kiln to dry cherry logs!".to_string();
			}
			dry_player_update(&mut player, "cherry").await
		},
		"purpleheart" => {
			if player.kiln < Kiln::WorldWide {
				return "You need a **World Wide** kiln to dry purpleheart logs!".to_string();
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
	let upgrade = player.upgrades.better_temperatures;
	let sawdust_upgrade = player.sawdust_upgrades.better_temperatures;
	
	(base_lumber + upgrade) * (1 + sawdust_upgrade)
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
			player.logs.pine -= 1;
			player.lumber.pine += amount;
			player.stats.pine_logs_dried += 1;
			player.stats.pine_lumber_earned += amount;
			player.sawdust_prestige.lumber.pine += amount;
			player.seed_prestige.lumber.pine += amount;
		},
		"oak" => {
			player.logs.oak -= 1;
			player.lumber.oak += amount;
			player.stats.oak_logs_dried += 1;
			player.stats.oak_lumber_earned += amount;
			player.sawdust_prestige.lumber.oak += amount;
			player.seed_prestige.lumber.oak += amount;
		},
		"maple" => {
			player.logs.maple -= 1;
			player.lumber.maple += amount;
			player.stats.maple_logs_dried += 1;
			player.stats.maple_lumber_earned += amount;
			player.sawdust_prestige.lumber.maple += amount;
			player.seed_prestige.lumber.maple += amount;
		},
		"walnut" => {
			player.logs.walnut -= 1;
			player.lumber.walnut += amount;
			player.stats.walnut_logs_dried += 1;
			player.stats.walnut_lumber_earned += amount;
			player.sawdust_prestige.lumber.walnut += amount;
			player.seed_prestige.lumber.walnut += amount;
		},
		"cherry" => {
			player.logs.cherry -= 1;
			player.lumber.cherry += amount;
			player.stats.cherry_logs_dried += 1;
			player.stats.cherry_lumber_earned += amount;
			player.sawdust_prestige.lumber.cherry += amount;
			player.seed_prestige.lumber.cherry += amount;
		},
		"purpleheart" => {
			player.logs.purpleheart -= 1;
			player.lumber.purpleheart += amount;
			player.stats.purpleheart_logs_dried += 1;
			player.stats.purpleheart_lumber_earned += amount;
			player.sawdust_prestige.lumber.purpleheart += amount;
			player.seed_prestige.lumber.purpleheart += amount;
		},
		_ => ()
	}
}