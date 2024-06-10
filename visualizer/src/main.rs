extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod draw;
mod parse_file;
mod place_piece_on_grid;
use draw::{draw, initialisation};
use parse_file::{parse_file, parse_player_name};
use place_piece_on_grid::{closest_position, get_enemy_positions, place_piece_on_grid};

fn main() {
    let player_name = parse_player_name();

    let (player, player2, enemy, enemy2) = if player_name == "p1" {
        ('a', '@', 's', '$')
    } else {
        ('s', '$', 'a', '@')
    };

    let (mut canvas, mut event_pump) = initialisation();

    let texture_creator = canvas.texture_creator();

    let player_texture = texture_creator
        .load_texture("visualizer/assets/playful.jpg")
        .unwrap();
    let enemy_texture = texture_creator
        .load_texture("visualizer/assets/angrier.jpg")
        .unwrap();

    'my_loop: loop {
        let (grid, piece) = parse_file();

        let valid_positions = place_piece_on_grid(&grid, &piece, enemy, enemy2);
        let enemy_positions = get_enemy_positions(&grid, enemy, enemy2);
        let closest = closest_position(valid_positions, enemy_positions);

        draw(
            &grid,
            &mut canvas,
            &player_texture,
            &enemy_texture,
            player,
            player2,
            enemy,
            enemy2,
        );

        if closest.is_none() {
            print!("0 0\n");
        } else {
            let (x, y) = closest.unwrap();
            print!("{} {}\n", y, x);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'my_loop,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
