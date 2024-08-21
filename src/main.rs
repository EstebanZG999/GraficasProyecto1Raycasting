mod maze;
mod player;
mod raycasting;
mod controls;
mod textures;
mod audio;  

use player::Player;
use raycasting::cast_ray;
use controls::process_events;
use minifb::{Key, Window, WindowOptions};
use nalgebra as na;
use textures::Texture;
use once_cell::sync::Lazy;
use std::sync::Arc;
use audio::AudioPlayer;
use std::time::{Duration, Instant}; 
use rusttype::{Font, Scale};
use image::{DynamicImage, GenericImageView};

const WIDTH: usize = 1040;
const HEIGHT: usize = 800;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("sprites/wall4.webp")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("sprites/wall4.webp")));
static FLOOR: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("sprites/floor7.webp")));
static SKY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("sprites/sky3.jpeg")));
static ENEMY_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("sprites/cagney2.png")));
static ENEMY_ANIM_FRAMES: Lazy<Vec<Arc<Texture>>> = Lazy::new(|| vec![
    Arc::new(Texture::new("sprites/cagney2.png")),
    Arc::new(Texture::new("sprites/cagney.png")),
    Arc::new(Texture::new("sprites/cagney2.png")),
]);



fn cell_to_texture_color(wall_type: char, is_vertical: bool, tx: f32, ty: f32) -> u32 {
    match wall_type {
        '|' => WALL1.get_pixel_color((tx * WALL1.width as f32) as u32, (ty * WALL1.height as f32) as u32),
        '-' => WALL2.get_pixel_color((tx * WALL2.width as f32) as u32, (ty * WALL2.height as f32) as u32),
        _ => WALL1.get_pixel_color((tx * WALL1.width as f32) as u32, (ty * WALL1.height as f32) as u32),
    }
}

fn draw_cell(framebuffer: &mut Vec<u32>, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '-' | '|' => 0xFFFFFF, 
        'p' => 0xFF0000, 
        'g' => 0x00FF00, 
        _ => 0x000000,   
    };

    for y in yo..(yo + block_size).min(HEIGHT) {
        for x in xo..(xo + block_size).min(WIDTH) {
            framebuffer[y * WIDTH + x] = color;
        }
    }
}

fn render2d(framebuffer: &mut Vec<u32>, maze: &Vec<Vec<char>>, block_size: usize, player: &Player) {
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            draw_cell(
                framebuffer,
                col * block_size,
                row * block_size,
                block_size,
                cell,
            );
        }
    }

    // Dibujar al jugador en la vista 2D
    let player_x = (player.pos.x * block_size as f32) as usize;
    let player_y = (player.pos.y * block_size as f32) as usize;
    let player_size = block_size / 4; // Tamaño del punto que representa al jugador

    for y in player_y..(player_y + player_size).min(HEIGHT) {
        for x in player_x..(player_x + player_size).min(WIDTH) {
            framebuffer[y * WIDTH + x] = 0xFF0000; // Rojo para representar al jugador
        }
    }
}

fn render_floor(framebuffer: &mut Vec<u32>) {
    for y in (HEIGHT / 2)..HEIGHT {
        let ty = (y - HEIGHT / 2) * FLOOR.height as usize / (HEIGHT / 2);

        for x in 0..WIDTH {
            let tx = x * FLOOR.width as usize / WIDTH;
            let color = FLOOR.get_pixel_color(tx as u32, ty as u32);
            framebuffer[y * WIDTH + x] = color;
        }
    }
}

fn render_sky(framebuffer: &mut Vec<u32>, player_angle: f32) {
    let sky_width = SKY.width as f32;
    let sky_height = SKY.height as f32;

    for y in 0..(HEIGHT / 2) {
        let ty = (y as f32 / (HEIGHT / 2) as f32 * sky_height) as u32;

        for x in 0..WIDTH {
            let tx = ((x as f32 / WIDTH as f32) * sky_width) as u32;

            let color = SKY.get_pixel_color(tx, ty);
            framebuffer[y * WIDTH + x] = color;
        }
    }
}

