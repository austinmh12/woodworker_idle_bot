use std::sync::Arc;
use serenity::prelude::Context;
use chrono::{
	DateTime,
	Utc
};

use crate::player::{get_players, Player};
use crate::utils;

pub async fn offline_progression(_ctx: Arc<Context>) {
	let players = get_players().await;
	for mut player in players {
		update_logging(&mut player);
		update_lumbering(&mut player);
		update_cncing(&mut player);
		player.update().await;
	}
}

fn time_from(dt: DateTime<Utc>) -> i64 {
	(Utc::now() - dt).num_seconds()
}

fn loops_in_time(time: i64, interval: i64) -> i64 {
	if interval == 0 {
		time
	} else {
		time / interval
	}
}

fn update_logging(player: &mut Player) {
	if player.assigned_loggers() == 0 {
		return;
	}
	if player.loggers_active.pine > 0 {
		// Use the time for one chop
		let time = time_from(player.offline_timer.pine_log);
		let interval = utils::get_tree_time(player, "pine", 1);
		let loops = loops_in_time(time, interval);
		if loops > 0 {
			player.logs.pine += loops * player.loggers_active.pine;
			player.sawdust_prestige.logs.pine += loops * player.loggers_active.pine;
			player.seed_prestige.logs.pine += loops * player.loggers_active.pine;
			player.stats.pine_logs_earned += loops * player.loggers_active.pine;
			player.offline_timer.pine_log = Utc::now();
		}
	}
	if player.loggers_active.oak > 0 {
		let time = time_from(player.offline_timer.oak_log);
		let interval = utils::get_tree_time(player, "oak", 1);
		let loops = loops_in_time(time, interval);
		if loops > 0 {
			player.logs.oak += loops * player.loggers_active.oak;
			player.sawdust_prestige.logs.oak += loops * player.loggers_active.oak;
			player.seed_prestige.logs.oak += loops * player.loggers_active.oak;
			player.stats.oak_logs_earned += loops * player.loggers_active.oak;
			player.offline_timer.oak_log = Utc::now();
		}
	}
	if player.loggers_active.maple > 0 {
		let time = time_from(player.offline_timer.maple_log);
		let interval = utils::get_tree_time(player, "maple", 1);
		let loops = loops_in_time(time, interval);
		if loops > 0 {
			player.logs.maple += loops * player.loggers_active.maple;
			player.sawdust_prestige.logs.maple += loops * player.loggers_active.maple;
			player.seed_prestige.logs.maple += loops * player.loggers_active.maple;
			player.stats.maple_logs_earned += loops * player.loggers_active.maple;
			player.offline_timer.maple_log = Utc::now();
		}
	}
	if player.loggers_active.walnut > 0 {
		let time = time_from(player.offline_timer.walnut_log);
		let interval = utils::get_tree_time(player, "walnut", 1);
		let loops = loops_in_time(time, interval);
		if loops > 0 {
			player.logs.walnut += loops * player.loggers_active.walnut;
			player.sawdust_prestige.logs.walnut += loops * player.loggers_active.walnut;
			player.seed_prestige.logs.walnut += loops * player.loggers_active.walnut;
			player.stats.walnut_logs_earned += loops * player.loggers_active.walnut;
			player.offline_timer.walnut_log = Utc::now();
		}
	}
	if player.loggers_active.cherry > 0 {
		let time = time_from(player.offline_timer.cherry_log);
		let interval = utils::get_tree_time(player, "cherry", 1);
		let loops = loops_in_time(time, interval);
		if loops > 0 {
			player.logs.cherry += loops * player.loggers_active.cherry;
			player.sawdust_prestige.logs.cherry += loops * player.loggers_active.cherry;
			player.seed_prestige.logs.cherry += loops * player.loggers_active.cherry;
			player.stats.cherry_logs_earned += loops * player.loggers_active.cherry;
			player.offline_timer.cherry_log = Utc::now();
		}
	}
	if player.loggers_active.purpleheart > 0 {
		let time = time_from(player.offline_timer.purpleheart_log);
		let interval = utils::get_tree_time(player, "purpleheart", 1);
		let loops = loops_in_time(time, interval);
		if loops > 0 {
			player.logs.purpleheart += loops * player.loggers_active.purpleheart;
			player.sawdust_prestige.logs.purpleheart += loops * player.loggers_active.purpleheart;
			player.seed_prestige.logs.purpleheart += loops * player.loggers_active.purpleheart;
			player.stats.purpleheart_logs_earned += loops * player.loggers_active.purpleheart;
			player.offline_timer.purpleheart_log = Utc::now();
		}
	}
}

