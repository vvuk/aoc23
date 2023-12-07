#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap};

fn dist_for_time(button_press_time: i64, total_time: i64) -> i64
{
    let travel_time = total_time - button_press_time;
    return travel_time * button_press_time; // 1 mm/ms
}

fn main() {
    let data = include_str!("../inputs/day6.txt");

    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();

    for line in data.lines() {
        if line.starts_with("Time: ") {
            line.trim().split_whitespace().skip(1).for_each(|s| times.push(s.parse::<i64>().unwrap()));
        } else if line.starts_with("Distance: ") {
            line.trim().split_whitespace().skip(1).for_each(|s| distances.push(s.parse::<i64>().unwrap()));
        }
    }

    println!("times: {:?}", times);
    println!("distances: {:?}", distances);

    let mut result: i64 = 1;
    for i in 0..times.len() {
        let time = times[i];
        let dist = distances[i];

        let mut num_ways_to_win = 0;
        for k in 1..time {
            let d = dist_for_time(k, time);
            if d > dist {
                num_ways_to_win += 1;
            }
        }

        if num_ways_to_win > 0 {
            result *= num_ways_to_win;
        }
    }

    println!("result: {}", result);
}
