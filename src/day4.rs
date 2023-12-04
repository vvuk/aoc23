#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}};

fn main() {
    let data = std::fs::read_to_string("inputs/day4.txt").unwrap();

    let mut sum: i64 = 0;

    for line in data.lines() {
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

        println!("Card wins: {}", num);
        if num > 0 {
            let base: i64 = 2;
            sum += base.pow(num-1);
        }
    }

    println!("sum: {}", sum);
}
