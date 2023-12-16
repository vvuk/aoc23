#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

mod helpers;
use helpers::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Empty,
    Moving,
    Fixed
}

impl Item {
    fn from_char(c: char) -> Item {
        match c {
            '.' => Item::Empty,
            '#' => Item::Fixed,
            'O' => Item::Moving,
            _ => panic!("Unknown item: {}", c)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Platform {
    data: Vec<Vec<Item>>,
}

impl Platform {
    fn from_vec(data: Vec<Vec<Item>>) -> Platform {
        Platform { data }
    }

    fn hashkey(&self) -> String {
        self.data.iter().map(|row| {
            row.iter().map(|item| {
                match item {
                    Item::Empty => '.',
                    Item::Fixed => '#',
                    Item::Moving => 'O',
                }
            }).collect::<String>()
        }).collect::<String>()
    }

    fn tilt(&mut self, dir: Direction) {
        let jmax = self.data.len();
        let imax = self.data[0].len();
        if dir == Direction::North {
            for j in 0..jmax {
                for i in 0..imax {
                    let item = self.data[j][i];

                    if item == Item::Empty || item == Item::Fixed {
                        continue;
                    }

                    if j == 0 { continue; }
                    let mut mj = j - 1;
                    while self.data[mj][i] == Item::Empty {
                        self.data[mj+1][i] = Item::Empty;
                        self.data[mj][i] = Item::Moving;
                        if mj == 0 { break; }
                        mj -= 1;
                    }
                }
            }
        } else if dir == Direction::South {
            for jq in 0..jmax {
                let j = jmax - jq - 1;
                for i in 0..imax {
                    let item = self.data[j][i];

                    if item == Item::Empty || item == Item::Fixed {
                        continue;
                    }

                    if j == jmax-1 { continue; }
                    let mut mj = j + 1;
                    while self.data[mj][i] == Item::Empty {
                        if mj == 100 {
                            println!("{}, {} -- {}", mj, i, jmax);
                        }
                        self.data[mj-1][i] = Item::Empty;
                        self.data[mj][i] = Item::Moving;
                        mj += 1;
                        if mj == jmax { break; }
                    }
                }
            }
        } else if dir == Direction::West {
            for i in 0..imax {
                for j in 0..jmax {
                    let item = self.data[j][i];

                    if item == Item::Empty || item == Item::Fixed {
                        continue;
                    }

                    if i == 0 { continue; }
                    let mut mi = i - 1;
                    while self.data[j][mi] == Item::Empty {
                        self.data[j][mi+1] = Item::Empty;
                        self.data[j][mi] = Item::Moving;
                        if mi == 0 { break; }
                        mi -= 1;
                    }
                }
            }
        } else if dir == Direction::East {
            for iq in 0..imax {
                let i = imax - iq - 1;
                for j in 0..jmax {
                    let item = self.data[j][i];

                    if item == Item::Empty || item == Item::Fixed {
                        continue;
                    }

                    if i == imax-1 { continue; }
                    let mut mi = i + 1;
                    while self.data[j][mi] == Item::Empty {
                        self.data[j][mi-1] = Item::Empty;
                        self.data[j][mi] = Item::Moving;
                        mi += 1;
                        if mi == imax { break; }
                    }
                }
            }
        }
    }

    fn count_load_north(&self) -> i64 {
        let mut result: i64 = 0;
        let jmax = self.data.len();
        for j in 0..self.data.len() {
            for i in 0..self.data[0].len() {
                let item = self.data[j][i];

                if item == Item::Moving {
                    result += (jmax - j) as i64;
                }
            }
        }
        result
    }

    fn print(&self) {
        for row in &self.data {
            for item in row {
                print!("{}", match item {
                    Item::Empty => '.',
                    Item::Fixed => '#',
                    Item::Moving => 'O',
                });
            }
            println!();
        }
    }
}

fn day14_inner(input_fname: &str) -> (i64, Vec<usize>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let mut platform = Platform::from_vec(data.lines().map(|line| {
        line.chars().map(|c| Item::from_char(c)).collect_vec()
    }).collect_vec());

    let mut memo = HashMap::new();
    let mut results = vec![];

    let num_spins = 1_000_000_000;
/*
        platform.tilt(Direction::North);
        platform.print();
        println!("");
        platform.tilt(Direction::West);
        platform.print();
        println!("");
        platform.tilt(Direction::South);
        platform.print();
        println!("");
        platform.tilt(Direction::East);
        platform.print();
        panic!("");
*/
    let mut loop_start = None;
    for spin in 0..num_spins {
        if spin % 1_000_000 == 0 {
            println!("Spin: {}", spin / 1_000_000);
        }

        platform.tilt(Direction::North);
        platform.tilt(Direction::West);
        platform.tilt(Direction::South);
        platform.tilt(Direction::East);

        if let Some(start) = memo.get(&platform.hashkey()) {
            println!("Found loop at spin: {}, start: {}, val: {}", spin, start, platform.count_load_north());
            loop_start = Some(*start);
            break;
        }

        memo.insert(platform.hashkey(), spin);
        results.push(platform.count_load_north());
    }

    let loop_start = loop_start.unwrap();

    let cycle_results = &results[loop_start..];
    let cycle = results.len() - loop_start;
    let result_index = (num_spins - loop_start) % cycle;
    println!("cycle: {} -- result_index: {}", cycle, result_index);
    println!("{:?}", results);
    println!("{:?}", cycle_results);
    let result = cycle_results[result_index-1];
    (result, vec![])
}

fn main() {
    let (r, d) = day14_inner("inputs/day14-sample.txt");
    println!("Result: {}", r);
    assert_eq!(64, r);

    let (r, d) = day14_inner("inputs/day14.txt");
    println!("Result: {}", r);
}