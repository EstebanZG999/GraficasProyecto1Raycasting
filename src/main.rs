mod maze;
mod player;
mod raycasting;
mod controls;
mod textures;

use player::Player;
use raycasting::cast_ray;
use controls::process_events;
use minifb::{Key, Window, WindowOptions};
use nalgebra as na;
use textures::Texture;
use once_cell::sync::Lazy;
use std::sync::Arc;

const WIDTH: usize = 1040;
const HEIGHT: usize = 900;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("sprites/wall.jpg")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("sprites/wall.jpg")));

fn cell_to_texture_color(wall_type: char, is_vertical: bool, tx: f32, ty: f32) -> u32 {
    match wall_type {
        '|' => WALL1.get_pixel_color((tx * WALL1.width as f32) as u32, (ty * WALL1.height as f32) as u32),
        '-' => WALL2.get_pixel_color((tx * WALL2.width as f32) as u32, (ty * WALL2.height as f32) as u32),
        _ => WALL1.get_pixel_color((tx * WALL1.width as f32) as u32, (ty * WALL1.height as f32) as u32),
    }
}

fn render3d(framebuffer: &mut Vec<u32>, maze: &Vec<Vec<char>>, player: &Player, block_size: usize) {
    let num_rays = WIDTH;
    let hh = HEIGHT as f32 / 2.0;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        let ray_hit = cast_ray(maze, player, a, block_size);

        // Corregir la distancia para evitar el efecto de fisheye
        let corrected_distance = ray_hit.distance * (a - player.a).cos();
        let stake_height = (hh / corrected_distance) as usize;

        let mut stake_top = hh as usize - (stake_height / 2);
        let mut stake_bottom = hh as usize + (stake_height / 2);

        if stake_top >= HEIGHT {
            stake_top = HEIGHT - 1;
        }
        if stake_bottom >= HEIGHT {
            stake_bottom = HEIGHT - 1;
        }

        // Determinar la coordenada X en la textura
        let texture_x = if ray_hit.is_vertical {
            (ray_hit.hit_y % 1.0 * WALL1.width as f32) as u32
        } else {
            (ray_hit.hit_x % 1.0 * WALL1.width as f32) as u32
        };

        for y in stake_top..stake_bottom {
            // Determinar la coordenada Y en la textura
            let texture_y = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * WALL1.height as f32) as u32;
            let color = cell_to_texture_color(ray_hit.wall_type, ray_hit.is_vertical, texture_x as f32 / WALL1.width as f32, texture_y as f32 / WALL1.height as f32);
            framebuffer[y * WIDTH + i] = color;
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Maze",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let maze = maze::load_maze("maze.txt");

    let mut framebuffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let block_size = 80;

    let mut player = Player {
        pos: na::Vector2::new(1.5, 1.5),
        a: std::f32::consts::FRAC_PI_3,
        fov: std::f32::consts::FRAC_PI_3,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        process_events(&window, &mut player, &maze, block_size);

        framebuffer.iter_mut().for_each(|pixel| *pixel = 0);

        render3d(&mut framebuffer, &maze, &player, block_size);

        window.update_with_buffer(&framebuffer, WIDTH, HEIGHT).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(16)); // Controla el frame rate
    }
}
