#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap};
use itertools::Itertools;

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Copy, Clone)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn num_cards() -> usize {
        13
    }

    fn to_index(card: Card) -> usize {
        match card {
            Card::J => 0,
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::T => 9,
            Card::Q => 10,
            Card::K => 11,
            Card::A => 12,
        }
    }

    fn from_index(idx: usize) -> Card {
        match idx {
            0 => Card::J,
            1 => Card::Two,
            2 => Card::Three,
            3 => Card::Four,
            4 => Card::Five,
            5 => Card::Six,
            6 => Card::Seven,
            7 => Card::Eight,
            8 => Card::Nine,
            9 => Card::T,
            10 => Card::Q,
            11 => Card::K,
            12 => Card::A,
            _ => panic!("unknown index: {}", idx),
        }
    }

    fn from_char(card: char) -> Card {
        match card {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("unknown card: {}", card),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_hand(hand: &Hand) -> HandType {
        let mut counts: HashMap<Card, i64> = HashMap::new();
        for card in hand {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }

        let num_jokers = *counts.get(&Card::J).unwrap_or(&0);
        let mut counts: Vec<(Card, i64)> = counts.into_iter().collect();
        counts.sort_by(|a: &(Card, i64), b: &(Card, i64)| {
            if a.1 == b.1 {
                return b.0.cmp(&a.0);
            }
            return b.1.cmp(&a.1);
        });

        if num_jokers > 0 {
            let mut joker_results: Vec<HandType> = vec![];
            let first_joker_at = hand.iter().position(|&c| c == Card::J).unwrap();
            for i in 1..Card::num_cards() {
                let mut hand_copy = hand.clone();
                hand_copy[first_joker_at] = Card::from_index(i);
                joker_results.push(HandType::from_hand(&hand_copy));
            }

            return *joker_results.iter().max().unwrap();
        }

        let mut hand_type = HandType::HighCard;
        for (card, count) in counts {
            if count == 5 {
                hand_type = HandType::FiveOfAKind;
                break;
            } else if count == 4 {
                hand_type = max(hand_type, HandType::FourOfAKind);
            } else if count == 3 {
                hand_type = max(hand_type, HandType::ThreeOfAKind);
            } else if count == 2 {
                if hand_type == HandType::ThreeOfAKind {
                    hand_type = max(hand_type, HandType::FullHouse);
                } else if hand_type == HandType::OnePair {
                    hand_type = max(hand_type, HandType::TwoPair);
                } else {
                    hand_type = max(hand_type, HandType::OnePair);
                }
            }
        }

        return hand_type;
    }
}

type Hand = Vec<Card>;

fn main() {
    let data = include_str!("../inputs/day7.txt");

    let mut hands: Vec<(Hand, i64, HandType)> = Vec::new();

    for line in data.lines() {
        let split = line.trim().split_whitespace().collect_vec();
        let mut hand: Vec<Card> = Vec::new();

        split[0].chars().for_each(|c| hand.push(Card::from_char(c)));

        let bid = split[1].parse::<i64>().unwrap();

        let hand_type = HandType::from_hand(&hand);
        hands.push((hand, bid, hand_type));
    }

    hands.sort_by(|a: &(Hand, i64, HandType), b: &(Hand, i64, HandType)| {
        let a_type = a.2;
        let b_type = b.2;

        let ah = &a.0;
        let bh = &b.0;
        //println!("a: {:?} -> {:?}", a.0, a_type);
        //println!("b: {:?} -> {:?}", b.0, b_type);

        if a_type == b_type {
            for i in 0..ah.len() {
                if ah[i] != bh[i] {
                    return bh[i].cmp(&ah[i]);
                }
            }

            panic!("No ordering found for hands: {:?} {:?}", a, b);
        }

        return b_type.cmp(&a_type);
    });

    let mut rank = hands.len() as i64;
    let mut result: i64 = 0;

    for hand in hands {
        println!("hand: {:?} -> {:?}, bid: {}", hand.0, hand.2, hand.1);
        result += hand.1 * rank;
        rank -= 1;
    }

    println!("result: {}", result);
}

