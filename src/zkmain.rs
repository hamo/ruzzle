use wasm_bindgen::prelude::*;
use game::constants::{ACTION_KEY};
use game::{Direction, Status};

#[cfg(feature = "zkwasm")]
use zkwasm_rust_sdk::{require, wasm_input};

use crate::game::World;
use audio::{AudioPlayer, DummyAudioPlayer};

// use web_sys::console::log_1;
//log_1(&format!("{:?}", replay_commands).into());

#[cfg(feature = "zkwasm")]
#[wasm_bindgen]
pub unsafe fn zkmain() -> u64 {
    //// u64: level_number
    //// u64: command_len
    //// array[u64, u64]: [timestamp, command]

    let level_number = wasm_input(1);
    let cmd_len = wasm_input(1);

    let mut commands: Vec<(u64, u64)> = Vec::new();

    for _i in 0..cmd_len {
        let ts = wasm_input(0);
        let c = wasm_input(0);

        commands.push((ts, c));
    }

    require(replay_and_check(level_number, commands));

    0
}

pub fn replay_and_check(level_number: u64, commands: Vec<(u64, u64)>) -> bool {
    if commands.len() == 0 {
        return false
    }

    let mut dummy_audio_player: Box<dyn AudioPlayer> = Box::new(DummyAudioPlayer::new());

    let mut world = World::new(vec![], vec![], vec![]);
    world.init_level(level_number as usize);

    let min_ts = commands.iter().min_by_key(|x| x.0).unwrap().0;
    let max_ts = commands.iter().max_by_key(|x| x.0).unwrap().0;

    let mut replay_commands: Vec<(u64, Option<u64>)> = commands.iter().map(|x| (x.0, Some(x.1))).collect();
    for i in (min_ts..=max_ts+2000).step_by(20) {
        replay_commands.push((i, None));
    }
    replay_commands.sort_by_key(|x| x.0);

    world.update(min_ts as f64, &mut dummy_audio_player);

    for c in replay_commands {
        let ts = c.0;
        let cmd = c.1;

        if let Some(event) = cmd {
            match event {
                0 => world.handle_player_movement(Some(Direction::Up), ts as f64),     // Up
                1 => world.handle_player_movement(Some(Direction::Down), ts as f64),   // Down
                2 => world.handle_player_movement(Some(Direction::Left), ts as f64),   // Left
                3 => world.handle_player_movement(Some(Direction::Right), ts as f64),  // Right
                4 => world.handle_action_event(ACTION_KEY, ts as f64), // ACTION z
                _ => (),
            }
        }

        world.update(ts as f64, &mut dummy_audio_player);
    }

    world.is_completed || world.player().borrow().status_manager().status == Status::Exiting
}
