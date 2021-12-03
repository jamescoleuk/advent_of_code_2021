use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    println!("Hello day_03!");
    let buffer = BufReader::new(File::open("day_03/input/real")?);
    let lines: Vec<String> = buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let oxygen_generator_rating = part_02(lines.clone(), 12, &oxygen_generator)?;
    let c02_scrubbed_rating = part_02(lines.clone(), 12, &c02_scribber)?;
    let radix = 2;
    let life_support_rating = i32::from_str_radix(&oxygen_generator_rating, radix).unwrap()
        * i32::from_str_radix(&c02_scrubbed_rating, radix).unwrap();
    println!("Life support rating: {}", life_support_rating);
    Ok(())
}

fn part_01_first_try(
    lines: Vec<String>,
    word_length: usize,
) -> Result<(String, String, i32), Error> {
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for i in 0..word_length {
        let mut gamma_count = 0;
        let mut epsilon_count = 0;
        for (j, item) in lines.iter().enumerate() {
            let char = item.chars().nth(i).unwrap();
            if char == '0' {
                gamma_count += 1;
            } else if char == '1' {
                epsilon_count += 1;
            }
        }

        if gamma_count < epsilon_count {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let power_consumption =
        i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap();
    println!("gamma: {} epsilon: {}", gamma, epsilon,);
    println!("power consumption: {}", power_consumption);
    Ok((gamma, epsilon, power_consumption))
}

fn part_02(
    lines: Vec<String>,
    word_length: usize,
    comparator: &dyn Fn(i32, i32) -> i32,
) -> Result<String, Error> {
    let mut our_lines = lines;
    for i in 0..word_length {
        let mut one_count = 0;
        let mut zero_count = 0;
        for item in our_lines.iter() {
            if item.chars().nth(i).unwrap() == '1' {
                one_count += 1;
            } else {
                zero_count += 1;
            }
        }

        // let this_bit_criterion = if one_count >= zero_count { 1 } else { 0 };
        // let this_bit_criterion = if one_count >= zero_count { 0 } else { 1 };
        let this_bit_criterion = comparator(one_count, zero_count);
        our_lines = our_lines
            .iter()
            .filter(|line| {
                let this_bit = line.chars().nth(i).unwrap();
                this_bit.to_string() == this_bit_criterion.to_string()
            })
            .cloned()
            .collect::<Vec<String>>();
        println!("---");
        our_lines.iter().for_each(|f| println!("{}", f));
        if our_lines.len() == 1 {
            break;
        }
    }

    Ok(our_lines[0].to_string())
}

fn oxygen_generator(one_count: i32, zero_count: i32) -> i32 {
    if one_count >= zero_count {
        1
    } else {
        0
    }
}

fn c02_scribber(one_count: i32, zero_count: i32) -> i32 {
    if one_count >= zero_count {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Error},
    };

    use crate::{c02_scribber, oxygen_generator, part_01_first_try, part_02};

    #[test]
    fn test() -> Result<(), Error> {
        let buffer = BufReader::new(File::open("input/test")?);
        let lines: Vec<String> = buffer
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        let results = part_01_first_try(lines, 5)?;
        assert_eq!(results.0, "10110");
        assert_eq!(results.1, "01001");
        assert_eq!(results.2, 198);

        Ok(())
    }

    #[test]
    fn test_part_02() -> Result<(), Error> {
        let buffer = BufReader::new(File::open("input/test")?);
        let lines: Vec<String> = buffer
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        let result = part_02(lines.clone(), 5, &oxygen_generator)?;
        assert_eq!(result, "10111");
        assert_eq!(i32::from_str_radix(&result, 2).unwrap(), 23);

        let result = part_02(lines.clone(), 5, &c02_scribber)?;
        assert_eq!(result, "01010");
        assert_eq!(i32::from_str_radix(&result, 2).unwrap(), 10);
        Ok(())
    }
}
