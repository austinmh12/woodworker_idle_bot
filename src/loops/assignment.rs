use std::sync::Arc;
use serenity::prelude::Context;
use chrono::{
	DateTime,
	Utc
};

use crate::player::{get_players, Player};
use crate::utils;

pub async fn update_assignments(_ctx: Arc<Context>) {
	let players = get_players().await;
	for mut player in players {
		update_logging(&mut player);
		update_lumbering(&mut player);
		update_cncing(&mut player);
		player.last_updated = Utc::now();
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
	let time = time_from(player.last_updated);
	if player.loggers_active.pine > 0 {
		// Use the time for one chop
		let interval = utils::get_tree_time(player, "pine", 1);
		let loops = loops_in_time(time, interval);
		player.logs.pine += loops * player.loggers_active.pine;
	}
	if player.loggers_active.oak > 0 {
		let interval = utils::get_tree_time(player, "oak", 1);
		let loops = loops_in_time(time, interval);
		player.logs.oak += loops * player.loggers_active.oak;
	}
	if player.loggers_active.maple > 0 {
		let interval = utils::get_tree_time(player, "maple", 1);
		let loops = loops_in_time(time, interval);
		player.logs.maple += loops * player.loggers_active.maple;
	}
	if player.loggers_active.walnut > 0 {
		let interval = utils::get_tree_time(player, "walnut", 1);
		let loops = loops_in_time(time, interval);
		player.logs.walnut += loops * player.loggers_active.walnut;
	}
	if player.loggers_active.cherry > 0 {
		let interval = utils::get_tree_time(player, "cherry", 1);
		let loops = loops_in_time(time, interval);
		player.logs.cherry += loops * player.loggers_active.cherry;
	}
	if player.loggers_active.purpleheart > 0 {
		let interval = utils::get_tree_time(player, "purpleheart", 1);
		let loops = loops_in_time(time, interval);
		player.logs.purpleheart += loops * player.loggers_active.purpleheart;
	}
}

fn update_lumbering(player: &mut Player) {
	if player.assigned_lumberers() == 0 {
		return;
	}
	let time = time_from(player.last_updated);
	if player.lumberers_active.pine > 0 && player.logs.pine >= player.lumberers_active.pine {
		// Use the time for one dry
		let interval = utils::get_dry_time(player, "pine", 1);
		let loops = loops_in_time(time, interval);
		player.lumber.pine += loops * player.lumberers_active.pine;
		player.logs.pine -= loops * player.lumberers_active.pine;
	}
	if player.lumberers_active.oak > 0 && player.logs.oak >= player.lumberers_active.oak {
		let interval = utils::get_dry_time(player, "oak", 1);
		let loops = loops_in_time(time, interval);
		player.lumber.oak += loops * player.lumberers_active.oak;
		player.logs.oak -= loops * player.lumberers_active.oak;
	}
	if player.lumberers_active.maple > 0 && player.logs.maple >= player.lumberers_active.maple {
		let interval = utils::get_dry_time(player, "maple", 1);
		let loops = loops_in_time(time, interval);
		player.lumber.maple += loops * player.lumberers_active.maple;
		player.logs.maple -= loops * player.lumberers_active.maple;
	}
	if player.lumberers_active.walnut > 0 && player.logs.walnut >= player.lumberers_active.walnut {
		let interval = utils::get_dry_time(player, "walnut", 1);
		let loops = loops_in_time(time, interval);
		player.lumber.walnut += loops * player.lumberers_active.walnut;
		player.logs.walnut -= loops * player.lumberers_active.walnut;
	}
	if player.lumberers_active.cherry > 0 && player.logs.cherry >= player.lumberers_active.cherry {
		let interval = utils::get_dry_time(player, "cherry", 1);
		let loops = loops_in_time(time, interval);
		player.lumber.cherry += loops * player.lumberers_active.cherry;
		player.logs.cherry -= loops * player.lumberers_active.cherry;
	}
	if player.lumberers_active.purpleheart > 0 && player.logs.purpleheart >= player.lumberers_active.purpleheart {
		let interval = utils::get_dry_time(player, "purpleheart", 1);
		let loops = loops_in_time(time, interval);
		player.lumber.purpleheart += loops * player.lumberers_active.purpleheart;
		player.logs.purpleheart -= loops * player.lumberers_active.purpleheart;
	}
}

fn update_cncing(player: &mut Player) {
	if player.assigned_cncs() == 0 {
		return;
	}
	let time = time_from(player.last_updated);
	if player.cncs_active.pine.total() > 0 {
		if player.cncs_active.pine.birdhouse > 0 && player.lumber.pine >= 1 * player.cncs_active.pine.birdhouse {
			// Use the time for one build
			let interval = utils::get_build_time(player, "pine", "birdhouse", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.pine.birdhouse += loops * player.cncs_active.pine.birdhouse;
			player.lumber.pine -= 1 * loops * player.cncs_active.pine.birdhouse;
		}
		if player.cncs_active.pine.shelf > 0 && player.lumber.pine >= 2 * player.cncs_active.pine.shelf {
			let interval = utils::get_build_time(player, "pine", "shelf", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.pine.shelf += loops * player.cncs_active.pine.shelf;
			player.lumber.pine -= 2 * loops * player.cncs_active.pine.shelf;
		}
		if player.cncs_active.pine.side_table > 0 && player.lumber.pine >= 3 * player.cncs_active.pine.side_table {
			let interval = utils::get_build_time(player, "pine", "side table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.pine.side_table += loops * player.cncs_active.pine.side_table;
			player.lumber.pine -= 3 * loops * player.cncs_active.pine.side_table;
		}
		if player.cncs_active.pine.coffee_table > 0 && player.lumber.pine >= 4 * player.cncs_active.pine.coffee_table {
			let interval = utils::get_build_time(player, "pine", "coffee table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.pine.coffee_table += loops * player.cncs_active.pine.coffee_table;
			player.lumber.pine -= 4 * loops * player.cncs_active.pine.coffee_table;
		}
		if player.cncs_active.pine.dining_set > 0 && player.lumber.pine >= 5 * player.cncs_active.pine.dining_set {
			let interval = utils::get_build_time(player, "pine", "dining set", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.pine.dining_set += loops * player.cncs_active.pine.dining_set;
			player.lumber.pine -= 5 * loops * player.cncs_active.pine.dining_set;
		}
	}
	if player.cncs_active.oak.total() > 0 {
		if player.cncs_active.oak.birdhouse > 0 && player.lumber.oak >= 1 * player.cncs_active.oak.birdhouse {
			let interval = utils::get_build_time(player, "oak", "birdhouse", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.oak.birdhouse += loops * player.cncs_active.oak.birdhouse;
			player.lumber.oak -= 1 * loops * player.cncs_active.oak.birdhouse;
		}
		if player.cncs_active.oak.shelf > 0 && player.lumber.oak >= 2 * player.cncs_active.oak.shelf {
			let interval = utils::get_build_time(player, "oak", "shelf", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.oak.shelf += loops * player.cncs_active.oak.shelf;
			player.lumber.oak -= 2 * loops * player.cncs_active.oak.shelf;
		}
		if player.cncs_active.oak.side_table > 0 && player.lumber.oak >= 3 * player.cncs_active.oak.side_table {
			let interval = utils::get_build_time(player, "oak", "side table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.oak.side_table += loops * player.cncs_active.oak.side_table;
			player.lumber.oak -= 3 * loops * player.cncs_active.oak.side_table;
		}
		if player.cncs_active.oak.coffee_table > 0 && player.lumber.oak >= 4 * player.cncs_active.oak.coffee_table {
			let interval = utils::get_build_time(player, "oak", "coffee table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.oak.coffee_table += loops * player.cncs_active.oak.coffee_table;
			player.lumber.oak -= 4 * loops * player.cncs_active.oak.coffee_table;
		}
		if player.cncs_active.oak.dining_set > 0 && player.lumber.oak >= 5 * player.cncs_active.oak.dining_set {
			let interval = utils::get_build_time(player, "oak", "dining set", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.oak.dining_set += loops * player.cncs_active.oak.dining_set;
			player.lumber.oak -= 5 * loops * player.cncs_active.oak.dining_set;
		}
	}
	if player.cncs_active.maple.total() > 0 {
		if player.cncs_active.maple.birdhouse > 0 && player.lumber.maple >= 1 * player.cncs_active.maple.birdhouse {
			let interval = utils::get_build_time(player, "maple", "birdhouse", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.maple.birdhouse += loops * player.cncs_active.maple.birdhouse;
			player.lumber.maple -= 1 * loops * player.cncs_active.maple.birdhouse;
		}
		if player.cncs_active.maple.shelf > 0 && player.lumber.maple >= 2 * player.cncs_active.maple.shelf {
			let interval = utils::get_build_time(player, "maple", "shelf", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.maple.shelf += loops * player.cncs_active.maple.shelf;
			player.lumber.maple -= 2 * loops * player.cncs_active.maple.shelf;
		}
		if player.cncs_active.maple.side_table > 0 && player.lumber.maple >= 3 * player.cncs_active.maple.side_table {
			let interval = utils::get_build_time(player, "maple", "side table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.maple.side_table += loops * player.cncs_active.maple.side_table;
			player.lumber.maple -= 3 * loops * player.cncs_active.maple.side_table;
		}
		if player.cncs_active.maple.coffee_table > 0 && player.lumber.maple >= 4 * player.cncs_active.maple.coffee_table {
			let interval = utils::get_build_time(player, "maple", "coffee table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.maple.coffee_table += loops * player.cncs_active.maple.coffee_table;
			player.lumber.maple -= 4 * loops * player.cncs_active.maple.coffee_table;
		}
		if player.cncs_active.maple.dining_set > 0 && player.lumber.maple >= 5 * player.cncs_active.maple.dining_set {
			let interval = utils::get_build_time(player, "maple", "dining set", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.maple.dining_set += loops * player.cncs_active.maple.dining_set;
			player.lumber.maple -= 5 * loops * player.cncs_active.maple.dining_set;
		}
	}
	if player.cncs_active.walnut.total() > 0 {
		if player.cncs_active.walnut.birdhouse > 0 && player.lumber.walnut >= 1 * player.cncs_active.walnut.birdhouse {
			let interval = utils::get_build_time(player, "walnut", "birdhouse", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.walnut.birdhouse += loops * player.cncs_active.walnut.birdhouse;
			player.lumber.walnut -= 1 * loops * player.cncs_active.walnut.birdhouse;
		}
		if player.cncs_active.walnut.shelf > 0 && player.lumber.walnut >= 2 * player.cncs_active.walnut.shelf {
			let interval = utils::get_build_time(player, "walnut", "shelf", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.walnut.shelf += loops * player.cncs_active.walnut.shelf;
			player.lumber.walnut -= 2 * loops * player.cncs_active.walnut.shelf;
		}
		if player.cncs_active.walnut.side_table > 0 && player.lumber.walnut >= 3 * player.cncs_active.walnut.side_table {
			let interval = utils::get_build_time(player, "walnut", "side table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.walnut.side_table += loops * player.cncs_active.walnut.side_table;
			player.lumber.walnut -= 3 * loops * player.cncs_active.walnut.side_table;
		}
		if player.cncs_active.walnut.coffee_table > 0 && player.lumber.walnut >= 4 * player.cncs_active.walnut.coffee_table {
			let interval = utils::get_build_time(player, "walnut", "coffee table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.walnut.coffee_table += loops * player.cncs_active.walnut.coffee_table;
			player.lumber.walnut -= 4 * loops * player.cncs_active.walnut.coffee_table;
		}
		if player.cncs_active.walnut.dining_set > 0 && player.lumber.walnut >= 5 * player.cncs_active.walnut.dining_set {
			let interval = utils::get_build_time(player, "walnut", "dining set", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.walnut.dining_set += loops * player.cncs_active.walnut.dining_set;
			player.lumber.walnut -= 5 * loops * player.cncs_active.walnut.dining_set;
		}
	}
	if player.cncs_active.cherry.total() > 0 {
		if player.cncs_active.cherry.birdhouse > 0 && player.lumber.cherry >= 1 * player.cncs_active.cherry.birdhouse {
			let interval = utils::get_build_time(player, "cherry", "birdhouse", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.cherry.birdhouse += loops * player.cncs_active.cherry.birdhouse;
			player.lumber.cherry -= 1 * loops * player.cncs_active.cherry.birdhouse;
		}
		if player.cncs_active.cherry.shelf > 0 && player.lumber.cherry >= 2 * player.cncs_active.cherry.shelf {
			let interval = utils::get_build_time(player, "cherry", "shelf", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.cherry.shelf += loops * player.cncs_active.cherry.shelf;
			player.lumber.cherry -= 2 * loops * player.cncs_active.cherry.shelf;
		}
		if player.cncs_active.cherry.side_table > 0 && player.lumber.cherry >= 3 * player.cncs_active.cherry.side_table {
			let interval = utils::get_build_time(player, "cherry", "side table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.cherry.side_table += loops * player.cncs_active.cherry.side_table;
			player.lumber.cherry -= 3 * loops * player.cncs_active.cherry.side_table;
		}
		if player.cncs_active.cherry.coffee_table > 0 && player.lumber.cherry >= 4 * player.cncs_active.cherry.coffee_table {
			let interval = utils::get_build_time(player, "cherry", "coffee table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.cherry.coffee_table += loops * player.cncs_active.cherry.coffee_table;
			player.lumber.cherry -= 4 * loops * player.cncs_active.cherry.coffee_table;
		}
		if player.cncs_active.cherry.dining_set > 0 && player.lumber.cherry >= 5 * player.cncs_active.cherry.dining_set {
			let interval = utils::get_build_time(player, "cherry", "dining set", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.cherry.dining_set += loops * player.cncs_active.cherry.dining_set;
			player.lumber.cherry -= 5 * loops * player.cncs_active.cherry.dining_set;
		}
	}
	if player.cncs_active.purpleheart.total() > 0 {
		if player.cncs_active.purpleheart.birdhouse > 0 && player.lumber.purpleheart >= 1 * player.cncs_active.purpleheart.birdhouse {
			let interval = utils::get_build_time(player, "purpleheart", "birdhouse", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.purpleheart.birdhouse += loops * player.cncs_active.purpleheart.birdhouse;
			player.lumber.purpleheart -= 1 * loops * player.cncs_active.purpleheart.birdhouse;
		}
		if player.cncs_active.purpleheart.shelf > 0 && player.lumber.purpleheart >= 2 * player.cncs_active.purpleheart.shelf {
			let interval = utils::get_build_time(player, "purpleheart", "shelf", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.purpleheart.shelf += loops * player.cncs_active.purpleheart.shelf;
			player.lumber.purpleheart -= 2 * loops * player.cncs_active.purpleheart.shelf;
		}
		if player.cncs_active.purpleheart.side_table > 0 && player.lumber.purpleheart >= 3 * player.cncs_active.purpleheart.side_table {
			let interval = utils::get_build_time(player, "purpleheart", "side table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.purpleheart.side_table += loops * player.cncs_active.purpleheart.side_table;
			player.lumber.purpleheart -= 3 * loops * player.cncs_active.purpleheart.side_table;
		}
		if player.cncs_active.purpleheart.coffee_table > 0 && player.lumber.purpleheart >= 4 * player.cncs_active.purpleheart.coffee_table {
			let interval = utils::get_build_time(player, "purpleheart", "coffee table", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.purpleheart.coffee_table += loops * player.cncs_active.purpleheart.coffee_table;
			player.lumber.purpleheart -= 4 * loops * player.cncs_active.purpleheart.coffee_table;
		}
		if player.cncs_active.purpleheart.dining_set > 0 && player.lumber.purpleheart >= 5 * player.cncs_active.purpleheart.dining_set {
			let interval = utils::get_build_time(player, "purpleheart", "dining set", 1);
			let loops = loops_in_time(time, interval);
			player.furniture.purpleheart.dining_set += loops * player.cncs_active.purpleheart.dining_set;
			player.lumber.purpleheart -= 5 * loops * player.cncs_active.purpleheart.dining_set;
		}
	}
}