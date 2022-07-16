use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PREFIX: &str = "./assets/rooms/";

pub struct Room {
    id: f32,
    tilemap: Vec<Vec<i32>>,
}

impl Room {
    pub fn new(file_name: String) -> Self {
        let spilt_file_name: Vec<&str> = file_name.split(".").collect();

        let id = spilt_file_name[0].parse::<f32>().unwrap();

        let path = format!("{}{}", PREFIX, file_name);
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("Can't open map file {}: {}", file_name, err.to_string()),
        };

        let reader = BufReader::new(file);

        let mut tilemap: Vec<Vec<i32>> = Vec::new();

        for line in reader.lines() {
            let str_line = line.unwrap();
            let str_numbers = str_line.split(" ");

            let mut row: Vec<i32> = Vec::new();

            for str_number in str_numbers {
                let number: i32 = str_number.parse::<i32>().unwrap();
                row.push(number);
            }
            tilemap.push(row);
        }

        Room { id, tilemap }
    }
}
