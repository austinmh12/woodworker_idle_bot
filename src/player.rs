use std::{
	collections::HashMap,
};

use bson::Bson;
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
	get_client, utils::ToDoc
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
	pub logs: Logs,
	pub loggers: i64,
	pub lumber: Lumber,
	pub lumberers: i64,
	pub blueprints: Blueprints,
	pub furniture: Furniture,
	pub cncs: i64,
	pub upgrades:  Upgrades,
	pub sawdust: i64,
	pub sawdust_total: i64,
	pub sawdust_upgrades: SawdustUpgrades,
	pub seeds: Seeds,
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
			logs: Logs::default(),
			loggers: 0,
			lumber: Lumber::default(),
			lumberers: 0,
			blueprints: Blueprints::default(),
			furniture: Furniture::default(),
			cncs: 0,
			upgrades: Upgrades::default(),
			sawdust: 0,
			sawdust_total: 0,
			sawdust_upgrades: SawdustUpgrades::default(),
			seeds: Seeds::default(),
			stats: Stats::default(),
		}
	}
}

impl ToDoc for Player {
	fn to_doc(&self) -> Document {
		doc! {
			"$set": {
				"cash": &self.cash,
				"axe": &self.axe,
				"last_checked": &self.last_checked,
				"logs": &self.logs.to_doc(),
				"loggers": &self.loggers,
				"lumber": &self.lumber.to_doc(),
				"lumberers": &self.lumberers,
				"blueprints": &self.blueprints.to_doc(),
				"furniture": &self.furniture.to_doc(),
				"cncs": &self.cncs,
				"upgrades": &self.upgrades.to_doc(),
				"sawdust": &self.sawdust,
				"sawdust_total": &self.sawdust_total,
				"sawdust_upgrades": &self.sawdust_upgrades.to_doc(),
				"seeds": &self.seeds.to_doc(),
				"stats": &self.stats.to_doc(),
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Axe {
	Stone = 1,
	Iron = 2,
	Steel = 3,
	Mithril = 4,
	Adamant = 5,
	Rune = 6
}

impl From<Bson> for Axe {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("Stone") {
			"Stone" => Axe::Stone,
			"Iron" => Axe::Iron,
			"Steel" => Axe::Steel,
			"Mithril" => Axe::Mithril,
			"Adamant" => Axe::Adamant,
			"Rune" => Axe::Rune,
			_ => Axe::Stone
		}
	}
}

impl From<Axe> for Bson {
	fn from(a: Axe) -> Self {
		match a {
			Axe::Stone => Bson::String("Stone".to_string()),
			Axe::Iron => Bson::String("Iron".to_string()),
			Axe::Steel => Bson::String("Steel".to_string()),
			Axe::Mithril => Bson::String("Mithril".to_string()),
			Axe::Adamant => Bson::String("Adamant".to_string()),
			Axe::Rune => Bson::String("Rune".to_string()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Logs {
	pub pine: i64,
	pub oak: i64,
	pub maple: i64,
	pub walnut: i64,
	pub cherry: i64,
	pub purpleheart: i64,
}

impl Default for Logs {
	fn default() -> Self {
		Self {
			pine: 0,
			oak: 0,
			maple: 0,
			walnut: 0,
			cherry: 0,
			purpleheart: 0,
		}
	}
}

impl ToDoc for Logs {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine,
			"oak": &self.oak,
			"maple": &self.maple,
			"walnut": &self.walnut,
			"cherry": &self.cherry,
			"purpleheart": &self.purpleheart,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lumber {
	pub pine: i64,
	pub oak: i64,
	pub maple: i64,
	pub walnut: i64,
	pub cherry: i64,
	pub purpleheart: i64,
}

impl Default for Lumber {
	fn default() -> Self {
		Self {
			pine: 0,
			oak: 0,
			maple: 0,
			walnut: 0,
			cherry: 0,
			purpleheart: 0,
		}
	}
}

impl ToDoc for Lumber {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine,
			"oak": &self.oak,
			"maple": &self.maple,
			"walnut": &self.walnut,
			"cherry": &self.cherry,
			"purpleheart": &self.purpleheart,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blueprints {
	pub pine: Vec<String>,
	pub oak: Vec<String>,
	pub maple: Vec<String>,
	pub walnut: Vec<String>,
	pub cherry: Vec<String>,
	pub purpleheart: Vec<String>,
}

impl Default for Blueprints {
	fn default() -> Self {
		Self {
			pine: vec!["birdhouse".to_string()],
			oak: vec![],
			maple: vec![],
			walnut: vec![],
			cherry: vec![],
			purpleheart: vec![],
		}
	}
}

impl ToDoc for Blueprints {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine,
			"oak": &self.oak,
			"maple": &self.maple,
			"walnut": &self.walnut,
			"cherry": &self.cherry,
			"purpleheart": &self.purpleheart,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Furniture {
	pub pine: Vec<String>,
	pub oak: Vec<String>,
	pub maple: Vec<String>,
	pub walnut: Vec<String>,
	pub cherry: Vec<String>,
	pub purpleheart: Vec<String>,
}

impl Default for Furniture {
	fn default() -> Self {
		Self {
			pine: vec![],
			oak: vec![],
			maple: vec![],
			walnut: vec![],
			cherry: vec![],
			purpleheart: vec![],
		}
	}
}

impl ToDoc for Furniture {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine,
			"oak": &self.oak,
			"maple": &self.maple,
			"walnut": &self.walnut,
			"cherry": &self.cherry,
			"purpleheart": &self.purpleheart,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Seeds {
	pub pine: i64,
	pub oak: i64,
	pub maple: i64,
	pub walnut: i64,
	pub cherry: i64,
	pub purpleheart: i64,
}

impl Default for Seeds {
	fn default() -> Self {
		Self {
			pine: 0,
			oak: 0,
			maple: 0,
			walnut: 0,
			cherry: 0,
			purpleheart: 0,
		}
	}
}

impl ToDoc for Seeds {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine,
			"oak": &self.oak,
			"maple": &self.maple,
			"walnut": &self.walnut,
			"cherry": &self.cherry,
			"purpleheart": &self.purpleheart,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Upgrades {
	pub sharper_axes: i64,
}

impl Default for Upgrades {
	fn default() -> Self {
		Self {
			sharper_axes: 0,
		}
	}
}

impl ToDoc for Upgrades {
	fn to_doc(&self) -> Document {
		doc! {
			"sharper_axes": &self.sharper_axes,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SawdustUpgrades {
	pub sharper_axes: i64,
}

impl Default for SawdustUpgrades {
	fn default() -> Self {
		Self {
			sharper_axes: 0,
		}
	}
}

impl ToDoc for SawdustUpgrades {
	fn to_doc(&self) -> Document {
		doc! {
			"sharper_axes": 0,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stats {
	pub times_prestiged: i64,
	pub pine_trees_chopped: i64,
	pub pine_logs_earned: i64,
	pub oak_trees_chopped: i64,
	pub oak_logs_earned: i64,
	pub maple_trees_chopped: i64,
	pub maple_logs_earned: i64,
	pub walnut_trees_chopped: i64,
	pub walnut_logs_earned: i64,
	pub cherry_trees_chopped: i64,
	pub cherry_logs_earned: i64,
	pub purpleheart_trees_chopped: i64,
	pub purpleheart_logs_earned: i64,
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

impl ToDoc for Stats {
	fn to_doc(&self) -> Document {
		doc! {
			"times_prestiged": &self.times_prestiged,
			"pine_trees_chopped": &self.pine_trees_chopped,
			"pine_logs_earned": &self.pine_logs_earned,
			"oak_trees_chopped": &self.oak_trees_chopped,
			"oak_logs_earned": &self.oak_logs_earned,
			"maple_trees_chopped": &self.maple_trees_chopped,
			"maple_logs_earned": &self.maple_logs_earned,
			"walnut_trees_chopped": &self.walnut_trees_chopped,
			"walnut_logs_earned": &self.walnut_logs_earned,
			"cherry_trees_chopped": &self.cherry_trees_chopped,
			"cherry_logs_earned": &self.cherry_logs_earned,
			"purpleheart_trees_chopped": &self.purpleheart_trees_chopped,
			"purpleheart_logs_earned": &self.purpleheart_logs_earned,
		}
	}
}

async fn get_player_collection() -> Collection<Player> {
	let client = get_client().await.unwrap();
	let collection = client.database("lumber-idle").collection::<Player>("players");

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