#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

mod helpers;
use helpers::*;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Beam {
    pos: (i64, i64),
    dir: (i64, i64),
}

impl Beam {
    fn step(&mut self) {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn tile(&self, p: (i64,i64)) -> char {
        self.map[p.1 as usize][p.0 as usize]
    }

    fn in_range(&self, p: (i64,i64)) -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < self.map[0].len() as i64 && p.1 < self.map.len() as i64
    }
}

fn count_energized(beam: Beam, map: &Map) -> u64 {
    // a beam has a current position and a direction ((x,y), (dx,dy))
    // and has an the initial beam
    let mut beams: Vec<Beam> = vec![];
    let mut new_beams: Vec<Beam> = vec![];

    let mut emap: Vec<Vec<i32>> = vec![vec![0; map.map[0].len()]; map.map.len()];

    beams.push(beam);

    let mut count = 0;

    while beams.len() > 0 {
        let bcount = beams.len();

        if false { //count % 1000 == 0 {
            print_map(&map.map);
            println!("");
            print_bool_map(&emap);
            println!("Beams: {:?}", beams);
        }

        for bi in 0..bcount {
            let mut beam = beams[bi];

            let dirbit = match beam.dir {
                (1,0) => 0,
                (0,1) => 1,
                (-1,0) => 2,
                (0,-1) => 3,
                _ => panic!("Invalid beam direction: {:?}", beam.dir),
            };

            beam.step();

            //println!("Beam: {:?}", beam);

            // are we out of range?
            if !map.in_range(beam.pos) {
                //println!("Out of range");
                continue;
            }

            // if this is already energized then a beam has been through here
            if emap[beam.pos.1 as usize][beam.pos.0 as usize] & (1 << dirbit) != 0 {
                continue;
            }

            emap[beam.pos.1 as usize][beam.pos.0 as usize] |= (1 << dirbit);

            // x = 1 to go right; x = -1 to go left
            // y = 1 to go down; y = -1 to go up
            let tile = map.tile(beam.pos);
            match tile {
                '/' => {
                    if beam.dir == (1,0) {
                        beam.dir = (0,-1)
                    } else if beam.dir == (-1,0) {
                        beam.dir = (0,1)
                    } else if beam.dir == (0,1) {
                        beam.dir = (-1,0)
                    } else if beam.dir == (0,-1) {
                        beam.dir = (1,0)
                    } else {
                        panic!("Invalid beam direction: {:?}", beam.dir);
                    }

                    new_beams.push(beam);
                },
                '\\' => {
                    if beam.dir == (1,0) {
                        beam.dir = (0,1)
                    } else if beam.dir == (-1,0) {
                        beam.dir = (0,-1)
                    } else if beam.dir == (0,1) {
                        beam.dir = (1,0)
                    } else if beam.dir == (0,-1) {
                        beam.dir = (-1,0)
                    } else {
                        panic!("Invalid beam direction: {:?}", beam.dir);
                    }

                    new_beams.push(beam);
                },
                '|' if beam.dir.0 != 0 => {
                    new_beams.push(Beam { pos: beam.pos, dir: (0,-1) });
                    new_beams.push(Beam { pos: beam.pos, dir: (0,1) });
                },
                '-' if beam.dir.1 != 0 => {
                    new_beams.push(Beam { pos: beam.pos, dir: (-1,0) });
                    new_beams.push(Beam { pos: beam.pos, dir: (1,0) });
                },
                _ => {
                    new_beams.push(beam);
                }
            }
        }

        std::mem::swap(&mut beams, &mut new_beams);
        new_beams.clear();
    }

    //print_map(&map.map); println!(""); print_bool_map(&emap);
    let mut result: u64 = 0;
    for y in 0..emap.len() {
        for x in 0..emap[0].len() {
            if emap[y][x] != 0 {
                result += 1;
            }
        }
    }

    result
}

fn day16_inner(input_fname: &str) -> (u64, Vec<usize>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let mut map: Vec<Vec<char>> = vec![];
    for line in data.lines() {
        map.push(line.trim().chars().collect());
    }

    let map = Map { map };

    let xlen = map.map[0].len();
    let ylen = map.map.len();

    let mut result = 0;

    for x in 0..xlen {
        let a = count_energized(Beam { pos: (x as i64, -1), dir: (0,1) }, &map);
        let b = count_energized(Beam { pos: (x as i64, ylen as i64), dir: (0,-1) }, &map);

        result = std::cmp::max(result, a);
        result = std::cmp::max(result, b);
    }

    for y in 0..ylen {
        let a = count_energized(Beam { pos: (-1, y as i64), dir: (1,0) }, &map);
        let b = count_energized(Beam { pos: (xlen as i64, y as i64), dir: (-1,0) }, &map);

        result = std::cmp::max(result, a);
        result = std::cmp::max(result, b);
    }

    (result, vec![])
}

fn main() {
    let (r, d) = day16_inner("inputs/day16-sample.txt");
    println!("Result: {}", r);
    assert_eq!(51, r);

    let (r, d) = day16_inner("inputs/day16.txt");
    println!("Result: {}", r);
}