use std::fmt::Display;

fn main() {
    let n = read_n().unwrap();
    let mut college = College::new();

    for _ in 0..n {
        match read_room_information() {
            Some(information) => {
                college.apply_information(information);
            }
            None => break,
        }
    }

    println!("{}", college);
}

fn read_n() -> Option<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().ok()
}

fn read_room_information() -> Option<Information> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let bfrv = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if bfrv.len() == 4 {
        Some(Information {
            building: bfrv[0],
            floor: bfrv[1],
            room: bfrv[2],
            value: bfrv[3],
        })
    } else {
        None
    }
}

struct Information {
    building: i32,
    floor: i32,
    room: i32,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
struct Room {
    residents_number: i32,
}

impl Room {
    fn new() -> Room {
        Room {
            residents_number: 0,
        }
    }

    fn increace_residence(&mut self, value: i32) {
        self.residents_number += value;
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.residents_number)
    }
}

#[derive(Debug, Clone, Copy)]
struct Floor {
    rooms: [Room; 10],
}

impl Floor {
    fn new() -> Floor {
        Floor {
            rooms: [Room::new(); 10],
        }
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .rooms
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, " {}", s)
    }
}

#[derive(Debug, Clone, Copy)]
struct Building {
    floors: [Floor; 3],
}

impl Building {
    fn new() -> Building {
        Building {
            floors: [Floor::new(); 3],
        }
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .floors
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy)]
struct College {
    buildings: [Building; 4],
}

impl College {
    fn new() -> College {
        College {
            buildings: [Building::new(); 4],
        }
    }

    fn apply_information(&mut self, information: Information) {
        let room = &mut self.buildings[information.building as usize - 1].floors
            [information.floor as usize - 1]
            .rooms[information.room as usize - 1];

        room.increace_residence(information.value);
    }
}

impl Display for College {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .buildings
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(&("\n".to_string() + &"#".repeat(20) + "\n"));
        write!(f, "{}", s)
    }
}
