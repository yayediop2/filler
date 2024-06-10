mod parse_file;
mod place_piece_on_grid;
use parse_file::{parse_file, parse_player_name};
use place_piece_on_grid::{closest_position, get_enemy_positions, place_piece_on_grid};

fn main() {
    let player_name = parse_player_name();

    let (enemy, enemy2) = if player_name == "p1" {
        ('s', '$')
    } else {
        ('a', '@')
    };

    loop {
        let (grid, piece) = parse_file();

        let valid_positions = place_piece_on_grid(&grid, &piece, enemy, enemy2);
        let enemy_positions = get_enemy_positions(&grid, enemy, enemy2);
        let closest = closest_position(valid_positions, enemy_positions);
        if closest.is_none() {
            print!("0 0\n");
        } else {
            let (x, y) = closest.unwrap();
            print!("{} {}\n", y, x);
        }
    }
}