fn render_minimap(framebuffer: &mut Vec<u32>, maze: &Vec<Vec<char>>, player: &Player, block_size: usize) {
    let minimap_scale = 20;
    let minimap_width = maze[0].len() * minimap_scale;
    let minimap_height = maze.len() * minimap_scale;

    // Posición del minimapa 
    let minimap_x_offset = 10;
    let minimap_y_offset = 10;

    // Dibujar el laberinto en el minimapa
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            let color = match cell {
                '+' | '-' | '|' => 0xFFFFFF, 
                'p' => 0xFF0000, 
                _ => 0x000000,   
            };

            for y in 0..minimap_scale {
                for x in 0..minimap_scale {
                    let pixel_x = minimap_x_offset + col * minimap_scale + x;
                    let pixel_y = minimap_y_offset + row * minimap_scale + y;
                    if pixel_x < WIDTH && pixel_y < HEIGHT {
                        framebuffer[pixel_y * WIDTH + pixel_x] = color;
                    }
                }
            }
        }
    }

    // Dibujar al jugador en el minimapa
    let player_minimap_x = minimap_x_offset + (player.pos.x * minimap_scale as f32) as usize;
    let player_minimap_y = minimap_y_offset + (player.pos.y * minimap_scale as f32) as usize;

    let player_minimap_size = 8;

    for y in 0..player_minimap_size {
        for x in 0..player_minimap_size {
            let pixel_x = player_minimap_x + x;
            let pixel_y = player_minimap_y + y;
            if pixel_x < WIDTH && pixel_y < HEIGHT {
                framebuffer[pixel_y * WIDTH + pixel_x] = 0xFF0000;
            }
        }
    }
}

fn render3d(framebuffer: &mut Vec<u32>, maze: &Vec<Vec<char>>, player: &Player, block_size: usize, frame_time: f32) {
    let num_rays = WIDTH;
    let hh = HEIGHT as f32 / 2.0;
    let mut z_buffer: Vec<f32> = vec![std::f32::MAX; WIDTH];

    // Renderizar el cielo primero
    render_sky(framebuffer, player.a);

    // Luego renderizar el suelo
    render_floor(framebuffer);

    // Renderizar las paredes
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        let ray_hit = cast_ray(maze, player, a, block_size);

        // Corregir la distancia para evitar el efecto de fisheye
        let corrected_distance = ray_hit.distance * (a - player.a).cos();
        z_buffer[i] = corrected_distance;
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

    // Renderizar los enemigos después de las paredes y antes de cualquier otro elemento
    render_enemies(framebuffer, player, &mut z_buffer, frame_time);

    // Llamada para dibujar el minimapa
    render_minimap(framebuffer, maze, player, block_size);
}

fn render_enemy(framebuffer: &mut Vec<u32>, player: &Player, pos: &na::Vector2<f32>, z_buffer: &mut [f32], frame_time: f32) {
    let sprite_dir = na::Vector2::new(
        pos.x - player.pos.x,
        pos.y - player.pos.y,
    );

    let sprite_distance = sprite_dir.norm();
    let sprite_angle = (sprite_dir.y).atan2(sprite_dir.x) - player.a;

    let sprite_angle = if sprite_angle < -std::f32::consts::PI {
        sprite_angle + 2.0 * std::f32::consts::PI
    } else if sprite_angle > std::f32::consts::PI {
        sprite_angle - 2.0 * std::f32::consts::PI
    } else {
        sprite_angle
    };

    if sprite_angle.abs() > player.fov / 2.0 || sprite_distance < 0.5 {
        return;
    }

    let screen_x = (WIDTH as f32 / 2.0) * (1.0 + sprite_angle / player.fov);
    let sprite_height = (HEIGHT as f32 / sprite_distance) * 0.4;
    let sprite_width = sprite_height;

    let start_x = screen_x as isize - (sprite_width as isize / 2);
    let start_y = (HEIGHT as isize / 2) - (sprite_height as isize / 2);
    let end_x = start_x + sprite_width as isize;
    let end_y = start_y + sprite_height as isize;

    if start_x >= 0 && end_x < WIDTH as isize && sprite_distance < z_buffer[screen_x as usize] {
        let frame_index = ((frame_time * 10.0) as usize) % ENEMY_ANIM_FRAMES.len();
        let enemy_texture = &ENEMY_ANIM_FRAMES[frame_index];

        for x in start_x..end_x {
            for y in start_y..end_y {
                if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
                    let x = x as usize;
                    let y = y as usize;

                    let tx = ((x - start_x as usize) * enemy_texture.width as usize / sprite_width as usize) as u32;
                    let ty = ((y - start_y as usize) * enemy_texture.height as usize / sprite_height as usize) as u32;
                    let color = enemy_texture.get_pixel_color(tx, ty);

                    if color != 0x000000 { // Ignorar color negro, hacerlo transparente
                        framebuffer[y * WIDTH + x] = color;
                    }
                }
            }
        }
        z_buffer[screen_x as usize] = sprite_distance;
    }
}

