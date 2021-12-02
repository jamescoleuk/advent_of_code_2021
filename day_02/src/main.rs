use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

/// Tomorrow I will:
/// 1. pass the data into the algorithm as a vec
/// 2. have a test that passes in the test data and the real thing can parse it
///    that way it parses for both the test and the real thing.
fn main() -> Result<(), Error> {
    println!("Hello day_02!");
    get_final_position()?;
    get_aimed_position("day_02/input/test.txt".to_string())?;
    get_aimed_position("day_02/input/movement.txt".to_string())?;
    Ok(())
}

// Part 1
fn get_final_position() -> Result<(), Error> {
    let movement = File::open("day_02/input/movement.txt")?;
    let buffer = BufReader::new(movement);
    let mut depth = 0;
    let mut forward = 0;
    for line in buffer.lines() {
        let line = line?;
        let movement = line.split(' ').collect::<Vec<&str>>();
        match movement[0] {
            "forward" => forward += movement[1].parse::<i32>().unwrap(),
            "down" => depth += movement[1].parse::<i32>().unwrap(),
            "up" => depth -= movement[1].parse::<i32>().unwrap(),
            &_ => panic!("Unknown movement: {}!", movement[0]),
        }
    }
    println!("Final depth: {}", depth);
    println!("Final forward: {}", forward);
    println!("Final position: {}", depth * forward);
    Ok(())
}

// Part 2
fn get_aimed_position(input_path: String) -> Result<(), Error> {
    let movement = File::open(input_path)?;
    let buffer = BufReader::new(movement);
    let mut depth: i64 = 0;
    let mut forward: i64 = 0;
    let mut aim: i64 = 0;
    for line in buffer.lines() {
        let line = line?;
        let movement = line.split(' ').collect::<Vec<&str>>();
        match movement[0] {
            "forward" => {
                forward += movement[1].parse::<i64>().unwrap();
                let depth_increase = movement[1].parse::<i64>().unwrap() * aim;
                depth += depth_increase;
            }
            "down" => {
                aim += movement[1].parse::<i64>().unwrap();
            }
            "up" => {
                aim -= movement[1].parse::<i64>().unwrap();
            }
            &_ => panic!("Unknown movement: {}!", movement[0]),
        }
    }
    println!("Aimed depth: {}", depth);
    println!("Aimed forward: {}", forward);
    println!("Final aim: {}", depth * forward);
    Ok(())
}
