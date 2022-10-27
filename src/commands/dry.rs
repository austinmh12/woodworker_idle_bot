use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Kiln, Player, Action, ActionEnum};
use crate::utils::Message;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> Message {
	let tree = &options
		.get(0)
		.expect("Expected a Subcommand");
	let mut actions = if &tree.options.len() == &0usize {
		1
	} else {
		match tree.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
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
	if player.kiln == Kiln::None {
		return Message::Content("You don't have a kiln! Buy one from the store!".to_string());
	}
	match tree.name.as_str() {
		// Don't need to check kiln::none since we do it above.
		"pine" => {
			if player.logs.pine + player.queued_logs("pine") < actions {
				return Message::Content("You don't have enough pine logs!".to_string())
			}
			Message::Content(dry_player_update(&mut player, "pine", actions).await)
		},
		"oak" => {
			if player.logs.oak + player.queued_logs("oak") < actions {
				return Message::Content("You don't have enough oak logs!".to_string())
			}
			if player.kiln < Kiln::Firebrick {
				return Message::Content("You need a **Firebrick** kiln to dry oak logs!".to_string());
			}
			Message::Content(dry_player_update(&mut player, "oak", actions).await)
		},
		"maple" => {
			if player.logs.maple + player.queued_logs("maple") < actions {
				return Message::Content("You don't have enough maple logs!".to_string())
			}
			if player.kiln < Kiln::Hobby {
				return Message::Content("You need a **Hobby** kiln to dry maple logs!".to_string());
			}
			Message::Content(dry_player_update(&mut player, "maple", actions).await)
		},
		"walnut" => {
			if player.logs.walnut + player.queued_logs("walnut") < actions {
				return Message::Content("You don't have enough walnut logs!".to_string())
			}
			if player.kiln < Kiln::LabGrade {
				return Message::Content("You need a **Lab Grade** kiln to dry walnut logs!".to_string());
			}
			Message::Content(dry_player_update(&mut player, "walnut", actions).await)
		},
		"cherry" => {
			if player.logs.cherry + player.queued_logs("cherry") < actions {
				return Message::Content("You don't have enough cherry logs!".to_string())
			}
			if player.kiln < Kiln::Industrial {
				return Message::Content("You need an **Industrial** kiln to dry cherry logs!".to_string());
			}
			Message::Content(dry_player_update(&mut player, "cherry", actions).await)
		},
		"purpleheart" => {
			if player.logs.purpleheart + player.queued_logs("purpleheart") < actions {
				return Message::Content("You don't have enough purpleheart logs!".to_string())
			}
			if player.kiln < Kiln::WorldWide {
				return Message::Content("You need a **World Wide** kiln to dry purpleheart logs!".to_string());
			}
			Message::Content(dry_player_update(&mut player, "purpleheart", actions).await)
		},
		_ => Message::how()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("dry").description("Dry logs for lumber!")
		.create_option(|option| {
			option
				.name("pine")
				.description("Dry a pine log")
				.kind(CommandOptionType::SubCommand).create_sub_option(|sub| {
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
				.description("Dry an oak log")
				.kind(CommandOptionType::SubCommand).create_sub_option(|sub| {
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
				.description("Dry a maple log")
				.kind(CommandOptionType::SubCommand).create_sub_option(|sub| {
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
				.description("Dry a walnut log")
				.kind(CommandOptionType::SubCommand).create_sub_option(|sub| {
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
				.description("Dry a cherry log")
				.kind(CommandOptionType::SubCommand).create_sub_option(|sub| {
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
				.description("Dry a purpleheart log")
				.kind(CommandOptionType::SubCommand).create_sub_option(|sub| {
					sub
						.name("amount")
						.description("amount to chop")
						.kind(CommandOptionType::Integer)
						.required(false)
						.min_int_value(1)
				})	
		})
}

fn dry_log(player: &Player, tree: &str, actions: i64) -> Option<Action> {
	// returns None if insta-dried.
	let dry_time = get_player_dry_time(player, tree, actions);
	if dry_time == 0 {
		return None;
	} else {
		return Some(Action::drying(dry_time, tree, actions));
	}
}

pub fn determine_player_lumber_earned(player: &Player) -> i64 {
	let base_lumber = 1;
	let upgrade = player.upgrades.pull_carts;
	let sawdust_upgrade = player.sawdust_upgrades.efficient_packing;
	
	(base_lumber + upgrade) * (1 + sawdust_upgrade)
}

pub fn determine_lumberer_lumber_earned(player: &Player) -> i64 {
	let base_lumber = 1;
	let upgrade = player.upgrades.better_temperatures;
	let sawdust_upgrade = player.sawdust_upgrades.reading_glasses;
	
	(base_lumber + upgrade) * (1 + sawdust_upgrade)
}

pub async fn dry_player_update(player: &mut Player, tree: &str, actions: i64) -> String {
	let action = dry_log(&player, tree, actions);
	match action {
		Some(a) => {
			match player.current_action.action {
				ActionEnum::None => {
					player.current_action = a.clone();
					player.update().await;

					format!("You started drying a **{}** log! You'll be done in **{}s**", tree, a.time_to_complete())
				},
				_ => {
					let queued_action = player.queue_action(a);
					player.update().await;

					format!("You started drying a **{}** log! You'll be done in **{}s**", tree, queued_action.time_to_complete())
				},
			}
		}
		None => {
			let amount = update_player_dry(player);
			player.update().await;
			format!("You dried **{} {}** lumber!", amount, tree)
		}
	}
}

pub fn update_player_dry(player: &mut Player) -> i64 {
	let times = player.current_action.amount;
	let amount = times * determine_player_lumber_earned(&player);
	let tree = player.current_action.tree.clone();
	player.current_action = Action::none();
	match tree.as_str() {
		"pine" => {
			player.logs.pine -= times;
			player.lumber.pine += amount;
			player.stats.pine_logs_dried += times;
			player.stats.pine_lumber_earned += amount;
			player.sawdust_prestige.lumber.pine += amount;
			player.seed_prestige.lumber.pine += amount;
		},
		"oak" => {
			player.logs.oak -= times;
			player.lumber.oak += amount;
			player.stats.oak_logs_dried += times;
			player.stats.oak_lumber_earned += amount;
			player.sawdust_prestige.lumber.oak += amount;
			player.seed_prestige.lumber.oak += amount;
		},
		"maple" => {
			player.logs.maple -= times;
			player.lumber.maple += amount;
			player.stats.maple_logs_dried += times;
			player.stats.maple_lumber_earned += amount;
			player.sawdust_prestige.lumber.maple += amount;
			player.seed_prestige.lumber.maple += amount;
		},
		"walnut" => {
			player.logs.walnut -= times;
			player.lumber.walnut += amount;
			player.stats.walnut_logs_dried += times;
			player.stats.walnut_lumber_earned += amount;
			player.sawdust_prestige.lumber.walnut += amount;
			player.seed_prestige.lumber.walnut += amount;
		},
		"cherry" => {
			player.logs.cherry -= times;
			player.lumber.cherry += amount;
			player.stats.cherry_logs_dried += times;
			player.stats.cherry_lumber_earned += amount;
			player.sawdust_prestige.lumber.cherry += amount;
			player.seed_prestige.lumber.cherry += amount;
		},
		"purpleheart" => {
			player.logs.purpleheart -= times;
			player.lumber.purpleheart += amount;
			player.stats.purpleheart_logs_dried += times;
			player.stats.purpleheart_lumber_earned += amount;
			player.sawdust_prestige.lumber.purpleheart += amount;
			player.seed_prestige.lumber.purpleheart += amount;
		},
		_ => ()
	}

	amount
}

pub fn get_player_dry_time(player: &Player, tree: &str, actions: i64) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 25.0,
		"walnut" => 35.0,
		"cherry" => 50.0,
		"purpleheart" => 80.0,
		_ => 10.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.thermodynamics as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.preheating as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult).round() as i64 * actions
}

pub fn get_lumberer_dry_time(player: &Player, tree: &str, actions: i64) -> i64 {
	let base_time = match tree {
		"pine" => 100.0,
		"oak" => 150.0,
		"maple" => 250.0,
		"walnut" => 350.0,
		"cherry" => 500.0,
		"purpleheart" => 800.0,
		_ => 100.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.hotter_kilns as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.electric_heaters as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult).round() as i64 * actions
}