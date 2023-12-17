#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

mod helpers;
use helpers::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn go(&self, dir: Vec2) -> Vec2 {
        Vec2 { x: self.x + dir.x, y: self.y + dir.y }
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
    fn reconstruct_path(cameFrom: &HashMap<Vec2, Vec2>, current: Vec2) -> Vec<Vec2> {
        let mut total_path = vec![current];
        let mut current = current;
        let origin = Vec2 { x: 0, y: 0 };
        while cameFrom.contains_key(&current) {
            current = cameFrom[&current];
            total_path.push(current);
            if current == origin {
                break;
            }
        }
        total_path.reverse();
        total_path
    }

    fn a_star(&self, start: Vec2, goal: Vec2) -> Vec<Vec2> {
        let mut openSet = HashSet::new();
        // cameFrom[n] is the node immediately preceding n on the cheapest path
        let mut cameFrom: HashMap<Vec2, Vec2> = HashMap::new();

        let mut gScore: HashMap<Vec2, i64> = HashMap::new();
        let mut fScore: HashMap<Vec2, i64> = HashMap::new();

        cameFrom.insert(start, Vec2 { x: -1, y: 0 });
        openSet.insert(start);
        gScore.insert(start, 0);
        fScore.insert(start, self.dist_to_end(start));

        let mut straight_count: i32 = -1; // special for the very first step since we haven't actually moved yet

        while !openSet.is_empty() {
            let current = openSet.iter().min_by_key(|p| fScore[p]).unwrap().clone();

            if current == goal {
                return Map::reconstruct_path(&cameFrom, current);
            }

            openSet.remove(&current);
            for dir in 0..2 {
                let neighbor = Map::direction_dx(current, cameFrom[&current], dir);
                //if dir == STRAIGHT && straight_count >= 3 {
                //    continue;
                //}

                let tentative_gScore = gScore[&current] + self.heat_at(neighbor);

                if !gScore.contains_key(&neighbor) || tentative_gScore < gScore[&neighbor] {
                    cameFrom.insert(neighbor, current);
                    gScore.insert(neighbor, tentative_gScore);
                    fScore.insert(neighbor, tentative_gScore + self.dist_to_end(neighbor));
                    openSet.insert(neighbor);
                }
            }
        }

        panic!("Can't find path")
    }

    fn direction_dx(p: Vec2, last_p: Vec2, dir: Direction) -> Vec2 {
        let dx = p.x - last_p.x;
        let dy = p.y - last_p.y;

        match dir {
            STRAIGHT => Vec2 {x: dx, y: dy },
            LEFT => Vec2 { x: dy, y: -dx },
            RIGHT => Vec2 { x: -dy, y: dx },
            _ => panic!()
        }
    }

    // Return the heat cost going in direction `dir` from `pos`, with the last
    // position being `last_pos` (to set the input direction vector).
    // `steps` is the number of steps to look ahead, taking the least cost path at each leaf.
    fn weight_next(&self, pos: Vec2, last_pos: Vec2, dir: Direction,
        straight_count: i32,
        seen: &[Vec<bool>],
        steps: usize) -> Option<i64>
    {
        assert!(steps > 0);
        // can't go more than 3 steps in the same direction. The initial "turn" step counts.
        if dir == STRAIGHT && straight_count >= 3 {
            return None;
        }

        let step = Map::direction_dx(pos, last_pos, dir);
        let next = pos.go(step);
        if !self.in_range(next) {
            return None;
        }

        // don't backtrack
        if seen[next.y as usize][next.x as usize] {
            return None;
        }

        // Heat cost of entering 'next'. 
        let weight = self.weight(next);
        // If steps is 1, this is the last step; so the cost is just "weight".
        if steps == 1 {
            return Some(weight);
        }

        // Otherwise, calculate the cost of a subsequent step in each direction.
        // Then take the minimum of those costs.
        let straight = self.weight_next(next, pos, STRAIGHT, straight_count + 1, seen, steps - 1).map(|w| w + weight);
        let left = self.weight_next(next, pos, LEFT, 0, seen, steps - 1).map(|w| w + weight);
        let right = self.weight_next(next, pos, RIGHT, 0, seen, steps - 1).map(|w| w + weight);

        if straight.is_none() && left.is_none() && right.is_none() {
            // likely backtracked on all
            return None;
        }

        Some(min(straight.unwrap_or(i64::MAX), min(left.unwrap_or(i64::MAX), right.unwrap_or(i64::MAX))))
    }

    fn heat_at(&self, pos: Vec2) -> i64 {
        self.map[pos.y as usize][pos.x as usize]
    }

    fn weight(&self, pos: Vec2) -> i64 {
        let heat = self.heat_at(pos);
        let dist_to_end = self.dist_to_end(pos);

        heat + dist_to_end
    }

    fn in_range(&self, p: Vec2) -> bool {
        p.x >= 0 && p.y >= 0 && p.x <= self.fin.x && p.y <= self.fin.y
    }

    fn dist_to_end(&self, pos: Vec2) -> i64 {
        let ndx = self.fin.x - pos.x;
        let ndy = self.fin.y - pos.y;

        // manhattan distance
        ndx.abs() + ndy.abs()
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

    let mut pos = Vec2 { x: 0, y: 0 };

    let apath = map.a_star(pos, map.fin);
    println!("{:?}", apath);

    let result = 0;
    (result, vec![])
}

fn main() {
    let (r, d) = day17_inner("inputs/day17-sample.txt");
    println!("Result: {}", r);
    assert_eq!(102, r);

    //let (r, d) = day17_inner("inputs/day17.txt");
    //println!("Result: {}", r);
}