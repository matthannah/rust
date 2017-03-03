extern crate rand;

use rand::Rng;

struct Puzzle {
    tiles: [[usize; 4]; 4]
}

impl Puzzle {
    fn new() -> Puzzle {
        let mut puzzle_tiles: [[usize; 4]; 4] = [[0; 4]; 4];
        let mut rng = rand::thread_rng();
        let mut shuffled_tiles: [usize; 16] = [ 1,  2,  3,  4,
                                                5,  6,  7,  8,
                                                9, 10, 11, 12,
                                               13, 14, 15,  0];
        rng.shuffle(&mut shuffled_tiles);

        for row in 0 .. 4 {
            for col in 0 .. 4 {
                puzzle_tiles[row][col] = shuffled_tiles[(row * 4) + col];
            }
        }

        Puzzle { tiles: puzzle_tiles }
    }

    fn move_tile(&mut self, tile: usize) {
        let (empty_row, empty_col) = self.get_tile_location(0);
        let (row, col) = self.get_tile_location(tile);
        let value = self.tiles[row][col];

        self.tiles[row][col] = 0;
        self.tiles[empty_row][empty_col] = value;
    }

    fn complete(&self) -> bool {
        for (row_num, row) in self.tiles.iter().enumerate() {
            for (col_num, tile) in row.iter().enumerate() {
                if row_num == 3 && col_num == 3 { return true; }
                if *tile != (row_num * 4) + col_num + 1 { return false; }
            }
        }
        false
    }

    fn get_adjacent_tiles(&self, tile: usize) -> Vec<usize> {
        let (row, col) = self.get_tile_location(tile); // destructure tuple
        let mut adjacent_tiles: Vec<usize> = Vec::new();

        if row != 3 { adjacent_tiles.push(self.tiles[row + 1][col]); }
        if row != 0 { adjacent_tiles.push(self.tiles[row - 1][col]); }
        if col != 3 { adjacent_tiles.push(self.tiles[row][col + 1]); }
        if col != 0 { adjacent_tiles.push(self.tiles[row][col - 1]); }

        adjacent_tiles
    }

    fn get_tile_location(&self, t: usize) -> (usize, usize) {
        let mut location = (0, 0);

        for (row_num, row) in self.tiles.iter().enumerate() {
            for (col_num, tile) in row.iter().enumerate() {
                if t == *tile { location.0 = row_num; location.1 = col_num; }
            }
        }

        location
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                if *tile == 0 {
                    try!(write!(f, "[    ]"));
                } else {
                    try!(write!(f, "[ {:>width$} ]", *tile, width=2));
                }
            }
            try!(writeln!(f, ""));
        }
        write!(f, "")
    }
}

fn main() {
    let mut puzzle = Puzzle::new();
    while !puzzle.complete() {
        println!("{}", puzzle);

        let valid_moves = puzzle.get_adjacent_tiles(0);
        let input = get_user_input(valid_moves);
        puzzle.move_tile(input);
    }

    println!("You Win!");
}

fn get_user_input(valid_input: Vec<usize>) -> usize {
    use std::io::Write;

    let mut input = 0;
    let mut s = String::new();

    print!("Please enter a valid move {:?}: ", valid_input);
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut s).expect("failed to read from stdin");
    let trimmed = s.trim();
    match trimmed.parse::<usize>() {
        Ok(i)  => { input = i; },
        Err(_) => {}
    };

    if !valid_input.iter().any(|e| *e == input) { get_user_input(valid_input); }

    input
}
