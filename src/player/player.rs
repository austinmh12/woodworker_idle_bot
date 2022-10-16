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
use serenity::{
	builder::CreateEmbed,
	utils::Colour
};

use crate::utils::{get_client, ToDoc};
use crate::player::{
	Axe,
	Furniture,
	Blueprints,
	Stats,
	Upgrades,
	SawdustUpgrades,
	WoodsInt,
	Action,
	Color,
	SawdustPrestige,
	SeedPrestige,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	id: Option<ObjectId>,
	pub discord_id: i64,
	pub cash: f64,
	pub axe: Axe,
	pub current_action: Action,
	pub logs: WoodsInt,
	pub loggers: i64,
	pub lumber: WoodsInt,
	pub lumberers: i64,
	pub blueprints: Blueprints,
	pub furniture: Furniture,
	pub cncs: i64,
	pub upgrades:  Upgrades,
	pub sawdust: i64,
	pub sawdust_total: i64,
	pub sawdust_upgrades: SawdustUpgrades,
	pub seeds: WoodsInt,
	pub stats: Stats,
	pub color: Color,
	pub sawdust_prestige: SawdustPrestige,
	pub seed_prestige: SeedPrestige,
}

impl Player {
	pub fn new(discord_id: u64) -> Self {
		Self {
			id: None,
			discord_id: discord_id as i64,
			cash: 0.0,
			axe: Axe::Stone,
			current_action: Action::none(),
			logs: WoodsInt::default(),
			loggers: 0,
			lumber: WoodsInt::default(),
			lumberers: 0,
			blueprints: Blueprints::default(),
			furniture: Furniture::default(),
			cncs: 0,
			upgrades: Upgrades::default(),
			sawdust: 0,
			sawdust_total: 0,
			sawdust_upgrades: SawdustUpgrades::default(),
			seeds: WoodsInt::default(),
			stats: Stats::default(),
			color: Color::default(),
			sawdust_prestige: SawdustPrestige::default(),
			seed_prestige: SeedPrestige::default(),
		}
	}

	pub async fn update(&self) {
		let player_collection = get_player_collection().await;
		let update = self.to_doc();
		player_collection
			.update_one(
				doc! {"_id": &self.id.unwrap() }, 
				update, 
				None)
			.await
			.unwrap();
	}

	pub async fn embed(&self, nickname: String, avatar: String) -> CreateEmbed {
		// let daily_reset_local: DateTime<Local> = DateTime::from(self.daily_reset);
		let mut desc = format!("**Wallet:** ${:.2}\n", &self.cash);
		desc.push_str(&format!("**Axe:** {}\n\n", &self.axe));
		desc.push_str(&format!("**Current Action:** {}\n\n", &self.current_action));
		desc.push_str("__**Total Logs and Lumber:**__\n");
		desc.push_str(&format!("<:GameCornerBlank:1030960408145698816> **Pine:** {} | {}\n", self.logs.pine, self.lumber.pine));
		desc.push_str(&format!("<:GameCornerBlank:1030960408145698816> **Oak:** {} | {}\n", self.logs.oak, self.lumber.oak));
		desc.push_str(&format!("<:GameCornerBlank:1030960408145698816> **Maple:** {} | {}\n", self.logs.maple, self.lumber.maple));
		desc.push_str(&format!("<:GameCornerBlank:1030960408145698816> **Walnut:** {} | {}\n", self.logs.walnut, self.lumber.walnut));
		desc.push_str(&format!("<:GameCornerBlank:1030960408145698816> **Cherry:** {} | {}\n", self.logs.cherry, self.lumber.cherry));
		desc.push_str(&format!("<:GameCornerBlank:1030960408145698816> **Purpleheart:** {} | {}", self.logs.purpleheart, self.lumber.purpleheart));

		let mut ret = CreateEmbed::default();
		ret
			.title(format!("{}'s profile", nickname))
			.thumbnail(avatar)
			.description(desc)
			.colour(Colour::from_rgb(self.color.red, self.color.green, self.color.blue));

		ret
	}
}

impl ToDoc for Player {
	fn to_doc(&self) -> Document {
		doc! {
			"$set": {
				"cash": &self.cash,
				"axe": &self.axe,
				"current_action": &self.current_action.to_doc(),
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
				"color": &self.color.to_doc(),
				"sawdust_prestige": &self.sawdust_prestige.to_doc(),
				"seed_prestige": &self.seed_prestige.to_doc(),
			}
		}
	}
}

async fn get_player_collection() -> Collection<Player> {
	let client = get_client().await.unwrap();
	let collection = client.database("lumber-idle").collection::<Player>("players");

	collection
}

// Database functions
pub async fn get_players() -> Vec<Player> {
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

pub async fn get_player(discord_id: u64) -> Player {
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