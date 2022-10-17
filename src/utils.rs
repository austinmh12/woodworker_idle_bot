use std::error::Error;
use mongodb::{
	Client,
	options::{
		ClientOptions,
	},
	bson::{
		Document,
	}, 
};
use serenity::utils::Colour;

use crate::player::{Player, Color};

pub trait ToDoc {
	fn to_doc(&self) -> Document;
}

pub fn get_tree_time(player: &Player, tree: &str) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 30.0,
		"walnut" => 45.0,
		"cherry" => 90.0,
		"purpleheart" => 300.0,
		_ => 10.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.sharper_axes as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.sharper_axes as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult) as i64
}

pub fn get_dry_time(player: &Player, tree: &str) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 30.0,
		"walnut" => 45.0,
		"cherry" => 90.0,
		"purpleheart" => 300.0,
		_ => 10.0
	};
	// TODO: Update to the equivalent drying upgrades
	let upgrade_mult = 1.0 + (player.upgrades.hotter_kilns as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.hotter_kilns as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult) as i64
}

pub fn get_build_time(player: &Player, tree: &str, furniture: &str) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 30.0,
		"walnut" => 45.0,
		"cherry" => 90.0,
		"purpleheart" => 300.0,
		_ => 10.0
	};
	let furniture_mult = match furniture {
		"birdhouse" => 1.0,
		"shelf" => 1.5,
		"side table" => 2.0,
		"coffee table" => 3.5,
		"dining set" => 5.0,
		_ => 1.0
	};
	let base_time = base_time * furniture_mult;
	// TODO: Update to the equivalent building upgrades
	let upgrade_mult = 1.0 + (player.upgrades.sharper_axes as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.sharper_axes as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult) as i64
}

pub async fn get_client() -> Result<Client, Box<dyn Error>> {
	let mon_client_uri = dotenv::var("MONGODB_URI").expect("No mongodb uri");
	let options = ClientOptions::parse(&mon_client_uri).await?;
	let client = Client::with_options(options)?;
	
	Ok(client)
}

pub fn default_colour() -> Colour {
	let color = Color::default();

	Colour::from_rgb(color.red, color.green, color.blue)
}