#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::{Itertools, MapInto};
use regex::Regex;

mod helpers;
use helpers::*;

type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Working,
    Damaged,
    Unknown,
}

static mut indent: usize = 0;

impl State {
    fn from_str(s: &str) -> Vec<State> {
        s.chars().map(|c| State::from_char(c)).collect_vec()
    }

    fn from_char(c: char) -> State {
        match c {
            '#' => State::Damaged,
            '.' => State::Working,
            '?' => State::Unknown,
            _ => panic!("Unknown char {}", c),
        }
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Working => write!(f, "."),
            State::Damaged => write!(f, "#"),
            State::Unknown => write!(f, "?"),
        }
    }
}


fn num_match(map: &[State], damaged_runs: &[i64], in_dmg_in: bool) -> i64 {
    let mut i = 0;
    while i < map.len() && map[i] == State::Working {
        i += 1;
    }

    let smap = smap(&map[i..]);
    println!("{} checking: {}     {:?}", " ".repeat(unsafe { indent }), smap, damaged_runs);
    unsafe { indent += 1; }
    let x = nx(&map[i..], damaged_runs, in_dmg_in);
    unsafe { indent -= 1; }
    println!("{} map: {} runs: {:?} => {}", " ".repeat(unsafe { indent }), smap, damaged_runs, x);
    x
}

fn smap(map: &[State]) -> String {
    map.iter().map(|s| match s {
        State::Working => ".",
        State::Damaged => "#",
        State::Unknown => "?",
    }).collect()
}

fn nx(map: &[State], damaged_runs: &[i64], in_dmg_in: bool) -> i64 {
    if damaged_runs.is_empty() {
        if map.iter().any(|s| *s == State::Damaged) {
            println!("returning 0 because damaged and no runs left");
            return 0;
        } else {
            println!("returning 1 because no runs left and no damage");
            return 1;
        }
    }

    let mut expected_run = damaged_runs[0];
    //println!("expected_run on entry: {}", expected_run);
    let mut count: i64 = 0;

    let maplen = map.len();
    let mut i = 0;
    let mut first_dmg = in_dmg_in;
    let mut in_dmg = in_dmg_in;
    while i < maplen {
        if map[i] == State::Working {
            in_dmg = false;

            if first_dmg && expected_run > 0 {
                println!("returning 0 because expected more run but found working");
                return 0;
            }

            // we matched a run
            if expected_run == 0 {
                count += num_match(&map[i+1..], &damaged_runs[1..], false);
                println!("returning {} because matched a run, so went next", count);
                return count;
            }

            while i < maplen && map[i] == State::Working {
                i += 1;
            }
            continue;
        }

        if map[i] == State::Damaged {
            first_dmg = true;
            in_dmg = true;
            while i < maplen && map[i] == State::Damaged {
                i += 1;
                expected_run -= 1;

                if expected_run < 0 {
                    return 0;
                }
            }
            continue;
        }

        if map[i] == State::Unknown {
            if in_dmg && expected_run > 0 {
                println!("Unknown but expected {}", expected_run);
                // this has to be a '#', so we can skip it and assume it'll be a '#'
                i += 1;
                expected_run -= 1;
                continue;
            }

            let mut new_map = map.to_vec();
            let mut ca = 0;
            let mut cb = 0;

            if expected_run == 0 {
                // we expect to end a run here, the only valid option is Working,
                new_map[i] = State::Working;
                println!("new_map A: {}", smap(&new_map));
                ca = num_match(&new_map[i..], &damaged_runs[1..], false);
            } else {
                // we are not in a dmg, but expected_run > 0.
                // previous char was a working.

                new_map[i] = State::Working;
                println!("new_map B1: {}", smap(&new_map));
                ca = num_match(&new_map[i..], &damaged_runs, false);

                new_map[i] = State::Damaged;
                println!("new_map B2: {}", smap(&new_map));
                cb = num_match(&new_map[i..], &damaged_runs, in_dmg);
            }

            count += ca + cb;
            println!("for {} {:?} returning SUM {} + {} = {} after handling ?", smap(map), damaged_runs, ca, cb, count);
            return count;
        }
    }

    // we ran out of characters.
    if expected_run != 0 {
        println!("returning 0 because run was wrong length");
        return 0;
    }

    if damaged_runs.len() > 1 {
        println!("returning 0 because there were more runs left and we ran out of chars");
        return 0;
    }

    println!("returning 1 at end");
    return 1;
}

