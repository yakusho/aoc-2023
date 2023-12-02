use std::collections::HashMap;

pub fn part_a() {
    let mut counter = 0;
    let file = std::fs::read_to_string("src/input/day_02").unwrap();
    for line in file.lines() {
        if let Some(colon_pos) = line.find(':') {
            let game_number: u32 = line[5..colon_pos].trim().parse().unwrap_or_default();
            let remaining_input = line[colon_pos + 1..].trim();
            let mut ignore_flag = false;

            let parts: Vec<&str> = remaining_input
                .split(";")
                .flat_map(|s| s.split_whitespace())
                .collect();

            let mut iter = parts.into_iter();
            while let Some(number_str) = iter.next() {
                if let Ok(number) = number_str.parse::<u32>() {
                    if let Some(color) = iter.next() {
                        let color = color.trim_end_matches(',');
                        match color {
                            "red" if number > 12 => ignore_flag = true,
                            "green" if number > 13 => ignore_flag = true,
                            "blue" if number > 14 => ignore_flag = true,
                            _ => {}
                        }
                    }
                }
            }

            if !ignore_flag {
                counter += game_number;
            }
        } else {
            println!("Invalid input format");
        }
    }

    println!("{}", counter)
}

pub fn part_b() {
    let mut counter = 0;
    let file = std::fs::read_to_string("src/input/day_02").unwrap();
    for line in file.lines() {
        if let Some(colon_pos) = line.find(':') {
            let remaining_input = line[colon_pos + 1..].trim();
            let mut max_values: HashMap<&str, u32> = HashMap::new();

            let parts: Vec<&str> = remaining_input
                .split(";")
                .flat_map(|s| s.split_whitespace())
                .collect();

            let mut iter = parts.into_iter();
            while let Some(number_str) = iter.next() {
                if let Ok(number) = number_str.parse::<u32>() {
                    if let Some(color) = iter.next() {
                        let color = color.trim_end_matches(',');
                        let max_value = max_values.entry(color).or_insert(0);
                        if number > *max_value {
                            *max_value = number;
                        }
                    }
                }
            }

            let mut multiplication_result = 1;
            for max_value in max_values.values() {
                multiplication_result *= max_value;
            }

            counter += multiplication_result;
        } else {
            println!("Invalid input format");
        }
    }

    println!("{}", counter)
}
