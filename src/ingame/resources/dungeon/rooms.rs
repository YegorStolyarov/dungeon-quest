use std::fs::File;
use std::io::{self, BufRead};

use crate::config::LIST_ROOM_FILE;
use crate::ingame::resources::dungeon::room::Room;

pub struct Rooms {
    rooms: Vec<Room>,
    total_rooms: usize,
}

impl Rooms {
    pub fn new() -> Self {
        let file = match File::open(LIST_ROOM_FILE) {
            Ok(file) => file,
            Err(err) => panic!("Can't open list room file: {}", err.to_string()),
        };

        let buffered = io::BufReader::new(file);
        let lines: Vec<_> = buffered.lines().collect();
        let total_rooms: usize = lines.len();

        let mut rooms: Vec<Room> = Vec::new();

        for line in lines {
            let file_name = line.unwrap();
            rooms.push(Room::new(file_name));
        }

        Rooms { rooms, total_rooms }
    }
}
