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

    let mut items: HashMap<String, Item> = HashMap::new();
    let mut lines = data.lines();
    let dr = lines.next().unwrap().chars().map(|c| Direction::from_char(c)).collect_vec();
    _ = lines.next();

    for line in lines {
        // syntax is "AAA = (BBB, CCC)", parse into Item
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let caps = re.captures(line).unwrap();
        let item = Item {
            code: caps[1].to_string(),
            left: caps[2].to_string(),
            right: caps[3].to_string(),
        };

        items.insert(item.code.clone(), item);
    }

    let mut cur = "AAA";
    let mut result: i64 = 0;
    let mut dir_idx = 0;
    while cur != "ZZZ" {
        let item = &items[&cur.to_string()];
        let dir = dr[dir_idx];

        cur = item.go(dir);
        result += 1;
        dir_idx = (dir_idx + 1) % dr.len();
    }

    println!("result: {}", result);
}

