#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap};

fn dist_for_time(button_press_time: i64, total_time: i64) -> i64
{
    let travel_time = total_time - button_press_time;
    return travel_time * button_press_time; // 1 mm/ms
}

fn main() {
    let data = include_str!("../inputs/day6.txt");

    let mut time: i64 = 0;
    let mut distance: i64 = 0;

    for line in data.lines() {
        if line.starts_with("Time: ") {
            time = line[6..].replace(" ", "").parse::<i64>().unwrap();
        } else if line.starts_with("Distance: ") {
            distance = line[10..].replace(" ", "").parse::<i64>().unwrap();
        }
    }

    println!("time: {:?}", time);
    println!("distance: {:?}", distance);

    let mut result: i64 = 1;
    let mut num_ways_to_win = 0;
    for k in 1..time {
        let d = dist_for_time(k, time);
        if d > distance {
            num_ways_to_win += 1;
        }
    }

    if num_ways_to_win > 0 {
        result *= num_ways_to_win;
    }

    println!("result: {}", result);
}
