pub fn place_piece_on_grid(
    grid: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    enemy: char,
    enemy2: char,
) -> Vec<(usize, usize)> {
    let mut valid_positions = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if can_place_piece(i, j, grid, piece, enemy, enemy2) {
                valid_positions.push((i, j));
            }
        }
    }
    valid_positions
}

pub fn can_place_piece(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    enemy: char,
    enemy2: char,
) -> bool {
    let mut overlap_counter = 0;

    for i in 0..piece.len() {
        for j in 0..piece[i].len() {
            if piece[i][j] == 'O' {
                let (grid_x, grid_y) = (i + x, j + y);

                if grid_x >= grid.len() || grid_y >= grid[0].len() {
                    return false;
                }

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

pub fn get_enemy_positions(grid: &Vec<Vec<char>>, enemy: char, enemy2: char) -> Vec<(usize, usize)> {
    let mut enemy_positions = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == enemy || grid[i][j] == enemy2 {
                enemy_positions.push((i, j));
            }
        }
    }
    enemy_positions
}


fn euclidean_distance(p1: (usize, usize), p2: (usize, usize)) -> f64 {
    let (x1, y1) = (p1.0 as f64, p1.1 as f64);
    let (x2, y2) = (p2.0 as f64, p2.1 as f64);
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn closest_position(
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