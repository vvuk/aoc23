#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

mod helpers;
use helpers::*;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn go(&mut self, dir: Vec2) {
        self.x += dir.x;
        self.y += dir.y;
    }
}

type Direction = usize;

const STRAIGHT: Direction = 0;
const LEFT: Direction = 1;
const RIGHT: Direction = 2;

fn dir_name(dir: usize) -> &'static str {
    match dir {
        STRAIGHT => "STRAIGHT",
        LEFT => "LEFT",
        RIGHT => "RIGHT",
        _ => panic!()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    map: Vec<Vec<i64>>,

    fin: Vec2,
}

impl Map {
    fn weight(&self, p: Vec2) -> i64 {
        self.map[p.y as usize][p.x as usize]
    }

    fn direction_dx(p: Vec2, last_p: Vec2, dir: Direction) -> (i64, i64) {
        let dx = p.x - last_p.x;
        let dy = p.y - last_p.y;

        let (dx, dy) = match dir {
            STRAIGHT => (dx, dy),
            LEFT => (dy, -dx),
            RIGHT => (-dy, dx),
            _ => panic!()
        };

        (dx, dy)
    }

    // the cost of going from 'p' in the direction of 'dir' from 'last_p'
    fn weight_next(&self, p: Vec2, last_p: Vec2, dir: Direction, straight_count: usize, seen: &[Vec<bool>], steps: usize) -> Option<i64> {
        assert!(steps > 0);

        if dir == STRAIGHT && straight_count >= 3 {
            return None;
        }

        let (dx, dy) = Map::direction_dx(p, last_p, dir);
        //println!("{:?} {} {} seen", dir, dx, dy);
        let next = Vec2 { x: p.x + dx, y: p.y + dy };

        if !self.in_range(next.x, next.y) {
            return None;
        }

        if seen[next.y as usize][next.x as usize] {
            //println!("{:?} {} {} seen", next, dx, dy);
            return None;
        }

        let weight = self.weight(next);

        if steps == 1 {
            return Some(weight);
        }

        let straight = self.weight_next(next, p, STRAIGHT, straight_count + 1, seen, steps - 1).map(|w| w + weight);
        let left = self.weight_next(next, p, LEFT, 0, seen, steps - 1).map(|w| w + weight);
        let right = self.weight_next(next, p, RIGHT, 0, seen, steps - 1).map(|w| w + weight);

        //println!("{:?} l: {} r: {} s: {}", next, left, right, straight);

        if straight.is_none() && left.is_none() && right.is_none() {
            // likely backtracked on all
            return None;
        }

        Some(min(straight.unwrap_or(i64::MAX), min(left.unwrap_or(i64::MAX), right.unwrap_or(i64::MAX))))
    }


    fn in_range(&self, px: i64, py: i64) -> bool {
        px >= 0 && py >= 0 && px <= self.fin.x && py <= self.fin.y
    }

    fn dist_to_end(&self, p: Vec2, last_p: Vec2, dir: Direction) -> i64 {
        let (dx, dy) = Map::direction_dx(p, last_p, dir);
        let next = Vec2 { x: p.x + dx, y: p.y + dy };
        let ndx = self.fin.x - next.x;
        let ndy = self.fin.y - next.y;

        ndx * ndx + ndy * ndy
    }
}

fn day17_inner(input_fname: &str) -> (i64, Vec<usize>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let mut map: Vec<Vec<i64>> = vec![];
    for line in data.lines() {
        map.push(line.trim().chars().map(|c| (c as i64) - ('0' as i64)).collect_vec());
    }

    let target = Vec2 { x: map[0].len() as i64 - 1, y: map.len() as i64 - 1 };

    let map = Map { map, fin: target };

    // I don't think -1 x or -1 y should matter
    let mut last_pos = Vec2 { x: -1, y: 0 };
    let mut pos = Vec2 { x: 0, y: 0 };

    let mut seen = vec![vec![false; map.map[0].len()]; map.map.len()];
    let mut straight_count = 0;
    let mut result = 0;

    while pos != target {
        println!("pos: {:?}, last_pos: {:?}", pos, last_pos);
        let dir_costs = [0, 1, 2].map(|dir| {
            map.weight_next(pos, last_pos, dir, straight_count, &seen, 5)
                .map(|w| (w, map.dist_to_end(pos, last_pos, dir)))
        });

        println!("... STRAIGHT: {:?} LEFT: {:?} RIGHT: {:?}",
            dir_costs[0], dir_costs[1], dir_costs[2]);

        // find index of lowest dir_cost
        let mut min_cost = std::i64::MAX;
        let mut min_cost_dir = usize::MAX;
        for (di, cost) in dir_costs.iter().enumerate() {
            if cost.is_none() {
                continue;
            }

            let (cost, dist) = cost.unwrap();
            let kost = cost + dist;
            if kost < min_cost {
                min_cost = kost;
                min_cost_dir = di;
            }
        }

        assert_ne!(min_cost, std::i64::MAX);

        let (dx, dy) = Map::direction_dx(pos, last_pos, min_cost_dir);

        last_pos = pos;
        pos = Vec2 { x: pos.x + dx, y: pos.y + dy };
        result += map.weight(pos);

        seen[pos.y as usize][pos.x as usize] = true;

        println!("{}", dir_name(min_cost_dir));
        if min_cost_dir == STRAIGHT {
            straight_count += 1;
        } else {
            straight_count = 0;
        }
    }

    (result, vec![])
}

fn main() {
    let (r, d) = day17_inner("inputs/day17-sample.txt");
    println!("Result: {}", r);
    assert_eq!(102, r);

    //let (r, d) = day17_inner("inputs/day17.txt");
    //println!("Result: {}", r);
}