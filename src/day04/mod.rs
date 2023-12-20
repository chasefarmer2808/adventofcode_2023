use std::borrow::Cow::Owned;
use std::cmp::min;
use std::collections::HashSet;
use crate::utils::read_lines;

pub fn part_one(input_file: &str) -> u32 {
    let mut sum = 0;
    let mut cards: Vec<Card> = Vec::new();
    let card_lines = read_lines(input_file);

    for line in card_lines {
        let nums: &str = line.split(':').collect::<Vec<&str>>()[1];
        let winning_nums: &str = nums.split('|').collect::<Vec<&str>>()[0];
        let player_nums: &str = nums.split('|').collect::<Vec<&str>>()[1];

        cards.push(Card::new(winning_nums.trim(), player_nums.trim()));
    }

    for (i, card) in cards.iter().enumerate() {
        let matches = card.get_num_matches();

        if matches == 0 {
            continue;
        }

        sum += matches;
    }

    return sum;
}

pub fn part_two(input_file: &str) -> u32 {
    let mut sum = 0;
    let mut cards: Vec<Card> = Vec::new();
    let card_lines = read_lines(input_file);

    for line in card_lines {
        let nums: &str = line.split(':').collect::<Vec<&str>>()[1];
        let winning_nums: &str = nums.split('|').collect::<Vec<&str>>()[0];
        let player_nums: &str = nums.split('|').collect::<Vec<&str>>()[1];

        cards.push(Card::new(winning_nums.trim(), player_nums.trim()));
    }

    for i in 0..cards.len() {
        let card = &cards[i];
        let mut count = card.count.clone();
        let matches = card.get_num_matches();

        if matches == 0 {
            continue;
        }

        let max_range: usize = i + usize::try_from(matches).unwrap() + 1;

        while count > 0 {
            for j in (i+1)..max_range {
                cards[j].increment_count(1);
            }
            count -= 1;
        }
    }

    println!("{:?}", cards);

    for card in cards {
        sum += card.count
    }

    return sum;
}

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<u32>,
    player_nums: HashSet<u32>,
    count: u32
}

impl Card {
    fn new(winning_nums: &str, player_nums: &str) -> Self {
        let w_nums: Vec<u32> = winning_nums.split(' ')
            .filter(|s| !s.is_empty())
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect();
        let p_nums: Vec<u32> = player_nums.split(' ')
            .filter(|s| !s.is_empty())
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect();

        Card { winning_nums: HashSet::from_iter(w_nums), player_nums: HashSet::from_iter(p_nums), count: 1 }
    }

    fn get_num_matches(&self) -> u32 {
        self.player_nums.intersection(&self.winning_nums).count() as u32
    }

    fn get_point_value(&self) -> u32 {
        let num_matches = self.get_num_matches();

        if (num_matches == 0) {
            return 0;
        }

        return 2u32.pow((num_matches - 1) as u32);
    }

    fn increment_count(&mut self, amount: u32) {
        self.count += amount;
    }
}