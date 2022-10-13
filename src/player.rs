use std::{
	collections::HashMap,
	cmp::Ordering,
};

use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
		oid::ObjectId,
	}, 
	Collection
};
use futures::stream::{TryStreamExt};
use chrono::{
	DateTime, 
	Utc, 
	Local,
	Duration
};
use crate::{
	get_client
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	id: Option<ObjectId>,
	pub discord_id: i64,
	pub cash: i64,
	pub axe: Axe,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub last_checked: DateTime<Utc>,
	pub logs: HashMap<String, i64>,
	pub loggers: i64,
	pub lumber: HashMap<String, i64>,
	pub lumberers: i64,
	pub blueprints: HashMap<String, Vec<String>>,
	pub furniture: HashMap<String, Vec<String>>,
	pub cncs: i64,
	pub upgrades:  Upgrades,
	pub sawdust: i64,
	pub sawdust_total: i64,
	pub sawdust_upgrades: SawdustUpgrades,
	pub seeds: HashMap<String, i64>,
	pub stats: Stats,
}

impl Player {
	pub fn new(discord_id: u64) -> Self {
		let mut blueprints: HashMap<String, Vec<String>> = HashMap::new();
		blueprints.insert("pine".to_string(), vec!["birdhouse".to_string()]);

		Self {
			id: None,
			discord_id: discord_id as i64,
			cash: 0,
			axe: Axe::Stone,
			last_checked: Utc::now(),
			logs: HashMap::new(),
			loggers: 0,
			lumber: HashMap::new(),
			lumberers: 0,
			blueprints,
			furniture: HashMap::new(),
			cncs: 0,
			upgrades: Upgrades::default(),
			sawdust: 0,
			sawdust_total: 0,
			sawdust_upgrades: SawdustUpgrades::default(),
			seeds: HashMap::new(),
			stats: Stats::default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Axe {
	Stone = 1,
	Iron = 2,
	Steel = 3,
	Mithril = 4,
	Adamant = 5,
	Rune = 6
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Upgrades {
	sharper_axes: i64,
}

impl Default for Upgrades {
	fn default() -> Self {
		Self {
			sharper_axes: 0,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SawdustUpgrades {
	sharper_axes: i64,
}

impl Default for SawdustUpgrades {
	fn default() -> Self {
		Self {
			sharper_axes: 0,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stats {
	times_prestiged: i64,
	pine_trees_chopped: i64,
	pine_logs_earned: i64,
	oak_trees_chopped: i64,
	oak_logs_earned: i64,
	maple_trees_chopped: i64,
	maple_logs_earned: i64,
	walnut_trees_chopped: i64,
	walnut_logs_earned: i64,
	cherry_trees_chopped: i64,
	cherry_logs_earned: i64,
	purpleheart_trees_chopped: i64,
	purpleheart_logs_earned: i64,
}

impl Default for Stats {
	fn default() -> Self {
		Self {
			times_prestiged: 0,
			pine_trees_chopped: 0,
			pine_logs_earned: 0,
			oak_trees_chopped: 0,
			oak_logs_earned: 0,
			maple_trees_chopped: 0,
			maple_logs_earned: 0,
			walnut_trees_chopped: 0,
			walnut_logs_earned: 0,
			cherry_trees_chopped: 0,
			cherry_logs_earned: 0,
			purpleheart_trees_chopped: 0,
			purpleheart_logs_earned: 0,
		}
	}
}

async fn get_player_collection() -> Collection<Player> {
	let client = get_client().await.unwrap();
	let collection = client.database("poketcg").collection::<Player>("players");

	collection
}

// Database functions
pub async fn get_players() -> Vec<Player> { // Will change to Player
	let player_collection = get_player_collection().await;
	let players = player_collection
		.find(None, None)
		.await
		.unwrap()
		.try_collect::<Vec<Player>>()
		.await
		.unwrap();

	players
}

pub async fn get_player(discord_id: u64) -> Player { // Will change to Player
	let player_collection = get_player_collection().await;
	let player = player_collection
		.find_one(doc! { "discord_id": discord_id as i64 }, None)
		.await
		.unwrap();
	match player {
		Some(x) => return x,
		None => return add_player(discord_id).await
	}
}

async fn add_player(discord_id: u64) -> Player {
	let ret = Player::new(discord_id);
	let player_collection = get_player_collection().await;
	player_collection
		.insert_one(&ret, None)
		.await
		.unwrap();
	
	ret
}

pub async fn update_player(player: &Player, update: Document) {
	let player_collection = get_player_collection().await;
	player_collection
		.update_one(
			doc! {"_id": &player.id.unwrap() }, 
			update, 
			None)
		.await
		.unwrap();
}