fn render_enemies(framebuffer: &mut Vec<u32>, player: &Player, z_buffer: &mut [f32], frame_time: f32) {
    let enemy_positions = vec![
        na::Vector2::new(2.0, 5.0),
        na::Vector2::new(11.0, 3.5),
        na::Vector2::new(5.0, 5.0),
        na::Vector2::new(8.0, 7.0),
        na::Vector2::new(7.0, 2.0), 
    ];

    for enemy_pos in &enemy_positions {
        render_enemy(framebuffer, player, enemy_pos, z_buffer, frame_time);
    }
}

fn draw_fps_box(framebuffer: &mut Vec<u32>, fps: u32) {
    let box_width = 100;
    let box_height = 30;
    let box_color = 0xFFFFFF; 
    let text_color = 0x000000; 

    for y in 0..box_height {
        for x in 0..box_width {
            framebuffer[y * WIDTH + x] = box_color;
        }
    }

    let fps_text = format!("FPS: {}", fps);
    let start_x = 10;
    let start_y = 10;

    for (i, c) in fps_text.chars().enumerate() {
        let x = start_x + i * 8;
        let y = start_y;
        if x < WIDTH && y < HEIGHT {
            framebuffer[y * WIDTH + x] = text_color;
        }
    }
}

fn render_text(
    framebuffer: &mut Vec<u32>, 
    text: &str, 
    x: usize, 
    y: usize, 
    scale: Scale, 
    color: u32
) {
    let font_data = include_bytes!(r#"C:\Users\esteb\OneDrive\Escritorio\Universidad\2024\Graficas por Computadora\GraficasProyecto1Raycasting\assets\font.ttf"#);
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(text, scale, rusttype::point(x as f32, y as f32 + v_metrics.ascent))
        .collect();

    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, gv| {
                let px = (gx as i32 + bb.min.x) as usize;
                let py = (gy as i32 + bb.min.y) as usize;

                if px < WIDTH && py < HEIGHT {
                    let alpha = (gv * 255.0) as u32;
                    let foreground = if alpha > 128 {
                        color & 0xFFFFFF
                    } else {
                        framebuffer[py * WIDTH + px]
                    };

                    framebuffer[py * WIDTH + px] = foreground;
                }
            });
        }
    }
}


fn load_frame(file_path: &str) -> DynamicImage {
    image::open(file_path).expect("Failed to load frame")
}

