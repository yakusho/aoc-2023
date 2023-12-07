use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    value: String,
    hand_type: String,
    bid: u32,
}

fn analyze_hand(card_count_map: HashMap<char, usize>) -> &'static str {
    if card_count_map.values().any(|&count| count == 5) {
        "Five of a kind"
    } else if card_count_map.values().any(|&count| count == 4) {
        "Four of a kind"
    } else if card_count_map.values().any(|&count| count == 3)
        && card_count_map.values().any(|&count| count == 2)
    {
        "Full house"
    } else if card_count_map.values().any(|&count| count == 3) {
        "Three of a kind"
    } else if card_count_map.values().filter(|&&count| count == 2).count() == 2 {
        "Two pair"
    } else if card_count_map.values().any(|&count| count == 2) {
        "One pair"
    } else {
        "High card"
    }
}

fn count_cards(cards: &Vec<char>) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for &card in cards.iter() {
        let count = map.entry(card).or_insert(0);
        *count += 1;
    }

    map
}

pub fn part_a() {
    let card_rank = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
    ];

    let hand_rank = vec![
        "High card",
        "One pair",
        "Two pair",
        "Three of a kind",
        "Full house",
        "Four of a kind",
        "Five of a kind",
    ];

    let input = std::fs::read_to_string("src/input/day_07").unwrap();
    let mut hands: Vec<Hand> = Vec::new();

    input.lines().for_each(|line| {
        let mut split_line = line.split_whitespace();
        let hand = split_line.next().unwrap();
        let bid = split_line.next().unwrap().parse().unwrap();
        let cards: Vec<char> = hand.chars().collect();
        let map = count_cards(&cards);

        hands.push(Hand {
            value: hand.to_string(),
            hand_type: analyze_hand(map).to_string(),
            bid,
        });
    });

    hands.sort_by(|a, b| {
        hand_rank
            .iter()
            .position(|&r| r == a.hand_type)
            .cmp(&hand_rank.iter().position(|&r| r == b.hand_type))
            .then_with(|| {
                a.value
                    .chars()
                    .zip(b.value.chars())
                    .map(|(char_a, char_b)| {
                        card_rank
                            .iter()
                            .position(|&r| r == char_a.to_string())
                            .cmp(&card_rank.iter().position(|&r| r == char_b.to_string()))
                    })
                    .find(|&ord| ord != std::cmp::Ordering::Equal)
                    .unwrap_or_else(|| a.value.len().cmp(&b.value.len()))
            })
    });

    let mut sum = 0;
    hands.iter().enumerate().for_each(|(index, hand)| {
        let i: u32 = index.try_into().unwrap();
        let multiplier = i + 1;
        sum += hand.bid * multiplier;
    });

    println!("{}", sum)
}

pub fn part_b() {
    let card_rank = vec![
        "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
    ];

    let hand_rank = vec![
        "High card",
        "One pair",
        "Two pair",
        "Three of a kind",
        "Full house",
        "Four of a kind",
        "Five of a kind",
    ];

    let input = std::fs::read_to_string("src/input/day_07").unwrap();
    let mut hands: Vec<Hand> = Vec::new();

    input.lines().for_each(|line| {
        let mut split_line = line.split_whitespace();
        let hand = split_line.next().unwrap();
        let bid = split_line.next().unwrap().parse().unwrap();
        let cards: Vec<char> = hand.chars().collect();
        let mut map = count_cards(&cards);
        let max_value = map
            .iter()
            .filter(|&(key, _)| key != &'J')
            .max_by_key(|(_, value)| *value)
            .map(|(key, _)| *key)
            .unwrap_or('J');

        let j_value = map.get(&'J').clone().unwrap_or(&0);

        if max_value != 'J' {
            let updated_value = map.get(&max_value).clone().unwrap_or(&0) + j_value;
            map.insert(max_value, updated_value);
        } else {
            map.insert('Q', *j_value);
        }

        map.remove_entry(&'J');

        hands.push(Hand {
            value: hand.to_string(),
            hand_type: analyze_hand(map).to_string(),
            bid,
        });
    });

    hands.sort_by(|a, b| {
        hand_rank
            .iter()
            .position(|&r| r == a.hand_type)
            .cmp(&hand_rank.iter().position(|&r| r == b.hand_type))
            .then_with(|| {
                a.value
                    .chars()
                    .zip(b.value.chars())
                    .map(|(char_a, char_b)| {
                        card_rank
                            .iter()
                            .position(|&r| r == char_a.to_string())
                            .cmp(&card_rank.iter().position(|&r| r == char_b.to_string()))
                    })
                    .find(|&ord| ord != std::cmp::Ordering::Equal)
                    .unwrap_or_else(|| a.value.len().cmp(&b.value.len()))
            })
    });

    let mut sum = 0;
    hands.iter().enumerate().for_each(|(index, hand)| {
        let i: u32 = index.try_into().unwrap();
        let multiplier = i + 1;
        sum += hand.bid * multiplier;
    });

    println!("{}", sum)
}
