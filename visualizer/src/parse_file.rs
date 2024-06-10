use std::io::{self, BufRead};

pub fn parse_player_name() -> String {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("$$$") && line.contains("solution") {
            let (_first, second) = line.split_at(9);
            return second[0..2].to_string();
        }
    }
    String::new()
}

pub fn parse_file() -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut piece: Vec<Vec<char>> = Vec::new();
    let mut counter = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with(char::is_numeric) && !line.starts_with("0123456") {
            let (_first, second) = line.split_at(4);
            let row = second.chars().collect::<Vec<char>>();
            grid.push(row);
        } else if line.starts_with("Piece") {
            let splitted = line.split_whitespace().collect::<Vec<&str>>();
            let piece_length = splitted[2].split(":").next().unwrap();
            counter = piece_length.parse::<i8>().unwrap_or(0);
        } else if counter >= 0 && (line.starts_with("O") || line.starts_with(".")) {
            let row: Vec<char> = line.chars().collect();
            piece.push(row);
            counter -= 1;
        }
        if counter == 0 && !piece.is_empty() {
            break;
        }
    }

    (grid, piece)
}
