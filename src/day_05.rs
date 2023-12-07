use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

struct Rule {
    source: i64,
    destination: i64,
    range: i64,
}

pub fn convert(seed: i64, source: i64, destination: i64, range: i64) -> i64 {
    if (source..source + range).contains(&seed) {
        return destination + (seed - source);
    }

    return seed;
}

pub fn part_a() {
    let input = std::fs::read_to_string("src/input/day_05").unwrap();
    let (split_seed, split_map) = input.split_once("\n\n").unwrap();
    let mut seeds: Vec<i64> = split_seed
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let maps: Vec<Vec<Rule>> = split_map
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let mut split_line = line.split_whitespace();
                    Rule {
                        destination: split_line.next().unwrap().parse().unwrap(),
                        source: split_line.next().unwrap().parse().unwrap(),
                        range: split_line.next().unwrap().parse().unwrap(),
                    }
                })
                .sorted_by(|a, b| a.source.cmp(&b.source))
                .collect()
        })
        .collect();

    seeds.iter_mut().for_each(|seed| {
        maps.iter().for_each(|map| {
            let mut found = false;
            map.iter().for_each(|rule| {
                let converted_seed = convert(*seed, rule.source, rule.destination, rule.range);
                if converted_seed != *seed && !found {
                    *seed = converted_seed;
                    found = true;
                }
            })
        })
    });

    let min_location = seeds.iter().min().unwrap();

    println!("{}", min_location);
}

pub fn part_b() {
    let input = std::fs::read_to_string("src/input/day_05").unwrap();
    let (split_seed, split_map) = input.split_once("\n\n").unwrap();
    let mut seeds: Vec<Range> = split_seed
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap();
            let range = chunk.next().unwrap();
            Range {
                start,
                end: start + range,
            }
        })
        .collect();

    let maps: Vec<Vec<Rule>> = split_map
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let mut split_line = line.split_whitespace();
                    Rule {
                        destination: split_line.next().unwrap().parse().unwrap(),
                        source: split_line.next().unwrap().parse().unwrap(),
                        range: split_line.next().unwrap().parse().unwrap(),
                    }
                })
                .sorted_by(|a, b| a.source.cmp(&b.source))
                .collect()
        })
        .collect();

    maps.iter().for_each(|map| {
        let mut ranges: Vec<Range> = Vec::new();
        seeds.iter_mut().for_each(|range| {
            let mut current_range = range.clone();

            map.iter().for_each(|rule| {
                let offset = rule.destination - rule.source;
                let valid = current_range.start <= rule.source + rule.range
                    && current_range.start <= current_range.end
                    && current_range.end >= rule.source;

                if valid {
                    if current_range.start < rule.source {
                        ranges.push(Range {
                            start: current_range.start,
                            end: rule.source - 1,
                        });

                        current_range.start = rule.source;

                        if current_range.end < rule.source + rule.range {
                            ranges.push(Range {
                                start: current_range.start + offset,
                                end: current_range.end + offset,
                            });

                            current_range.start = current_range.end + 1;
                        } else {
                            ranges.push(Range {
                                start: current_range.start + offset,
                                end: rule.source + rule.range - 1 + offset,
                            });

                            current_range.start = rule.source + rule.range;
                        }
                    } else if current_range.end < rule.source + rule.range {
                        ranges.push(Range {
                            start: current_range.start + offset,
                            end: current_range.end + offset,
                        });

                        current_range.start = current_range.end + 1;
                    } else {
                        ranges.push(Range {
                            start: current_range.start + offset,
                            end: rule.source + rule.range - 1 + offset,
                        });

                        current_range.start = rule.source + rule.range;
                    }
                }
            });

            if current_range.start <= current_range.end {
                ranges.push(current_range);
            }
        });

        seeds = ranges;
    });

    println!("{}", seeds.iter().map(|range| range.start).min().unwrap());
}