fn update_lumbering(player: &mut Player) {
	if player.assigned_lumberers() == 0 {
		return;
	}
	if player.lumberers_active.pine > 0 && player.logs.pine >= player.lumberers_active.pine {
		// Use the time for one dry
		let time = time_from(player.offline_timer.pine_lumber);
		let interval = utils::get_dry_time(player, "pine", 1);
		let loops = vec![player.logs.pine / player.lumberers_active.pine, loops_in_time(time, interval)];
		let loops_min = *loops.iter().min().unwrap();
		if loops_min > 0 {
			player.logs.pine -= loops_min * player.lumberers_active.pine;
			player.lumber.pine += loops_min * player.lumberers_active.pine;
			player.sawdust_prestige.lumber.pine += loops_min * player.lumberers_active.pine;
			player.seed_prestige.lumber.pine += loops_min * player.lumberers_active.pine;
			player.stats.pine_lumber_earned += loops_min * player.lumberers_active.pine;
			player.offline_timer.pine_lumber = Utc::now();
		}
	}
	if player.lumberers_active.oak > 0 && player.logs.oak >= player.lumberers_active.oak {
		let time = time_from(player.offline_timer.oak_lumber);
		let interval = utils::get_dry_time(player, "oak", 1);
		let loops = vec![player.logs.oak / player.lumberers_active.oak, loops_in_time(time, interval)];
		let loops_min = *loops.iter().min().unwrap();
		if loops_min > 0 {
			player.logs.oak -= loops_min * player.lumberers_active.oak;
			player.lumber.oak += loops_min * player.lumberers_active.oak;
			player.sawdust_prestige.lumber.oak += loops_min * player.lumberers_active.oak;
			player.seed_prestige.lumber.oak += loops_min * player.lumberers_active.oak;
			player.stats.oak_lumber_earned += loops_min * player.lumberers_active.oak;
			player.offline_timer.oak_lumber = Utc::now();
		}
	}
	if player.lumberers_active.maple > 0 && player.logs.maple >= player.lumberers_active.maple {
		let time = time_from(player.offline_timer.maple_lumber);
		let interval = utils::get_dry_time(player, "maple", 1);
		let loops = vec![player.logs.maple / player.lumberers_active.maple, loops_in_time(time, interval)];
		let loops_min = *loops.iter().min().unwrap();
		if loops_min > 0 {
			player.logs.maple -= loops_min * player.lumberers_active.maple;
			player.lumber.maple += loops_min * player.lumberers_active.maple;
			player.sawdust_prestige.lumber.maple += loops_min * player.lumberers_active.maple;
			player.seed_prestige.lumber.maple += loops_min * player.lumberers_active.maple;
			player.stats.maple_lumber_earned += loops_min * player.lumberers_active.maple;
			player.offline_timer.maple_lumber = Utc::now();
		}
	}
	if player.lumberers_active.walnut > 0 && player.logs.walnut >= player.lumberers_active.walnut {
		let time = time_from(player.offline_timer.walnut_lumber);
		let interval = utils::get_dry_time(player, "walnut", 1);
		let loops = vec![player.logs.walnut / player.lumberers_active.walnut, loops_in_time(time, interval)];
		let loops_min = *loops.iter().min().unwrap();
		if loops_min > 0 {
			player.logs.walnut -= loops_min * player.lumberers_active.walnut;
			player.lumber.walnut += loops_min * player.lumberers_active.walnut;
			player.sawdust_prestige.lumber.walnut += loops_min * player.lumberers_active.walnut;
			player.seed_prestige.lumber.walnut += loops_min * player.lumberers_active.walnut;
			player.stats.walnut_lumber_earned += loops_min * player.lumberers_active.walnut;
			player.offline_timer.walnut_lumber = Utc::now();
		}
	}
	if player.lumberers_active.cherry > 0 && player.logs.cherry >= player.lumberers_active.cherry {
		let time = time_from(player.offline_timer.cherry_lumber);
		let interval = utils::get_dry_time(player, "cherry", 1);
		let loops = vec![player.logs.cherry / player.lumberers_active.cherry, loops_in_time(time, interval)];
		let loops_min = *loops.iter().min().unwrap();
		if loops_min > 0 {
			player.logs.cherry -= loops_min * player.lumberers_active.cherry;
			player.lumber.cherry += loops_min * player.lumberers_active.cherry;
			player.sawdust_prestige.lumber.cherry += loops_min * player.lumberers_active.cherry;
			player.seed_prestige.lumber.cherry += loops_min * player.lumberers_active.cherry;
			player.stats.cherry_lumber_earned += loops_min * player.lumberers_active.cherry;
			player.offline_timer.cherry_lumber = Utc::now();
		}
	}
	if player.lumberers_active.purpleheart > 0 && player.logs.purpleheart >= player.lumberers_active.purpleheart {
		let time = time_from(player.offline_timer.purpleheart_lumber);
		let interval = utils::get_dry_time(player, "purpleheart", 1);
		let loops = vec![player.logs.purpleheart / player.lumberers_active.purpleheart, loops_in_time(time, interval)];
		let loops_min = *loops.iter().min().unwrap();
		if loops_min > 0 {
			player.logs.purpleheart -= loops_min * player.lumberers_active.purpleheart;
			player.lumber.purpleheart += loops_min * player.lumberers_active.purpleheart;
			player.sawdust_prestige.lumber.purpleheart += loops_min * player.lumberers_active.purpleheart;
			player.seed_prestige.lumber.purpleheart += loops_min * player.lumberers_active.purpleheart;
			player.stats.purpleheart_lumber_earned += loops_min * player.lumberers_active.purpleheart;
			player.offline_timer.purpleheart_lumber = Utc::now();
		}
	}
}

