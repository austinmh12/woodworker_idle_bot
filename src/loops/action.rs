use std::collections::VecDeque;
use std::sync::Arc;
use serenity::prelude::Context;

use crate::player::get_players;
use crate::player::ActionEnum;
use crate::commands::chop::update_player_chop;
use crate::commands::dry::update_player_dry;
use crate::commands::build::update_player_build;

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
				update_player_chop(&mut player);
				if player.queued_actions.len() > 0usize {
					let mut queued_actions = VecDeque::from(player.queued_actions.clone());
					player.current_action = queued_actions.pop_front().unwrap().clone();
					player.queued_actions = Vec::from(queued_actions.clone());
				}
				player.update().await;
			},
			ActionEnum::Drying => {
				update_player_dry(&mut player);
				if player.queued_actions.len() > 0usize {
					let mut queued_actions = VecDeque::from(player.queued_actions.clone());
					player.current_action = queued_actions.pop_front().unwrap().clone();
					player.queued_actions = Vec::from(queued_actions.clone());
				}
				player.update().await;
			},
			ActionEnum::Building => {
				update_player_build(&mut player);
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