use itertools::Itertools;
use std::collections::HashMap;

pub fn part_a() {
    let input = std::fs::read_to_string("src/input/day_08").unwrap();
    let (commands_str, maps_str) = input.split_once("\n\n").unwrap();

    let mut maps: HashMap<String, Vec<String>> = HashMap::new();
    let split_map = maps_str.split("\n");

    split_map.for_each(|map| {
        let (key, dest1, dest2) = map
            .split(|c| c == '=' || c == ',')
            .map(|s| s.trim().trim_end_matches(")").trim_start_matches("("))
            .collect_tuple()
            .expect("Invalid input format");
        maps.insert(key.to_string(), vec![dest1.to_string(), dest2.to_string()]);
    });

    let commands: Vec<char> = commands_str.chars().collect();
    let modulo = commands.len();

    let mut end_found = false;
    let mut i = 0;
    let mut curr_map = "AAA".to_string();

    while !end_found {
        let instruction = i % modulo;
        let curr_command = commands[instruction];
        let values = maps.get(&curr_map).unwrap();
        if curr_command == 'L' {
            curr_map = values[0].clone();
        } else if curr_command == 'R' {
            curr_map = values[1].clone();
        }

        if curr_map == "ZZZ" {
            end_found = true;
        }

        i += 1;
    }

    println!("{}", i);
}
