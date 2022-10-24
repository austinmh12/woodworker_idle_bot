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
					let amounts = vec![amount, player.loggers, player.loggers_active.pine];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.pine -= amount;
					player.offline_timer.pine_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You unassigned **{}** loggers from chopping **{}** trees", amount, tree.name})
				},
				"oak" => {
					let amounts = vec![amount, player.loggers, player.loggers_active.oak];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.oak -= amount;
					player.offline_timer.oak_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You unassigned **{}** loggers from chopping **{}** trees", amount, tree.name})
				},
				"maple" => {
					let amounts = vec![amount, player.loggers, player.loggers_active.maple];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.maple -= amount;
					player.offline_timer.maple_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You unassigned **{}** loggers from chopping **{}** trees", amount, tree.name})
				},
				"walnut" => {
					let amounts = vec![amount, player.loggers, player.loggers_active.walnut];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.walnut -= amount;
					player.offline_timer.walnut_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You unassigned **{}** loggers from chopping **{}** trees", amount, tree.name})
				},
				"cherry" => {
					let amounts = vec![amount, player.loggers, player.loggers_active.cherry];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.cherry -= amount;
					player.offline_timer.cherry_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You unassigned **{}** loggers from chopping **{}** trees", amount, tree.name})
				},
				"purpleheart" => {
					let amounts = vec![amount, player.loggers, player.loggers_active.purpleheart];
					let amount = *amounts.iter().min().unwrap();
					player.loggers_active.purpleheart -= amount;
					player.offline_timer.purpleheart_log = Utc::now();
					player.update().await;
	
					Message::Content(format!{"You unassigned **{}** loggers from chopping **{}** trees", amount, tree.name})
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
					let amounts = vec![amount, player.lumberers, player.lumberers_active.pine];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.pine -= amount;
					player.offline_timer.pine_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You unassigned **{}** lumberers from drying **{}** logs", amount, tree.name))
				},
				"oak" => {
					let amounts = vec![amount, player.lumberers, player.lumberers_active.oak];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.oak -= amount;
					player.offline_timer.oak_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You unassigned **{}** lumberers from drying **{}** logs", amount, tree.name))
				},
				"maple" => {
					let amounts = vec![amount, player.lumberers, player.lumberers_active.maple];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.maple -= amount;
					player.offline_timer.maple_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You unassigned **{}** lumberers from drying **{}** logs", amount, tree.name))
				},
				"walnut" => {
					let amounts = vec![amount, player.lumberers, player.lumberers_active.walnut];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.walnut -= amount;
					player.offline_timer.walnut_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You unassigned **{}** lumberers from drying **{}** logs", amount, tree.name))
				},
				"cherry" => {
					let amounts = vec![amount, player.lumberers, player.lumberers_active.cherry];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.cherry -= amount;
					player.offline_timer.cherry_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You unassigned **{}** lumberers from drying **{}** logs", amount, tree.name))
				},
				"purpleheart" => {
					let amounts = vec![amount, player.lumberers, player.lumberers_active.purpleheart];
					let amount = *amounts.iter().min().unwrap();
					player.lumberers_active.purpleheart -= amount;
					player.offline_timer.purpleheart_lumber = Utc::now();
					player.update().await;
	
					Message::Content(format!("You unassigned **{}** lumberers from drying **{}** logs", amount, tree.name))
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
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.pine.birdhouse];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.birdhouse -= amount;
							player.offline_timer.pine_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.pine.shelf];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.shelf -= amount;
							player.offline_timer.pine_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.pine.side_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.side_table -= amount;
							player.offline_timer.pine_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.pine.coffee_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.coffee_table -= amount;
							player.offline_timer.pine_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.pine.dining_set];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.pine.dining_set -= amount;
							player.offline_timer.pine_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"oak" => {
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.oak.birdhouse];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.birdhouse -= amount;
							player.offline_timer.oak_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.oak.shelf];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.shelf -= amount;
							player.offline_timer.oak_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.oak.side_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.side_table -= amount;
							player.offline_timer.oak_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.oak.coffee_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.coffee_table -= amount;
							player.offline_timer.oak_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.oak.dining_set];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.oak.dining_set -= amount;
							player.offline_timer.oak_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"maple" => {
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.maple.birdhouse];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.birdhouse -= amount;
							player.offline_timer.maple_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.maple.shelf];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.shelf -= amount;
							player.offline_timer.maple_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.maple.side_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.side_table -= amount;
							player.offline_timer.maple_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.maple.coffee_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.coffee_table -= amount;
							player.offline_timer.maple_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.maple.dining_set];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.maple.dining_set -= amount;
							player.offline_timer.maple_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"walnut" => {
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.walnut.birdhouse];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.birdhouse -= amount;
							player.offline_timer.walnut_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.walnut.shelf];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.shelf -= amount;
							player.offline_timer.walnut_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.walnut.side_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.side_table -= amount;
							player.offline_timer.walnut_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.walnut.coffee_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.coffee_table -= amount;
							player.offline_timer.walnut_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.walnut.dining_set];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.walnut.dining_set -= amount;
							player.offline_timer.walnut_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"cherry" => {
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.cherry.birdhouse];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.birdhouse -= amount;
							player.offline_timer.cherry_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.cherry.shelf];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.shelf -= amount;
							player.offline_timer.cherry_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.cherry.side_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.side_table -= amount;
							player.offline_timer.cherry_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.cherry.coffee_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.coffee_table -= amount;
							player.offline_timer.cherry_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.cherry.dining_set];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.cherry.dining_set -= amount;
							player.offline_timer.cherry_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** dining sets", amount, tree.name))
						},
						_ => Message::Content("No such furniture".to_string())
					}
				},
				"purpleheart" => {
					match furniture.to_lowercase().as_str() {
						"birdhouse" | "bird house" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.purpleheart.birdhouse];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.birdhouse -= amount;
							player.offline_timer.purpleheart_birdhouse = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** bird houses", amount, tree.name))
						},
						"shelf" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.purpleheart.shelf];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.shelf -= amount;
							player.offline_timer.purpleheart_shelf = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** shelves", amount, tree.name))
						},
						"sidetable" | "side table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.purpleheart.side_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.side_table -= amount;
							player.offline_timer.purpleheart_side_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** side tables", amount, tree.name))
						},
						"coffeetable" | "coffee table" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.purpleheart.coffee_table];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.coffee_table -= amount;
							player.offline_timer.purpleheart_coffee_table = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** coffee tables", amount, tree.name))
						},
						"diningset" | "dining set" => {
							let amounts = vec![amount, player.cncs, player.cncs_active.purpleheart.dining_set];
							let amount = *amounts.iter().min().unwrap();
							player.cncs_active.purpleheart.dining_set -= amount;
							player.offline_timer.purpleheart_dining_set = Utc::now();
							player.update().await;
			
							Message::Content(format!("You unassigned **{}** CNCs from building **{}** dining sets", amount, tree.name))
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
	command.name("unassign").description("unassign loggers, lumberers, and cncs from automating things!")
		.create_option(|option| {
			option
				.name("loggers")
				.description("unassign loggers")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("unassign loggers from pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("unassign loggers from oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("unassign loggers from maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("unassign loggers from walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("unassign loggers from cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("unassign loggers from purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
		.create_option(|option| {
			option
				.name("lumberers")
				.description("unassign lumberers")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("unassign lumberers from pine")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("unassign lumberers from oak")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("unassign lumberers from maple")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("unassign lumberers from walnut")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("unassign lumberers from cherry")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("unassign lumberers from purpleheart")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
		.create_option(|option| {
			option
				.name("cncs")
				.description("unassign cncs")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("pine")
						.description("unassign cncs from pine")
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
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("oak")
						.description("unassign cncs from oak")
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
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("maple")
						.description("unassign cncs from maple")
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
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("walnut")
						.description("unassign cncs from walnut")
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
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("cherry")
						.description("unassign cncs from cherry")
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
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("purpleheart")
						.description("unassign cncs from purpleheart")
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
								.description("the amount to unassign")
								.kind(CommandOptionType::Integer)
								.required(true)
								.min_int_value(1)
						})
				})
		})
}