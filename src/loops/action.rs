use std::sync::Arc;
use serenity::prelude::Context;

use crate::player::get_players;
use crate::player::ActionEnum;
use crate::commands::chop::{update_player_chop, determine_logs_earned};

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
				update_player_chop(&mut player, amount, tree);
				player.update().await;
			},
			ActionEnum::Drying => {
				todo!()
			},
			ActionEnum::Building => {
				todo!()
			},
		}
	}
}