#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap, str::FromStr};
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
    code: String,
    left: String,
    right: String,
}

impl Item {
    fn go(&self, d: Direction) -> &str {
        match d {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn main() {
    let data = include_str!("../inputs/day8.txt");

    let mut items_array: Vec<Item> = Vec::new();
    let mut items: HashMap<&str, &Item> = HashMap::new();
    let mut lines = data.lines();
    let dr = lines.next().unwrap().chars().map(|c| Direction::from_char(c)).collect_vec();
    _ = lines.next();
    let mut path: Vec<&str> = Vec::new();

    for line in lines {
        // syntax is "AAA = (BBB, CCC)", parse into Item
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let caps = re.captures(line).unwrap();
        let item = Item {
            code: caps[1].to_string(),
            left: caps[2].to_string(),
            right: caps[3].to_string(),
        };

        items_array.push(item);
    }

    for item in &items_array {
        if item.code.chars().last().unwrap() == 'A' {
            path.push(&item.code);
        }

        items.insert(&item.code, item);
    }

    let mut result: i64 = 0;
    let mut dir_idx = 0;
    let plen = path.len();
    'OUTER: loop {
        if result % 100000 == 0 {
            println!("{}: {:?}", result, path);
        }
        let dir = dr[dir_idx];

        for i in 0..plen {
            let item = items[&path[i]];
            path[i] = item.go(dir);
        }

        result += 1;
        dir_idx = (dir_idx + 1) % dr.len();

        for p in &path {
            if !p.ends_with('Z') {
                continue 'OUTER;
            }
        }
        break
    }

    println!("result: {}", result);
}

