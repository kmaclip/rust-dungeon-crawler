use macroquad::prelude::*;
use crate::GameState;

pub fn update_ui(game_state: &mut GameState) {
    draw_text(&format!("Health: {:.0}", game_state.player_health), 10.0, 20.0, 20.0, WHITE);
    draw_text(&format!("Score: {}", game_state.score), screen_width() - 120.0, 20.0, 20.0, YELLOW);
}

pub fn show_pause_menu() {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));
    draw_text("PAUSED - Press P to Resume", screen_width() / 2.0 - 140.0, screen_height() / 2.0, 30.0, WHITE);
}

pub fn show_game_over() {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));
    draw_text("GAME OVER", screen_width() / 2.0 - 80.0, screen_height() / 2.0, 40.0, RED);
}