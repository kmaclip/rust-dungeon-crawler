use macroquad::prelude::*;
use crate::{GameState, project_3d_to_2d};
use noise::{NoiseFn, Perlin};
use ::rand::{thread_rng, Rng};

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
}

pub fn update_enemies(game_state: &mut GameState, perlin: &Perlin, camera: &Camera3D) {
    let dt = get_frame_time();

    game_state.enemies.retain_mut(|enemy| {
        let dx = game_state.player.x - enemy.x;
        let dz = game_state.player.z - enemy.z;
        let dist = (dx * dx + dz * dz).sqrt();
        if dist > 0.5 {
            enemy.x += (dx / dist * enemy.speed * dt).clamp(-63.5, 63.5);
            enemy.z += (dz / dist * enemy.speed * dt).clamp(-63.5, 63.5);
        }
        enemy.y = perlin.get([enemy.x as f64 * 0.1, enemy.z as f64 * 0.1]) as f32 * 8.0 + 0.5;

        game_state.player.bullets.retain_mut(|bullet| {
            let dist = ((enemy.x - bullet.x).powi(2) + (enemy.z - bullet.z).powi(2)).sqrt();
            if dist < 0.5 {
                enemy.health -= bullet.damage;
                false
            } else {
                true
            }
        });

        if enemy.health <= 0.0 {
            game_state.score += 10;
            false
        } else {
            true
        }
    });

    if get_time() % 4.0 < 0.1 && game_state.enemies.len() < 5 {
        spawn_enemy(game_state, perlin);
    }

    for enemy in &game_state.enemies {
        let screen_pos = project_3d_to_2d(vec3(enemy.x, enemy.y, enemy.z), camera);
        draw_texture_ex(
            &game_state.enemy_texture,
            screen_pos.x - 24.0,
            screen_pos.y - 24.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(48.0, 48.0)),
                ..Default::default()
            },
        );
    }

    for bullet in &game_state.player.bullets {
        let bullet_pos = project_3d_to_2d(vec3(bullet.x, bullet.y, bullet.z), camera);
        draw_texture_ex(
            &game_state.bullet_texture,
            bullet_pos.x - 12.0,
            bullet_pos.y - 12.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(24.0, 24.0)),
                ..Default::default()
            },
        );
    }
}

fn spawn_enemy(game_state: &mut GameState, perlin: &Perlin) {
    let mut rng = thread_rng();
    let x = rng.gen_range(0.0..64.0);
    let z = rng.gen_range(0.0..64.0);
    let y = perlin.get([x as f64 * 0.1, z as f64 * 0.1]) as f32 * 8.0 + 0.5;
    game_state.enemies.push(Enemy {
        x,
        y,
        z,
        health: 20.0,
        speed: 2.0,
        damage: 10.0,
    });
}