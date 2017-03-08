extern crate rand;

use std::ops::Index;
use std::ops::IndexMut;
use rand::Rng;

const ROWS: usize = 100;
const COLS: usize = 20;

enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    row: usize,
    col: usize,
    walls: [bool; 4],
    visited: bool
}

impl Cell {
    fn new() -> Cell {
        Cell { walls: [true; 4], visited: false, row: 0, col: 0 }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.walls[Direction::East]  { try!(write!(f, "|")); } else { try!(write!(f, "_")); }
        if self.walls[Direction::South] { try!(write!(f, "__")); } else { try!(write!(f, "  ")); }
        write!(f, "")
    }
}

impl Index<Direction> for [bool] {
    type Output = bool;

    fn index(&self, direction: Direction) -> &bool {
        match direction {
            Direction::North => &self[0],
            Direction::East => &self[1],
            Direction::South => &self[2],
            Direction::West => &self[3],
        }
    }
}

impl IndexMut<Direction> for [bool] {
    fn index_mut(&mut self, direction: Direction) -> &mut bool {
        match direction {
            Direction::North => &mut self[0],
            Direction::East => &mut self[1],
            Direction::South => &mut self[2],
            Direction::West => &mut self[3],
        }
    }
}

struct Maze {
    cells: [[Cell; COLS]; ROWS]
}

impl Maze {
    fn new() -> Maze {
        let mut cells = [[Cell::new(); COLS]; ROWS];
        for row in 0 .. ROWS {
            for col in 0 .. COLS {
                cells[row][col].row = row;
                cells[row][col].col = col;
            }
        }
        Maze { cells: cells }
    }

    fn generate(&mut self) {
        // Make the initial cell the current cell and mark it as visited
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut current_cell = (0, 0);
        let mut visited = 1;
        self.cells[current_cell.0][current_cell.1].visited = true;

        // While there are unvisited cells
        while visited < ROWS * COLS {

            // Get unvisited neighbours of current cell
            let mut neighbours: Vec<(usize, usize)> = Vec::new();
            let row = current_cell.0;
            let col = current_cell.1;

            if row < ROWS - 1 {
                if !self.cells[row + 1][col].visited { neighbours.push((row + 1, col)); }
            }
            if row != 0   {
                if !self.cells[row - 1][col].visited { neighbours.push((row - 1, col)); }
            }
            if col < COLS - 1 {
                if !self.cells[row][col + 1].visited { neighbours.push((row, col + 1)); }
            }
            if col != 0   {
                if !self.cells[row][col - 1].visited { neighbours.push((row, col - 1)); }
            }

            // If the current cell has any neighbours which have not been visited
            if neighbours.len() > 0 {
                // Choose randomly one of the unvisited neighbours
                let n: usize = rng.gen_range(0, neighbours.len());
                let next_cell = neighbours.remove(n);

                // Remove the walls between current_cell and next_cell
                // (current_cell, next_cell)
                let directions: (Direction, Direction) = if next_cell.0 != current_cell.0 {
                    if next_cell.0 > current_cell.0 { (Direction::South, Direction::North) } else { (Direction::North, Direction::South) }
                } else {
                    if next_cell.1 > current_cell.1 { (Direction::West, Direction::East) } else { (Direction::East, Direction::West) }
                };

                // Push the current cell to the stack
                stack.push((current_cell.0, current_cell.1));

                // Remove the wall between the current cell and the chosen cell
                self.cells[current_cell.0][current_cell.1].walls[directions.0] = false;
                self.cells[next_cell.0][next_cell.1].walls[directions.1] = false;

                 // Make the chosen cell the current cell and mark it as visited
                current_cell = next_cell;
                self.cells[current_cell.0][current_cell.1].visited = true;
                visited += 1;
            } else { // Else if stack is not empty
                // Pop a cell from the stack and make it the current cell
                current_cell = stack.pop().unwrap();
            }
        }
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for _ in 0 .. COLS {
            try!(write!(f, "___"));
        }
        for row in self.cells.iter() {
            try!(write!(f, "\n"));
            for (col_num, col) in row.iter().enumerate() {
                try!(write!(f, "{}", col));
                if col_num == COLS - 1 { try!(write!(f, "|")); }
            }
        }
        write!(f, "")
    }
}

fn main() {
    let mut maze = Maze::new();
    maze.generate();
    println!("{}", maze);
}
