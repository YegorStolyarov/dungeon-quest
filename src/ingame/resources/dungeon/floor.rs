use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::ingame::resources::dungeon::position::Position;

pub struct Floor {
    pub map: Vec<Vec<f32>>,
    pub total_rows: usize,
    pub total_columns: usize,
    pub current_position: Position,
    pub end_room_position: Position,
    pub start_room_position: Position,
    pub cleared_positions: Vec<Position>,
}

const PREFIX: &str = "./assets/floors/";

impl Floor {
    pub fn new(file_name: String) -> Floor {
        let path = format!("{}{}", PREFIX, file_name);
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("Can't open map file {}: {}", file_name, err.to_string()),
        };

        let reader = BufReader::new(file);

        let mut map: Vec<Vec<f32>> = Vec::new();

        let mut start_room_position: Position = Position {
            row_index: 0,
            column_index: 0,
        };
        let mut end_room_position: Position = Position {
            row_index: 0,
            column_index: 0,
        };

        for (row_index, line) in reader.lines().enumerate() {
            let str_line = line.unwrap();
            let str_numbers = str_line.split(" ");

            let mut row: Vec<f32> = Vec::new();

            for (column_index, str_number) in str_numbers.enumerate() {
                let number: f32 = str_number.parse::<f32>().unwrap();

                if number == number.floor() + 0.1 {
                    start_room_position = Position {
                        row_index,
                        column_index,
                    };
                } else if number == number.floor() + 0.2 {
                    end_room_position = Position {
                        row_index,
                        column_index,
                    }
                }
                row.push(number);
            }
            map.push(row);
        }

        let total_rows = map.len();
        let total_columns = map[0].len();

        Floor {
            map,
            start_room_position,
            end_room_position,
            current_position: start_room_position,
            cleared_positions: Vec::new(),
            total_rows,
            total_columns,
        }
    }
}
