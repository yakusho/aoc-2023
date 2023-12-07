pub fn part_a() {
    let input = std::fs::read_to_string("src/input/day_06").unwrap();
    let mut aggregator: Vec<u64> = Vec::new();
    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut n = 0;

        for i in 1..time {
            let distance_traveled = i * (time - i);
            if distance_traveled > distance {
                n += 1
            }
        }

        aggregator.push(n);
    }

    println!("{}", aggregator.iter().product::<u64>());
}

pub fn part_b() {
    let input = std::fs::read_to_string("src/input/day_06").unwrap();
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    let distance: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    let mut n = 0;
    for i in 1..time {
        let distance_traveled = i * (time - i);
        if distance_traveled > distance {
            n += 1
        }
    }

    println!("{}", n);
}
