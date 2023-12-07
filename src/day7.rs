#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap};
use itertools::Itertools;

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

fn parse_card(card: char) -> Card {
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

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_hand(hand: &Hand) -> HandType {
        let mut counts: HashMap<Card, i64> = HashMap::new();
        for card in hand {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }

        let mut num_jokers = *counts.get(&Card::J).unwrap_or(&0);

        let mut counts: Vec<(Card, i64)> = counts.into_iter().collect();
        counts.sort_by(|a: &(Card, i64), b: &(Card, i64)| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            return b.1.cmp(&a.1);
        });

        //println!("hand {:?} counts: {:?} jokers: {}", hand, counts, num_jokers);
        let mut hand_type = HandType::HighCard;
        for (card, count) in counts {
            if card == Card::J {
                let mut joker_type: HandType;
                if num_jokers == 5 || num_jokers == 4{
                    joker_type = HandType::FiveOfAKind;
                } else if num_jokers == 3 {
                    joker_type = HandType::FourOfAKind;
                } else if num_jokers == 2 {
                    joker_type = HandType::ThreeOfAKind;
                } else {
                    break;
                }

                //println!("joker_type: {:?} hand_type {:?}", joker_type, hand_type);

                if joker_type < hand_type {
                    hand_type = joker_type;
                }
                break;
            }

            if count == 5 {
                hand_type = HandType::FiveOfAKind;
            } else if count == 4 {
                if num_jokers == 1 {
                    hand_type = min(hand_type, HandType::FiveOfAKind);
                    num_jokers -= 1;
                } else {
                    hand_type = min(hand_type, HandType::FourOfAKind);
                }
            } else if count == 3 {
                if num_jokers == 2 {
                    hand_type = min(hand_type, HandType::FiveOfAKind);
                    num_jokers -= 2;
                } else if num_jokers == 1 {
                    hand_type = min(hand_type, HandType::FourOfAKind);
                    //println!("hand {:?} jokers: {} 4 of a kind", hand, num_jokers);
                    num_jokers -= 1;
                } else {
                    hand_type = min(hand_type, HandType::ThreeOfAKind);
                }
            } else if count == 2 {
                if hand_type == HandType::ThreeOfAKind {
                    hand_type = min(hand_type, HandType::FullHouse);
                } else if num_jokers == 3 {
                    hand_type = min(hand_type, HandType::FiveOfAKind);
                    num_jokers -= 3;
                } else if num_jokers == 2 {
                    hand_type = min(hand_type, HandType::FourOfAKind);
                    num_jokers -= 2;
                } else if num_jokers == 1 {
                    if hand_type == HandType::OnePair {
                        hand_type = min(hand_type, HandType::FullHouse);
                    } else {
                        hand_type = min(hand_type, HandType::ThreeOfAKind);
                    }
                    num_jokers -= 1;
                } else if hand_type == HandType::OnePair {
                    hand_type = min(hand_type, HandType::TwoPair);
                } else {
                    hand_type = min(hand_type, HandType::OnePair);
                }
            }
        }

        return hand_type;
    }
}

type Hand = Vec<Card>;

fn main() {
    let data = include_str!("../inputs/day7.txt");

    let mut hands: Vec<(Hand, i64)> = Vec::new();

    for line in data.lines() {
        let split = line.trim().split_whitespace().collect_vec();
        let mut hand: Vec<Card> = Vec::new();

        split[0].chars().for_each(|c| hand.push(parse_card(c)));

        let bid = split[1].parse::<i64>().unwrap();

        hands.push((hand, bid));
    }

    hands.sort_by(|a: &(Hand, i64), b: &(Hand, i64)| {
        let a_type = HandType::from_hand(&a.0);
        let b_type = HandType::from_hand(&b.0);

        //println!("a: {:?} -> {:?}", a.0, a_type);
        //println!("b: {:?} -> {:?}", b.0, b_type);

        if a_type == b_type {
            let mut ah = a.0.clone();
            let mut bh = b.0.clone();

            //println!("a: {:?} -> {:?}", a.0, ah);
            //println!("b: {:?} -> {:?}", b.0, bh);

            for i in 0..ah.len() {
                if ah[i] != bh[i] {
                    return ah[i].cmp(&bh[i]);
                }
            }

            return 0.cmp(&0);
        } else {
            return a_type.cmp(&b_type);
        }
    });

    let mut rank = hands.len() as i64;
    let mut result: i64 = 0;

    for hand in hands {
        println!("hand: {:?} -> {:?}, bid: {}", hand.0, HandType::from_hand(&hand.0), hand.1);
        result += hand.1 * rank;
        rank -= 1;
    }

    println!("result: {}", result);
}

