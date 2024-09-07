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
    map: Vec<Vec<u8>>,
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

        // replace the map with the number of reachable plots from each plot
        let mut nmap = vec![vec![0; width]; height];
        for j in 0..height {
            for i in 0..width {
                if map[j][i] == '#' {
                    nmap[j][i] = 0;
                    continue;
                }

                let mut cnt = 0;
                [Direction::North, Direction::East, Direction::South, Direction::West].iter().for_each(|dir| {
                    let nc = Map::go_static((i, j), *dir, width, height);
                    if map[nc.1][nc.0] != '#' {
                        cnt += 1;
                    }
                });
                nmap[j][i] = cnt;
            }
        }

        Self {
            map: nmap,
            start,
            width,
            height,
        }
    }

    fn go(&self, coord: (usize, usize), dir: Direction) -> (usize, usize) {
        Map::go_static(coord, dir, self.width, self.height)
    }

    fn go_static(coord: (usize, usize), dir: Direction, width: usize, height: usize) -> (usize, usize) {
        let (x,y) = coord;

        // map wraps around
        match dir {
            Direction::North => { if y == 0 { return (x, height-1); } else { return (x, y-1); } },
            Direction::East => { if x == width-1 { return (0, y); } else { return (x+1, y); } },
            Direction::South => { if y == height-1 { return (x, 0); } else { return (x, y+1); } },
            Direction::West => { if x == 0 { return (width-1, y); } else { return (x-1, y); } },
        }
    }
}

fn day21_inner(input_fname: &str, target: usize) -> u128 {
    let data = std::fs::read_to_string(input_fname).unwrap();
    let map = Map::from_string(&data);
    let mut queue = HashSet::new();
    let mut seen = HashSet::new();
    let mut result: u128 = 0;

    let mut mmax = 0;

    queue.insert((map.start, 0, 1 as u128));
    while !queue.is_empty() {
        let (coord, dist, cnt) = queue.iter().next().unwrap().clone();
        let val_at = map.map[coord.1][coord.0];

        queue.remove(&(coord, dist, cnt));
        seen.insert((coord, dist));

        if max(dist, mmax) != mmax {
            println!("{}", dist);
            mmax = dist;
        }

        if dist == target {
            result += cnt;
            continue;
        }

        [Direction::North, Direction::East, Direction::South, Direction::West].iter().for_each(|dir| {
            let new_coord = map.go(coord, *dir);
            let new_coord_v = map.map[new_coord.1][new_coord.0];
            if new_coord_v == 0 { return; }
            println!("{} {}", cnt, new_coord_v);
            let n = (new_coord, dist+1, cnt * new_coord_v as u128);
            if seen.contains(&(new_coord, dist+1)) { return; }
            queue.insert(n);
        });
    }

    result
}

fn main() {
    let r = day21_inner("inputs/day21-sample.txt", 1000);
    println!("Result: {}", r);

    println!("===== Real =====");
    //let r = day21_inner("inputs/day21.txt", 26501365);
    //println!("Result: {}", r);
}