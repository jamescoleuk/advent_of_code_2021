use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    println!("Hello day_01!");
    part_01()?;
    part_02()?;
    part_02_window()?;
    Ok(())
}

fn part_01() -> Result<(), Error> {
    let depth = File::open("day_01/input/depths.txt")?;
    let buffer = BufReader::new(depth);
    let mut increase_count = 0;
    let mut previous_depth = 0;
    for line in buffer.lines() {
        let current_depth = line.unwrap().parse::<u16>().unwrap();
        // if the previous_depth is 0 then we're on the first path and don't want to count that.
        if previous_depth != 0 && current_depth > previous_depth {
            increase_count += 1;
        }
        previous_depth = current_depth
    }
    println!("Part 1: individual increases: {}", increase_count);

    Ok(())
}

/// I think I'm going to do this in two passes. That'll keep the
/// second part the same as the first part, and all that's left
/// to do is calculate the windows.
fn part_02() -> Result<(), Error> {
    let depth = File::open("day_01/input/depths.txt")?;
    let buffer = BufReader::new(depth);

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    let mut increase_count = 0;
    for (index, line) in buffer.lines().into_iter().enumerate() {
        let current_depth = line.unwrap().parse::<u16>().unwrap();
        // Ignore calculations while we're starting out and don't have a full window.
        if index == 0 {
            a = current_depth
        } else if index == 1 {
            b = current_depth
        } else if index == 2 {
            c = current_depth
        } else {
            let previous_depth_agg = a + b + c;
            // Shuffle things down
            a = b;
            b = c;
            c = current_depth;
            let current_depth_agg = a + b + c;
            if previous_depth_agg != 0 && current_depth_agg > previous_depth_agg {
                increase_count += 1;
            }
        }
    }
    println!("Part 2: aggregated increases: {}", increase_count);
    Ok(())
}

fn part_02_window() -> Result<(), Error> {
    let depth = File::open("day_01/input/depths.txt")?;
    let buffer = BufReader::new(depth);
    let mut depths: Vec<u16> = vec![];
    for line in buffer.lines() {
        depths.push(line.unwrap().parse::<u16>().unwrap());
    }

    let mut prev_window = 0;
    let mut increases = 0;
    // Considered using iterators all the way but we're doing side-effects
    // in prev_window, so a for loop is more idiomatic.
    for window in depths.windows(3) {
        let current_window: u16 = window.iter().sum();
        if prev_window != 0 && current_window > prev_window {
            increases += 1;
        }
        prev_window = current_window;
    }
    println!("Part 2: aggregated increases using windows: {}", increases);
    Ok(())
}
