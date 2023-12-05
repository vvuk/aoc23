#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap};

#[derive(Debug)]
struct Item {
    start_in: i64,
    start_out: i64,
    length: i64,
}

#[derive(Debug)]
enum Mapped {
    None,
    Full((i64, i64)),
    Partial((i64, i64), (i64, i64)),
}

impl Item {
    fn new(start_in: i64, start_out: i64, length: i64) -> Item {
        Item { start_in, start_out, length }
    }
    fn map_d(&self, item: (i64, i64)) -> Mapped {
        let r = self.map(item);
        //println!("map {:?} @ {:?} = {:?}", item, self, r);
        r
    }

    fn map(&self, item: (i64, i64)) -> Mapped {
        let sin = self.start_in;
        let sout = self.start_out;
        let slen = self.length;

        let rin = item.0;
        let rlen = item.1;

        // if rin is inside this mapping
        if rin >= sin && rin < sin + slen {
            // does the full mapping apply? if so return a Full mapping.
            // otherwise, map up to the length, and return the remainder
            if rin + rlen <= sin + slen {
                Mapped::Full((sout + (rin - sin), rlen))
            } else {
                let num_mapped = (sin+slen) - rin;
                Mapped::Partial((sout + (rin - sin), num_mapped), (rin + num_mapped, rlen - num_mapped))
            }
        } else {
            Mapped::None
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

    // sigh. we have to do it by intervals
        let mut cur_all: Vec<(i64, i64)> = seeds.clone();

        for (i, map) in maps_in_order.iter().enumerate() {
            //println!("============= Map {} =============", i);
            let mut next_all: Vec<(i64, i64)> = vec![];
            for cur in cur_all {
                let mut cur_work = Some(cur);
                for map_item in *map {
                    if cur_work.is_none() {
                        break;
                    }

                    match map_item.map(cur_work.unwrap()) {
                        Mapped::None => (),
                        Mapped::Full((start, len)) => {
                            next_all.push((start, len));
                            cur_work = None;
                        },
                        Mapped::Partial((start, len), (start2, len2)) => {
                            next_all.push((start, len));
                            cur_work = Some((start2, len2));
                        },
                    }
                }
                if let Some(left) = cur_work {
                    next_all.push(left);
                }
            }

            next_all.sort_by(|a: &(i64, i64), b: &(i64, i64)| a.0.cmp(&b.0));
            cur_all = next_all;
        }

        result = cur_all[0].0;

        //println!("seed: {}, result: {}", seed, cur);
        //result = min(result, cur);

    println!("result: {}", result);
}
