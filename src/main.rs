#![allow(unused)]
mod udp_client;

use glam::{IVec2, Vec2};
use raylib::prelude::*;
use udp_client::*;
use smg_lib::*;
use uuid::Uuid;

struct State {
    width: i32,
    height: i32,
    delta: f32,
    fps: u32,
    player: Player,
    uuid_list: Vec<Uuid>,
    player_list: Vec<PlayerState>,
    udp_client: Option<UdpGameClient>,
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

    let mut state = State {
        width: START_SIZE.x,
        height: START_SIZE.y,
        delta: rl.get_frame_time(),
        fps: rl.get_fps(),
        player: Player {
            uuid: Uuid::new_v4(),
            pos: Vec2::new(320., 240.),
            color: Color::DARKBLUE,
        },
        uuid_list: Vec::new(),
        player_list: Vec::new(),
        udp_client: None,
    };

    let client =
        UdpGameClient::connect("127.0.0.1:5000", state.player.uuid).expect("failed to connect");

    println!("connected");

    while !rl.window_should_close() {
        state.width = rl.get_screen_width();
        state.height = rl.get_screen_height();
        state.delta = rl.get_frame_time();
        state.fps = rl.get_fps();

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
    d.draw_text(&format!("fps: {}", state.fps), 0, 0, 20, Color::BLACK);
    d.draw_text(
        &format!("uuid: {:?}", state.player.uuid),
        0,
        20,
        20,
        Color::BLACK,
    );
    d.draw_text(
        &format!("pos: ({}, {})", state.player.pos.x, state.player.pos.y),
        0,
        40,
        20,
        Color::BLACK,
    );   
    d.draw_text(
        &format!("player list: {:?}", state.player_list),
        0,
        80,
        20,
        Color::BLACK,
    );
    state.player.draw(d);
    for player in &state.player_list {
        if player.uuid == state.player.uuid {
            continue;
        }

        let player: Player = Player::from(player);
        player.draw(d);
    }
}