fn render_frame(framebuffer: &mut Vec<u32>, frame: &DynamicImage) {
    let frame_rgb = frame.to_rgba8();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x < frame.width() as usize && y < frame.height() as usize {
                let pixel = frame_rgb.get_pixel(x as u32, y as u32);
                let color = ((pixel[3] as u32) << 24) | ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                framebuffer[y * WIDTH + x] = color;
            }
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

    let mut framebuffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let block_size = 80;

    let frames = vec![
        load_frame("assets/introframe1.jpeg").resize_exact(WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Lanczos3),
        load_frame("assets/introframe2.jpeg").resize_exact(WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Lanczos3),
        load_frame("assets/introframe3.jpeg").resize_exact(WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Lanczos3),
    ];

    let mut frame_index = 0;
    let mut last_frame_time = Instant::now();

    // Bucle de bienvenida
    while window.is_open() && !window.is_key_down(Key::Enter) {
        if last_frame_time.elapsed() >= Duration::from_millis(500) {
            frame_index = (frame_index + 1) % frames.len();
            last_frame_time = Instant::now();
        }

        render_frame(&mut framebuffer, &frames[frame_index]);

        let scale = Scale::uniform(40.0);
        let welcome_text = "Welcome to the Cuphead Maze Game!";
        let instruction_text = "Press Enter to Start";
        
        let text_width = welcome_text.len() * 20;
        let instruction_width = instruction_text.len() * 20;
        let x_pos = (WIDTH - text_width) / 2;

        let y_pos = (HEIGHT / 3) + 350; 
        let instruction_x_pos = (WIDTH - instruction_width) / 2;
        let instruction_y_pos = y_pos + 80; 
        
        render_text(&mut framebuffer, welcome_text, x_pos, y_pos, scale, 0x000000);
        render_text(&mut framebuffer, instruction_text, instruction_x_pos, instruction_y_pos, scale, 0x000000);

        window.update_with_buffer(&framebuffer, WIDTH, HEIGHT).unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }

    // Música de fondo
    let background_music = AudioPlayer::new("assets/FloralFury.mp3").expect("Failed to initialize background music");
    background_music.set_volume(0.2);
    background_music.play();

    // Sonido para los pasos
    let steps_sound = AudioPlayer::new("assets/footsteps.mp3").expect("Failed to initialize steps sound");

    let enemy_positions = vec![
        na::Vector2::new(2.0, 5.0),
        na::Vector2::new(11.0, 3.5),
        na::Vector2::new(5.0, 5.0),
        na::Vector2::new(8.0, 7.0),
    ];

    let maze = maze::load_maze("maze.txt");

    let mut player = Player {
        pos: na::Vector2::new(1.5, 1.5),
        a: std::f32::consts::FRAC_PI_3,
        fov: std::f32::consts::FRAC_PI_3,
    };

    let mut mode = "3D";

    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps_text = String::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start_time = Instant::now();

        // Ahora el block_size está definido en este ámbito
        process_events(&window, &mut player, &maze, block_size, &steps_sound);

        framebuffer.iter_mut().for_each(|pixel| *pixel = 0);
        let mut z_buffer: Vec<f32> = vec![std::f32::MAX; WIDTH];

        if mode == "2D" {
            render2d(&mut framebuffer, &maze, block_size, &player);
        } else {
            render3d(&mut framebuffer, &maze, &player, block_size, Instant::now().duration_since(last_time).as_secs_f32());
        }

        // Calcular FPS
        frame_count += 1;
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_time);

        if elapsed >= std::time::Duration::from_secs(1) {
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            fps_text = format!("FPS: {:.0}", fps);
            last_time = current_time;
            frame_count = 0;
        }

        let box_width = 100;
        let box_height = 40;
        let box_x = WIDTH - box_width - 10; 
        let box_y = 10; 

        for y in box_y..(box_y + box_height) {
            for x in box_x..(box_x + box_width) {
                framebuffer[y * WIDTH + x] = 0xFFFFFF; 
            }
        }

        let scale = Scale::uniform(24.0);
        render_text(&mut framebuffer, &fps_text, box_x + 10, box_y + 10, scale, 0x000000);

        window.update_with_buffer(&framebuffer, WIDTH, HEIGHT).unwrap();

        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        let frame_end_time = Instant::now();
        let frame_duration_actual = frame_end_time.duration_since(frame_start_time);
        if frame_duration_actual < std::time::Duration::from_millis(16) {
            let sleep_duration = std::time::Duration::from_millis(16) - frame_duration_actual;
            if sleep_duration > std::time::Duration::from_millis(0) {
                std::thread::sleep(sleep_duration);
            }
        }
    }
}
