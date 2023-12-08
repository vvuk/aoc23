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

    let mut result: i64 = 0;
    let mut dir_idx = 0;
    let plen = path.len();
    loop {
        //if result % 100000 == 0 {
            //println!("{}: {:?}", result, path);
       // }
        let dir = dr[dir_idx];

        let mut is_end = true;
        for i in 0..plen {
            let item = items[path[i]].as_ref().unwrap();
            is_end = is_end && item.is_end;
            path[i] = item.go(dir);
        }

        if is_end {
            break;
        }

        result += 1;
        dir_idx = (dir_idx + 1) % dr.len();
    }

    println!("result: {}", result);
}

