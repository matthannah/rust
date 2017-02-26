struct Block {
    letter_one: char,
    letter_two: char,
    used: bool
}

impl Block {
    fn new(letter_one: char, letter_two: char) -> Block {
        Block{ letter_one: letter_one, letter_two: letter_two, used: false}
    }

    fn has(&self, letter: char) -> bool {
        if !self.used {
            if self.letter_one == letter || self.letter_two == letter {
                return true
            }
        }
        false
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {})", self.letter_one, self.letter_two)
    }
}

fn main() {
    let mut blocks: Vec<Block> = initialise_blocks();
    // collect arguments from the env
    let args: Vec<_> = std::env::args().collect();
    for arg in args.iter().skip(1) { // skip first arg: /path/to/main.exe
        println!("can_make_word('{}')", arg);
        println!("{}", can_make_word(arg, &mut blocks));
        reset_blocks(&mut blocks);
    }
}

fn can_make_word(string: &str, blocks: &mut Vec<Block>) -> bool {
    for c in string.chars() {
        let mut pass = false;
        for block in blocks.iter_mut() {
            if block.has(c) {
                block.used = true;
                pass = true;
                break;
            }
        }
        if !pass {
            return false
        }
    }
    true
}

fn reset_blocks(blocks: &mut Vec<Block>) {
    for block in blocks {
        block.used = false;
    }
}

fn initialise_blocks() -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    blocks.push(Block::new('B', 'O'));
    blocks.push(Block::new('X', 'K'));
    blocks.push(Block::new('D', 'Q'));
    blocks.push(Block::new('C', 'P'));
    blocks.push(Block::new('N', 'A'));
    blocks.push(Block::new('G', 'T'));
    blocks.push(Block::new('R', 'E'));
    blocks.push(Block::new('T', 'G'));
    blocks.push(Block::new('Q', 'D'));
    blocks.push(Block::new('F', 'S'));
    blocks.push(Block::new('J', 'W'));
    blocks.push(Block::new('H', 'U'));
    blocks.push(Block::new('V', 'I'));
    blocks.push(Block::new('A', 'N'));
    blocks.push(Block::new('O', 'B'));
    blocks.push(Block::new('E', 'R'));
    blocks.push(Block::new('F', 'S'));
    blocks.push(Block::new('L', 'Y'));
    blocks.push(Block::new('P', 'C'));
    blocks.push(Block::new('Z', 'M'));
    blocks
}
