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
