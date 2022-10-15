use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Axe, Player, Action};
use crate::utils;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> String {
	let option = options
		.get(0)
		.expect("Expected a string")
		.resolved
		.as_ref()
		.expect("Expected str");

	// println!("{}", player_id);
	let mut player = get_player(player_id).await;
	if let CommandDataOptionValue::String(tree) = option {
		match tree.as_str() {
			"pine" => {
				chop_player_update(&mut player, "pine").await
			},
			"oak" => {
				if player.axe < Axe::Iron {
					return "You need an **Iron** axe to chop oak logs!".to_string();
				}
				chop_player_update(&mut player, "oak").await
			},
			_ => "No such tree".to_string()
		}
	} else {
		"No such tree".to_string()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("chop").description("Chop trees for lumber!").create_option(|option| {
		option
			.name("tree")
			.description("The type of tree to chop")
			.kind(CommandOptionType::String)
			.required(true)
			.add_string_choice("Pine", "pine")
			.add_string_choice("Oak", "oak")
			.add_string_choice("Maple", "maple")
			.add_string_choice("Walnut", "walnut")
			.add_string_choice("Cherry", "cherry")
			.add_string_choice("Purpleheart", "purpleheart")
	})
}

fn chop_log(player: &Player, tree: &str) -> Option<Action> {
	// returns true if insta-chopped.
	let chop_time = utils::get_tree_time(player, tree);
	if chop_time == 0 {
		return None;
	} else {
		return Some(Action::chopping(chop_time, tree));
	}
}

fn determine_logs_earned(player: &Player) -> i64 {
	let base_logs = 1;
	let upgrade = player.upgrades.wider_axes;
	let sawdust_upgrade = player.sawdust_upgrades.wider_axes;
	let sawdust = player.sawdust_total; // each is a permanent 5% increase to output
	
	((base_logs + upgrade + sawdust_upgrade) as f64 * (1.0 + (0.05 * sawdust as f64))) as i64
}

async fn chop_player_update(player: &mut Player, tree: &str) -> String {
	let action = chop_log(&player, tree);
	match action {
		Some(a) => {
			player.current_action = Some(a.clone());
			player.update().await;
			
			format!("You started chopping a **pine** tree! You'll be done in **{}s**", a.time_to_complete())
		}
		None => {
			let amount = determine_logs_earned(&player);
			player.current_action = None;
			match tree {
				"pine" => {
					player.logs.pine += amount;
					player.stats.pine_trees_chopped += 1;
					player.stats.pine_logs_earned += amount;
				},
				"oak" => {
					player.logs.oak += amount;
					player.stats.oak_trees_chopped += 1;
					player.stats.oak_logs_earned += amount;
				},
				"maple" => {
					player.logs.maple += amount;
					player.stats.maple_trees_chopped += 1;
					player.stats.maple_logs_earned += amount;
				},
				"walnut" => {
					player.logs.walnut += amount;
					player.stats.walnut_trees_chopped += 1;
					player.stats.walnut_logs_earned += amount;
				},
				"cherry" => {
					player.logs.cherry += amount;
					player.stats.cherry_trees_chopped += 1;
					player.stats.cherry_logs_earned += amount;
				},
				"purpleheart" => {
					player.logs.purpleheart += amount;
					player.stats.purpleheart_trees_chopped += 1;
					player.stats.purpleheart_logs_earned += amount;
				},
				_ => ()
			}
			let s = if amount >= 1 {
				"s"
			} else {
				""
			};
			format!("You chopped **{} {}** log{}!", amount, tree, s)
		}
	}
}