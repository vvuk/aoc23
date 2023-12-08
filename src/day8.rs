#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    left: usize,
    right: usize,
    is_end: bool,
}

impl Item {
    fn go(&self, d: Direction) -> usize {
        match d {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

fn get_factors_functional(n: u64) -> Vec<u64> {
    (1..n + 1).into_iter().filter(|&x| n % x == 0).collect::<Vec<u64>>()
}

fn main() {
    let data = include_str!("../inputs/day8.txt");
    let mut lines = data.lines();
    let dr = lines.next().unwrap().chars().map(|c| Direction::from_char(c)).collect_vec();
    _ = lines.next();

    let mut name_to_item: HashMap<String, usize> = HashMap::new();
    let mut items: Vec<Option<Item>> = Vec::new();

    let mut path: Vec<usize> = Vec::new();

    for line in lines {
        // syntax is "AAA = (BBB, CCC)", parse into Item
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let caps = re.captures(line).unwrap();
        let code_s = &caps[1];
        let left_s = &caps[2];
        let right_s = &caps[3];

        let code = *name_to_item.entry(code_s.to_string()).or_insert_with(|| {
            let value = items.len();
            items.push(None);
            value
        });

        let left = *name_to_item.entry(left_s.to_string()).or_insert_with(|| {
            let value = items.len();
            items.push(None);
            value
        });

        let right = *name_to_item.entry(right_s.to_string()).or_insert_with(|| {
            let value = items.len();
            items.push(None);
            value
        });

        let is_end = code_s.ends_with('Z');
        if code_s.ends_with('A') {
            path.push(code);
        }

        let item = Item { left, right, is_end };
        items[code] = Some(item);
    }

    let mut path_factors: HashSet<u64> = HashSet::new();
    let mut result: u64 = 1;
    for start in path.iter() {
        let mut p = *start;
        let mut dir_idx = 0;
        let mut count = 0;
        loop {
            let dir = dr[dir_idx];
            let item = items[p].as_ref().unwrap();
            if item.is_end {
                break;
            }
            p = item.go(dir);
            count += 1;
            dir_idx = (dir_idx + 1) % dr.len();
        }
        println!("count: {} -> {:?}", count, get_factors_functional(count as u64));
        for k in get_factors_functional(count as u64) {
            if k != count {
                path_factors.insert(k);
            }
        }
    }

    result = path_factors.iter().product();
    println!("couts: {:?}", path_factors);
    println!("result: {}", result);
}