fn is_match(map: &[State], damaged_runs: &[i64]) -> bool {
    let mut ri: usize = 0;
    let mut in_run = false;
    let mut cur_run = 0;
    for i in 0..map.len() {
        assert!(map[i] != State::Unknown);

        if map[i] == State::Damaged {
            if ri >= damaged_runs.len() {
                return false;
            }
            cur_run += 1;
            in_run = true;
        } else if map[i] == State::Working {
            if in_run {
                if cur_run != damaged_runs[ri] {
                    return false;
                }
                in_run = false;
                cur_run = 0;
                ri += 1;
            }
        }
    }
    if in_run {
        if cur_run != damaged_runs[ri] {
            return false;
        }
        ri += 1;
    }

    if ri != damaged_runs.len() {
        return false;
    }

    true
}

fn day12_line(line: &str) -> i64 {
    let parts = line.split_whitespace().collect_vec();

    let map = State::from_str(parts[0]);
    let damaged_runs = parts[1].split(",").map(|s| s.parse::<i64>().unwrap()).collect_vec();

    let mut to_check: Vec<Vec<State>> = vec![map];
    let mut line_result = 0;
    while !to_check.is_empty() {
        let work = to_check;
        to_check = vec![];
        for workline in work {
            if let Some(first) = workline.iter().position(|s| *s == State::Unknown) {
                let mut new_workline = workline.clone();
                new_workline[first] = State::Working;
                to_check.push(new_workline.clone());
                new_workline[first] = State::Damaged;
                to_check.push(new_workline);
            } else {
                let ok = is_match(&workline, &damaged_runs);
                if ok {
                    line_result += 1;
                }
            }
        }
    }
    line_result
}


fn day12_inner(input_fname: &str) -> (i64, Vec<i64>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let mut springmap: Vec<i64> = vec![];
    let mut result: i64 = 0;

    for line in data.lines().map(|l| l.trim()) {
        let parts = line.split_whitespace().collect_vec();

        let map_1 = State::from_str(parts[0]);
        let damaged_runs_1 = parts[1].split(",").map(|s| s.parse::<i64>().unwrap()).collect_vec();

        let mut map: Vec<State> = vec![];
        let mut damaged_runs: Vec<i64> = vec![];

        if true {
        for i in 0..5 {
            map.extend(&map_1);
            if i != 4 {
                map.push(State::Unknown);
            }
            damaged_runs.extend(&damaged_runs_1);
        }
        } else {
            map = map_1;
            damaged_runs = damaged_runs_1;
        }

        println!("");
        println!("");
        println!("");
        println!("====== {}", smap(&map));
        let line_result = num_match(&map, &damaged_runs, false);
        springmap.push(line_result);

        result += line_result;
    }

    (result, springmap)
}

fn main() {
    /*
    println!("{}", is_match(&State::from_str("#.#.###"), &[1, 1, 3]));
    println!("{}", is_match(&State::from_str("##..###"), &[2, 3]));
    println!("{}", is_match(&State::from_str(".##.###"), &[2, 3]));
    println!("{}", is_match(&State::from_str("..#####"), &[2, 3]));
    println!("{}", is_match(&State::from_str("#####"), &[5]));
    */

    let (r, d) = day12_inner("inputs/day12-sample.txt");
    println!("{:?}", d);
    expect_vec(&[1, 16384, 1, 16, 2500, 506250], &d);
    //expect_vec(&[1, 4, 1, 1, 4, 10], &d);
    println!("Result: {}", r);

    //let (r, d) = day12_inner("inputs/day12.txt");
    //let r = num_match(&State::from_str("?###????????"), &[3,2,1]);
    //let r = num_match(&State::from_str("???????"), &[2,1], false);
    //let a = day12_line("??????? 2,1");

    //println!("Result: {} {}", r, a);
}