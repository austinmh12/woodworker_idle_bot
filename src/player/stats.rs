use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};

use crate::utils::ToDoc;

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