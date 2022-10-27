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
		let mut points = self.cash as i64; // Base points is cash
		
		// Pine calculations
		let mut pine = 0;
		pine += self.logs.pine * 10;
		pine += self.lumber.pine * 10;
		pine += self.furniture.pine.birdhouse * 10;
		pine += self.furniture.pine.shelf * 15;
		pine += self.furniture.pine.side_table * 20;
		pine += self.furniture.pine.coffee_table * 25;
		pine += self.furniture.pine.dining_set * 30;
		points += pine * 10;

		// Oak calculations
		let mut oak = 0;
		oak += self.logs.oak * 15;
		oak += self.lumber.oak * 15;
		oak += self.furniture.oak.birdhouse * 15;
		oak += self.furniture.oak.shelf * 22;
		oak += self.furniture.oak.side_table * 30;
		oak += self.furniture.oak.coffee_table * 37;
		oak += self.furniture.oak.dining_set * 45;
		points += oak * 20;

		// Maple
		let mut maple = 0;
		maple += self.logs.maple * 25;
		maple += self.lumber.maple * 25;
		maple += self.furniture.maple.birdhouse * 25;
		maple += self.furniture.maple.shelf * 37;
		maple += self.furniture.maple.side_table * 50;
		maple += self.furniture.maple.coffee_table * 62;
		maple += self.furniture.maple.dining_set * 75;
		points += maple * 30;

		// Walnut
		let mut walnut = 0;
		walnut += self.logs.walnut * 35;
		walnut += self.lumber.walnut * 35;
		walnut += self.furniture.walnut.birdhouse * 35;
		walnut += self.furniture.walnut.shelf * 52;
		walnut += self.furniture.walnut.side_table * 70;
		walnut += self.furniture.walnut.coffee_table * 87;
		walnut += self.furniture.walnut.dining_set * 105;
		points += walnut * 40;

		// Cherry
		let mut cherry = 0;
		cherry += self.logs.cherry * 50;
		cherry += self.lumber.cherry * 50;
		cherry += self.furniture.cherry.birdhouse * 50;
		cherry += self.furniture.cherry.shelf * 75;
		cherry += self.furniture.cherry.side_table * 100;
		cherry += self.furniture.cherry.coffee_table * 125;
		cherry += self.furniture.cherry.dining_set * 150;
		points += cherry * 50;

		// Purpleheart
		let mut purpleheart = 0;
		purpleheart += self.logs.purpleheart * 80;
		purpleheart += self.lumber.purpleheart * 80;
		purpleheart += self.furniture.purpleheart.birdhouse * 80;
		purpleheart += self.furniture.purpleheart.shelf * 120;
		purpleheart += self.furniture.purpleheart.side_table * 160;
		purpleheart += self.furniture.purpleheart.coffee_table * 200;
		purpleheart += self.furniture.purpleheart.dining_set * 240;
		points += purpleheart * 60;

		(points as f64 / 10.0_f64.powi(6)).powf(0.28) as i64
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

// impl SeedPrestige {
// 	pub fn calculate_seeds(&self) -> i64 {
// 		let mut points = (self.cash / 10.0) as i64; // Cash means less here
// 		points += self.sawdust;

// 		// Pine calculations
// 		let mut pine = 0;
// 		pine += self.logs.pine * 10;
// 		pine += self.lumber.pine * 10;
// 		pine += self.furniture.pine.birdhouse * 10;
// 		pine += self.furniture.pine.shelf * 15;
// 		pine += self.furniture.pine.side_table * 20;
// 		pine += self.furniture.pine.coffee_table * 25;
// 		pine += self.furniture.pine.dining_set * 30;
// 		points += pine * 10;

// 		// Oak calculations
// 		let mut oak = 0;
// 		oak += self.logs.oak * 15;
// 		oak += self.lumber.oak * 15;
// 		oak += self.furniture.oak.birdhouse * 15;
// 		oak += self.furniture.oak.shelf * 22;
// 		oak += self.furniture.oak.side_table * 30;
// 		oak += self.furniture.oak.coffee_table * 37;
// 		oak += self.furniture.oak.dining_set * 45;
// 		points += oak * 20;

// 		// Maple
// 		let mut maple = 0;
// 		maple += self.logs.maple * 25;
// 		maple += self.lumber.maple * 25;
// 		maple += self.furniture.maple.birdhouse * 25;
// 		maple += self.furniture.maple.shelf * 37;
// 		maple += self.furniture.maple.side_table * 50;
// 		maple += self.furniture.maple.coffee_table * 62;
// 		maple += self.furniture.maple.dining_set * 75;
// 		points += maple * 30;

// 		// Walnut
// 		let mut walnut = 0;
// 		walnut += self.logs.walnut * 35;
// 		walnut += self.lumber.walnut * 35;
// 		walnut += self.furniture.walnut.birdhouse * 35;
// 		walnut += self.furniture.walnut.shelf * 52;
// 		walnut += self.furniture.walnut.side_table * 70;
// 		walnut += self.furniture.walnut.coffee_table * 87;
// 		walnut += self.furniture.walnut.dining_set * 105;
// 		points += walnut * 40;

// 		// Cherry
// 		let mut cherry = 0;
// 		cherry += self.logs.cherry * 50;
// 		cherry += self.lumber.cherry * 50;
// 		cherry += self.furniture.cherry.birdhouse * 50;
// 		cherry += self.furniture.cherry.shelf * 75;
// 		cherry += self.furniture.cherry.side_table * 100;
// 		cherry += self.furniture.cherry.coffee_table * 125;
// 		cherry += self.furniture.cherry.dining_set * 150;
// 		points += cherry * 50;

// 		// Purpleheart
// 		let mut purpleheart = 0;
// 		purpleheart += self.logs.purpleheart * 80;
// 		purpleheart += self.lumber.purpleheart * 80;
// 		purpleheart += self.furniture.purpleheart.birdhouse * 80;
// 		purpleheart += self.furniture.purpleheart.shelf * 120;
// 		purpleheart += self.furniture.purpleheart.side_table * 160;
// 		purpleheart += self.furniture.purpleheart.coffee_table * 200;
// 		purpleheart += self.furniture.purpleheart.dining_set * 240;
// 		points += purpleheart * 60;

// 		(points as f64 / 10.0_f64.powi(7)).powf(0.14) as i64
// 	}
// }

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