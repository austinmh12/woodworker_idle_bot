use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};
use serenity::{builder::CreateEmbed, utils::Colour};

use crate::utils::{ToDoc, default_colour};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Stats {
	pub cash_earned: f64,
	pub pine_trees_chopped: i64,
	pub pine_logs_earned: i64,
	pub pine_logs_dried: i64,
	pub pine_lumber_earned: i64,
	pub pine_birdhouses_built: i64,
	pub pine_shelves_built: i64,
	pub pine_side_tables_built: i64,
	pub pine_coffee_tables_built: i64,
	pub pine_dining_sets_built: i64,
	pub oak_trees_chopped: i64,
	pub oak_logs_earned: i64,
	pub oak_logs_dried: i64,
	pub oak_lumber_earned: i64,
	pub oak_birdhouses_built: i64,
	pub oak_shelves_built: i64,
	pub oak_side_tables_built: i64,
	pub oak_coffee_tables_built: i64,
	pub oak_dining_sets_built: i64,
	pub maple_trees_chopped: i64,
	pub maple_logs_earned: i64,
	pub maple_logs_dried: i64,
	pub maple_lumber_earned: i64,
	pub maple_birdhouses_built: i64,
	pub maple_shelves_built: i64,
	pub maple_side_tables_built: i64,
	pub maple_coffee_tables_built: i64,
	pub maple_dining_sets_built: i64,
	pub walnut_trees_chopped: i64,
	pub walnut_logs_earned: i64,
	pub walnut_logs_dried: i64,
	pub walnut_lumber_earned: i64,
	pub walnut_birdhouses_built: i64,
	pub walnut_shelves_built: i64,
	pub walnut_side_tables_built: i64,
	pub walnut_coffee_tables_built: i64,
	pub walnut_dining_sets_built: i64,
	pub cherry_trees_chopped: i64,
	pub cherry_logs_earned: i64,
	pub cherry_logs_dried: i64,
	pub cherry_lumber_earned: i64,
	pub cherry_birdhouses_built: i64,
	pub cherry_shelves_built: i64,
	pub cherry_side_tables_built: i64,
	pub cherry_coffee_tables_built: i64,
	pub cherry_dining_sets_built: i64,
	pub purpleheart_trees_chopped: i64,
	pub purpleheart_logs_earned: i64,
	pub purpleheart_logs_dried: i64,
	pub purpleheart_lumber_earned: i64,
	pub purpleheart_birdhouses_built: i64,
	pub purpleheart_shelves_built: i64,
	pub purpleheart_side_tables_built: i64,
	pub purpleheart_coffee_tables_built: i64,
	pub purpleheart_dining_sets_built: i64,
	pub times_sawdust_prestiged: i64,
	pub sawdust_earned: i64,
	pub times_seed_prestiged: i64,
	pub pine_seeds_earned: i64,
	pub oak_seeds_earned: i64,
	pub maple_seeds_earned: i64,
	pub walnut_seeds_earned: i64,
	pub cherry_seeds_earned: i64,
	pub purpleheart_seeds_earned: i64,
}

