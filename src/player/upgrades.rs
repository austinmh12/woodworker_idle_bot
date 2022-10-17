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
	pub sharper_axes: i64, // Decreases time to chop
	pub wider_axes: i64, // Increases logs per chop
	pub hotter_kilns: i64, // Decreases time to dry
	pub better_temperatures: i64, // Increases lumber per log dried
	pub fast_drying_glue: i64, // Decreases time to build
	pub industrial_nails: i64, // Increases builds per loop
	pub accurate_measuring: i64, // Decreases lumber used when building
	pub multitasking: i64, // Increases the max number of actions at a time
}

impl Default for Upgrades {
	fn default() -> Self {
		Self {
			sharper_axes: 0,
			wider_axes: 0,
			hotter_kilns: 0,
			better_temperatures: 0,
			fast_drying_glue: 0,
			industrial_nails: 0,
			accurate_measuring: 0,
			multitasking: 0,
		}
	}
}

impl ToDoc for Upgrades {
	fn to_doc(&self) -> Document {
		doc! {
			"sharper_axes": &self.sharper_axes,
			"wider_axes": &self.wider_axes,
			"hotter_kilns": self.hotter_kilns,
			"better_temperatures": self.better_temperatures,
			"fast_drying_glue": self.fast_drying_glue,
			"industrial_nails": self.industrial_nails,
			"accurate_measuring": self.accurate_measuring,
			"multitasking": self.multitasking,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SawdustUpgrades {
	pub sharper_axes: i64, // Decreases time to chop
	pub wider_axes: i64, // Increases logs per chop
	pub hotter_kilns: i64, // Decreases time to dry
	pub better_temperatures: i64, // Increases lumber per log dried
	pub fast_drying_glue: i64, // Decreases time to build
	pub industrial_nails: i64, // Increases builds per loop
	pub accurate_measuring: i64, // Decreases lumber used when building
	pub multitasking: i64, // Increases the max number of actions at a time
}

impl Default for SawdustUpgrades {
	fn default() -> Self {
		Self {
			sharper_axes: 0,
			wider_axes: 0,
			hotter_kilns: 0,
			better_temperatures: 0,
			fast_drying_glue: 0,
			industrial_nails: 0,
			accurate_measuring: 0,
			multitasking: 0,
		}
	}
}

impl ToDoc for SawdustUpgrades {
	fn to_doc(&self) -> Document {
		doc! {
			"sharper_axes": &self.sharper_axes,
			"wider_axes": &self.wider_axes,
			"hotter_kilns": self.hotter_kilns,
			"better_temperatures": self.better_temperatures,
			"fast_drying_glue": self.fast_drying_glue,
			"industrial_nails": self.industrial_nails,
			"accurate_measuring": self.accurate_measuring,
			"multitasking": self.multitasking,
		}
	}
}