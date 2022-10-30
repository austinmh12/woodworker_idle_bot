use std::str::FromStr;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Axe, Player, Action, ActionEnum};
use crate::utils::Message;
use crate::enums::Tree;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> Message {
	let tree_input = &options
		.get(0)
		.expect("Expected a Subcommand");
	let mut actions = if &tree_input.options.len() == &0usize {
		1
	} else {
		match tree_input.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
			CommandDataOptionValue::Integer(i) => i.to_owned(),
			_ => 1
		}
	};
	let mut player = get_player(player_id).await;
	if actions > 5 + player.sawdust_upgrades.endurance_training {
		actions = 5 + player.sawdust_upgrades.endurance_training;
	}

	match player.current_action.action {
		ActionEnum::None => (),
		_ => {
			if player.queued_actions.len() < (player.sawdust_upgrades.multitasking + 2) as usize {
				()
			} else {
				return Message::Content(format!("You're busy for another **{}s**!", player.current_action.time_to_complete()));
			}
		},
	}
	let tree = Tree::from_str(&tree_input.name).expect("No such tree.");
	match tree {
		Tree::Oak => {
			if player.axe < Axe::Iron {
				return Message::Content("You need an **Iron** axe to chop oak logs!".to_string());
			};
		},
		Tree::Maple => {
			if player.axe < Axe::Steel {
				return Message::Content("You need a **Steel** axe to chop maple logs!".to_string());
			};
		},
		Tree::Walnut => {
			if player.axe < Axe::Mithril {
				return Message::Content("You need a **Mithril** axe to chop walnut logs!".to_string());
			};
		},
		Tree::Cherry => {
			if player.axe < Axe::Adamant {
				return Message::Content("You need an **Adamant** axe to chop cherry logs!".to_string());
			};
		},
		Tree::PurpleHeart => {
			if player.axe < Axe::Rune {
				return Message::Content("You need a **Rune** axe to chop purpleheart logs!".to_string());
			};
		},
		_ => ()
	};

	Message::Content(chop_player_update(&mut player, tree, actions).await)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("chop").description("Chop trees for logs!")
		.create_option(|option| {
			option
				.name("pine")
				.description("Chop a pine tree")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to chop")
						.kind(CommandOptionType::Integer)
						.required(false)
						.min_int_value(1)
				})
		})
		.create_option(|option| {
			option
				.name("oak")
				.description("Chop an oak tree")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to chop")
						.kind(CommandOptionType::Integer)
						.required(false)
						.min_int_value(1)
				})
		})
		.create_option(|option| {
			option
				.name("maple")
				.description("Chop a maple tree")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to chop")
						.kind(CommandOptionType::Integer)
						.required(false)
						.min_int_value(1)
				})
		})
		.create_option(|option| {
			option
				.name("walnut")
				.description("Chop a walnut tree")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to chop")
						.kind(CommandOptionType::Integer)
						.required(false)
						.min_int_value(1)
				})
		})
		.create_option(|option| {
			option
				.name("cherry")
				.description("Chop a cherry tree")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to chop")
						.kind(CommandOptionType::Integer)
						.required(false)
						.min_int_value(1)
				})
		})
		.create_option(|option| {
			option
				.name("purpleheart")
				.description("Chop a purpleheart tree")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to chop")
						.kind(CommandOptionType::Integer)
						.required(false)
						.min_int_value(1)
				})
		})
}

fn chop_log(player: &Player, tree: Tree, actions: i64) -> Option<Action> {
	// returns None if insta-chopped.
	let chop_time = get_player_chop_time(player, tree, actions);
	if chop_time == 0 {
		return None;
	} else {
		return Some(Action::chopping(chop_time, tree, actions));
	}
}

pub fn determine_player_logs_earned(player: &Player) -> i64 {
	let base_logs = 1;
	let upgrade = player.upgrades.gym_pass;
	let sawdust_upgrade = player.sawdust_upgrades.less_bark;
	let sawdust = player.sawdust_total; // each is a permanent 1% increase to output
	
	(((base_logs + upgrade) * (1 + sawdust_upgrade)) as f64 * (1.0 + (0.01 * sawdust as f64))) as i64
}

