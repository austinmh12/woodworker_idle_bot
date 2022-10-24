use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};
use chrono::{
	Utc,
};

use crate::player::{get_player, Axe, Kiln, Hammer};
use crate::utils::Message;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> Message {
	let unit = &options
		.get(0)
		.expect("Expected a Subcommand");
	let tree = &unit
		.options
		.get(0)
		.expect("Subcommand");
	
	let mut player = get_player(player_id).await;

	match unit.name.as_str() {
		"loggers" => {
			let amount = match &tree
				.options
				.get(0)
				.expect("int")
				.resolved
				.as_ref()
				.expect("int") 
			{
				CommandDataOptionValue::Integer(i) => i.to_owned(),
				_ => 0
			};
			match tree.name.as_str() {
				"pine" => {
					let amounts = vec![amount, player.available_loggers()];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.pine += amount;
					player.offline_timer.pine_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
				},
				"oak" => {
					if player.axe < Axe::Iron {
						return Message::Content("You need an **Iron** axe to assign loggers to this tree!".to_string());
					}
					let amounts = vec![amount, player.available_loggers()];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.oak += amount;
					player.offline_timer.oak_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
				},
				"maple" => {
					if player.axe < Axe::Steel {
						return Message::Content("You need a **Steel** axe to assign loggers to this tree!".to_string());
					}
					let amounts = vec![amount, player.available_loggers()];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.maple += amount;
					player.offline_timer.maple_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
				},
				"walnut" => {
					if player.axe < Axe::Mithril {
						return Message::Content("You need a **Mithril** axe to assign loggers to this tree!".to_string());
					}
					let amounts = vec![amount, player.available_loggers()];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.walnut += amount;
					player.offline_timer.walnut_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
				},
				"cherry" => {
					if player.axe < Axe::Adamant {
						return Message::Content("You need an **Adamant** axe to assign loggers to this tree!".to_string());
					}
					let amounts = vec![amount, player.available_loggers()];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.cherry += amount;
					player.offline_timer.cherry_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
				},
				"purpleheart" => {
					if player.axe < Axe::Rune {
						return Message::Content("You need a **Rune** axe to assign loggers to this tree!".to_string());
					}
					let amounts = vec![amount, player.available_loggers()];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.purpleheart += amount;
					player.offline_timer.purpleheart_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You assigned **{}** loggers to chop **{}** trees", amount, tree.name})
				},
				_ => Message::how()
			}
		},
		"lumberers" => {
			let amount = match &tree
				.options
				.get(0)
				.expect("int")
				.resolved
				.as_ref()
				.expect("int") 
			{
				CommandDataOptionValue::Integer(i) => i.to_owned(),
				_ => 0
			};
			match tree.name.as_str() {
				"pine" => {
					if player.kiln < Kiln::SteelBucket {
						return Message::Content("You need a **Steel Bucket** kiln to assign lumberers to this log!".to_string());
					}
					let amounts = vec![amount, player.available_lumberers()];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.pine += amount;
					player.offline_timer.pine_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You assigned **{}** lumberers to dry **{}** logs", amount, tree.name))
				},
				"oak" => {
					if player.kiln < Kiln::Firebrick {
						return Message::Content("You need a **Firebrick** kiln to assign lumberers to this log!".to_string());
					}
					let amounts = vec![amount, player.available_lumberers()];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.oak += amount;
					player.offline_timer.oak_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You assigned **{}** lumberers to dry **{}** logs", amount, tree.name))
				},
				"maple" => {
					if player.kiln < Kiln::Hobby {
						return Message::Content("You need a **Hobby** kiln to assign lumberers to this log!".to_string());
					}
					let amounts = vec![amount, player.available_lumberers()];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.maple += amount;
					player.offline_timer.maple_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You assigned **{}** lumberers to dry **{}** logs", amount, tree.name))
				},
				"walnut" => {
					if player.kiln < Kiln::LabGrade {
						return Message::Content("You need a **Lab Grade** kiln to assign lumberers to this log!".to_string());
					}
					let amounts = vec![amount, player.available_lumberers()];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.walnut += amount;
					player.offline_timer.walnut_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You assigned **{}** lumberers to dry **{}** logs", amount, tree.name))
				},
				"cherry" => {
					if player.kiln < Kiln::Industrial {
						return Message::Content("You need an **Industrial** kiln to assign lumberers to this log!".to_string());
					}
					let amounts = vec![amount, player.available_lumberers()];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.cherry += amount;
					player.offline_timer.cherry_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You assigned **{}** lumberers to dry **{}** logs", amount, tree.name))
				},
				"purpleheart" => {
					if player.kiln < Kiln::WorldWide {
						return Message::Content("You need a **World Wide** kiln to assign lumberers to this log!".to_string());
					}
					let amounts = vec![amount, player.available_lumberers()];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.purpleheart += amount;
					player.offline_timer.purpleheart_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You assigned **{}** lumberers to dry **{}** logs", amount, tree.name))
				},
				_ => Message::how()
			}
		},
		"cncs" => {
			let furniture = match &tree
				.options
				.get(0)
				.expect("str")
				.resolved
				.as_ref()
				.expect("str")
			{
				CommandDataOptionValue::String(s) => s.to_owned(),
				_ => "".to_string()
			};
			let amount = match &tree
				.options
				.get(1)
				.expect("int")
				.resolved
				.as_ref()
				.expect("int") 
			{
				CommandDataOptionValue::Integer(i) => i.to_owned(),
				_ => 0
			};

			match tree.name.as_str() {
				"pine" => {
					if player.hammer < Hammer::Stone {
						return Message::Content("You need a **Stone** hammer to assign CNCs to build with this lumber".to_string())
					}
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							if !player.blueprints.pine.birdhouse {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.birdhouse += amount;
							player.offline_timer.pine_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							if !player.blueprints.pine.shelf {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.shelf += amount;
							player.offline_timer.pine_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							if !player.blueprints.pine.side_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.side_table += amount;
							player.offline_timer.pine_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							if !player.blueprints.pine.coffee_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.coffee_table += amount;
							player.offline_timer.pine_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							if !player.blueprints.pine.dining_set {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.dining_set += amount;
							player.offline_timer.pine_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"oak" => {
					if player.hammer < Hammer::Iron {
						return Message::Content("You need an **Iron** hammer to assign CNCs to build with this lumber".to_string())
					}
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							if !player.blueprints.oak.birdhouse {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.birdhouse += amount;
							player.offline_timer.oak_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							if !player.blueprints.oak.shelf {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.shelf += amount;
							player.offline_timer.oak_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							if !player.blueprints.oak.side_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.side_table += amount;
							player.offline_timer.oak_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							if !player.blueprints.oak.coffee_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.coffee_table += amount;
							player.offline_timer.oak_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							if !player.blueprints.oak.dining_set {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.dining_set += amount;
							player.offline_timer.oak_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"maple" => {
					if player.hammer < Hammer::Steel {
						return Message::Content("You need a **Steel** hammer to assign CNCs to build with this lumber".to_string())
					}
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							if !player.blueprints.maple.birdhouse {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.birdhouse += amount;
							player.offline_timer.maple_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							if !player.blueprints.maple.shelf {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.shelf += amount;
							player.offline_timer.maple_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							if !player.blueprints.maple.side_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.side_table += amount;
							player.offline_timer.maple_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							if !player.blueprints.maple.coffee_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.coffee_table += amount;
							player.offline_timer.maple_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							if !player.blueprints.maple.dining_set {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.dining_set += amount;
							player.offline_timer.maple_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"walnut" => {
					if player.hammer < Hammer::Mithril {
						return Message::Content("You need a **Mithril** hammer to assign CNCs to build with this lumber".to_string())
					}
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							if !player.blueprints.walnut.birdhouse {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.birdhouse += amount;
							player.offline_timer.walnut_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							if !player.blueprints.walnut.shelf {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.shelf += amount;
							player.offline_timer.walnut_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							if !player.blueprints.walnut.side_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.side_table += amount;
							player.offline_timer.walnut_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							if !player.blueprints.walnut.coffee_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.coffee_table += amount;
							player.offline_timer.walnut_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							if !player.blueprints.walnut.dining_set {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.dining_set += amount;
							player.offline_timer.walnut_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"cherry" => {
					if player.hammer < Hammer::Adamant {
						return Message::Content("You need an **Adamant** hammer to assign CNCs to build with this lumber".to_string())
					}
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							if !player.blueprints.cherry.birdhouse {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.birdhouse += amount;
							player.offline_timer.cherry_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							if !player.blueprints.cherry.shelf {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.shelf += amount;
							player.offline_timer.cherry_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							if !player.blueprints.cherry.side_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.side_table += amount;
							player.offline_timer.cherry_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							if !player.blueprints.cherry.coffee_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.coffee_table += amount;
							player.offline_timer.cherry_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							if !player.blueprints.cherry.dining_set {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.dining_set += amount;
							player.offline_timer.cherry_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"purpleheart" => {
					if player.hammer < Hammer::Rune {
						return Message::Content("You need a **Rune** hammer to assign CNCs to build with this lumber".to_string())
					}
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							if !player.blueprints.purpleheart.birdhouse {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.birdhouse += amount;
							player.offline_timer.purpleheart_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							if !player.blueprints.purpleheart.shelf {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.shelf += amount;
							player.offline_timer.purpleheart_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							if !player.blueprints.purpleheart.side_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.side_table += amount;
							player.offline_timer.purpleheart_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							if !player.blueprints.purpleheart.coffee_table {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.coffee_table += amount;
							player.offline_timer.purpleheart_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							if !player.blueprints.purpleheart.dining_set {
								return Message::Content("You don't have this blueprint!".to_string())
							}
							let amounts = vec![amount, player.available_cncs()];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.dining_set += amount;
							player.offline_timer.purpleheart_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You assigned **{}** CNCs to build **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				_ => Message::how()
			}
		},
		_ => Message::how()
	}	
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("assign").description("assign loggers, lumberers, and cncs to automate things!")
		.create_option(|option| {
			option
				.name("loggers")
				.description("assign loggers")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("assign loggers to pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("assign loggers to oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("assign loggers to maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("assign loggers to walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("assign loggers to cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("assign loggers to purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
		.create_option(|option| {
			option
				.name("lumberers")
				.description("assign lumberers")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("assign lumberers to pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("assign lumberers to oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("assign lumberers to maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("assign lumberers to walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("assign lumberers to cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("assign lumberers to purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
		.create_option(|option| {
			option
				.name("cncs")
				.description("assign cncs")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("assign cncs to pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("kind")
								.description("the kind of furniture")
								.kind(CommandOptionType::String)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("assign cncs to oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("kind")
								.description("the kind of furniture")
								.kind(CommandOptionType::String)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("assign cncs to maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("kind")
								.description("the kind of furniture")
								.kind(CommandOptionType::String)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("assign cncs to walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("kind")
								.description("the kind of furniture")
								.kind(CommandOptionType::String)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("assign cncs to cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("kind")
								.description("the kind of furniture")
								.kind(CommandOptionType::String)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("assign cncs to purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("kind")
								.description("the kind of furniture")
								.kind(CommandOptionType::String)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to assign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
}