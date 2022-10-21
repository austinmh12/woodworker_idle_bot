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
use chrono::{
	DateTime, 
	Utc,
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
	ActionEnum,
	Color,
	SawdustPrestige,
	SeedPrestige,
	Kiln,
	Hammer,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	id: Option<ObjectId>,
	pub discord_id: i64,
	pub cash: f64,
	pub axe: Axe,
	pub kiln: Kiln,
	pub hammer: Hammer,
	pub current_action: Action,
	pub queued_actions: Vec<Action>,
	pub logs: WoodsInt,
	pub loggers: i64,
	pub loggers_active: WoodsInt,
	pub lumber: WoodsInt,
	pub lumberers: i64,
	pub lumberers_active: WoodsInt,
	pub blueprints: Blueprints,
	pub furniture: Furniture,
	pub cncs: i64,
	pub cncs_active: Furniture,
	pub upgrades:  Upgrades,
	pub sawdust: i64,
	pub sawdust_total: i64,
	pub sawdust_upgrades: SawdustUpgrades,
	pub seeds: WoodsInt,
	pub stats: Stats,
	pub color: Color,
	pub sawdust_prestige: SawdustPrestige,
	pub seed_prestige: SeedPrestige,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub last_updated: DateTime<Utc>,
}

impl Player {
	pub fn new(discord_id: u64) -> Self {
		Self {
			id: None,
			discord_id: discord_id as i64,
			cash: 0.0,
			axe: Axe::Stone,
			kiln: Kiln::None,
			hammer: Hammer::None,
			current_action: Action::none(),
			queued_actions: vec![],
			logs: WoodsInt::default(),
			loggers: 0,
			loggers_active: WoodsInt::default(),
			lumber: WoodsInt::default(),
			lumberers: 0,
			lumberers_active: WoodsInt::default(),
			blueprints: Blueprints::default(),
			furniture: Furniture::default(),
			cncs: 0,
			cncs_active: Furniture::default(),
			upgrades: Upgrades::default(),
			sawdust: 0,
			sawdust_total: 0,
			sawdust_upgrades: SawdustUpgrades::default(),
			seeds: WoodsInt::default(),
			stats: Stats::default(),
			color: Color::default(),
			sawdust_prestige: SawdustPrestige::default(),
			seed_prestige: SeedPrestige::default(),
			last_updated: Utc::now(),
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
		desc.push_str(&format!("**Axe:** {}\n", &self.axe));
		desc.push_str(&format!("**Kiln:** {}\n", &self.kiln));
		desc.push_str(&format!("**Hammer:** {}\n\n", &self.hammer));
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

	pub fn queue_action(&mut self, a: Action) -> Action {
		// Pushes the action on the queued_actions and returns it with it's new start/end time
		let mut action = a.clone();
		let duration = action.end - action.start;
		if self.queued_actions.len() > 0usize {
			let last_action = self.queued_actions.get(self.queued_actions.len() - 1usize).unwrap();
			action.start = last_action.end;
			action.end = action.start + duration;
		} else {
			action.start = self.current_action.end;
			action.end = action.start + duration;
		}
		self.queued_actions.push(action.clone());
		// self.update().await;

		action
	}

	pub fn queued_logs(&self, req: &str) -> i64 {
		let mut all_actions = vec![self.current_action.clone()];
		for qa in &self.queued_actions {
			all_actions.push(qa.clone());
		}
		let req = all_actions
			.iter()
			.filter(|a| a.action == ActionEnum::Chopping && a.tree == req)
			.map(|a| a.amount)
			.sum::<i64>();
		
		req
	}

	pub fn queued_lumber(&self, req: &str) -> i64 {
		let mut all_actions = vec![self.current_action.clone()];
		for qa in &self.queued_actions {
			all_actions.push(qa.clone());
		}
		let req = all_actions
			.iter()
			.filter(|a| a.action == ActionEnum::Drying && a.tree == req)
			.map(|a| a.amount)
			.sum::<i64>();
		
		req
	}
}

impl ToDoc for Player {
	fn to_doc(&self) -> Document {
		doc! {
			"$set": {
				"cash": &self.cash,
				"axe": &self.axe,
				"kiln": &self.kiln,
				"hammer": &self.hammer,
				"current_action": &self.current_action.to_doc(),
				"queued_actions": &self.queued_actions.iter().map(|a| a.to_doc()).collect::<Vec<Document>>(),
				"logs": &self.logs.to_doc(),
				"loggers": &self.loggers,
				"loggers_active": &self.loggers_active.to_doc(),
				"lumber": &self.lumber.to_doc(),
				"lumberers": &self.lumberers,
				"lumberers_active": &self.lumberers_active.to_doc(),
				"blueprints": &self.blueprints.to_doc(),
				"furniture": &self.furniture.to_doc(),
				"cncs": &self.cncs,
				"cncs_active": &self.cncs_active.to_doc(),
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