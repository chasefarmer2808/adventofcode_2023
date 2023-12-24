/*
Need to find sum of all products of hand bid and rank.  Given hand and bid amount.  Need to find rank
and strength.

Possible strengths:
Five of a kind - 7
Four of a kind - 6
Full house - 5
Three of a kind - 4
Two pair - 3
One pair - 2
High card - 1

First, collect all hands and bids.  During hand initialization, calculate strength.  Then place in
list of hands.

Calculate rank while inserting into list.
Ex: 5 hands:
    A: S2, B: S3, C: S3, D: S4, E: S1;  All start with rank 1.
    Insert A
        First in list so rank doesn't change.
    Insert B
        Compare with A
            B wins
            B.rank = A.rank + 1
    Insert C
        Compare with A
            C wins
            C.rank = A.rank + 1
        Compare with B
            C wins
            C.rank = B.rank + 1
    Insert D
        Compare with A
            D wins
            D.rank = A.rank + 1
        Compare with B
            D wins
            D.rank = B.rank + 1
        Compare with C
            D wins
            D.rank = C.rank + 1
    Insert E
        Compare with A
            E looses

Don't really need to track rank.  It's just the index of the hand in the sorted list + 1.
 */

use std::collections::HashMap;
use std::fs::rename;
use crate::utils::read_lines;

pub fn part_one(input_file: &str) -> u64 {
    let enable_jokers = false;
    let lines = read_lines(input_file);
    let mut sorted_hands: Vec<Hand> = Vec::new();
    let mut ans = 0;

    for line in lines.iter() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let cards: String = String::from(tokens[0]);
        let bid: u64 = tokens[1].parse().unwrap();
        let cur_hand = Hand::new(cards, bid, enable_jokers);
        insert_sorted(&mut sorted_hands, cur_hand);
    }

    for (i, hands) in sorted_hands.iter().enumerate() {
        ans += hands.bid * (i as u64 + 1);
    }

    return ans;
}

pub fn part_two(input_file: &str) -> u64 {
    let enable_jokers = true;
    let lines = read_lines(input_file);
    let mut sorted_hands: Vec<Hand> = Vec::new();
    let mut ans = 0;

    for line in lines.iter() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let cards: String = String::from(tokens[0]);
        let bid: u64 = tokens[1].parse().unwrap();
        let cur_hand = Hand::new(cards, bid, enable_jokers);
        insert_sorted(&mut sorted_hands, cur_hand);
    }

    for (i, hands) in sorted_hands.iter().enumerate() {
        ans += hands.bid * (i as u64 + 1);
    }

    return ans;
}

struct Hand {
    cards: String,
    bid: u64,
    strength: u8,
    enable_jokers: bool
}

impl Hand {
    pub fn new(cards: String, bid: u64, enable_jokers: bool) -> Self {
        if enable_jokers {
            Self { cards: cards.clone(), bid, strength: calculate_strength_with_jokers(&cards), enable_jokers }
        } else {
            Self { cards: cards.clone(), bid, strength: calculate_strength(&cards), enable_jokers }
        }
    }

    fn is_stronger(&self, hand: &Hand) -> bool {
        if self.strength > hand.strength {
            return true;
        }

        if self.strength == hand.strength {
            for (card, other_card) in self.cards.chars().zip(hand.cards.chars()) {
                if get_card_value(card, self.enable_jokers) > get_card_value(other_card, self.enable_jokers) {
                    return true;
                }

                if get_card_value(card, self.enable_jokers) < get_card_value(other_card, self.enable_jokers) {
                    return false;
                }

                continue;
            }
        }

        return false;
    }
}

fn calculate_strength(cards: &str) -> u8 {
    let mut hand_map: HashMap<char, u8> = HashMap::new();

    for card in cards.chars() {
        if hand_map.contains_key(&card) {
            *hand_map.get_mut(&card).unwrap() += 1;
        } else {
            hand_map.insert(card, 1);
        }
    }

    // Check for 5 of a kind
    if hand_map.keys().len() == 1 {
        return 7;
    }

    // Check for 4 of a kind or full house
    if hand_map.keys().len() == 2 {
        return if hand_map.values().any(|&n| n == 1) {
            6
        } else {
            5
        }
    }


    // Check for 3 of a kind or 2-pair
    if hand_map.keys().len() == 3 {
        return if hand_map.values().any(|&n| n == 3) {
            4
        } else {
            3
        }
    }

    // Check for 1-pair
    if hand_map.keys().len() == 4 {
        return 2;
    }

    // Default to high card strength
    return 1;
}

fn calculate_strength_with_jokers(cards: &str) -> u8 {
    let mut hand_map: HashMap<char, u8> = HashMap::new();

    for card in cards.chars() {
        if hand_map.contains_key(&card) {
            *hand_map.get_mut(&card).unwrap() += 1;
        } else {
            hand_map.insert(card, 1);
        }
    }

    // Check for 5 of a kind
    if hand_map.keys().len() == 1 {
        return 7;
    }

    // Check for 4 of a kind or full house
    // 4 w/ jokers: AAAAJ or JJJJA
    // FH w/ jokers: AAAJJ or JJJAA
    if hand_map.keys().len() == 2 {
        return if hand_map.contains_key(&'J') {
            7 // Make hand 5 of a kind
        }
        else if hand_map.values().any(|&n| n == 1) {
            6 // 4 of a kind
        } else {
            5 // full house
        }
    }


    // Check for 3 of a kind or 2-pair
    // 3 w/ joker: AAABJ or JJJAB
    // 2-pair w/ joker: AABBJ or AAJJC
    if hand_map.keys().len() == 3 {
        return if hand_map.values().any(|&n| n == 3) {
            if hand_map.contains_key(&'J') {
                6 // Make 4 of a kind
            } else {
                4 // 3 of a kind
            }
        } else { // possible 2-pair
            if hand_map.contains_key(&'J') {
                if *hand_map.get(&'J').unwrap() == 2 {
                    6 // Make 4 of a kind
                } else {
                    5 // make full house
                }
            } else {
                3 // 2-pair
            }
        }
    }

    // Check for 1-pair
    if hand_map.keys().len() == 4 {
        return if hand_map.contains_key(&'J') {
            4 // Make hand 3 of a kind
        } else {
            2
        }
    }

    // Default to high card strength
    return 1;
}

fn insert_sorted(list: &mut Vec<Hand>, new_item: Hand) {
    if list.is_empty() {
        list.push(new_item);
        return;
    }
    for (i, item) in list.iter().enumerate() {
        if new_item.is_stronger(item) {
            if i == list.len() - 1 {
                list.push(new_item);
                break;
            }
            continue;
        } else {
            list.insert(i, new_item);
            break;
        }
    }
}

//A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
fn get_card_value(card: char, has_joker: bool) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if has_joker {
                1
            } else {
                11
            }
        },
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0
    }
}