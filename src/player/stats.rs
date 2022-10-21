use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};

use crate::utils::ToDoc;

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