use regex::Regex;

#[derive(Debug)]
struct Plant {
    seed: u64,
    soil: u64,
    fertilizer: u64,
    water: u64,
    light: u64,
    temperature: u64,
    humidity: u64,
    location: u64,
}

pub fn find_n(n: u64, x: u64, y: u64, range: u64) -> Option<u64> {
    if n >= x && n < x + range {
        Some(y + (n - x))
    } else {
        None
    }
}

fn convert_category(plants: &mut Vec<Plant>, current_step: &str, line: &str) {
    let mut iter = line.split_whitespace();
    let y: u64 = iter.next().unwrap_or_default().parse().ok().unwrap_or(0);
    let x: u64 = iter.next().unwrap_or_default().parse().ok().unwrap_or(0);
    let range: u64 = iter.next().unwrap_or_default().parse().ok().unwrap_or(0);

    plants.iter_mut().for_each(|plant| {
        let found_n = match current_step {
            "seed-to-soil map:" => find_n(plant.seed, x, y, range),
            "soil-to-fertilizer map:" => find_n(plant.soil, x, y, range),
            "fertilizer-to-water map:" => find_n(plant.fertilizer, x, y, range),
            "water-to-light map:" => find_n(plant.water, x, y, range),
            "light-to-temperature map:" => find_n(plant.light, x, y, range),
            "temperature-to-humidity map:" => find_n(plant.temperature, x, y, range),
            "humidity-to-location map:" => find_n(plant.humidity, x, y, range),
            _ => Some(0),
        };
        match current_step {
            "seed-to-soil map:" => {
                found_n.map(|value| {
                    plant.soil = value;
                    plant.fertilizer = value;
                    plant.water = value;
                    plant.light = value;
                    plant.temperature = value;
                    plant.humidity = value;
                    plant.location = value;
                });
            }
            "soil-to-fertilizer map:" => {
                found_n.map(|value| {
                    plant.fertilizer = value;
                    plant.water = value;
                    plant.light = value;
                    plant.temperature = value;
                    plant.humidity = value;
                    plant.location = value;
                });
            }
            "fertilizer-to-water map:" => {
                found_n.map(|value| {
                    plant.water = value;
                    plant.light = value;
                    plant.temperature = value;
                    plant.humidity = value;
                    plant.location = value;
                });
            }
            "water-to-light map:" => {
                found_n.map(|value| {
                    plant.light = value;
                    plant.temperature = value;
                    plant.humidity = value;
                    plant.location = value;
                });
            }
            "light-to-temperature map:" => {
                found_n.map(|value| {
                    plant.temperature = value;
                    plant.humidity = value;
                    plant.location = value;
                });
            }
            "temperature-to-humidity map:" => {
                found_n.map(|value| {
                    plant.humidity = value;
                    plant.location = value;
                });
            }
            "humidity-to-location map:" => {
                found_n.map(|value| {
                    plant.location = value;
                });
            }
            _ => {}
        }
    });
}

pub fn part_a() {
    let file = std::fs::read_to_string("src/input/day_05").unwrap();
    let mut plants: Vec<Plant> = Vec::new();
    let mut current_step = String::new();

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if line.starts_with("seeds:") {
                let pattern = Regex::new(r"seeds:\s*(?P<values>.+)").unwrap();
                let seeds: Option<Vec<&str>> = pattern
                    .captures(line)
                    .map(|captures| captures.name("values"))
                    .map(|values| values.unwrap().as_str().split_whitespace().collect());

                seeds.unwrap().iter().for_each(|seed| {
                    plants.push(Plant {
                        seed: seed.parse().ok().unwrap_or(0),
                        soil: seed.parse().ok().unwrap_or(0),
                        fertilizer: seed.parse().ok().unwrap_or(0),
                        water: seed.parse().ok().unwrap_or(0),
                        light: seed.parse().ok().unwrap_or(0),
                        temperature: seed.parse().ok().unwrap_or(0),
                        humidity: seed.parse().ok().unwrap_or(0),
                        location: seed.parse().ok().unwrap_or(0),
                    })
                });
            }

            if line.contains("map") {
                current_step = line.to_string();
                return;
            }

            convert_category(&mut plants, &current_step, &line);
        });

    let min_location = plants
        .iter()
        .min_by_key(|plant| plant.location)
        .unwrap()
        .location;

    println!("{}", min_location);
}

// pub fn part_b() {
// }
