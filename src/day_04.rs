use std::collections::{HashMap, HashSet};

pub fn part_a() {
    let file = std::fs::read_to_string("src/test_input/day_04").unwrap();
    let result: u32 = file
        .lines()
        .map(|line| {
            let numbers: Vec<HashSet<&str>> = line
                .split(":")
                .nth(1)
                .unwrap()
                .split("|")
                .map(|s| s.split_whitespace().collect::<HashSet<_>>())
                .collect();

            let (your_numbers, winning_numbers) = (&numbers[0], &numbers[1]);

            let overlap_count: u32 = your_numbers
                .intersection(&winning_numbers)
                .count()
                .try_into()
                .unwrap();

            if overlap_count > 0 {
                return 2_u32.pow(overlap_count - 1);
            } else {
                return 0;
            }
        })
        .sum();

    println!("{}", result);
}

pub fn part_b() {
    let file = std::fs::read_to_string("src/test_input/day_04").unwrap();
    let games_count: usize = file.lines().count();
    let mut game_occurrences: HashMap<usize, usize> = (1..=games_count).map(|key| (key, 1)).collect();

    file.lines().for_each(|line| {
        let game_number = line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.trim_end_matches(":").parse().ok())
            .unwrap_or(0);

        let numbers: Vec<HashSet<&str>> = line
            .split(":")
            .nth(1)
            .unwrap()
            .split("|")
            .map(|s| s.split_whitespace().collect::<HashSet<_>>())
            .collect();

        let (your_numbers, winning_numbers) = (&numbers[0], &numbers[1]);
        let overlap_count = your_numbers.intersection(&winning_numbers).count();

        let occurrences = *game_occurrences.get(&game_number).unwrap_or(&0);
        for i in (game_number + 1)..=(overlap_count + game_number) {
            game_occurrences
                .entry(i)
                .and_modify(|e| (*e += 1 * occurrences))
                .or_insert(1);
        }
    });

    println!(
        "{:?}",
        game_occurrences.values().sum::<usize>()
    )
}
