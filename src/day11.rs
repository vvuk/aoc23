#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

mod helpers;
use helpers::*;

type Coord = (usize, usize);

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
    galaxies: Vec<Coord>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_str(data: &str) -> Map {
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut galaxies: Vec<Coord> = Vec::new();

        let nums = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for (y, line) in data.lines().enumerate() {
            let mut mapline = line.trim().chars().collect_vec();
            let empty = mapline.iter().all(|x| *x == '.');
            if empty {
                map.push("*".repeat(mapline.len()).chars().collect_vec());
            } else {
                for (x, item) in mapline.iter_mut().enumerate() {
                    if *item == '#' {
                        //*item = nums[galaxies.len()];
                        galaxies.push((x, y));
                    }
                }
                map.push(mapline);
            }
        }

        let width = map[0].len();
        let height = map.len();

        for x in 0..width {
            let empty = map.iter().all(|line| line[x] == '.' || line[x] == '*');
            if empty {
                for y in 0..height {
                    map[y][x] = '*';
                }
            }   
        }

        Map { map, galaxies, width, height }
    }

    fn weight(&self, px: i64, py: i64) -> i64 {
        if self.map[py as usize][px as usize] == '*' { 1000000 } else { 1 }
    }

    fn dist(&self, start: Coord, end: Coord) -> i64 {
        let mut x = start.0 as i64;
        let mut y = start.1 as i64;
        let x1 = end.0 as i64;
        let y1 = end.1 as i64;

        let mut dist: i64 = 0;
        loop {
            if x == x1 && y == y1 { break; }

            let dx = x1 - x;
            let dy = y1 - y;

            let mut a: Option<(i64, i64)> = None;
            let mut b: Option<(i64, i64)> = None;

            if dx != 0 {
                let dirx = if dx < 0 { -1 } else { 1 };
                a = Some((x + dirx, y));
            }
            if dy != 0 {
                let diry = if dy < 0 { -1 } else { 1 };
                b = Some((x, y + diry));
            }

            if dy > dx {
                std::mem::swap(&mut a, &mut b);
            }

            let aw = a.map(|p| self.weight(p.0, p.1)).unwrap_or(0);
            let bw = b.map(|p| self.weight(p.0, p.1)).unwrap_or(0);

            match (a, b) {
                (Some(a), Some(b)) => {
                    if aw > bw {
                        dist += bw;
                        x = b.0;
                        y = b.1;
                    } else {
                        dist += aw;
                        x = a.0;
                        y = a.1;
                    }
                },
                (Some(a), None) => {
                    dist += aw;
                    x = a.0;
                    y = a.1;
                },
                (None, Some(b)) => {
                    dist += bw;
                    x = b.0;
                    y = b.1;
                },
                (None, None) => {
                    panic!();
                }
            } 
        }
        dist
    }
}

fn day11_inner(input_fname: &str) -> (i64, HashMap<(usize, usize), i64>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let map = Map::from_str(&data);
    //print_map(&map.map);
    let num_galaxies = map.galaxies.len();
    println!("Galaxies [{}]: {:?}", input_fname, num_galaxies);

    let mut result: i64 = 0;
    let mut dists: HashMap<(usize, usize), i64> = HashMap::new();

    for i in 0..num_galaxies {
        for j in i+1..num_galaxies {
            let pair = (i, j);
            let dist = map.dist(map.galaxies[pair.0], map.galaxies[pair.1]);
            dists.insert(pair, dist);
            result += dist;
        }
    }

    (result, dists)
}

fn main() {
    let (r, d) = day11_inner("inputs/day11-sample.txt");
    expect(&d, (4, 8), 9);
    expect(&d, (0, 6), 15);
    expect(&d, (2, 5), 17);
    expect(&d, (7, 8), 5);
    println!("Result: {}", r);

    let (r, d) = day11_inner("inputs/day11.txt");
    println!("Result: {}", r);
}