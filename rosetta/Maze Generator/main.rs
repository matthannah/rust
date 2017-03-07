use std::ops::Index;

enum Direction {
    North,
    East,
    South,
    West
}

struct Cell {
    walls: [bool; 4]
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

fn main() {
    let c = Cell { walls: [true, false, true, false] };
    println!("{}", c.walls[Direction::East]);
}
