use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng; // Import Rng trait

#[macroquad::main("Forest Explorer")]
async fn main() {
    let foliage_texture = load_texture("assets/foliage.png").await.unwrap_or_else(|_| {
        println!("Failed to load foliage texture, using default");
        Texture2D::from_rgba8(16, 16, &[0, 255, 0, 128; 16 * 16]) // Semi-transparent green
    });

    let mut world = World::new();

    loop {
        clear_background(Color::new(0.1, 0.2, 0.3, 1.0)); // Sky blue

        let camera = world.get_camera();
        set_camera(&camera);

        world.update();
        world.render(&foliage_texture);

        next_frame().await;
    }
}

struct Player {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    speed: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            position: vec3(32.0, 0.0, 32.0),
            yaw: 0.0,
            pitch: 0.0,
            speed: 5.0,
        }
    }

    fn update(&mut self, dt: f32, terrain: &Mesh) {
        let mouse_delta = mouse_delta_position();
        self.yaw -= mouse_delta.x * 0.005;
        self.pitch = (self.pitch - mouse_delta.y * 0.005).clamp(-1.5, 1.5);

        let mut dx = 0.0;
        let mut dz = 0.0;
        if is_key_down(KeyCode::W) { dz -= 1.0; }
        if is_key_down(KeyCode::S) { dz += 1.0; }
        if is_key_down(KeyCode::A) { dx -= 1.0; }
        if is_key_down(KeyCode::D) { dx += 1.0; }

        let len = ((dx * dx + dz * dz) as f32).sqrt(); // Corrected cast
        if len > 0.0 {
            dx /= len;
            dz /= len;
            let forward = vec2(self.yaw.cos(), self.yaw.sin());
            let right = vec2(-forward.y, forward.x);
            let move_dir = forward * dz + right * dx;
            self.position.x = (self.position.x + move_dir.x * self.speed * dt).clamp(0.5, 63.5);
            self.position.z = (self.position.z + move_dir.y * self.speed * dt).clamp(0.5, 63.5);
        }

        let x = self.position.x as usize;
        let z = self.position.z as usize;
        if x < 64 && z < 64 {
            self.position.y = terrain.vertices[x * 64 + z].position.y;
        }
    }

    fn get_camera(&self) -> Camera3D {
        Camera3D {
            position: vec3(self.position.x, self.position.y + 1.5, self.position.z),
            target: vec3(
                self.position.x + self.yaw.cos() * self.pitch.cos(),
                self.position.y + 1.5 + self.pitch.sin(),
                self.position.z + self.yaw.sin() * self.pitch.cos(),
            ),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 60.0f32.to_radians(),
            aspect: Some(screen_width() / screen_height()),
            projection: Projection::Perspective,
            ..Default::default()
        }
    }
}

struct Tree {
    position: Vec3,
    trunk_height: f32,
    trunk_base_radius: f32,
    trunk_lean: Vec2,
    foliage_layers: Vec<(f32, f32)>, // (height offset, radius)
}

struct World {
    terrain: Mesh,
    trees: Vec<Tree>,
    perlin: Perlin,
    player: Player,
}

