enum Door {
    Open,
    Closed
}

impl Door {
    fn toggle(&mut self) {
        match *self {
            Door::Open => { *self = Door::Closed },
            Door::Closed => { *self = Door::Open }
        }
    }
}

impl std::fmt::Display for Door {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Door::Open => write!(f, "Open"),
            Door::Closed => write!(f, "Closed")
        }
    }
}

fn main() {
    let num_of_passes: usize = 100;
    let mut doors: Vec<Door> = initialise_doors();

    for pass_number in 1 .. num_of_passes + 1 {
        for (door_num, door) in doors.iter_mut().enumerate() {
            if (door_num + 1) % pass_number == 0 { door.toggle() }
        }
    }

    print_doors(&mut doors);
}

fn initialise_doors() -> Vec<Door> {
    let num_of_doors: u8 = 100;
    let mut doors: Vec<Door> = Vec::new();

    for _ in 0 .. num_of_doors {
        doors.push(Door::Closed);
    }

    doors
}

fn print_doors(doors: &mut Vec<Door>) {
    for (door_num, door) in doors.iter().enumerate() {
        println!("door number {} is {}", door_num + 1, door);
    }
}