pub fn determine_logger_logs_earned(player: &Player) -> i64 {
	let base_logs = 1;
	let upgrade = player.upgrades.wider_axes;
	let sawdust_upgrade = player.sawdust_upgrades.dual_wielding;
	let sawdust = player.sawdust_total; // each is a permanent 1% increase to output
	
	(((base_logs + upgrade) * (1 + sawdust_upgrade)) as f64 * (1.0 + (0.01 * sawdust as f64))) as i64
}

pub async fn chop_player_update(player: &mut Player, tree: Tree, actions: i64) -> String {
	let action = chop_log(&player, tree, actions);
	match action {
		Some(a) => {
			match player.current_action.action {
				ActionEnum::None => {
					player.current_action = a.clone();
					player.update().await;

					format!("You started chopping a **{}** tree! You'll be done in **{}s**", &tree, a.time_to_complete())
				},
				_ => {
					let queued_action = player.queue_action(a);
					player.update().await;

					format!("You started chopping a **{}** tree! You'll be done in **{}s**", &tree, queued_action.time_to_complete())
				},
			}
		}
		None => {
			let amount = update_player_chop(player);
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

pub fn update_player_chop(player: &mut Player) -> i64 {
	let times = player.current_action.amount;
	let amount = times * determine_player_logs_earned(&player);
	let tree = player.current_action.tree;
	player.current_action = Action::none();
	match tree {
		Tree::Pine => {
			player.logs.pine += amount;
			player.stats.pine_trees_chopped += times;
			player.stats.pine_logs_earned += amount;
			player.sawdust_prestige.logs.pine += amount;
			player.seed_prestige.logs.pine += amount;
		},
		Tree::Oak => {
			player.logs.oak += amount;
			player.stats.oak_trees_chopped += times;
			player.stats.oak_logs_earned += amount;
			player.sawdust_prestige.logs.oak += amount;
			player.seed_prestige.logs.oak += amount;
		},
		Tree::Maple => {
			player.logs.maple += amount;
			player.stats.maple_trees_chopped += times;
			player.stats.maple_logs_earned += amount;
			player.sawdust_prestige.logs.maple += amount;
			player.seed_prestige.logs.maple += amount;
		},
		Tree::Walnut => {
			player.logs.walnut += amount;
			player.stats.walnut_trees_chopped += times;
			player.stats.walnut_logs_earned += amount;
			player.sawdust_prestige.logs.walnut += amount;
			player.seed_prestige.logs.walnut += amount;
		},
		Tree::Cherry => {
			player.logs.cherry += amount;
			player.stats.cherry_trees_chopped += times;
			player.stats.cherry_logs_earned += amount;
			player.sawdust_prestige.logs.cherry += amount;
			player.seed_prestige.logs.cherry += amount;
		},
		Tree::PurpleHeart => {
			player.logs.purpleheart += amount;
			player.stats.purpleheart_trees_chopped += times;
			player.stats.purpleheart_logs_earned += amount;
			player.sawdust_prestige.logs.purpleheart += amount;
			player.seed_prestige.logs.purpleheart += amount;
		}
	}

	amount
}

pub fn get_player_chop_time(player: &Player, tree: Tree, actions: i64) -> i64 {
	let base_time = match tree {
		Tree::Pine => 10.0,
		Tree::Oak => 15.0,
		Tree::Maple => 25.0,
		Tree::Walnut => 35.0,
		Tree::Cherry => 50.0,
		Tree::PurpleHeart => 80.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.sharpening_books as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.tree_fertilizer as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult).round() as i64 * actions
}

pub fn get_logger_chop_time(player: &Player, tree: Tree, actions: i64) -> i64 {
	let base_time = match tree {
		Tree::Pine => 100.0,
		Tree::Oak => 150.0,
		Tree::Maple => 250.0,
		Tree::Walnut => 350.0,
		Tree::Cherry => 500.0,
		Tree::PurpleHeart => 800.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.sharper_axes as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.double_swings as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult).round() as i64 * actions
}