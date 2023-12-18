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
const DIR_NONE: Direction = 4;

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Span {
    start: Vec2,
    end: Vec2,
    end_inside: Option<bool>,
    dig_dir: i32,
}

impl Span {
    fn new(start: Vec2, end: Vec2) -> Span {
        Span { start, end, end_inside: None, dig_dir: DIR_NONE }
    }

    fn new_dir(start: Vec2, end: Vec2, dir: Direction) -> Span {
        Span { start, end, end_inside: None, dig_dir: dir }
    }
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

fn shoelace(points: &[Vec2]) -> i64 {
    let mut sum = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += points[i].x * points[j].y - points[j].x * points[i].y;
    }
    let result = sum.abs() / 2;

    println!("SHOELACE: {}", result);

    let mut pathlen: i64 = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        pathlen += (points[i].x - points[j].x).abs() + (points[i].y - points[j].y).abs();
    }
    println!("PATHLEN {} SHOELACE + PATHLEN/2 {}", pathlen, result + pathlen/2 + 1);
    result
}

fn day18_inner(input_fname: &str) -> (i64, Vec<usize>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    // parse 'A 123 (#abcdef)'
    let re = Regex::new(r"^(?P<dir>[LRUD]) (?P<count>\d+) \(#(?P<color>[0-9a-f]+)\)$").unwrap();

    let mut minv: Vec2 = Vec2 { x: 0, y: 0 };
    let mut maxv: Vec2 = Vec2 { x: 0, y: 0 };
    let mut pos: Vec2 = Vec2 { x: 0, y: 0 };

    let mut hspans: Vec<Span> = vec![];
    let mut vspans: Vec<Span> = vec![];

    let mut points: Vec<Vec2> = vec![];

    points.push(pos);

    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let (count, dir) = if false {
            let dir = match caps.name("dir").unwrap().as_str() {
                "U" => UP,
                "R" => RIGHT,
                "D" => DOWN,
                "L" => LEFT,
                _ => panic!()
            };
            let count = caps.name("count").unwrap().as_str().parse::<usize>().unwrap() as i64;
            (count, dir)
        } else {
            let color = caps.name("color").unwrap().as_str();
            let count = usize::from_str_radix(&color[0..5], 16).unwrap() as i64;
            let dir = i32::from_str(&color[5..6]).unwrap();
            (count, dir)
        };

        let addend = match dir {
            UP => Vec2 { x: 0, y: -count },
            RIGHT => Vec2 { x: count, y: 0 },
            DOWN => Vec2 { x: 0, y: count },
            LEFT => Vec2 { x: -count, y: 0 },
            _ => panic!()
        };

        let new_pos = pos.go(addend);

        if dir == UP || dir == DOWN {
            if pos.y < new_pos.y {
                vspans.push(Span::new(pos, new_pos));
            } else {
                vspans.push(Span::new(new_pos, pos));
            }
        } else {
            if pos.x < new_pos.x {
                hspans.push(Span::new(pos, new_pos));
            } else {
                hspans.push(Span::new(new_pos, pos));
            }
        }

        points.push(new_pos);

        pos = new_pos;
        minv.x = min(minv.x, new_pos.x);
        minv.y = min(minv.y, new_pos.y);
        maxv.x = max(maxv.x, new_pos.x);
        maxv.y = max(maxv.y, new_pos.y);
    }

    shoelace(&points);

    vspans.sort_by(|a, b| {
        if a.start.x == b.start.x {
            a.start.y.cmp(&b.start.y)
        } else {
            a.start.x.cmp(&b.start.x)
        }
    });

    println!("minv: {:?}, maxv: {:?}", minv, maxv);

    if true {
        return (0, vec![]);
    }

    let mut zmap = vec![vec!['.'; (maxv.x - minv.x + 1) as usize]; (maxv.y - minv.y + 1) as usize];

    let mut result = 0;
    let mut jcount: i64 = 0;
    let mut spanvec: Vec<Span> = vec![];
    for j in minv.y..maxv.y+1 {
        if jcount % 10_000 == 0 {
            println!("j: {}, left: {}/{}", j, jcount, maxv.y - j);
        }
        jcount += 1;

        spanvec.clear();
        // pull out this span from the vertical set
        // the spans are sorted by x first and then y
        for span in &vspans {
            // this span is relevant
            if j >= span.start.y && j <= span.end.y {
                spanvec.push(*span);
            }
        }

        let mut hspanvec: Vec<Span> = hspans.iter().filter(|span| { j == span.start.y }).copied().collect_vec();

        // we know every span overlaps y, so we just care about x now
        spanvec.sort_by(|a, b| { a.start.x.cmp(&b.start.x) });

        //println!("{} hspanvec: {:?}", j, hspanvec);
        //println!("{}", j);
        for sv in &spanvec {
            //println!("  {:?}", sv);
            let spanx = sv.start.x;
            if hspanvec.iter().any(|h| { h.start.x == spanx || h.end.x == spanx } ) {
                // already handled by existing spans (corner)
                continue;
            }

            hspanvec.push(Span::new(Vec2 { x: spanx, y: j }, Vec2 { x: spanx, y: j }));
        }

        // now just sort these by x.  Each span is always "inside", and they each cause a transition
        hspanvec.sort_by(|a, b| { a.start.x.cmp(&b.start.x) });
        //println!("{} hspanvec: {:?}", j, hspanvec);

        let mut inside = false;
        let mut lastx = minv.x;
        for sv in &hspanvec {
            if inside {
                let count = sv.start.x - lastx - 1;
                for x in 0..count {
                    zmap[(j - minv.y) as usize][(x + lastx - minv.x + 1) as usize] = '#';
                }
                //println!("  {:?} pre {}", sv, count);
                result += count;
            }

            let count = sv.end.x - sv.start.x + 1;
            //println!("  {:?} span {}", sv, count);
            for x in 0..count {
                zmap[(j - minv.y) as usize][(x + sv.start.x - minv.x) as usize] = '#';
            }

            result += count;
            inside = !inside;
            lastx = sv.end.x;
        }
    }
    print_map(&zmap);
    (result, vec![])
}

fn spangraph(spans: &[Span]) {
    // figure out min/max coordinates
    let mut minv: Vec2 = Vec2 { x: 0, y: 0 };
    let mut maxv: Vec2 = Vec2 { x: 0, y: 0 };

    for span in spans {
        minv.x = min(minv.x, span.start.x);
        minv.y = min(minv.y, span.start.y);
        maxv.x = max(maxv.x, span.end.x);
        maxv.y = max(maxv.y, span.end.y);
    }

    let width = maxv.x - minv.x + 1;
    let height = maxv.y - minv.y + 1;
    println!("width: {}, height: {}", width, height);
    let mut map: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];

    for span in spans {
        for x in span.start.x..span.end.x+1 {
            for y in span.start.y..span.end.y+1 {
                map[(y - minv.y) as usize][(x - minv.x) as usize] = '#';
            }
        }
    }

    print_map(&map)
}

fn main() {
    let (r, d) = day18_inner("inputs/day18-sample.txt");
    println!("Result: {}", r);

    println!("===== Real =====");
    let (r, d) = day18_inner("inputs/day18.txt");
    println!("Result: {}", r);
}