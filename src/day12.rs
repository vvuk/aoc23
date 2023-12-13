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

fn day12_inner(input_fname: &str) -> (i64, Vec<i64>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let mut springmap: Vec<i64> = vec![];
    let mut result: i64 = 0;

    for line in data.lines().map(|l| l.trim()) {
        let parts = line.split_whitespace().collect_vec();

        let map_1 = State::from_str(parts[0]);
        let damaged_runs_1 = parts[1].split(",").map(|s| s.parse::<i64>().unwrap()).collect_vec();

        let mut ex_results: Vec<i64> = vec![];

        for ex in 0..3 {

            let mut map: Vec<State> = vec![];
            let mut damaged_runs: Vec<i64> = vec![];

            map.extend(&map_1);
            if ex == 1 {
                map.push(State::Unknown);
            } else if ex == 2 {
                map.insert(0, State::Unknown);
            }

            damaged_runs.extend(&damaged_runs_1);

            let total_damaged: i64 = damaged_runs.iter().sum();

            //???.### 1,1,3

            //println!("=====");
            //println!("{:?}", map);
            //println!("=====");
            let mut line_result: i64 = 0;
            let mut to_check: Vec<Vec<State>> = vec![map];

            let mut did = 0;

            while !to_check.is_empty() {
                let work = to_check.clone();
                //println!("did {}, left to check {}", did, to_check.len());
                to_check.clear();
                for workline in work {
                    if let Some(first) = workline.iter().position(|s| *s == State::Unknown) {
                        let mut new_workline = workline.clone();
                        new_workline[first] = State::Working;
                        to_check.push(new_workline.clone());
                        new_workline[first] = State::Damaged;
                        to_check.push(new_workline);
                    } else {
                        let ok = is_match(&workline, &damaged_runs);
                        //println!("{:?} -> {}", workline, ok);
                        if ok {
                            line_result += 1;
                        }

                        //did += 1;
                        //if did % 100000 == 0 {
                        //    println!("did {}, left to check {}", did, to_check.len());
                        //}
                    }
                }
            }

            ex_results.push(line_result);
        }

        println!("ex_results: {:?}", ex_results);
        let orig_r = ex_results[0];
        let append_r = ex_results[1];
        let prepend_r = ex_results[2];

        let by_appending = append_r.pow(4) * orig_r;
        let by_prepending = prepend_r.pow(4) * orig_r;
        let by_orig = orig_r * orig_r.pow(5);

        // if the input map ends with a run of damaged with no ?'s, then we must use orig
        let must_use_orig = if map_1.last() == Some(&State::Damaged) {
            let mut must_use_orig = None;
            for &item in map_1.iter().rev() {
                if item == State::Damaged {
                    continue;
                }
                if item == State::Working {
                    must_use_orig = Some(true);
                    break;
                }
                if item == State::Unknown {
                    must_use_orig = Some(false);
                    break;
                }
            }
            must_use_orig.unwrap_or(false)
        } else {
            false
        };

        let line_result;

        if must_use_orig {
            line_result = by_appending;
        } else {
            line_result = max(by_appending, by_prepending);
        }

        println!("line result: {}", line_result);
        result += line_result;
        springmap.push(line_result);
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
    println!("Result: {}", r);

    //let (r, d) = day12_inner("inputs/day12.txt");

    println!("Result: {}", r);
}