#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;
use std::fmt::Debug;

mod helpers;
use helpers::*;

type Direction = i32;

const RIGHT: Direction = 0;
const DOWN: Direction = 1;
const LEFT: Direction = 2;
const UP: Direction = 3;

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

    fn go_dir(&self, dir: Direction) -> Vec2 {
        match dir {
            UP => Vec2 { x: self.x, y: self.y - 1 },
            RIGHT => Vec2 { x: self.x + 1, y: self.y },
            DOWN => Vec2 { x: self.x, y: self.y + 1 },
            LEFT => Vec2 { x: self.x - 1, y: self.y },
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Span {
    start: Vec2,
    end: Vec2,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Spot {
    dug: bool,
    color_up: Option<u32>,
    color_right: Option<u32>,
    color_down: Option<u32>,
    color_left: Option<u32>,
}

impl Spot {
    fn new() -> Spot {
        Spot {
            dug: false,
            color_up: None,
            color_right: None,
            color_down: None,
            color_left: None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Instruction {
    dir: Direction,
    count: usize,
    color: u32,
}

fn day18_inner(input_fname: &str) -> (i64, Vec<usize>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    // parse 'A 123 (#abcdef)'
    let re = Regex::new(r"^(?P<dir>[LRUD]) (?P<count>\d+) \(#(?P<color>[0-9a-f]+)\)$").unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    let mut minv: Vec2 = Vec2 { x: 0, y: 0 };
    let mut maxv: Vec2 = Vec2 { x: 0, y: 0 };
    let mut pos: Vec2 = Vec2 { x: 0, y: 0 };

    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let color = caps.name("color").unwrap().as_str();

        let count = usize::from_str_radix(&color[0..5], 16).unwrap();
        let dir = i32::from_str(&color[5..6]).unwrap();

        match dir {
            UP => pos.y -= count as i64,
            RIGHT => pos.x += count as i64,
            DOWN => pos.y += count as i64,
            LEFT => pos.x -= count as i64,
            _ => panic!()
        }

        minv.x = min(minv.x, pos.x);
        minv.y = min(minv.y, pos.y);
        maxv.x = max(maxv.x, pos.x);
        maxv.y = max(maxv.y, pos.y);
    }

    println!("minv: {:?}, maxv: {:?}", minv, maxv);

    let width = (maxv.x - minv.x + 1) as usize;
    let height = (maxv.y - minv.y + 1) as usize;

    let mut map: Vec<Vec<Spot>> = vec![vec![Spot::new(); width]; height];

    map[(0 - minv.y) as usize][(0 - minv.x) as usize].dug = true;

    let mut pos = Vec2 { x: 0, y: 0 };
    for insn in instructions {
        let dir = insn.dir;
        for i in 0..insn.count {
            pos = pos.go_dir(dir);
            let spot = &mut map[(pos.y - minv.y) as usize][(pos.x - minv.x) as usize];
            spot.dug = true;

            match insn.dir {
                UP | DOWN => {
                    spot.color_left = Some(insn.color);
                    spot.color_right = Some(insn.color);
                },
                RIGHT | LEFT => {
                    spot.color_up = Some(insn.color);
                    spot.color_down = Some(insn.color);
                },
                _ => panic!()
            }
        }
    }

    // flood fill, even though I know it won't be enough
    let mut fill_queue: HashSet<(usize,usize)> = HashSet::new();

    // find a point to start from
    'OUTER: for j in 0..height {
        for i in 0..width {
            if map[j][i].dug {
                if (i+1) < width && !map[j][i+1].dug {
                    fill_queue.insert((i+1,j));
                    break 'OUTER;
                }
                continue 'OUTER;
            }
        }
    }

    println!("fill_queue: {:?}", fill_queue);
    while !fill_queue.is_empty() {
        let item = fill_queue.iter().next().unwrap().clone();
        fill_queue.remove(&item);

        let (i,j) = item;
        if map[j][i].dug {
            continue;
        }

        map[j][i].dug = true;

        if j > 0 { fill_queue.insert((i,j-1)); }
        if j < height-1 { fill_queue.insert((i,j+1)); }
        if i > 0 { fill_queue.insert((i-1,j)); }
        if i < width-1 { fill_queue.insert((i+1,j)); }
    }

    let mut result = 0;

    if true {
        for j in 0..height {
            for i in 0..width {
                if map[j][i].dug {
                    result += 1;
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
    (result, vec![])
}

fn main() {
    let (r, d) = day18_inner("inputs/day18-sample.txt");
    println!("Result: {}", r);

    let (r, d) = day18_inner("inputs/day18.txt");
    println!("Result: {}", r);
}