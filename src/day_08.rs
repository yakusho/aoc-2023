use itertools::Itertools;
use std::collections::HashMap;
use num::integer::lcm;

fn lcm_vector(numbers: &[u64]) -> u64 {
  let mut result = numbers[0];
  for &num in &numbers[1..] {
      result = lcm(result, num);
  }

  result
}

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

    let mut sides: HashMap<char, usize> = HashMap::new();
    sides.insert('L', 0);
    sides.insert('R', 1);

    let mut curr_map = "AAA".to_string();
    let mut end_found = false;
    let mut i = 0;

    while !end_found {
        let instruction = i % modulo;
        let curr_command = commands[instruction];
        let values = maps.get(&curr_map).unwrap();
        curr_map = values[*sides.get(&curr_command).unwrap()].clone();

        if curr_map == "ZZZ" {
            end_found = true;
        }

        i += 1;
    }

    println!("{}", i);
}

pub fn part_b() {
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

    let mut sides: HashMap<char, usize> = HashMap::new();
    sides.insert('L', 0);
    sides.insert('R', 1);


    let mut curr_maps: Vec<String> = maps
        .keys()
        .filter(|&key| key.ends_with("A"))
        .cloned()
        .collect();

    let mut steps: Vec<u64> = Vec::new();

    curr_maps.iter_mut().for_each(|curr_map| {
      let mut end_found = false;
      let mut i = 0;
  
      while !end_found {
          let instruction = i % modulo;
          let curr_command = commands[instruction];
          let values = maps.get(curr_map).unwrap();
          *curr_map = values[*sides.get(&curr_command).unwrap()].clone();
  
          if curr_map.ends_with('Z') {
              end_found = true;
          }
  
          i += 1;
      }

      steps.push(i.try_into().unwrap());
    });

    println!("{}", lcm_vector(&steps));
}
