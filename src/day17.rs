#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;
use std::fmt::Debug;

mod helpers;
use helpers::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
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

    fn can_go_straight(cameFrom: &HashMap<Vec2, Vec2>, current: Vec2) -> i32 {
        let origin = Vec2 { x: 0, y: 0 };
        if current == origin { return 3; }
        let prev1 = cameFrom[&current];
        if prev1 == origin { return 2; }
        let prev2 = cameFrom[&prev1];
        if prev2 == origin { return 1; }
        let prev3 = cameFrom[&prev2];

        if (current.x == prev1.x && prev1.x == prev2.x && prev2.x == prev3.x) ||
           (current.y == prev1.y && prev1.y == prev2.y && prev2.y == prev3.y)
        {
            println!("   {:?} {:?} {:?} {:?} are all in a line", current, prev1, prev2, prev3);
            return 0;
        }

        if (current.x == prev1.x && prev1.x == prev2.x) ||
           (current.y == prev1.y && prev1.y == prev2.y)
        {
            return 1;
        }

        if (current.x == prev1.x) ||
           (current.y == prev1.y)
        {
            return 2;
        }


        return 3;
    }

    fn can_go_straight_2(cameFrom: &HashMap<Vec2, Vec2>, current: Vec2, prev1: Vec2) -> i32 {
        let origin = Vec2 { x: 0, y: 0 };
        if current == origin { return 3; }
        if prev1 == origin { return 2; }
        let prev2 = cameFrom[&prev1];
        if prev2 == origin { return 1; }
        let prev3 = cameFrom[&prev2];
        if prev3 == origin { return 1; }

        if (current.x == prev1.x && prev1.x == prev2.x && prev2.x == prev3.x) ||
           (current.y == prev1.y && prev1.y == prev2.y && prev2.y == prev3.y)
        {
            return 0;
        }

        if (current.x == prev1.x && prev1.x == prev2.x) ||
           (current.y == prev1.y && prev1.y == prev2.y)
        {
            return 1;
        }

        if (current.x == prev1.x) ||
           (current.y == prev1.y)
        {
            return 2;
        }


        return 3;
    }

    fn always_sorted(&self, a: &Vec2, b: &Vec2) -> std::cmp::Ordering {
        let da = self.dist_to_end(*a);
        let db = self.dist_to_end(*b);
        if da != db { return a.cmp(&b); }

        let wa = self.heat_at(*a);
        let wb = self.heat_at(*b);
        if wa != wb { return wa.cmp(&wb); }

        if a.x != b.x { return a.x.cmp(&b.x); }
        if a.y != b.y { return a.y.cmp(&b.y); }

        panic!("They're really equal!");
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

        while !openSet.is_empty() {
            let current = openSet.iter().min_by_key(|p| fScore[p]).unwrap().clone();
            let cur_fscore = fScore[&current];
            // all items in openSet that have the same cur_fscoe
            let mut all_current = openSet.iter().filter(|p| fScore[p] == cur_fscore).map(|p| p.clone()).collect_vec();
            // take the one with the shortest distance to the goal
            all_current.sort_by(|a, b| self.always_sorted(a, b));
            let current = all_current[0];

            println!("-- current: {:?} from {:?}", current, cameFrom[&current]);
            //println!("-- best path: {:?}", Map::reconstruct_path(&cameFrom, current));

            if current == goal {
                return Map::reconstruct_path(&cameFrom, current);
            }

            openSet.remove(&current);
            for dir in 0..3 {
                let dp = Map::direction_dx(current, cameFrom[&current], dir);
                let neighbor = current.go(dp);

                if !self.in_range(neighbor) {
                    println!("    can't go {} (not in range)", dir_name(dir));
                    continue;
                }

                let cgs = Map::can_go_straight(&cameFrom, current);
                if dir == STRAIGHT && cgs == 0 {
                    println!("    can't go straight");
                    continue;
                }

                let tentative_gScore = gScore[&current] + self.heat_at(neighbor);
                println!("    checking {} ({:?}) -> g {} cgs {}", dir_name(dir), neighbor, tentative_gScore, cgs);

                let mut update = false;
                if !gScore.contains_key(&neighbor) {
                    update = true;
                } else if tentative_gScore < gScore[&neighbor] {
                    update = true;
                } else if tentative_gScore == gScore[&neighbor] {
                    println!("    same gscore as before");
                    if dir == STRAIGHT &&
                        Map::can_go_straight_2(&cameFrom, current, neighbor) > Map::can_go_straight(&cameFrom, neighbor) {
                        update = true;
                    }
                    // if we're going straight, we want to prefer the one that's has more straight capacity?
                }

                if update {
                    println!("        updating to {}; fscore: {} (DTE: {})", tentative_gScore, tentative_gScore + self.dist_to_end(neighbor), self.dist_to_end(neighbor));
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

    fn heat_at(&self, pos: Vec2) -> i64 {
        self.map[pos.y as usize][pos.x as usize]
    }

    fn in_range(&self, p: Vec2) -> bool {
        p.x >= 0 && p.y >= 0 && p.x <= self.fin.x && p.y <= self.fin.y
    }

    fn approx_to_end(&self, pos: Vec2) -> i64 {
        let mut total = 0;
        for y in pos.y..(self.map.len() as i64) {
            for x in pos.x..(self.map[0].len() as i64) {
                total += self.heat_at(Vec2 { x, y });
            }
        }
        total
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

    let start = Vec2 { x: 0, y: 0 };
    let apath = map.a_star(start, map.fin);
    println!("\n\n");
    println!("PATH: {:?}", apath);

    let mut cmap: Vec<Vec<char>> = vec![vec!['.'; map.map[0].len()]; map.map.len()];
    let mut result = 0;
    for p in &apath[1..] {
        result += map.heat_at(*p);
        cmap[p.y as usize][p.x as usize] = '#';
    }

    print_map(&cmap);
    (result, vec![])
}

fn main() {
    let (r, d) = day17_inner("inputs/day17-sample.txt");
    println!("Result: {}", r);
    assert_eq!(102, r);

    let (r, d) = day17_inner("inputs/day17.txt");
    println!("Result: {}", r);
}