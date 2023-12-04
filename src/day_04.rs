pub fn part_a() {
    let file = std::fs::read_to_string("src/input/day_04").unwrap();
    let result: u32 = file
        .lines()
        .map(|line| {
            let numbers = line.split(":").nth(1).unwrap();
            let your_numbers = numbers
                .split("|")
                .nth(0)
                .unwrap()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u32>>();

            let winning_numbers = numbers
                .split("|")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u32>>();

            let overlap_count: u32 = your_numbers
                .iter()
                .filter(|&element| winning_numbers.contains(element))
                .count()
                .try_into()
                .ok()
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
    let file = std::fs::read_to_string("src/input/day_04").unwrap();
    let mut repeat_games: Vec<u32> = Vec::new();
    let games_count = file.lines().count();

    file.lines().for_each(|line| {
        let game_number = line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.trim_end_matches(":").parse().ok())
            .unwrap_or(0);

        let occurences = repeat_games
            .iter()
            .filter(|&&num| num == game_number)
            .count();

        let numbers = line.split(":").nth(1).unwrap().split("|");
        let your_numbers = numbers
            .clone()
            .nth(0)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let winning_numbers = numbers
            .clone()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let overlap_count = your_numbers
            .iter()
            .filter(|&element| winning_numbers.contains(element))
            .count();

        for _ in 0..=occurences {
            for i in 1..=overlap_count {
                let index: u32 = i.try_into().ok().unwrap_or(0);
                repeat_games.push(game_number + index);
            }
        }
    });

    println!("{:?}", repeat_games.len() + games_count)
}
