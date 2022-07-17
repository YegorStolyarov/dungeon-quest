use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::{self, BufRead};

pub mod door;
pub mod doors;
pub mod floor;
pub mod ladder;
pub mod position;
pub mod room;
pub mod rooms;
pub mod walls;

use crate::config::LIST_FLOOR_FILE;
use floor::Floor;

const TOTAL_DUNGEON_FLOORS: usize = 4;

pub struct Dungeon {
    pub floors: Vec<Floor>,
    pub current_floor: Floor,
}

impl Dungeon {
    pub fn new() -> Self {
        let file = match File::open(LIST_FLOOR_FILE) {
            Ok(file) => file,
            Err(err) => panic!("Can't open list floor file: {}", err.to_string()),
        };

        let buffered = io::BufReader::new(file);
        let lines: Vec<_> = buffered.lines().collect();
        let total_available_floors: usize = lines.len();

        let mut floors: Vec<Floor> = Vec::new();

        let mut rng = rand::thread_rng();
        let dice = Uniform::from(0..total_available_floors);

        let mut floors_indexes: Vec<usize> = Vec::new();
        loop {
            let throw = dice.sample(&mut rng);

            if !floors_indexes.contains(&throw) {
                floors_indexes.push(throw);
            }

            if floors_indexes.len() == TOTAL_DUNGEON_FLOORS {
                break;
            }
        }

        for (index, line) in lines.into_iter().enumerate() {
            if floors_indexes.contains(&index) {
                let map_name = line.unwrap();
                floors.push(Floor::new(map_name));
            }
        }

        let current_floor = floors[0].clone();

        Dungeon {
            floors,
            current_floor,
        }
    }
}
