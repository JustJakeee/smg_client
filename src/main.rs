#![allow(unused)]
mod game_scene;
mod menu_scene;
mod udp_client;

use std::ops::Deref;

use game_scene::*;
use glam::{IVec2, Vec2};
use menu_scene::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use udp_client::*;
use udp_client::*;
use uuid::Uuid;

#[derive(Clone)]
enum Scene {
    Menu(),
    Game(),
}

struct AppState {
    uuid: Uuid,
    scene: Scene,
    game_state: Option<GameState>,
    client: Option<UdpGameClient>,
}

#[derive(Clone)]
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

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Hello, World")
        //.msaa_4x()
        .build();

    let mut app_state = AppState {
        uuid: Uuid::new_v4(),
        scene: Scene::Menu(),
        game_state: None,
        client: None,
    };

    while !rl.window_should_close() {
        let mut scene = app_state.scene.clone();
        match scene {
            Scene::Menu() => {
                menu_scene::update(&mut rl, &mut app_state);
                menu_scene::draw(&mut rl, &thread, &app_state);
            }
            Scene::Game() => {
                game_scene::update(&mut rl, &mut app_state);
                game_scene::draw(&mut rl, &thread, &app_state);
            }
        }
    }

    match app_state
        .client
        .unwrap()
        .send(Packet::Disconnect(app_state.uuid))
    {
        Ok(_) => println!("disconnected"),
        Err(e) => eprintln!("failed to send disconnect packet: {:?}", e),
    }
}
