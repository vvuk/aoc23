#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use core::num;
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd, borrow::BorrowMut, u8};
use std::collections::VecDeque;
use itertools::{Itertools, MinMaxResult};
use regex::Regex;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use typed_arena::Arena;
use bumpalo::Bump;

use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};

mod helpers;
use helpers::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}

struct Map {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    width: usize,
    height: usize,
}

impl Map {
    fn from_string(s: &str) -> Self {
        let mut map = Vec::new();
        for line in s.split('\n') {
            map.push(line.chars().collect_vec());
        }

        let mut start = (0,0);
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 'S' {
                    start = (x,y);
                    break;
                }
            }
        }

        let width = map[0].len();
        let height = map.len();

        Self {
            map,
            start,
            width,
            height,
        }
    }

    fn is_plot(&self, coord: (usize, usize)) -> bool {
        let (x,y) = coord;
        if x >= self.width || y >= self.height { return false; }
        return self.map[y][x] != '#'
    }

    fn at(&self, coord: (usize, usize)) -> Option<char> {
        let (x,y) = coord;
        if y >= self.map.len() {
            return None;
        }
        let row = &self.map[y];
        if x >= row.len() {
            return None;
        }
        Some(row[x])
    }

    fn go(&self, coord: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let (x,y) = coord;
        match dir {
            Direction::North => { if y == 0 { None } else { Some((x,y-1)) } },
            Direction::East => { if x == self.map[y].len()-1 { None } else { Some((x+1,y)) } },
            Direction::South => { if y == self.map.len()-1 { None } else { Some((x,y+1)) } },
            Direction::West => { if x == 0 { None } else { Some((x-1,y)) } },
        }
    }
}

fn day21_inner(input_fname: &str, target: usize) -> i64 {
    let data = std::fs::read_to_string(input_fname).unwrap();
    let map = Map::from_string(&data);
    let mut queue = HashSet::new();
    let mut seen = HashSet::new();
    let mut results = HashSet::new();

    let mut mmax = 0;

    queue.insert((map.start, 0));
    while !queue.is_empty() {
        let (coord, dist) = queue.iter().next().unwrap().clone();
        queue.remove(&(coord, dist));
        seen.insert((coord, dist));

        if max(dist, mmax) != mmax {
            println!("{}", dist);
            mmax = dist;
        }

        if dist == target {
            results.insert(coord);
            continue;
        }

        [Direction::North, Direction::East, Direction::South, Direction::West].iter().for_each(|dir| {
            if let Some(new_coord) = map.go(coord, *dir) {
                if map.is_plot(new_coord) {
                    let n = (new_coord, dist+1);
                    if seen.contains(&n) {
                        return;
                    }
                    queue.insert(n);
                }
            }
        });
    }

    results.len() as i64
}

fn main() {
    let r = day21_inner("inputs/day21-sample.txt", 6);
    println!("Result: {}", r);

    println!("===== Real =====");
    let r = day21_inner("inputs/day21.txt", 64);
    println!("Result: {}", r);
}