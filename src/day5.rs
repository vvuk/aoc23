#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap};

#[derive(Debug)]
struct Item {
    start_in: i64,
    start_out: i64,
    length: i64,
}

impl Item {
    fn new(start_in: i64, start_out: i64, length: i64) -> Item {
        Item { start_in, start_out, length }
    }

    fn map(&self, item: i64) -> Option<i64> {
        if item >= self.start_in && item < self.start_in + self.length {
            Some(self.start_out + item - self.start_in)
        } else {
            None
        }
    }
}

fn main() {
    //let data = std::fs::read_to_string("inputs/day4.txt").unwrap();
    let data = include_str!("../inputs/day5.txt");

    let mut sum: i64 = 0;

    let num_cards = data.lines().count();
    let mut card_counts = vec![1; num_cards];

    let mut seeds: Vec<(i64, i64)> = vec![];
    let mut dict: HashMap<String, Vec<Item>> = HashMap::new();

    let mut map_name = String::new();
    let mut cur_map = Vec::new();

    for line in data.lines() {
        if line.starts_with("seeds: ") {
            // collect into a vector of pairs of tuples
            let mut iter = line[7..]
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap());
            loop {
                if let Some(a) = iter.next() {
                    let b = iter.next().unwrap();
                    seeds.push((a, b));
                } else {
                    break;
                }
            }

            continue;
        }

        if line.trim().is_empty() {
            if map_name.len() > 0 {
                cur_map.sort_by(|a: &Item, b: &Item| a.start_in.cmp(&b.start_in));
                dict.insert(map_name, cur_map);
                cur_map = Vec::new();
                map_name = String::new();
            }
            continue;
        }

        if line.ends_with(" map:") {
            map_name = line[..line.len()-5].to_string();
            continue;
        }

        let values = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        cur_map.push(Item::new(values[1], values[0], values[2]));
    }

    if map_name.len() > 0 && cur_map.len() > 0 {
        cur_map.sort_by(|a: &Item, b: &Item| a.start_in.cmp(&b.start_in));
        dict.insert(map_name, cur_map);
    }

    let maps_in_order = vec![
        &dict["seed-to-soil"],
        &dict["soil-to-fertilizer"],
        &dict["fertilizer-to-water"],
        &dict["water-to-light"],
        &dict["light-to-temperature"],
        &dict["temperature-to-humidity"],
        &dict["humidity-to-location"],
    ];

    let mut result: i64 = i64::MAX;

    for seedpair in seeds {
        for seed in seedpair.0..seedpair.0+seedpair.1 {
        let mut cur: i64 = seed;
        let mut next: Option<i64> = None;

        for map in &maps_in_order {
            next = None;

            //println!("{:?}", map);
            for item in *map {
                if let Some(mapped) = item.map(cur) {
                    next = Some(mapped);
                    break;
                }
            }
            //println!("   cur: {} -> next: {:?}", cur, next);

            cur = next.unwrap_or(cur);
        }

        //println!("seed: {}, result: {}", seed, cur);
        result = min(result, cur);
    }
    }

    println!("result: {}", result);
}
