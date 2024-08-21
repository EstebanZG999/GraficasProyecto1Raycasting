use minifb::{Key, Window};
use crate::player::Player;
use crate::audio::AudioPlayer;

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize, steps_player: &AudioPlayer) {
    const MOVE_SPEED: f32 = 0.1;
    const ROTATION_SPEED: f32 = std::f32::consts::PI / 30.0;
    let mut moved = false;

    // Rotación del jugador con A y D
    if window.is_key_down(Key::A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.a += ROTATION_SPEED;
    }

    let mut next_pos_x = player.pos.x;
    let mut next_pos_y = player.pos.y;

    // Movimiento del jugador con W y S
    if window.is_key_down(Key::W) {
        next_pos_x += player.a.cos() * MOVE_SPEED;
        next_pos_y += player.a.sin() * MOVE_SPEED;
        moved = true;
    }
    if window.is_key_down(Key::S) {
        next_pos_x -= player.a.cos() * MOVE_SPEED;
        next_pos_y -= player.a.sin() * MOVE_SPEED;
        moved = true;
    }

    let next_cell_x = next_pos_x as usize;
    let next_cell_y = next_pos_y as usize;

    if next_cell_x < maze[0].len() && next_cell_y < maze.len() && maze[next_cell_y][next_cell_x] == ' ' {
        player.pos.x = next_pos_x;
        player.pos.y = next_pos_y;
    }

    // Reproducir o pausar el sonido de los pasos dependiendo si el jugador se mueve o no
    if moved {
        steps_player.play();
    } else {
        steps_player.pause();
    }
}

// Función que comprueba si una posición está dentro de una pared o muy cerca de ella
fn is_colliding(x: f32, y: f32, maze: &Vec<Vec<char>>, threshold: f32) -> bool {
    let map_x = x.floor() as isize;
    let map_y = y.floor() as isize;

    if map_x < 0 || map_x >= maze[0].len() as isize || map_y < 0 || map_y >= maze.len() as isize {
        return true;
    }

    let cell = maze[map_y as usize][map_x as usize];

    if cell != ' ' {
        let dist_x = x - map_x as f32;
        let dist_y = y - map_y as f32;

        if dist_x < threshold || dist_x > (1.0 - threshold) || dist_y < threshold || dist_y > (1.0 - threshold) {
            return true;
        }
    }

    false
}