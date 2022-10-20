use std::collections::VecDeque;
use std::sync::Arc;
use serenity::prelude::Context;

use crate::player::get_players;
use crate::player::ActionEnum;
use crate::commands::chop::{update_player_chop, determine_logs_earned};
use crate::commands::dry::{update_player_dry, determine_lumber_earned};
use crate::commands::build::{update_player_build, determine_furniture_earned};

pub async fn check_actions(_ctx: Arc<Context>) {
	let players = get_players().await;
	for mut player in players {
		let current_action = player.current_action.clone();
		if current_action.time_to_complete() > 0 {
			continue;
		}
		match current_action.action {
			ActionEnum::None => continue,
			ActionEnum::Chopping => {
				let amount = determine_logs_earned(&player);
				let tree = current_action.tree.as_str();
				println!("{} done chopping, earned {} {} logs", &player.discord_id, &amount, &tree);
				update_player_chop(&mut player, amount, tree);
				if player.queued_actions.len() > 0usize {
					let mut queued_actions = VecDeque::from(player.queued_actions.clone());
					player.current_action = queued_actions.pop_front().unwrap().clone();
					player.queued_actions = Vec::from(queued_actions.clone());
				}
				player.update().await;
			},
			ActionEnum::Drying => {
				let amount = determine_lumber_earned(&player);
				let tree = current_action.tree.as_str();
				update_player_dry(&mut player, amount, tree);
				if player.queued_actions.len() > 0usize {
					let mut queued_actions = VecDeque::from(player.queued_actions.clone());
					player.current_action = queued_actions.pop_front().unwrap().clone();
					player.queued_actions = Vec::from(queued_actions.clone());
				}
				player.update().await;
			},
			ActionEnum::Building => {
				let amount = determine_furniture_earned(&player);
				let tree = current_action.tree.as_str();
				let furniture = current_action.furniture.unwrap();
				update_player_build(&mut player, amount, tree, furniture.as_str());
				if player.queued_actions.len() > 0usize {
					let mut queued_actions = VecDeque::from(player.queued_actions.clone());
					player.current_action = queued_actions.pop_front().unwrap().clone();
					player.queued_actions = Vec::from(queued_actions.clone());
				}
				player.update().await;
			},
		}
	}
}