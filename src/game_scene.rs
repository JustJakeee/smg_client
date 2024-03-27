use glam::Vec2;
use raylib::prelude::*;
use uuid::Uuid;

use crate::{AppState, Packet, Player, PlayerState, Scene};

#[derive(Clone)]
pub struct GameState {
    player: Player,
    player_list: Vec<PlayerState>,
    uuid_list: Vec<Uuid>,
}

impl GameState {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            player_list: Vec::new(),
            uuid_list: Vec::new(),
        }
    }
}

pub fn update(rl: &mut RaylibHandle, app_state: &mut AppState) {
    let mut client = app_state.client.as_ref().unwrap();
    let game_state = app_state.game_state.as_mut().unwrap();
    let speed = 200.;
    let mut move_dir = Vec2::ZERO;
    if rl.is_key_down(KeyboardKey::KEY_W) {
        move_dir.y = -1.;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        move_dir.y = 1.;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        move_dir.x = -1.;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        move_dir.x = 1.;
    }
    game_state.player.pos += move_dir * speed * rl.get_frame_time();
    let packet = Packet::Player(PlayerState::new(
        game_state.player.uuid,
        game_state.player.pos,
    ));
    client.send(packet).unwrap();
    let data = client.recv().unwrap();
    let response: Vec<PlayerState> = bincode::deserialize(&data).expect("failed to deserialize");
    game_state.player_list = response;
}

pub fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, app_state: &AppState) {
    let d = &mut rl.begin_drawing(thread);
    let game_state = app_state.game_state.as_ref().unwrap();
    d.clear_background(Color::WHITE);
    game_state.player.draw(d);
    for player in game_state.player_list.iter().map(|x| Player::from(x)) {
        if player.uuid == game_state.player.uuid {
            continue;
        }
        player.draw(d);
    }
}
