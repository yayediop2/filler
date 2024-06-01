mod parse_file;
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

pub fn place_piece_on_grid(grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, enemy: char, enemy2: char) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if can_place_piece(j, i, grid, piece, enemy, enemy2) {
               return (j, i);            
            }
        }
    }
    (0, 0)
}

pub fn can_place_piece(x: usize, y: usize, grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>,  enemy: char,  enemy2: char) -> bool {
    let grid_x_length = grid[0].len();
    let grid_y_length = grid.len(); 
    let mut overlap_counter = 0;

    for i in 0..piece.len() {
        for j in 0..piece[i].len() {
            let (grid_x, grid_y) = (j + x, i + y);

            if grid_x >= grid_x_length || grid_y >= grid_y_length {
                return false;
            }

            if  piece[i][j] == 'O' {
                if grid[grid_x][grid_y] == enemy2 || grid[grid_x][grid_y] == enemy {
                    return false;
                } else if grid[grid_x][grid_y] != '.' {
                    overlap_counter += 1;
                }
            }
        }   
    }
    overlap_counter == 1
}


