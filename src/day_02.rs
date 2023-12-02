use std::collections::HashMap;

pub fn part_a() {
    let file = std::fs::read_to_string("src/input/day_02").unwrap();
    let count = file
        .lines()
        .filter_map(|line| {
            let game_number = line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.trim_end_matches(":").parse().ok())
                .unwrap_or(0);

            let input = line.split(":").nth(1).unwrap();
            let colon_replaced = input.replace(";", ",");

            let pairs: Vec<(u32, &str)> = colon_replaced
                .split(",")
                .map(|s| {
                    let parts: Vec<&str> = s.trim().split_whitespace().collect();
                    let color = parts[1];
                    let count = parts[0].parse().unwrap_or(0);
                    (count, color)
                })
                .collect();

            let is_invalid = pairs.iter().any(|&(value, color)| match color {
                "red" if value > 12 => true,
                "green" if value > 13 => true,
                "blue" if value > 14 => true,
                _ => false,
            });

            if is_invalid {
                None
            } else {
                Some(game_number)
            }
        })
        .sum::<u32>();

    println!("{}", count);
}

pub fn part_b() {
    let file = std::fs::read_to_string("src/input/day_02").unwrap();
    let count = file
        .lines()
        .map(|line| {
            let input = line.split(":").nth(1).unwrap();
            let colon_replaced = input.replace(";", ",");

            let pairs: Vec<(u32, &str)> = colon_replaced
                .split(",")
                .map(|s| {
                    let parts: Vec<&str> = s.trim().split_whitespace().collect();
                    let color = parts[1];
                    let count = parts[0].parse().unwrap_or(0);
                    (count, color)
                })
                .collect();

            let value = pairs
                .into_iter()
                .fold(HashMap::new(), |mut acc, (value, key)| {
                    acc.entry(key)
                        .and_modify(|current_value| {
                            if value > *current_value {
                                *current_value = value;
                            }
                        })
                        .or_insert(value);
                    acc
                });

            let result: u32 = value.values().product();
            return result;
        })
        .sum::<u32>();

    println!("{}", count);
}
