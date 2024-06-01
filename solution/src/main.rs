mod parse_file;
mod place_piece_on_grid;
use place_piece_on_grid::place_piece_on_grid;
use parse_file::{parse_file, parse_player_name};

fn main() {
    let player_name = parse_player_name();

    let (player_char, player_char2, enemy, enemy2) = if player_name == "p1" {
        ('a', '@', 's', '$')
    } else {
        ('s', '$', 'a', '@')
    };

    loop {
        let (grid, piece) = parse_file();

        let (x, y) = place_piece_on_grid(&grid, &piece, enemy, enemy2);
        print!("{} {}\n", x, y);
    }
}
