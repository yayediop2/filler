extern crate sdl2;
use sdl2::image::{self, InitFlag};
use sdl2::pixels::Color;
use sdl2::render::Canvas;

pub fn initialisation() -> (Canvas<sdl2::video::Window>, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // Initialize SDL2_image
    let _image_context = image::init(InitFlag::PNG).unwrap();
    
    let window = video_subsystem
    .window("Filler visualizer", 1300, 1000)
    .position_centered()
    .build()
    .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

pub fn draw(
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
