pub fn part_a() {
  let mut counter = 0;
  let file = std::fs::read_to_string("input/day_01").unwrap();
  file
    .lines()
    .for_each(|line| {
      let mut first_appearance = '0';
      let mut last_appearance = '0';

      for character in line.chars() {
        if character.is_ascii_digit() {
          first_appearance = character;
          break;
        }
      }

      for character in line.chars().rev() {
        if character.is_ascii_digit() {
          last_appearance = character;
          break;
        }
      }

      let concatenated = format!("{}{}", first_appearance, last_appearance);
      counter += concatenated.parse::<i32>().unwrap()
    });

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

  let mut counter = 0;

  let file = std::fs::read_to_string("input/day_01").unwrap();
  file
    .lines()
    .for_each(|line| {
      let mut first_appearance = '0';
      let mut last_appearance = '0';
      let mut word = String::new();

      for character in line.chars() {
        if let Some((_, value)) = word_to_number.iter().find(|&&(key, _)| word.contains(key)) {
          first_appearance = value.parse().unwrap();
          word.clear();
          break;
        }


        if character.is_alphabetic() {
          word.push(character)
        }

        if character.is_ascii_digit() {
          first_appearance = character;
          word.clear();
          break;
        }
      }

      for character in line.chars().rev() {
        if let Some((_, value)) = word_to_number.iter().find(|&&(key, _)| {
          let reversed_key = key.chars().rev().collect::<String>();
          return word.contains(reversed_key.as_str())
        }) {
          last_appearance = value.parse().unwrap();
          word.clear();
          break;
        }

        if character.is_alphabetic() {
          word.push(character)
        }

        if character.is_ascii_digit() {
          last_appearance = character;
          word.clear();
          break;
        }
      }

      let concatenated = format!("{}{}", first_appearance, last_appearance);
      counter += concatenated.parse::<i32>().unwrap()
    });

  println!("{}", counter)
}