use crate::AppState;
use crate::GameState;
use crate::Player;
use crate::Scene;
use crate::UdpGameClient;
use egui::Context;
use glam::Vec2;
use raylib::prelude::*;
use uuid::Uuid;

//menu will try to connect player to server on press enter

pub fn update(rl: &mut RaylibHandle, app_state: &mut AppState) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        let client = UdpGameClient::connect("127.0.0.1:5000", app_state.uuid).unwrap();
        app_state.client = Some(client);
        app_state.scene = Scene::Game();
        app_state.game_state = Some(GameState::new(Player {
            uuid: app_state.uuid,
            pos: Vec2::new(100., 100.),
            color: Color::BLACK,
        }));
    }
}
pub fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, app_state: &AppState) {
    let d = &mut rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);
    d.draw_text("Press Enter to connect", 10, 10, 20, Color::BLACK);
}