impl World {
    fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let perlin = Perlin::new(rng.gen::<u32>());
        let terrain = Self::generate_terrain(&perlin);
        let trees = Self::generate_trees(&perlin);
        let player = Player::new();
        Self { terrain, trees, perlin, player }
    }

    fn generate_terrain(perlin: &Perlin) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        const SIZE: usize = 64;
        let scale = 0.1;

        for x in 0..SIZE {
            for z in 0..SIZE {
                let y = perlin.get([x as f64 * scale, z as f64 * scale]) as f32 * 2.0;
                let color = Color::new(0.0, 0.4 + y * 0.1, 0.0, 1.0);
                vertices.push(Vertex {
                    position: vec3(x as f32, y, z as f32),
                    uv: vec2(x as f32 / SIZE as f32, z as f32 / SIZE as f32),
                    color: color.into(),
                    normal: vec4(0.0, 1.0, 0.0, 0.0),
                });
            }
        }

        for x in 0..SIZE - 1 {
            for z in 0..SIZE - 1 {
                let i = x * SIZE + z;
                indices.extend_from_slice(&[
                    i as u16,
                    (i + 1) as u16,
                    (i + SIZE) as u16,
                    (i + 1) as u16,
                    (i + SIZE + 1) as u16,
                    (i + SIZE) as u16,
                ]);
            }
        }

        Mesh { vertices, indices, texture: None }
    }

    fn generate_trees(perlin: &Perlin) -> Vec<Tree> {
        let mut trees = Vec::new();
        let mut rng = ::rand::thread_rng();
        let tree_count = 40;

        for _ in 0..tree_count {
            let x = rng.gen_range(2.0..62.0);
            let z = rng.gen_range(2.0..62.0);
            let y = perlin.get([x as f64 * 0.1, z as f64 * 0.1]) as f32 * 2.0;
            let trunk_height = rng.gen_range(3.0..6.0);
            let trunk_base_radius = rng.gen_range(0.2..0.4);
            let trunk_lean = vec2(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));

            let mut foliage_layers = Vec::new();
            let layer_count = rng.gen_range(3..5);
            for i in 0..layer_count {
                let height_offset = trunk_height * (0.5 + i as f32 * 0.2);
                let radius = trunk_base_radius * (layer_count - i) as f32 * 0.8;
                foliage_layers.push((height_offset, radius));
            }

            trees.push(Tree {
                position: vec3(x, y, z),
                trunk_height,
                trunk_base_radius,
                trunk_lean,
                foliage_layers,
            });
        }
        trees
    }

    fn update(&mut self) {
        let dt = get_frame_time();
        self.player.update(dt, &self.terrain);
    }

    fn render(&self, foliage_texture: &Texture2D) {
        draw_mesh(&self.terrain);

        for tree in &self.trees {
            let base_pos = vec3(
                tree.position.x + tree.trunk_lean.x * 0.5,
                tree.position.y,
                tree.position.z + tree.trunk_lean.y * 0.5,
            );
            let top_pos = vec3(
                tree.position.x + tree.trunk_lean.x,
                tree.position.y + tree.trunk_height,
                tree.position.z + tree.trunk_lean.y,
            );
            draw_cylinder_ex(
                base_pos,
                top_pos,
                tree.trunk_base_radius,
                tree.trunk_base_radius * 0.3,
                Some(12), // Number of sides
                BROWN,
                DrawCylinderParams::default(),
            );

            for (height_offset, radius) in &tree.foliage_layers {
                let pos = vec3(tree.position.x, tree.position.y + height_offset, tree.position.z);
                let screen_pos = self.project_3d_to_2d(pos);
                draw_texture_ex(
                    foliage_texture,
                    screen_pos.x - radius * 16.0,
                    screen_pos.y - radius * 16.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(radius * 32.0, radius * 32.0)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn project_3d_to_2d(&self, pos: Vec3) -> Vec2 {
        let camera = self.get_camera();
        let view_matrix = Mat4::look_at_rh(camera.position, camera.target, camera.up);
        let proj_matrix = Mat4::perspective_rh_gl(camera.fovy, camera.aspect.unwrap_or(1.0), 0.1, 100.0);
        let world_to_screen = proj_matrix * view_matrix;
        let clip_space = world_to_screen * vec4(pos.x, pos.y, pos.z, 1.0);
        if clip_space.w <= 0.0 {
            return vec2(-1000.0, -1000.0);
        }
        let ndc = vec2(clip_space.x / clip_space.w, clip_space.y / clip_space.w);
        vec2(
            (ndc.x + 1.0) * 0.5 * screen_width(),
            (1.0 - ndc.y) * 0.5 * screen_height(),
        )
    }

    fn get_camera(&self) -> Camera3D {
        self.player.get_camera()
    }
}