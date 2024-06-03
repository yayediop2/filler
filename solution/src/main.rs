mod parse_file;
mod place_piece_on_grid;
use parse_file::{parse_file, parse_player_name};
use place_piece_on_grid::{get_enemy_positions, place_piece_on_grid};

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

fn euclidean_distance(p1: (usize, usize), p2: (usize, usize)) -> f64 {
    let (x1, y1) = (p1.0 as f64, p1.1 as f64);
    let (x2, y2) = (p2.0 as f64, p2.1 as f64);
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn closest_position(
    valid_positions: Vec<(usize, usize)>,
    enemy_positions: Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    if valid_positions.is_empty() {
        return None;
    }
    let mut closest = valid_positions[0];
    let mut min_distance = euclidean_distance(valid_positions[0], enemy_positions[0]);

    for &valid_position in valid_positions.iter().skip(1) {
        for &enemy_position in enemy_positions.iter().skip(1) {
            let distance = euclidean_distance(valid_position, enemy_position);
            if distance < min_distance {
                closest = valid_position;
                min_distance = distance;
            }
        }
    }
    Some(closest)
}
