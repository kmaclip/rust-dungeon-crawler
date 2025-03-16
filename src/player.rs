use macroquad::prelude::*;
use crate::{GameState, Tree, Apple};
use noise::Perlin; // Removed NoiseFn

const PLAYER_SPEED: f32 = 5.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub bullets: Vec<Bullet>,
}

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vz: f32,
    pub damage: f32,
}

pub fn setup_player() -> Player {
    Player {
        x: 32.0,
        y: 0.0,
        z: 32.0,
        yaw: 0.0,
        pitch: 0.0,
        bullets: Vec::new(),
    }
}

pub fn update_player(game_state: &mut GameState, _perlin: &Perlin, _trees: &[Tree], apples: &mut Vec<Apple>) {
    let dt = get_frame_time();

    let mouse_delta = mouse_delta_position();
    game_state.player.yaw -= mouse_delta.x * 0.005;
    game_state.player.pitch = (game_state.player.pitch - mouse_delta.y * 0.005).clamp(-1.5, 1.5);

    let mut dx = 0.0;
    let mut dz = 0.0;
    if is_key_down(KeyCode::W) { dz -= 1.0; }
    if is_key_down(KeyCode::S) { dz += 1.0; }
    if is_key_down(KeyCode::A) { dx -= 1.0; }
    if is_key_down(KeyCode::D) { dx += 1.0; }

    let len = ((dx * dx + dz * dz) as f32).sqrt();
    if len > 0.0 {
        dx /= len;
        dz /= len;
        let forward = vec2(game_state.player.yaw.cos(), game_state.player.yaw.sin());
        let right = vec2(-forward.y, forward.x);
        let move_dir = forward * dz + right * dx;
        let new_x = (game_state.player.x + move_dir.x * PLAYER_SPEED * dt).clamp(0.5, 63.5);
        let new_z = (game_state.player.z + move_dir.y * PLAYER_SPEED * dt).clamp(0.5, 63.5);
        game_state.player.x = new_x;
        game_state.player.z = new_z;
    }

    game_state.player.y = 0.0;

    if is_mouse_button_pressed(MouseButton::Left) && get_time() - game_state.last_shoot_time > 0.5 {
        shoot_bullet(game_state);
        game_state.last_shoot_time = get_time();
    }

    game_state.player.bullets.retain_mut(|bullet| {
        bullet.x += bullet.vx * dt;
        bullet.z += bullet.vz * dt;
        bullet.x >= 0.0 && bullet.x <= 64.0 && bullet.z >= 0.0 && bullet.z <= 64.0
    });

    apples.retain_mut(|apple| {
        let dist = ((apple.x - game_state.player.x).powi(2) + (apple.z - game_state.player.z).powi(2)).sqrt();
        if dist < 0.5 {
            game_state.score += 5;
            false
        } else {
            true
        }
    });
}

fn shoot_bullet(game_state: &mut GameState) {
    let dir = vec3(
        game_state.player.yaw.cos() * game_state.player.pitch.cos(),
        game_state.player.pitch.sin(),
        game_state.player.yaw.sin() * game_state.player.pitch.cos(),
    ).normalize();
    let speed = 10.0;
    let bullet = Bullet {
        x: game_state.player.x,
        y: game_state.player.y + 1.0,
        z: game_state.player.z,
        vx: dir.x * speed,
        vz: dir.z * speed,
        damage: 10.0,
    };
    game_state.player.bullets.push(bullet);
}