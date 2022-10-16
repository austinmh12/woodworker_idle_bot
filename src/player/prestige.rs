use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	}, 
};

use crate::player::{
	WoodsInt,
	Furniture
};
use crate::utils::ToDoc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SawdustPrestige {
	pub cash: f64,
	pub logs: WoodsInt,
	pub lumber: WoodsInt,
	pub furniture: Furniture,
}

impl SawdustPrestige {
	pub fn calculate_sawdust(&self) -> i64 {
		let mut points = f64::log10(self.cash) / 100.0;
		points += self.logs.pine as f64 / 100000.0;
		points += self.logs.oak as f64 / 50000.0;
		points += self.logs.maple as f64 / 25000.0;
		points += self.logs.walnut as f64 / 20000.0;
		points += self.logs.cherry as f64 / 15000.0;
		points += self.logs.purpleheart as f64 / 10000.0;
		points += self.lumber.pine as f64 / 90000.0;
		points += self.lumber.oak as f64 / 45000.0;
		points += self.lumber.maple as f64 / 20000.0;
		points += self.lumber.walnut as f64 / 15000.0;
		points += self.lumber.cherry as f64 / 10000.0;
		points += self.lumber.purpleheart as f64 / 8000.0;


		f64::log10(points) as i64
	}
}

impl Default for SawdustPrestige {
	fn default() -> Self {
		Self {
			cash: 0.0,
			logs: WoodsInt::default(),
			lumber: WoodsInt::default(),
			furniture: Furniture::default(),
		}
	}
}

impl ToDoc for SawdustPrestige {
	fn to_doc(&self) -> bson::Document {
		doc! {
			"cash": self.cash,
			"logs": self.logs.to_doc(),
			"lumber": self.lumber.to_doc(),
			"furniture": self.furniture.to_doc(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeedPrestige {
	pub cash: f64,
	pub logs: WoodsInt,
	pub lumber: WoodsInt,
	pub furniture: Furniture,
	pub sawdust: i64,
}

impl SeedPrestige {
	pub fn calculate_seeds(&self) -> i64 {
		let mut points = self.cash / 100.0;
		points += self.logs.pine as f64;


		f64::log10(points) as i64
	}
}

impl Default for SeedPrestige {
	fn default() -> Self {
		Self {
			cash: 0.0,
			logs: WoodsInt::default(),
			lumber: WoodsInt::default(),
			furniture: Furniture::default(),
			sawdust: 0,
		}
	}
}

impl ToDoc for SeedPrestige {
	fn to_doc(&self) -> bson::Document {
		doc! {
			"cash": self.cash,
			"logs": self.logs.to_doc(),
			"lumber": self.lumber.to_doc(),
			"furniture": self.furniture.to_doc(),
			"sawdust": self.sawdust,
		}
	}
}