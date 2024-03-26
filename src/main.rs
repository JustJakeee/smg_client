#![allow(unused)]
mod udp_client;
mod scenes;

use glam::{IVec2, Vec2};
use raylib::prelude::*;
use udp_client::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use udp_client::*;

struct AppState {
    width: i32,
    height: i32,
    delta: f32,
    fps: u32,
    scene: Scene,
    client: Option<UdpGameClient>,
}

struct GameState {
    player: Player,
    player_list: Vec<PlayerState>,
    uuid_list: Vec<Uuid>,
}

enum Scene {
    Game(GameState),
    Menu,
}

pub struct Player {
    pub uuid: Uuid,
    pub pos: Vec2,
    pub color: Color,
}

impl Player {
    // draw func
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.pos.x as i32, self.pos.y as i32, 15., self.color);
    }
}

impl From<&PlayerState> for Player {
    fn from(player_state: &PlayerState) -> Self {
        Self {
            uuid: player_state.uuid,
            pos: Vec2::new(player_state.x, player_state.y),
            color: Color::ORANGE,
        }
    }
    
}

const START_SIZE: IVec2 = IVec2::new(640, 480);
const PLAYER_SPEED: f32 = 200.;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(START_SIZE.x, START_SIZE.y)
        .title("Hello, World")
        .msaa_4x()
        .build();

    let mut app_state = AppState {
        width: START_SIZE.x,
        height: START_SIZE.y,
        delta: rl.get_frame_time(),
        fps: rl.get_fps(),
        scene: Scene::Menu,
        client: None,
    };

    while !rl.window_should_close() {
        app_state.width = rl.get_screen_width();
        app_state.height = rl.get_screen_height();
        app_state.delta = rl.get_frame_time();
        app_state.fps = rl.get_fps();

        match app_state.scene {
            Scene::Menu => {
                menu::update(rl, &mut app_state);
                menu::draw(rl, &mut app_state);
            }
            Scene::Game(ref mut game_state) => {
                game::update(rl, &mut app_state, game_state);
                game::draw(rl, &mut app_state, game_state);
            }
        }

        let mut move_dir = Vec2::ZERO;
        if rl.is_key_down(KeyboardKey::KEY_W) {
            move_dir.y = -1.;
        } else if rl.is_key_down(KeyboardKey::KEY_S) {
            move_dir.y = 1.;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            move_dir.x = -1.;
        } else if rl.is_key_down(KeyboardKey::KEY_D) {
            move_dir.x = 1.;
        }

        state.player.pos += move_dir.normalize_or_zero() * PLAYER_SPEED * state.delta;

        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            &client
                .send(Packet::Message("hello".to_string()))
                .expect("failed to send");
        }

        &client
            .send(Packet::Player(PlayerState {
                uuid: state.player.uuid,
                x: state.player.pos.x,
                y: state.player.pos.y,
            }))
            .expect("failed to send");
        let response = &client.recv().expect("failed to recv");
        state.player_list = bincode::deserialize::<Vec<PlayerState>>(response)
            .expect("failed to deserialize player list");

        &client.send(Packet::List()).expect("failed to send");
        let response = &client.recv().expect("failed to recv");
        state.uuid_list = bincode::deserialize(response).expect("failed to deserialize");

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        let list_str = state
            .player_list
            .iter()
            .map(|x| format!("{}: ({}, {})", x.uuid, x.x, x.y))
            .collect::<Vec<String>>()
            .join("\n");
        d.draw_text(&list_str, 0, 0, 20, Color::BLACK);
        state.player.draw(&mut d);
        for player in &state.player_list {
            if player.uuid == state.player.uuid {
                continue;
            }

            let player: Player = Player::from(player);
            player.draw(&mut d);
        }
    }

    client
        .send(Packet::Disconnect(state.player.uuid))
        .expect("failed to send disconnect");
}

fn draw_game(d: &mut RaylibDrawHandle, state: &State) {
    d.clear_background(Color::WHITE);
        let list_str = state
            .player_list
            .iter()
            .map(|x| format!("{}: ({}, {})", x.uuid, x.x, x.y))
            .collect::<Vec<String>>()
            .join("\n");
        d.draw_text(&list_str, 0, 0, 20, Color::BLACK);
        state.player.draw(&mut d);
        for player in &state.player_list {
            if player.uuid == state.player.uuid {
                continue;
            }

            let player: Player = Player::from(player);
            player.draw(&mut d);
        }
}
