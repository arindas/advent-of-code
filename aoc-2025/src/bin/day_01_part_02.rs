use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file_path = std::env::args()
        .nth(1)
        .expect("Input file path not specified!");

    println!("{}", input_file_path);

    let file = File::open(input_file_path).expect("unable to open file");
    let buf_reader = BufReader::new(file);

    let mut dial_value: i32 = 50;
    let mut line_count: i32 = 0;

    let mut zero_count: i32 = 0;

    const NUM_DIAL_ENTRIES: i32 = 100;

    for line in buf_reader.lines().map_while(|x| x.ok()) {
        line_count += 1;

        let (direction, rotations) = line.split_at(1);

        let rotations: i32 = rotations.parse().expect("rotations not a number!");

        let delta = match direction {
            "L" => -1,
            "R" => 1,
            _ => break,
        };

        for _ in 0..rotations {
            dial_value += delta;
            dial_value %= NUM_DIAL_ENTRIES;

            if dial_value == 0 {
                zero_count += 1;
            }
        }
    }

    println!("Answer: {zero_count}, read {line_count} lines.");
}
