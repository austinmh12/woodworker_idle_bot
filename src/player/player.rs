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
	ActionEnum,
	Color,
	SawdustPrestige,
	SeedPrestige,
	Kiln,
	Hammer,
	Tree,
	OfflineTimer,
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
	pub offline_timer: OfflineTimer,
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
			offline_timer: OfflineTimer::default(),
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

	pub fn embed(&self, nickname: String, avatar: String) -> CreateEmbed {
		// let daily_reset_local: DateTime<Local> = DateTime::from(self.daily_reset);
		let mut desc = format!("**Wallet:** ${:.2}\n", &self.cash);
		desc.push_str(&format!("**Axe:** {}\n", &self.axe));
		desc.push_str(&format!("**Kiln:** {}\n", &self.kiln));
		desc.push_str(&format!("**Hammer:** {}\n\n", &self.hammer));
		desc.push_str(&format!("**Current Action:** {}\n", &self.current_action));
		desc.push_str("**Queued Actions**\n");
		for queued_action in &self.queued_actions {
			desc.push_str(&format!("{}\n", queued_action));
		}

		let mut ret = CreateEmbed::default();
		ret
			.title(format!("{}'s Profile", nickname))
			.thumbnail(avatar)
			.description(desc)
			.colour(Colour::from_rgb(self.color.red, self.color.green, self.color.blue));

		ret
	}

	pub fn inventory_pages(&self, nickname: String, avatar: String) -> Vec<CreateEmbed> {
		
		fn bool_to_emoji(b: bool) -> &'static str {
			match b {
				true => ":white_check_mark:",
				false => ":x:"
			}
		}

		let mut ret = vec![];
		for i in 0..6 {
			let (
				desc,
				(r, g, b),
				logs,
				loggers,
				lumber,
				lumberers,
				birdhouse_bp,
				birdhouses,
				cnc_birdhouse,
				shelf_bp,
				shelfs,
				cnc_shelf,
				sidetable_bp,
				sidetables,
				cnc_sidetable,
				coffeetable_bp,
				coffeetables,
				cnc_coffeetable,
				diningset_bp,
				diningsets,
				cnc_diningset,
			) = match i {
				0 => {
					(
						"***Pine***",
						(178, 147, 116),
						self.logs.pine,
						self.loggers_active.pine,
						self.lumber.pine,
						self.lumberers_active.pine,
						self.blueprints.pine.birdhouse,
						self.furniture.pine.birdhouse,
						self.cncs_active.pine.birdhouse,
						self.blueprints.pine.shelf,
						self.furniture.pine.shelf,
						self.cncs_active.pine.shelf,
						self.blueprints.pine.side_table,
						self.furniture.pine.side_table,
						self.cncs_active.pine.side_table,
						self.blueprints.pine.coffee_table,
						self.furniture.pine.coffee_table,
						self.cncs_active.pine.coffee_table,
						self.blueprints.pine.dining_set,
						self.furniture.pine.dining_set,
						self.cncs_active.pine.dining_set,
					)
				},
				1 => {
					(
						"***Oak***",
						(211, 146, 90),
						self.logs.oak,
						self.loggers_active.oak,
						self.lumber.oak,
						self.lumberers_active.oak,
						self.blueprints.oak.birdhouse,
						self.furniture.oak.birdhouse,
						self.cncs_active.oak.birdhouse,
						self.blueprints.oak.shelf,
						self.furniture.oak.shelf,
						self.cncs_active.oak.shelf,
						self.blueprints.oak.side_table,
						self.furniture.oak.side_table,
						self.cncs_active.oak.side_table,
						self.blueprints.oak.coffee_table,
						self.furniture.oak.coffee_table,
						self.cncs_active.oak.coffee_table,
						self.blueprints.oak.dining_set,
						self.furniture.oak.dining_set,
						self.cncs_active.oak.dining_set,
					)
				},
				2 => {
					(
						"***Maple***",
						(233, 186, 134),
						self.logs.maple,
						self.loggers_active.maple,
						self.lumber.maple,
						self.lumberers_active.maple,
						self.blueprints.maple.birdhouse,
						self.furniture.maple.birdhouse,
						self.cncs_active.maple.birdhouse,
						self.blueprints.maple.shelf,
						self.furniture.maple.shelf,
						self.cncs_active.maple.shelf,
						self.blueprints.maple.side_table,
						self.furniture.maple.side_table,
						self.cncs_active.maple.side_table,
						self.blueprints.maple.coffee_table,
						self.furniture.maple.coffee_table,
						self.cncs_active.maple.coffee_table,
						self.blueprints.maple.dining_set,
						self.furniture.maple.dining_set,
						self.cncs_active.maple.dining_set,
					)
				},
				3 => {
					(
						"***Walnut***",
						(135, 93, 79),
						self.logs.walnut,
						self.loggers_active.walnut,
						self.lumber.walnut,
						self.lumberers_active.walnut,
						self.blueprints.walnut.birdhouse,
						self.furniture.walnut.birdhouse,
						self.cncs_active.walnut.birdhouse,
						self.blueprints.walnut.shelf,
						self.furniture.walnut.shelf,
						self.cncs_active.walnut.shelf,
						self.blueprints.walnut.side_table,
						self.furniture.walnut.side_table,
						self.cncs_active.walnut.side_table,
						self.blueprints.walnut.coffee_table,
						self.furniture.walnut.coffee_table,
						self.cncs_active.walnut.coffee_table,
						self.blueprints.walnut.dining_set,
						self.furniture.walnut.dining_set,
						self.cncs_active.walnut.dining_set,
					)
				},
				4 => {
					(
						"***Cherry***",
						(124, 46, 42),
						self.logs.cherry,
						self.loggers_active.cherry,
						self.lumber.cherry,
						self.lumberers_active.cherry,
						self.blueprints.cherry.birdhouse,
						self.furniture.cherry.birdhouse,
						self.cncs_active.cherry.birdhouse,
						self.blueprints.cherry.shelf,
						self.furniture.cherry.shelf,
						self.cncs_active.cherry.shelf,
						self.blueprints.cherry.side_table,
						self.furniture.cherry.side_table,
						self.cncs_active.cherry.side_table,
						self.blueprints.cherry.coffee_table,
						self.furniture.cherry.coffee_table,
						self.cncs_active.cherry.coffee_table,
						self.blueprints.cherry.dining_set,
						self.furniture.cherry.dining_set,
						self.cncs_active.cherry.dining_set,
					)
				},
				5 => {
					(
						"***Purple Heart***",
						(138, 93, 100),
						self.logs.purpleheart,
						self.loggers_active.purpleheart,
						self.lumber.purpleheart,
						self.lumberers_active.purpleheart,
						self.blueprints.purpleheart.birdhouse,
						self.furniture.purpleheart.birdhouse,
						self.cncs_active.purpleheart.birdhouse,
						self.blueprints.purpleheart.shelf,
						self.furniture.purpleheart.shelf,
						self.cncs_active.purpleheart.shelf,
						self.blueprints.purpleheart.side_table,
						self.furniture.purpleheart.side_table,
						self.cncs_active.purpleheart.side_table,
						self.blueprints.purpleheart.coffee_table,
						self.furniture.purpleheart.coffee_table,
						self.cncs_active.purpleheart.coffee_table,
						self.blueprints.purpleheart.dining_set,
						self.furniture.purpleheart.dining_set,
						self.cncs_active.purpleheart.dining_set,
					)
				},
				_ => todo!()
			};
			let mut emb = CreateEmbed::default();
			emb
				.title(format!("{}'s Inventory", nickname))
				.thumbnail(avatar.clone())
				.description(desc)
				.colour(Colour::from_rgb(r, g, b))
				.field("Logs", logs, true)
				.field("Loggers Assigned", loggers, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Lumber", lumber, true)
				.field("Lumberers Assigned", lumberers, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Bird House BP", bool_to_emoji(birdhouse_bp), true)
				.field("Bird Houses", birdhouses, true)
				.field("CNCs Assigned", cnc_birdhouse, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Shelf BP", bool_to_emoji(shelf_bp), true)
				.field("Shelves", shelfs, true)
				.field("CNCs Assigned", cnc_shelf, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Side Table BP", bool_to_emoji(sidetable_bp), true)
				.field("Side Tables", sidetables, true)
				.field("CNCs Assigned", cnc_sidetable, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Coffee Table BP", bool_to_emoji(coffeetable_bp), true)
				.field("Coffee Tables", coffeetables, true)
				.field("CNCs Assigned", cnc_coffeetable, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Dining Set BP", bool_to_emoji(diningset_bp), true)
				.field("Dining Sets", diningsets, true)
				.field("CNCs Assigned", cnc_diningset, true);

			ret.push(emb);
		}

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

	pub fn unlock_next_blueprint(&mut self) -> Option<Tree> {
		match self.blueprints.next_unlock() {
			Some(t) => {
				let ret = t.clone();
				self.blueprints.unlock(t);

				Some(ret)
			},
			None => None
		}
	}

	pub fn assigned_loggers(&self) -> i64 {
		self.loggers_active.total()
	}

	pub fn available_loggers(&self) -> i64 {
		if self.loggers - self.assigned_loggers() < 0 {
			0
		} else {
			self.loggers - self.assigned_loggers()
		}
	}

	pub fn assigned_lumberers(&self) -> i64 {
		self.lumberers_active.total()
	}

	pub fn available_lumberers(&self) -> i64 {
		if self.lumberers - self.assigned_lumberers() < 0 {
			0
		} else {
			self.lumberers - self.assigned_lumberers()
		}
	}

	pub fn assigned_cncs(&self) -> i64 {
		self.cncs_active.total()
	}

	pub fn available_cncs(&self) -> i64 {
		if self.cncs - self.assigned_cncs() < 0 {
			0
		} else {
			self.cncs - self.assigned_cncs()
		}
	}

	pub fn perform_sawdust_prestige(&mut self, sawdust_earned: i64) {
		self.cash = 0.0;
		self.axe = Axe::Stone;
		self.kiln = Kiln::None;
		self.hammer = Hammer::None;
		self.current_action = Action::none();
		self.queued_actions = vec![];
		self.logs = WoodsInt::default();
		self.loggers = 0;
		self.loggers_active = WoodsInt::default();
		self.lumber = WoodsInt::default();
		self.lumberers = 0;
		self.lumberers_active = WoodsInt::default();
		self.furniture = Furniture::default();
		self.cncs = 0;
		self.cncs_active = Furniture::default();
		self.upgrades = Upgrades::default();
		self.sawdust += sawdust_earned;
		self.seed_prestige.sawdust += sawdust_earned;
		self.stats.sawdust_earned += sawdust_earned;
		self.sawdust_total += sawdust_earned;
		self.sawdust_prestige = SawdustPrestige::default();
	}

	pub fn perform_seed_prestige(&mut self) {

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
				"offline_timer": &self.offline_timer.to_doc(),
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