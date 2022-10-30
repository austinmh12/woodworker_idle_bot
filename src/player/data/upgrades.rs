use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};

use crate::utils::ToDoc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Upgrades {
	pub sharpening_books: i64, // Decreases time to chop
	pub gym_pass: i64, // Increases logs per chop
	pub sharper_axes: i64, // Decreases loggers time to chop
	pub wider_axes: i64, // Increases loggers logs per chop
	pub thermodynamics: i64, // Decreases time to dry
	pub pull_carts: i64, // Increases lumber per dry
	pub hotter_kilns: i64, // Decreases lumberers time to dry
	pub better_temperatures: i64, // Increases lumberers lumber per log dried
	pub fast_drying_glue: i64, // Decreases time to build
	pub industrial_nails: i64, // Increases builds per loop
	pub wd_40: i64, // Decreases cncs time to build
	pub high_quality_bits: i64, // Increases cnc builds per loop
}

impl Default for Upgrades {
	fn default() -> Self {
		Self {
			sharpening_books: 0,
			gym_pass: 0,
			sharper_axes: 0,
			wider_axes: 0,
			thermodynamics: 0,
			pull_carts: 0,
			hotter_kilns: 0,
			better_temperatures: 0,
			fast_drying_glue: 0,
			industrial_nails: 0,
			wd_40: 0,
			high_quality_bits: 0,
		}
	}
}

impl ToDoc for Upgrades {
	fn to_doc(&self) -> Document {
		doc! {
			"sharpening_books": self.sharpening_books,
			"gym_pass": self.gym_pass,
			"sharper_axes": self.sharper_axes,
			"wider_axes": self.wider_axes,
			"thermodynamics": self.thermodynamics,
			"pull_carts": self.pull_carts,
			"hotter_kilns": self.hotter_kilns,
			"better_temperatures": self.better_temperatures,
			"fast_drying_glue": self.fast_drying_glue,
			"industrial_nails": self.industrial_nails,
			"wd_40": self.wd_40,
			"high_quality_bits": self.high_quality_bits,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SawdustUpgrades {
	pub tree_fertilizer: i64, // Decreases time to chop
	pub less_bark: i64, // Increases logs per chop
	pub double_swings: i64, // Decreases loggers time to chop
	pub dual_wielding: i64, // Increases loggers logs per chop
	pub preheating: i64, // Decreases time to dry
	pub efficient_packing: i64, // Increases lumber per log
	pub electric_heaters: i64, // Decreases lumberers time to dry
	pub reading_glasses: i64, // Increases lumberers lumber per log dried
	pub longer_clamps: i64, // Decreases time to build
	pub self_tapping_screws: i64, // Increases builds per loop
	pub saved_gcode: i64, // Decreases cncs time to build
	pub stronger_motors: i64, // Increases cncs builds per loop
	pub dust_collection: i64, // Increases sawdust collection amount
	pub fire_starter: i64, // Increases the bonus of sawdust
	pub multitasking: i64, // Increases the max number of queued actions at a time
	pub endurance_training: i64, // Increased max number of actions per action
}

impl Default for SawdustUpgrades {
	fn default() -> Self {
		Self {
			tree_fertilizer: 0,
			less_bark: 0,
			double_swings: 0,
			dual_wielding: 0,
			preheating: 0,
			efficient_packing: 0,
			electric_heaters: 0,
			reading_glasses: 0,
			longer_clamps: 0,
			self_tapping_screws: 0,
			saved_gcode: 0,
			stronger_motors: 0,
			dust_collection: 0,
			fire_starter: 0,
			multitasking: 0,
			endurance_training: 0,
		}
	}
}

impl ToDoc for SawdustUpgrades {
	fn to_doc(&self) -> Document {
		doc! {
			"tree_fertilizer": self.tree_fertilizer,
			"less_bark": self.less_bark,
			"double_swings": self.double_swings,
			"dual_wielding": self.dual_wielding,
			"preheating": self.preheating,
			"efficient_packing": self.efficient_packing,
			"electric_heaters": self.electric_heaters,
			"reading_glasses": self.reading_glasses,
			"longer_clamps": self.longer_clamps,
			"self_tapping_screws": self.self_tapping_screws,
			"saved_gcode": self.saved_gcode,
			"stronger_motors": self.stronger_motors,
			"dust_collection": self.dust_collection,
			"fire_starter": self.fire_starter,
			"multitasking": self.multitasking,
			"endurance_training": self.endurance_training,
		}
	}
}