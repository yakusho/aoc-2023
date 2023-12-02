pub fn part_a() {
    let file = std::fs::read_to_string("src/input/day_01").unwrap();
    let counter: u32 = file
        .lines()
        .map(|line| {
            let mut first_digit = 0;
            let mut last_digit = 0;

            for character in line.chars() {
                if character.is_ascii_digit() {
                    first_digit = character.to_digit(10).unwrap();
                    break;
                }
            }

            for character in line.chars().rev() {
                if character.is_ascii_digit() {
                    last_digit = character.to_digit(10).unwrap();
                    break;
                }
            }

            return first_digit * 10 + last_digit;
        })
        .sum();

    println!("{}", counter)
}

pub fn part_b() {
    let word_to_number: Vec<(&str, &str)> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let file = std::fs::read_to_string("src/input/day_01").unwrap();
    let counter: u32 = file
        .lines()
        .map(|line| {
            let mut first_digit = 0;
            let mut last_digit = 0;
            let mut word = String::new();

            for character in line.chars() {
                if let Some((_, value)) =
                    word_to_number.iter().find(|&&(key, _)| word.contains(key))
                {
                    first_digit = value.parse::<u32>().unwrap();
                    word.clear();
                    break;
                }

                if character.is_ascii_digit() {
                    first_digit = character.to_digit(10).unwrap();
                    word.clear();
                    break;
                }

                if character.is_alphabetic() {
                    word.push(character)
                }
            }

            for character in line.chars().rev() {
                if let Some((_, value)) = word_to_number.iter().find(|&&(key, _)| {
                    let reversed_key = key.chars().rev().collect::<String>();
                    return word.contains(reversed_key.as_str());
                }) {
                    last_digit = value.parse::<u32>().unwrap();
                    word.clear();
                    break;
                }

                if character.is_ascii_digit() {
                    last_digit = character.to_digit(10).unwrap();
                    word.clear();
                    break;
                }

                if character.is_alphabetic() {
                    word.push(character)
                }
            }

            return first_digit * 10 + last_digit;
        })
        .sum();

    println!("{}", counter)
}
