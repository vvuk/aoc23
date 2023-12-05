#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}};

fn main() {
    let data = std::fs::read_to_string("inputs/day4.txt").unwrap();

    let mut sum: i64 = 0;

    let num_cards = data.lines().count();
    let mut card_counts = vec![1; num_cards];

    for (i, line) in data.lines().enumerate() {
        let count = card_counts[i];
        for _ in 0..count {
            let rest = &line[line.find(":").unwrap()+2..];
            let both = rest.split("|").map(|p| {
                    p.trim().split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                    })
                .collect::<Vec<_>>();
            let card = &both[0];
            let winning = &both[1];

            let mut num = 0;
            for i in 0..card.len() {
                if winning.contains(&card[i]) {
                    num += 1;
                }
            }

            //println!("Card wins: {}", num);
            for k in 0..num {
                if i+k+1 >= num_cards {
                    break;
                }
                card_counts[i+k+1] += 1;
            }
        }
    }

    // sum everything in card_counts
    for i in 0..num_cards {
        sum += card_counts[i] as i64;
    }
    println!("sum: {}", sum);
}
