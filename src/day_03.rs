fn check_adjacent(
    coord: (usize, usize),
    number_coords: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let (x, y) = coord;
    let neighbors = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let mut valid_coords: Vec<(usize, usize)> = Vec::new();

    // println!("x is {} and y is {}", x, y);
    for (dx, dy) in neighbors.iter() {
        // Calculate the coordinates of the adjacent block
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;

        if number_coords.contains(&(new_x, new_y)) {
            valid_coords.push((new_x, new_y))
        }
    }

    return valid_coords;
}

pub fn part_a() {
    let file = std::fs::read_to_string("src/test_input/day_03").unwrap();
    let mut input_vec: Vec<Vec<char>> = Vec::new();
    let mut symbol_coordinates: Vec<(usize, usize)> = Vec::new();
    let mut number_coordinates: Vec<(usize, usize)> = Vec::new();
    let mut visited_coordinates: Vec<(usize, usize)> = Vec::new();
    let mut numbers: Vec<u32> = Vec::new();

    for (line_index, line) in file.lines().enumerate() {
        let mut line_vec: Vec<char> = Vec::new();
        for (char_index, character) in line.chars().enumerate() {
            line_vec.push(character);

            if !character.is_digit(10) && character != '.' {
                symbol_coordinates.push((char_index, line_index))
            }

            if character.is_digit(10) {
                number_coordinates.push((char_index, line_index))
            }
        }

        input_vec.push(line_vec);
    }

    for coords in symbol_coordinates.iter() {
        let valid_coords = check_adjacent(coords.clone(), number_coordinates.clone());

        for (x, y) in valid_coords.iter() {
            let mut number = String::new();
            let mut current_x = *x;

            if !visited_coordinates.contains(&(current_x, *y)) {
              number.push(input_vec[*y][current_x]);
            }

            visited_coordinates.push((current_x, *y));

            while current_x > 0
                && input_vec[*y][current_x - 1].is_digit(10)
                && !visited_coordinates.contains(&(current_x - 1, *y))
            {
                number.insert(0, input_vec[*y][current_x - 1]);
                visited_coordinates.push((current_x - 1, *y));
                current_x -= 1
            }

            current_x = *x;

            while current_x < input_vec[*y].len() - 1
                && input_vec[*y][current_x + 1].is_digit(10)
                && !visited_coordinates.contains(&(current_x + 1, *y))
            {
                number.push(input_vec[*y][current_x + 1]);
                visited_coordinates.push((current_x + 1, *y));
                current_x += 1
            }

            if number.len() > 0 {
              numbers.push(number.parse().unwrap())
            }
        }
    }

    let result: u32 = numbers.iter().sum();
    println!("{:?}", result);
}

pub fn part_b() {
  let file = std::fs::read_to_string("src/test_input/day_03").unwrap();
  let mut input_vec: Vec<Vec<char>> = Vec::new();
  let mut symbol_coordinates: Vec<(usize, usize)> = Vec::new();
  let mut number_coordinates: Vec<(usize, usize)> = Vec::new();
  let mut visited_coordinates: Vec<(usize, usize)> = Vec::new();
  let mut numbers: Vec<u32> = Vec::new();
  let mut result: u32 = 0;

  for (line_index, line) in file.lines().enumerate() {
      let mut line_vec: Vec<char> = Vec::new();
      for (char_index, character) in line.chars().enumerate() {
          line_vec.push(character);

          if character == '*' {
              symbol_coordinates.push((char_index, line_index))
          }

          if character.is_digit(10) {
              number_coordinates.push((char_index, line_index))
          }
      }

      input_vec.push(line_vec);
  }

  for coords in symbol_coordinates.iter() {
      let valid_coords = check_adjacent(coords.clone(), number_coordinates.clone());

      for (x, y) in valid_coords.iter() {
          let mut number = String::new();
          let mut current_x = *x;

          if !visited_coordinates.contains(&(current_x, *y)) {
            number.push(input_vec[*y][current_x]);
          }

          visited_coordinates.push((current_x, *y));

          while current_x > 0
              && input_vec[*y][current_x - 1].is_digit(10)
              && !visited_coordinates.contains(&(current_x - 1, *y))
          {
              number.insert(0, input_vec[*y][current_x - 1]);
              visited_coordinates.push((current_x - 1, *y));
              current_x -= 1
          }

          current_x = *x;

          while current_x < input_vec[*y].len() - 1
              && input_vec[*y][current_x + 1].is_digit(10)
              && !visited_coordinates.contains(&(current_x + 1, *y))
          {
              number.push(input_vec[*y][current_x + 1]);
              visited_coordinates.push((current_x + 1, *y));
              current_x += 1
          }

          if number.len() > 0 {
            numbers.push(number.parse().unwrap())
          }
      }

      if numbers.len() > 1 {
        let numbers_product: u32 = numbers.iter().product();
        result += numbers_product;
      }

      numbers.clear();
  }

  println!("{:?}", result);
}