impl Stats {
	pub fn embed(&self, nickname: String, avatar: String) -> Vec<CreateEmbed> {
		let mut ret = vec![];
		let mut emb1 = CreateEmbed::default();
		let title = format!("{}'s Stats", nickname);
		emb1
			.title(title.clone())
			.thumbnail(avatar.clone())
			.description("***General Stats***")
			.colour(default_colour())
			.field("Cash Earned", self.cash_earned, false)
			.fields(vec![
				("Times Sawdust Prestiged", self.times_sawdust_prestiged, false),
				("Sawdust Earned", self.sawdust_earned, false),
				("Times Seed Prestiged", self.times_seed_prestiged, false),
			]);
		ret.push(emb1);
		for i in 0..6 {
			let (
				desc,
				(r, g, b),
				trees_chopped,
				logs_earned,
				logs_dried,
				lumber_earned,
				birdhouses_built,
				shelves_built,
				side_tables_built,
				coffee_tables_built,
				dining_sets_built,
				seeds_earned,
			) = match i {
				0 => {
					(
						"***Pine***",
						(178, 147, 116),
						self.pine_trees_chopped,
						self.pine_logs_earned,
						self.pine_logs_dried,
						self.pine_lumber_earned,
						self.pine_birdhouses_built,
						self.pine_shelves_built,
						self.pine_side_tables_built,
						self.pine_coffee_tables_built,
						self.pine_dining_sets_built,
						self.pine_seeds_earned,
					)
				},
				1 => {
					(
						"***Oak***",
						(211, 146, 90),
						self.oak_trees_chopped,
						self.oak_logs_earned,
						self.oak_logs_dried,
						self.oak_lumber_earned,
						self.oak_birdhouses_built,
						self.oak_shelves_built,
						self.oak_side_tables_built,
						self.oak_coffee_tables_built,
						self.oak_dining_sets_built,
						self.oak_seeds_earned,
					)
				},
				2 => {
					(
						"***Maple***",
						(233, 186, 134),
						self.maple_trees_chopped,
						self.maple_logs_earned,
						self.maple_logs_dried,
						self.maple_lumber_earned,
						self.maple_birdhouses_built,
						self.maple_shelves_built,
						self.maple_side_tables_built,
						self.maple_coffee_tables_built,
						self.maple_dining_sets_built,
						self.maple_seeds_earned,
					)
				},
				3 => {
					(
						"***Walnut***",
						(135, 93, 79),
						self.walnut_trees_chopped,
						self.walnut_logs_earned,
						self.walnut_logs_dried,
						self.walnut_lumber_earned,
						self.walnut_birdhouses_built,
						self.walnut_shelves_built,
						self.walnut_side_tables_built,
						self.walnut_coffee_tables_built,
						self.walnut_dining_sets_built,
						self.walnut_seeds_earned,
					)
				},
				4 => {
					(
						"***Cherry***",
						(124, 46, 42),
						self.cherry_trees_chopped,
						self.cherry_logs_earned,
						self.cherry_logs_dried,
						self.cherry_lumber_earned,
						self.cherry_birdhouses_built,
						self.cherry_shelves_built,
						self.cherry_side_tables_built,
						self.cherry_coffee_tables_built,
						self.cherry_dining_sets_built,
						self.cherry_seeds_earned,
					)
				},
				5 => {
					(
						"***Purple Heart***",
						(138, 93, 100),
						self.purpleheart_trees_chopped,
						self.purpleheart_logs_earned,
						self.purpleheart_logs_dried,
						self.purpleheart_lumber_earned,
						self.purpleheart_birdhouses_built,
						self.purpleheart_shelves_built,
						self.purpleheart_side_tables_built,
						self.purpleheart_coffee_tables_built,
						self.purpleheart_dining_sets_built,
						self.purpleheart_seeds_earned,
					)
				},
				_ => todo!()
			};

			let mut emb = CreateEmbed::default();
			emb
				.title(title.clone())
				.thumbnail(avatar.clone())
				.description(desc)
				.colour(Colour::from_rgb(r, g, b))
				.field("Trees Chopped", trees_chopped, true)
				.field("Logs Earned", logs_earned, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Logs Dried", logs_dried, true)
				.field("Lumber Earned", lumber_earned, true)
				.field("\u{200b}", "\u{200b}", false)
				.field("Bird Houses Built", birdhouses_built, false)
				.field("Shelves Built", shelves_built, false)
				.field("Side Tables Built", side_tables_built, false)
				.field("Coffee Tables Built", coffee_tables_built, false)
				.field("Dining Sets Built", dining_sets_built, false)
				.field("Seeds Earned", seeds_earned, false);

			ret.push(emb)
		}

		ret
	}
}

impl ToDoc for Stats {
	fn to_doc(&self) -> Document {
		let mut doc = Document::new();
		doc.insert("cash_earned", &self.cash_earned);
		doc.insert("pine_trees_chopped", &self.pine_trees_chopped);
		doc.insert("pine_logs_earned", &self.pine_logs_earned);
		doc.insert("pine_logs_dried", &self.pine_logs_dried);
		doc.insert("pine_lumber_earned", &self.pine_lumber_earned);
		doc.insert("pine_birdhouses_built", &self.pine_birdhouses_built);
		doc.insert("pine_shelves_built", &self.pine_shelves_built);
		doc.insert("pine_side_tables_built", &self.pine_side_tables_built);
		doc.insert("pine_coffee_tables_built", &self.pine_coffee_tables_built);
		doc.insert("pine_dining_sets_built", &self.pine_dining_sets_built);
		doc.insert("oak_trees_chopped", &self.oak_trees_chopped);
		doc.insert("oak_logs_earned", &self.oak_logs_earned);
		doc.insert("oak_logs_dried", &self.oak_logs_dried);
		doc.insert("oak_lumber_earned", &self.oak_lumber_earned);
		doc.insert("oak_birdhouses_built", &self.oak_birdhouses_built);
		doc.insert("oak_shelves_built", &self.oak_shelves_built);
		doc.insert("oak_side_tables_built", &self.oak_side_tables_built);
		doc.insert("oak_coffee_tables_built", &self.oak_coffee_tables_built);
		doc.insert("oak_dining_sets_built", &self.oak_dining_sets_built);
		doc.insert("maple_trees_chopped", &self.maple_trees_chopped);
		doc.insert("maple_logs_earned", &self.maple_logs_earned);
		doc.insert("maple_logs_dried", &self.maple_logs_dried);
		doc.insert("maple_lumber_earned", &self.maple_lumber_earned);
		doc.insert("maple_birdhouses_built", &self.maple_birdhouses_built);
		doc.insert("maple_shelves_built", &self.maple_shelves_built);
		doc.insert("maple_side_tables_built", &self.maple_side_tables_built);
		doc.insert("maple_coffee_tables_built", &self.maple_coffee_tables_built);
		doc.insert("maple_dining_sets_built", &self.maple_dining_sets_built);
		doc.insert("walnut_trees_chopped", &self.walnut_trees_chopped);
		doc.insert("walnut_logs_earned", &self.walnut_logs_earned);
		doc.insert("walnut_logs_dried", &self.walnut_logs_dried);
		doc.insert("walnut_lumber_earned", &self.walnut_lumber_earned);
		doc.insert("walnut_birdhouses_built", &self.walnut_birdhouses_built);
		doc.insert("walnut_shelves_built", &self.walnut_shelves_built);
		doc.insert("walnut_side_tables_built", &self.walnut_side_tables_built);
		doc.insert("walnut_coffee_tables_built", &self.walnut_coffee_tables_built);
		doc.insert("walnut_dining_sets_built", &self.walnut_dining_sets_built);
		doc.insert("cherry_trees_chopped", &self.cherry_trees_chopped);
		doc.insert("cherry_logs_earned", &self.cherry_logs_earned);
		doc.insert("cherry_logs_dried", &self.cherry_logs_dried);
		doc.insert("cherry_lumber_earned", &self.cherry_lumber_earned);
		doc.insert("cherry_birdhouses_built", &self.cherry_birdhouses_built);
		doc.insert("cherry_shelves_built", &self.cherry_shelves_built);
		doc.insert("cherry_side_tables_built", &self.cherry_side_tables_built);
		doc.insert("cherry_coffee_tables_built", &self.cherry_coffee_tables_built);
		doc.insert("cherry_dining_sets_built", &self.cherry_dining_sets_built);
		doc.insert("purpleheart_trees_chopped", &self.purpleheart_trees_chopped);
		doc.insert("purpleheart_logs_earned", &self.purpleheart_logs_earned);
		doc.insert("purpleheart_logs_dried", &self.purpleheart_logs_dried);
		doc.insert("purpleheart_lumber_earned", &self.purpleheart_lumber_earned);
		doc.insert("purpleheart_birdhouses_built", &self.purpleheart_birdhouses_built);
		doc.insert("purpleheart_shelves_built", &self.purpleheart_shelves_built);
		doc.insert("purpleheart_side_tables_built", &self.purpleheart_side_tables_built);
		doc.insert("purpleheart_coffee_tables_built", &self.purpleheart_coffee_tables_built);
		doc.insert("purpleheart_dining_sets_built", &self.purpleheart_dining_sets_built);
		doc.insert("times_sawdust_prestiged", &self.times_sawdust_prestiged);
		doc.insert("sawdust_earned", &self.sawdust_earned);
		doc.insert("times_seed_prestiged", &self.times_seed_prestiged);
		doc.insert("pine_seeds_earned", &self.pine_seeds_earned);
		doc.insert("oak_seeds_earned", &self.oak_seeds_earned);
		doc.insert("maple_seeds_earned", &self.maple_seeds_earned);
		doc.insert("walnut_seeds_earned", &self.walnut_seeds_earned);
		doc.insert("cherry_seeds_earned", &self.cherry_seeds_earned);
		doc.insert("purpleheart_seeds_earned", &self.purpleheart_seeds_earned);

		doc
	}
}