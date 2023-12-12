pub fn part_a() {
    let input = std::fs::read_to_string("src/input/day_09").unwrap();
    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut sum: i64 = 0;

    input.lines().for_each(|line| {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().ok().unwrap())
            .collect();

        nums.push(numbers);

        let mut found = false;
        let mut i = 0;

        while !found {
            let mut temp = Vec::new();

            for j in 1..nums[i].len() {
                temp.push(nums[i][j] - nums[i][j - 1])
            }

            if temp.iter().all(|x| *x == 0) {
                found = true
            }

            nums.push(temp);
            i += 1;
        }

        let mut last = 0;
        let mut current = 0;
        for n in nums.iter().rev().skip(1) {
            current = n[n.len() - 1] + last;
            last = current;
        }

        sum += current;
        nums.clear();
    });

    println!("{}", sum);
}

pub fn part_b() {
    let input = std::fs::read_to_string("src/input/day_09").unwrap();
    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut sum: i64 = 0;

    input.lines().for_each(|line| {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().ok().unwrap())
            .collect();

        nums.push(numbers);

        let mut found = false;
        let mut i = 0;

        while !found {
            let mut temp = Vec::new();

            for j in 1..nums[i].len() {
                temp.push(nums[i][j] - nums[i][j - 1])
            }

            if temp.iter().all(|x| *x == 0) {
                found = true
            }

            nums.push(temp);
            i += 1;
        }

        let mut first = 0;
        let mut current = 0;
        for n in nums.iter().rev().skip(1) {
            current = n[0] - first;
            first = current;
        }

        sum += current;
        nums.clear();
    });

    println!("{}", sum);
}
