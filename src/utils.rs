use mongodb::{
	bson::{
		Document,
	}, 
};
use serenity::builder::CreateEmbed;

use crate::player::Player;

pub trait ToDoc {
	fn to_doc(&self) -> Document;
}

pub trait ToEmbed {
	fn embed(&self) -> CreateEmbed;
}

pub fn get_tree_time(player: &Player, tree: &str) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 30.0,
		"walnut" => 45.0,
		"cherry" => 90.0,
		"purpleheart" => 300.0,
		_ => 15.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.sharper_axes as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.sharper_axes as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult) as i64
}