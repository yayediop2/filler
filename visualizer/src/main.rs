extern crate sdl2;

use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, render::Canvas};
use std::time::Duration;

mod parse_file;
mod place_piece_on_grid;
use parse_file::{parse_file, parse_player_name};
use place_piece_on_grid::{closest_position, get_enemy_positions, place_piece_on_grid};

fn main() {
    let player_name = parse_player_name();

    let (player, player2, enemy, enemy2) = if player_name == "p1" {
        ('a', '@', 's', '$')
    } else {
        ('s', '$', 'a', '@')
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Initialize SDL2_image
    let _image_context = image::init(InitFlag::PNG).unwrap();

    let window = video_subsystem
        .window("Filler visualizer", 1300, 1000)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();

    let player_texture = texture_creator.load_texture("visualizer/assets/old.jpg").unwrap();
    let enemy_texture = texture_creator.load_texture("visualizer/assets/angry.jpg").unwrap();

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

fn draw(
    grid: &Vec<Vec<char>>,
    canvas: &mut Canvas<sdl2::video::Window>,
    player_texture: &sdl2::render::Texture,
    enemy_texture: &sdl2::render::Texture,
    player: char,
    player2: char,
    enemy: char,
    enemy2: char,
) -> () {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let (width, height) = canvas.output_size().unwrap();
    let cell_width = width / grid[0].len() as u32;
    let cell_height = height / grid.len() as u32;

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let texture = if col == &player || col == &player2 {
                Some(player_texture)
            } else if col == &enemy || col == &enemy2 {
                Some(enemy_texture)
            } else {
                None
            };

            let x = (j as u32 * cell_width) as i32;
            let y = (i as u32 * cell_height) as i32;
            let rect = sdl2::rect::Rect::new(x, y, cell_width, cell_height);

            if let Some(texture) = texture {
                canvas.copy(texture, None, rect).unwrap();
            } else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
}