fn update_cncing(player: &mut Player) {
	if player.assigned_cncs() == 0 {
		return;
	}
	if player.cncs_active.pine.total() > 0 {
		if player.cncs_active.pine.birdhouse > 0 && player.lumber.pine >= 1 * player.cncs_active.pine.birdhouse {
			// Use the time for one build
			let time = time_from(player.offline_timer.pine_birdhouse);
			let interval = utils::get_build_time(player, "pine", "birdhouse", 1);
			let loops = vec![player.lumber.pine / (1 * player.cncs_active.pine.birdhouse), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.pine -= 1 * loops_min * player.cncs_active.pine.birdhouse;
				player.furniture.pine.birdhouse += loops_min * player.cncs_active.pine.birdhouse;
				player.sawdust_prestige.furniture.pine.birdhouse += loops_min * player.cncs_active.pine.birdhouse;
				player.seed_prestige.furniture.pine.birdhouse += loops_min * player.cncs_active.pine.birdhouse;
				player.stats.pine_birdhouses_built += loops_min * player.cncs_active.pine.birdhouse;
				player.offline_timer.pine_birdhouse = Utc::now();
			}
		}
		if player.cncs_active.pine.shelf > 0 && player.lumber.pine >= 2 * player.cncs_active.pine.shelf {
			let time = time_from(player.offline_timer.pine_shelf);
			let interval = utils::get_build_time(player, "pine", "shelf", 1);
			let loops = vec![player.lumber.pine / (1 * player.cncs_active.pine.shelf), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.pine -= 1 * loops_min * player.cncs_active.pine.shelf;
				player.furniture.pine.shelf += loops_min * player.cncs_active.pine.shelf;
				player.sawdust_prestige.furniture.pine.shelf += loops_min * player.cncs_active.pine.shelf;
				player.seed_prestige.furniture.pine.shelf += loops_min * player.cncs_active.pine.shelf;
				player.stats.pine_shelves_built += loops_min * player.cncs_active.pine.shelf;
				player.offline_timer.pine_shelf = Utc::now();
			}
		}
		if player.cncs_active.pine.side_table > 0 && player.lumber.pine >= 3 * player.cncs_active.pine.side_table {
			let time = time_from(player.offline_timer.pine_side_table);
			let interval = utils::get_build_time(player, "pine", "side_table", 1);
			let loops = vec![player.lumber.pine / (1 * player.cncs_active.pine.side_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.pine -= 1 * loops_min * player.cncs_active.pine.side_table;
				player.furniture.pine.side_table += loops_min * player.cncs_active.pine.side_table;
				player.sawdust_prestige.furniture.pine.side_table += loops_min * player.cncs_active.pine.side_table;
				player.seed_prestige.furniture.pine.side_table += loops_min * player.cncs_active.pine.side_table;
				player.stats.pine_side_tables_built += loops_min * player.cncs_active.pine.side_table;
				player.offline_timer.pine_side_table = Utc::now();
			}
		}
		if player.cncs_active.pine.coffee_table > 0 && player.lumber.pine >= 4 * player.cncs_active.pine.coffee_table {
			let time = time_from(player.offline_timer.pine_coffee_table);
			let interval = utils::get_build_time(player, "pine", "coffee_table", 1);
			let loops = vec![player.lumber.pine / (1 * player.cncs_active.pine.coffee_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.pine -= 1 * loops_min * player.cncs_active.pine.coffee_table;
				player.furniture.pine.coffee_table += loops_min * player.cncs_active.pine.coffee_table;
				player.sawdust_prestige.furniture.pine.coffee_table += loops_min * player.cncs_active.pine.coffee_table;
				player.seed_prestige.furniture.pine.coffee_table += loops_min * player.cncs_active.pine.coffee_table;
				player.stats.pine_coffee_tables_built += loops_min * player.cncs_active.pine.coffee_table;
				player.offline_timer.pine_coffee_table = Utc::now();
			}
		}
		if player.cncs_active.pine.dining_set > 0 && player.lumber.pine >= 5 * player.cncs_active.pine.dining_set {
			let time = time_from(player.offline_timer.pine_dining_set);
			let interval = utils::get_build_time(player, "pine", "dining_set", 1);
			let loops = vec![player.lumber.pine / (1 * player.cncs_active.pine.dining_set), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.pine -= 1 * loops_min * player.cncs_active.pine.dining_set;
				player.furniture.pine.dining_set += loops_min * player.cncs_active.pine.dining_set;
				player.sawdust_prestige.furniture.pine.dining_set += loops_min * player.cncs_active.pine.dining_set;
				player.seed_prestige.furniture.pine.dining_set += loops_min * player.cncs_active.pine.dining_set;
				player.stats.pine_dining_sets_built += loops_min * player.cncs_active.pine.dining_set;
				player.offline_timer.pine_dining_set = Utc::now();
			}
		}
	}
	if player.cncs_active.oak.total() > 0 {
		if player.cncs_active.oak.birdhouse > 0 && player.lumber.oak >= 1 * player.cncs_active.oak.birdhouse {
			// Use the time for one build
			let time = time_from(player.offline_timer.oak_birdhouse);
			let interval = utils::get_build_time(player, "oak", "birdhouse", 1);
			let loops = vec![player.lumber.oak / (1 * player.cncs_active.oak.birdhouse), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.oak -= 1 * loops_min * player.cncs_active.oak.birdhouse;
				player.furniture.oak.birdhouse += loops_min * player.cncs_active.oak.birdhouse;
				player.sawdust_prestige.furniture.oak.birdhouse += loops_min * player.cncs_active.oak.birdhouse;
				player.seed_prestige.furniture.oak.birdhouse += loops_min * player.cncs_active.oak.birdhouse;
				player.stats.oak_birdhouses_built += loops_min * player.cncs_active.oak.birdhouse;
				player.offline_timer.oak_birdhouse = Utc::now();
			}
		}
		if player.cncs_active.oak.shelf > 0 && player.lumber.oak >= 2 * player.cncs_active.oak.shelf {
			let time = time_from(player.offline_timer.oak_shelf);
			let interval = utils::get_build_time(player, "oak", "shelf", 1);
			let loops = vec![player.lumber.oak / (1 * player.cncs_active.oak.shelf), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.oak -= 1 * loops_min * player.cncs_active.oak.shelf;
				player.furniture.oak.shelf += loops_min * player.cncs_active.oak.shelf;
				player.sawdust_prestige.furniture.oak.shelf += loops_min * player.cncs_active.oak.shelf;
				player.seed_prestige.furniture.oak.shelf += loops_min * player.cncs_active.oak.shelf;
				player.stats.oak_shelves_built += loops_min * player.cncs_active.oak.shelf;
				player.offline_timer.oak_shelf = Utc::now();
			}
		}
		if player.cncs_active.oak.side_table > 0 && player.lumber.oak >= 3 * player.cncs_active.oak.side_table {
			let time = time_from(player.offline_timer.oak_side_table);
			let interval = utils::get_build_time(player, "oak", "side_table", 1);
			let loops = vec![player.lumber.oak / (1 * player.cncs_active.oak.side_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.oak -= 1 * loops_min * player.cncs_active.oak.side_table;
				player.furniture.oak.side_table += loops_min * player.cncs_active.oak.side_table;
				player.sawdust_prestige.furniture.oak.side_table += loops_min * player.cncs_active.oak.side_table;
				player.seed_prestige.furniture.oak.side_table += loops_min * player.cncs_active.oak.side_table;
				player.stats.oak_side_tables_built += loops_min * player.cncs_active.oak.side_table;
				player.offline_timer.oak_side_table = Utc::now();
			}
		}
		if player.cncs_active.oak.coffee_table > 0 && player.lumber.oak >= 4 * player.cncs_active.oak.coffee_table {
			let time = time_from(player.offline_timer.oak_coffee_table);
			let interval = utils::get_build_time(player, "oak", "coffee_table", 1);
			let loops = vec![player.lumber.oak / (1 * player.cncs_active.oak.coffee_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.oak -= 1 * loops_min * player.cncs_active.oak.coffee_table;
				player.furniture.oak.coffee_table += loops_min * player.cncs_active.oak.coffee_table;
				player.sawdust_prestige.furniture.oak.coffee_table += loops_min * player.cncs_active.oak.coffee_table;
				player.seed_prestige.furniture.oak.coffee_table += loops_min * player.cncs_active.oak.coffee_table;
				player.stats.oak_coffee_tables_built += loops_min * player.cncs_active.oak.coffee_table;
				player.offline_timer.oak_coffee_table = Utc::now();
			}
		}
		if player.cncs_active.oak.dining_set > 0 && player.lumber.oak >= 5 * player.cncs_active.oak.dining_set {
			let time = time_from(player.offline_timer.oak_dining_set);
			let interval = utils::get_build_time(player, "oak", "dining_set", 1);
			let loops = vec![player.lumber.oak / (1 * player.cncs_active.oak.dining_set), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.oak -= 1 * loops_min * player.cncs_active.oak.dining_set;
				player.furniture.oak.dining_set += loops_min * player.cncs_active.oak.dining_set;
				player.sawdust_prestige.furniture.oak.dining_set += loops_min * player.cncs_active.oak.dining_set;
				player.seed_prestige.furniture.oak.dining_set += loops_min * player.cncs_active.oak.dining_set;
				player.stats.oak_dining_sets_built += loops_min * player.cncs_active.oak.dining_set;
				player.offline_timer.oak_dining_set = Utc::now();
			}
		}
	}
	if player.cncs_active.maple.total() > 0 {
		if player.cncs_active.maple.birdhouse > 0 && player.lumber.maple >= 1 * player.cncs_active.maple.birdhouse {
			// Use the time for one build
			let time = time_from(player.offline_timer.maple_birdhouse);
			let interval = utils::get_build_time(player, "maple", "birdhouse", 1);
			let loops = vec![player.lumber.maple / (1 * player.cncs_active.maple.birdhouse), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.maple -= 1 * loops_min * player.cncs_active.maple.birdhouse;
				player.furniture.maple.birdhouse += loops_min * player.cncs_active.maple.birdhouse;
				player.sawdust_prestige.furniture.maple.birdhouse += loops_min * player.cncs_active.maple.birdhouse;
				player.seed_prestige.furniture.maple.birdhouse += loops_min * player.cncs_active.maple.birdhouse;
				player.stats.maple_birdhouses_built += loops_min * player.cncs_active.maple.birdhouse;
				player.offline_timer.maple_birdhouse = Utc::now();
			}
		}
		if player.cncs_active.maple.shelf > 0 && player.lumber.maple >= 2 * player.cncs_active.maple.shelf {
			let time = time_from(player.offline_timer.maple_shelf);
			let interval = utils::get_build_time(player, "maple", "shelf", 1);
			let loops = vec![player.lumber.maple / (1 * player.cncs_active.maple.shelf), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.maple -= 1 * loops_min * player.cncs_active.maple.shelf;
				player.furniture.maple.shelf += loops_min * player.cncs_active.maple.shelf;
				player.sawdust_prestige.furniture.maple.shelf += loops_min * player.cncs_active.maple.shelf;
				player.seed_prestige.furniture.maple.shelf += loops_min * player.cncs_active.maple.shelf;
				player.stats.maple_shelves_built += loops_min * player.cncs_active.maple.shelf;
				player.offline_timer.maple_shelf = Utc::now();
			}
		}
		if player.cncs_active.maple.side_table > 0 && player.lumber.maple >= 3 * player.cncs_active.maple.side_table {
			let time = time_from(player.offline_timer.maple_side_table);
			let interval = utils::get_build_time(player, "maple", "side_table", 1);
			let loops = vec![player.lumber.maple / (1 * player.cncs_active.maple.side_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.maple -= 1 * loops_min * player.cncs_active.maple.side_table;
				player.furniture.maple.side_table += loops_min * player.cncs_active.maple.side_table;
				player.sawdust_prestige.furniture.maple.side_table += loops_min * player.cncs_active.maple.side_table;
				player.seed_prestige.furniture.maple.side_table += loops_min * player.cncs_active.maple.side_table;
				player.stats.maple_side_tables_built += loops_min * player.cncs_active.maple.side_table;
				player.offline_timer.maple_side_table = Utc::now();
			}
		}
		if player.cncs_active.maple.coffee_table > 0 && player.lumber.maple >= 4 * player.cncs_active.maple.coffee_table {
			let time = time_from(player.offline_timer.maple_coffee_table);
			let interval = utils::get_build_time(player, "maple", "coffee_table", 1);
			let loops = vec![player.lumber.maple / (1 * player.cncs_active.maple.coffee_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.maple -= 1 * loops_min * player.cncs_active.maple.coffee_table;
				player.furniture.maple.coffee_table += loops_min * player.cncs_active.maple.coffee_table;
				player.sawdust_prestige.furniture.maple.coffee_table += loops_min * player.cncs_active.maple.coffee_table;
				player.seed_prestige.furniture.maple.coffee_table += loops_min * player.cncs_active.maple.coffee_table;
				player.stats.maple_coffee_tables_built += loops_min * player.cncs_active.maple.coffee_table;
				player.offline_timer.maple_coffee_table = Utc::now();
			}
		}
		if player.cncs_active.maple.dining_set > 0 && player.lumber.maple >= 5 * player.cncs_active.maple.dining_set {
			let time = time_from(player.offline_timer.maple_dining_set);
			let interval = utils::get_build_time(player, "maple", "dining_set", 1);
			let loops = vec![player.lumber.maple / (1 * player.cncs_active.maple.dining_set), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.maple -= 1 * loops_min * player.cncs_active.maple.dining_set;
				player.furniture.maple.dining_set += loops_min * player.cncs_active.maple.dining_set;
				player.sawdust_prestige.furniture.maple.dining_set += loops_min * player.cncs_active.maple.dining_set;
				player.seed_prestige.furniture.maple.dining_set += loops_min * player.cncs_active.maple.dining_set;
				player.stats.maple_dining_sets_built += loops_min * player.cncs_active.maple.dining_set;
				player.offline_timer.maple_dining_set = Utc::now();
			}
		}
	}
	if player.cncs_active.walnut.total() > 0 {
		if player.cncs_active.walnut.birdhouse > 0 && player.lumber.walnut >= 1 * player.cncs_active.walnut.birdhouse {
			// Use the time for one build
			let time = time_from(player.offline_timer.walnut_birdhouse);
			let interval = utils::get_build_time(player, "walnut", "birdhouse", 1);
			let loops = vec![player.lumber.walnut / (1 * player.cncs_active.walnut.birdhouse), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.walnut -= 1 * loops_min * player.cncs_active.walnut.birdhouse;
				player.furniture.walnut.birdhouse += loops_min * player.cncs_active.walnut.birdhouse;
				player.sawdust_prestige.furniture.walnut.birdhouse += loops_min * player.cncs_active.walnut.birdhouse;
				player.seed_prestige.furniture.walnut.birdhouse += loops_min * player.cncs_active.walnut.birdhouse;
				player.stats.walnut_birdhouses_built += loops_min * player.cncs_active.walnut.birdhouse;
				player.offline_timer.walnut_birdhouse = Utc::now();
			}
		}
		if player.cncs_active.walnut.shelf > 0 && player.lumber.walnut >= 2 * player.cncs_active.walnut.shelf {
			let time = time_from(player.offline_timer.walnut_shelf);
			let interval = utils::get_build_time(player, "walnut", "shelf", 1);
			let loops = vec![player.lumber.walnut / (1 * player.cncs_active.walnut.shelf), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.walnut -= 1 * loops_min * player.cncs_active.walnut.shelf;
				player.furniture.walnut.shelf += loops_min * player.cncs_active.walnut.shelf;
				player.sawdust_prestige.furniture.walnut.shelf += loops_min * player.cncs_active.walnut.shelf;
				player.seed_prestige.furniture.walnut.shelf += loops_min * player.cncs_active.walnut.shelf;
				player.stats.walnut_shelves_built += loops_min * player.cncs_active.walnut.shelf;
				player.offline_timer.walnut_shelf = Utc::now();
			}
		}
		if player.cncs_active.walnut.side_table > 0 && player.lumber.walnut >= 3 * player.cncs_active.walnut.side_table {
			let time = time_from(player.offline_timer.walnut_side_table);
			let interval = utils::get_build_time(player, "walnut", "side_table", 1);
			let loops = vec![player.lumber.walnut / (1 * player.cncs_active.walnut.side_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.walnut -= 1 * loops_min * player.cncs_active.walnut.side_table;
				player.furniture.walnut.side_table += loops_min * player.cncs_active.walnut.side_table;
				player.sawdust_prestige.furniture.walnut.side_table += loops_min * player.cncs_active.walnut.side_table;
				player.seed_prestige.furniture.walnut.side_table += loops_min * player.cncs_active.walnut.side_table;
				player.stats.walnut_side_tables_built += loops_min * player.cncs_active.walnut.side_table;
				player.offline_timer.walnut_side_table = Utc::now();
			}
		}
		if player.cncs_active.walnut.coffee_table > 0 && player.lumber.walnut >= 4 * player.cncs_active.walnut.coffee_table {
			let time = time_from(player.offline_timer.walnut_coffee_table);
			let interval = utils::get_build_time(player, "walnut", "coffee_table", 1);
			let loops = vec![player.lumber.walnut / (1 * player.cncs_active.walnut.coffee_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.walnut -= 1 * loops_min * player.cncs_active.walnut.coffee_table;
				player.furniture.walnut.coffee_table += loops_min * player.cncs_active.walnut.coffee_table;
				player.sawdust_prestige.furniture.walnut.coffee_table += loops_min * player.cncs_active.walnut.coffee_table;
				player.seed_prestige.furniture.walnut.coffee_table += loops_min * player.cncs_active.walnut.coffee_table;
				player.stats.walnut_coffee_tables_built += loops_min * player.cncs_active.walnut.coffee_table;
				player.offline_timer.walnut_coffee_table = Utc::now();
			}
		}
		if player.cncs_active.walnut.dining_set > 0 && player.lumber.walnut >= 5 * player.cncs_active.walnut.dining_set {
			let time = time_from(player.offline_timer.walnut_dining_set);
			let interval = utils::get_build_time(player, "walnut", "dining_set", 1);
			let loops = vec![player.lumber.walnut / (1 * player.cncs_active.walnut.dining_set), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.walnut -= 1 * loops_min * player.cncs_active.walnut.dining_set;
				player.furniture.walnut.dining_set += loops_min * player.cncs_active.walnut.dining_set;
				player.sawdust_prestige.furniture.walnut.dining_set += loops_min * player.cncs_active.walnut.dining_set;
				player.seed_prestige.furniture.walnut.dining_set += loops_min * player.cncs_active.walnut.dining_set;
				player.stats.walnut_dining_sets_built += loops_min * player.cncs_active.walnut.dining_set;
				player.offline_timer.walnut_dining_set = Utc::now();
			}
		}
	}
	if player.cncs_active.cherry.total() > 0 {
		if player.cncs_active.cherry.birdhouse > 0 && player.lumber.cherry >= 1 * player.cncs_active.cherry.birdhouse {
			// Use the time for one build
			let time = time_from(player.offline_timer.cherry_birdhouse);
			let interval = utils::get_build_time(player, "cherry", "birdhouse", 1);
			let loops = vec![player.lumber.cherry / (1 * player.cncs_active.cherry.birdhouse), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.cherry -= 1 * loops_min * player.cncs_active.cherry.birdhouse;
				player.furniture.cherry.birdhouse += loops_min * player.cncs_active.cherry.birdhouse;
				player.sawdust_prestige.furniture.cherry.birdhouse += loops_min * player.cncs_active.cherry.birdhouse;
				player.seed_prestige.furniture.cherry.birdhouse += loops_min * player.cncs_active.cherry.birdhouse;
				player.stats.cherry_birdhouses_built += loops_min * player.cncs_active.cherry.birdhouse;
				player.offline_timer.cherry_birdhouse = Utc::now();
			}
		}
		if player.cncs_active.cherry.shelf > 0 && player.lumber.cherry >= 2 * player.cncs_active.cherry.shelf {
			let time = time_from(player.offline_timer.cherry_shelf);
			let interval = utils::get_build_time(player, "cherry", "shelf", 1);
			let loops = vec![player.lumber.cherry / (1 * player.cncs_active.cherry.shelf), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.cherry -= 1 * loops_min * player.cncs_active.cherry.shelf;
				player.furniture.cherry.shelf += loops_min * player.cncs_active.cherry.shelf;
				player.sawdust_prestige.furniture.cherry.shelf += loops_min * player.cncs_active.cherry.shelf;
				player.seed_prestige.furniture.cherry.shelf += loops_min * player.cncs_active.cherry.shelf;
				player.stats.cherry_shelves_built += loops_min * player.cncs_active.cherry.shelf;
				player.offline_timer.cherry_shelf = Utc::now();
			}
		}
		if player.cncs_active.cherry.side_table > 0 && player.lumber.cherry >= 3 * player.cncs_active.cherry.side_table {
			let time = time_from(player.offline_timer.cherry_side_table);
			let interval = utils::get_build_time(player, "cherry", "side_table", 1);
			let loops = vec![player.lumber.cherry / (1 * player.cncs_active.cherry.side_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.cherry -= 1 * loops_min * player.cncs_active.cherry.side_table;
				player.furniture.cherry.side_table += loops_min * player.cncs_active.cherry.side_table;
				player.sawdust_prestige.furniture.cherry.side_table += loops_min * player.cncs_active.cherry.side_table;
				player.seed_prestige.furniture.cherry.side_table += loops_min * player.cncs_active.cherry.side_table;
				player.stats.cherry_side_tables_built += loops_min * player.cncs_active.cherry.side_table;
				player.offline_timer.cherry_side_table = Utc::now();
			}
		}
		if player.cncs_active.cherry.coffee_table > 0 && player.lumber.cherry >= 4 * player.cncs_active.cherry.coffee_table {
			let time = time_from(player.offline_timer.cherry_coffee_table);
			let interval = utils::get_build_time(player, "cherry", "coffee_table", 1);
			let loops = vec![player.lumber.cherry / (1 * player.cncs_active.cherry.coffee_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.cherry -= 1 * loops_min * player.cncs_active.cherry.coffee_table;
				player.furniture.cherry.coffee_table += loops_min * player.cncs_active.cherry.coffee_table;
				player.sawdust_prestige.furniture.cherry.coffee_table += loops_min * player.cncs_active.cherry.coffee_table;
				player.seed_prestige.furniture.cherry.coffee_table += loops_min * player.cncs_active.cherry.coffee_table;
				player.stats.cherry_coffee_tables_built += loops_min * player.cncs_active.cherry.coffee_table;
				player.offline_timer.cherry_coffee_table = Utc::now();
			}
		}
		if player.cncs_active.cherry.dining_set > 0 && player.lumber.cherry >= 5 * player.cncs_active.cherry.dining_set {
			let time = time_from(player.offline_timer.cherry_dining_set);
			let interval = utils::get_build_time(player, "cherry", "dining_set", 1);
			let loops = vec![player.lumber.cherry / (1 * player.cncs_active.cherry.dining_set), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.cherry -= 1 * loops_min * player.cncs_active.cherry.dining_set;
				player.furniture.cherry.dining_set += loops_min * player.cncs_active.cherry.dining_set;
				player.sawdust_prestige.furniture.cherry.dining_set += loops_min * player.cncs_active.cherry.dining_set;
				player.seed_prestige.furniture.cherry.dining_set += loops_min * player.cncs_active.cherry.dining_set;
				player.stats.cherry_dining_sets_built += loops_min * player.cncs_active.cherry.dining_set;
				player.offline_timer.cherry_dining_set = Utc::now();
			}
		}
	}
	if player.cncs_active.purpleheart.total() > 0 {
		if player.cncs_active.purpleheart.birdhouse > 0 && player.lumber.purpleheart >= 1 * player.cncs_active.purpleheart.birdhouse {
			// Use the time for one build
			let time = time_from(player.offline_timer.purpleheart_birdhouse);
			let interval = utils::get_build_time(player, "purpleheart", "birdhouse", 1);
			let loops = vec![player.lumber.purpleheart / (1 * player.cncs_active.purpleheart.birdhouse), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.purpleheart -= 1 * loops_min * player.cncs_active.purpleheart.birdhouse;
				player.furniture.purpleheart.birdhouse += loops_min * player.cncs_active.purpleheart.birdhouse;
				player.sawdust_prestige.furniture.purpleheart.birdhouse += loops_min * player.cncs_active.purpleheart.birdhouse;
				player.seed_prestige.furniture.purpleheart.birdhouse += loops_min * player.cncs_active.purpleheart.birdhouse;
				player.stats.purpleheart_birdhouses_built += loops_min * player.cncs_active.purpleheart.birdhouse;
				player.offline_timer.purpleheart_birdhouse = Utc::now();
			}
		}
		if player.cncs_active.purpleheart.shelf > 0 && player.lumber.purpleheart >= 2 * player.cncs_active.purpleheart.shelf {
			let time = time_from(player.offline_timer.purpleheart_shelf);
			let interval = utils::get_build_time(player, "purpleheart", "shelf", 1);
			let loops = vec![player.lumber.purpleheart / (1 * player.cncs_active.purpleheart.shelf), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.purpleheart -= 1 * loops_min * player.cncs_active.purpleheart.shelf;
				player.furniture.purpleheart.shelf += loops_min * player.cncs_active.purpleheart.shelf;
				player.sawdust_prestige.furniture.purpleheart.shelf += loops_min * player.cncs_active.purpleheart.shelf;
				player.seed_prestige.furniture.purpleheart.shelf += loops_min * player.cncs_active.purpleheart.shelf;
				player.stats.purpleheart_shelves_built += loops_min * player.cncs_active.purpleheart.shelf;
				player.offline_timer.purpleheart_shelf = Utc::now();
			}
		}
		if player.cncs_active.purpleheart.side_table > 0 && player.lumber.purpleheart >= 3 * player.cncs_active.purpleheart.side_table {
			let time = time_from(player.offline_timer.purpleheart_side_table);
			let interval = utils::get_build_time(player, "purpleheart", "side_table", 1);
			let loops = vec![player.lumber.purpleheart / (1 * player.cncs_active.purpleheart.side_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.purpleheart -= 1 * loops_min * player.cncs_active.purpleheart.side_table;
				player.furniture.purpleheart.side_table += loops_min * player.cncs_active.purpleheart.side_table;
				player.sawdust_prestige.furniture.purpleheart.side_table += loops_min * player.cncs_active.purpleheart.side_table;
				player.seed_prestige.furniture.purpleheart.side_table += loops_min * player.cncs_active.purpleheart.side_table;
				player.stats.purpleheart_side_tables_built += loops_min * player.cncs_active.purpleheart.side_table;
				player.offline_timer.purpleheart_side_table = Utc::now();
			}
		}
		if player.cncs_active.purpleheart.coffee_table > 0 && player.lumber.purpleheart >= 4 * player.cncs_active.purpleheart.coffee_table {
			let time = time_from(player.offline_timer.purpleheart_coffee_table);
			let interval = utils::get_build_time(player, "purpleheart", "coffee_table", 1);
			let loops = vec![player.lumber.purpleheart / (1 * player.cncs_active.purpleheart.coffee_table), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.purpleheart -= 1 * loops_min * player.cncs_active.purpleheart.coffee_table;
				player.furniture.purpleheart.coffee_table += loops_min * player.cncs_active.purpleheart.coffee_table;
				player.sawdust_prestige.furniture.purpleheart.coffee_table += loops_min * player.cncs_active.purpleheart.coffee_table;
				player.seed_prestige.furniture.purpleheart.coffee_table += loops_min * player.cncs_active.purpleheart.coffee_table;
				player.stats.purpleheart_coffee_tables_built += loops_min * player.cncs_active.purpleheart.coffee_table;
				player.offline_timer.purpleheart_coffee_table = Utc::now();
			}
		}
		if player.cncs_active.purpleheart.dining_set > 0 && player.lumber.purpleheart >= 5 * player.cncs_active.purpleheart.dining_set {
			let time = time_from(player.offline_timer.purpleheart_dining_set);
			let interval = utils::get_build_time(player, "purpleheart", "dining_set", 1);
			let loops = vec![player.lumber.purpleheart / (1 * player.cncs_active.purpleheart.dining_set), loops_in_time(time, interval)];
			let loops_min = *loops.iter().min().unwrap();
			if loops_min > 0 {
				player.lumber.purpleheart -= 1 * loops_min * player.cncs_active.purpleheart.dining_set;
				player.furniture.purpleheart.dining_set += loops_min * player.cncs_active.purpleheart.dining_set;
				player.sawdust_prestige.furniture.purpleheart.dining_set += loops_min * player.cncs_active.purpleheart.dining_set;
				player.seed_prestige.furniture.purpleheart.dining_set += loops_min * player.cncs_active.purpleheart.dining_set;
				player.stats.purpleheart_dining_sets_built += loops_min * player.cncs_active.purpleheart.dining_set;
				player.offline_timer.purpleheart_dining_set = Utc::now();
			}
		}
	}
}