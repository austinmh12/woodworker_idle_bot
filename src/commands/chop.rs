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
		_ => {
			if player.queued_actions.len() < (player.sawdust_upgrades.multitasking + 2) as usize {
				()
			} else {
				return format!("You're busy for another **{}s**!", player.current_action.time_to_complete());
			}
		},
	}
	match tree.as_str() {
		"pine" => {
			chop_player_update(&mut player, "pine").await
		},
		"oak" => {
			if player.axe < Axe::Iron {
				return "You need an **Iron** axe to chop oak logs!".to_string();
			}
			chop_player_update(&mut player, "oak").await
		},
		"maple" => {
			if player.axe < Axe::Steel {
				return "You need a **Steel** axe to chop maple logs!".to_string();
			}
			chop_player_update(&mut player, "maple").await
		},
		"walnut" => {
			if player.axe < Axe::Mithril {
				return "You need a **Mithril** axe to chop walnut logs!".to_string();
			}
			chop_player_update(&mut player, "walnut").await
		},
		"cherry" => {
			if player.axe < Axe::Adamant {
				return "You need an **Adamant** axe to chop cherry logs!".to_string();
			}
			chop_player_update(&mut player, "cherry").await
		},
		"purpleheart" => {
			if player.axe < Axe::Rune {
				return "You need a **Rune** axe to chop purpleheart logs!".to_string();
			}
			chop_player_update(&mut player, "purpleheart").await
		},
		_ => "No such tree".to_string()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("chop").description("Chop trees for logs!")
		.create_option(|option| {
			option
				.name("pine")
				.description("Chop a pine tree")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("oak")
				.description("Chop an oak tree")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("maple")
				.description("Chop a maple tree")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("walnut")
				.description("Chop a walnut tree")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("cherry")
				.description("Chop a cherry tree")
				.kind(CommandOptionType::SubCommand)	
		})
		.create_option(|option| {
			option
				.name("purpleheart")
				.description("Chop a purpleheart tree")
				.kind(CommandOptionType::SubCommand)	
		})
}

fn chop_log(player: &Player, tree: &str) -> Option<Action> {
	// returns None if insta-chopped.
	let chop_time = utils::get_tree_time(player, tree);
	if chop_time == 0 {
		return None;
	} else {
		return Some(Action::chopping(chop_time, tree));
	}
}

pub fn determine_logs_earned(player: &Player) -> i64 {
	let base_logs = 1;
	let upgrade = player.upgrades.wider_axes;
	let sawdust_upgrade = player.sawdust_upgrades.wider_axes;
	let sawdust = player.sawdust_total; // each is a permanent 1% increase to output
	
	(((base_logs + upgrade) * (1 + sawdust_upgrade)) as f64 * (1.0 + (0.01 * sawdust as f64))) as i64
}

pub async fn chop_player_update(player: &mut Player, tree: &str) -> String {
	let action = chop_log(&player, tree);
	match action {
		Some(a) => {
			match player.current_action.action {
				ActionEnum::None => {
					player.current_action = a.clone();
					player.update().await;

					format!("You started chopping a **{}** tree! You'll be done in **{}s**", tree, a.time_to_complete())
				},
				_ => {
					let queued_action = player.queue_action(a);
					player.update().await;

					format!("You started chopping a **{}** tree! You'll be done in **{}s**", tree, queued_action.time_to_complete())
				},
			}
		}
		None => {
			let amount = determine_logs_earned(&player);
			update_player_chop(player, amount, tree);
			player.update().await;
			let s = if amount != 1 {
				"s"
			} else {
				""
			};
			format!("You chopped **{} {}** log{}!", amount, tree, s)
		}
	}
}

pub fn update_player_chop(player: &mut Player, amount: i64, tree: &str) {
	player.current_action = Action::none();
	match tree {
		"pine" => {
			player.logs.pine += amount;
			player.stats.pine_trees_chopped += 1;
			player.stats.pine_logs_earned += amount;
			player.sawdust_prestige.logs.pine += amount;
			player.seed_prestige.logs.pine += amount;
		},
		"oak" => {
			player.logs.oak += amount;
			player.stats.oak_trees_chopped += 1;
			player.stats.oak_logs_earned += amount;
			player.sawdust_prestige.logs.oak += amount;
			player.seed_prestige.logs.oak += amount;
		},
		"maple" => {
			player.logs.maple += amount;
			player.stats.maple_trees_chopped += 1;
			player.stats.maple_logs_earned += amount;
			player.sawdust_prestige.logs.maple += amount;
			player.seed_prestige.logs.maple += amount;
		},
		"walnut" => {
			player.logs.walnut += amount;
			player.stats.walnut_trees_chopped += 1;
			player.stats.walnut_logs_earned += amount;
			player.sawdust_prestige.logs.walnut += amount;
			player.seed_prestige.logs.walnut += amount;
		},
		"cherry" => {
			player.logs.cherry += amount;
			player.stats.cherry_trees_chopped += 1;
			player.stats.cherry_logs_earned += amount;
			player.sawdust_prestige.logs.cherry += amount;
			player.seed_prestige.logs.cherry += amount;
		},
		"purpleheart" => {
			player.logs.purpleheart += amount;
			player.stats.purpleheart_trees_chopped += 1;
			player.stats.purpleheart_logs_earned += amount;
			player.sawdust_prestige.logs.purpleheart += amount;
			player.seed_prestige.logs.purpleheart += amount;
		},
		_ => ()
	}
}