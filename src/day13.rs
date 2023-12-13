#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

mod helpers;
use helpers::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    normal: Vec<Vec<bool>>,
    transposed: Vec<Vec<bool>>,
}

impl Pattern {
    fn from_normal(normal: Vec<Vec<bool>>) -> Pattern {
        let mut transposed = vec![vec![false; normal.len()]; normal[0].len()];

        for j in 0..normal[0].len() {
            for i in 0..normal.len() {
                transposed[j][i] = normal[i][j];
            }
        }

        Pattern { normal, transposed }
    }

    // 01|234  len = 5, i = 2.  
    // top: 2. bot: len - 2

    fn print(&self) {
        for line in &self.normal {
            for c in line {
                print!("{}", if *c { '#' } else { '.' });
            }
            println!();
        }
        println!();
        for line in &self.transposed {
            for c in line {
                print!("{}", if *c { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }

    fn find_vert_reflection_inner(p: &Vec<Vec<bool>>) -> Option<usize> {
        for i in 1..p.len() {
            if p[i] == p[i-1] {
                let top_lines = i;
                let bot_lines = p.len() - i;

                let relevant = min(top_lines, bot_lines);

                println!("Found reflection at {}, top: {} bot: {}, relevant: {}", i, top_lines, bot_lines, relevant);

                let mut ok = true;
                for r in 0..relevant {
                    println!("Checking {} {}", i+r, i-r-1);
                    if p[i+r] != p[i-r-1] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    return Some(top_lines);
                }
            }
        }

        return None
    }

    fn find_reflection(&self) -> Option<usize> {
        Pattern::find_vert_reflection_inner(&self.normal)
    }

    fn find_transposed_reflection(&self) -> Option<usize> {
        Pattern::find_vert_reflection_inner(&self.transposed)
    }
}

fn parse_patterns(data: &str) -> Vec<Pattern> {
    let mut result = Vec::new();

    let mut cur_pattern_lines = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            result.push(Pattern::from_normal(cur_pattern_lines));
            cur_pattern_lines = Vec::new();
            continue;
        }

        cur_pattern_lines.push(line.chars().map(|c| c == '#').collect_vec());
    }

    if !cur_pattern_lines.is_empty() {
        result.push(Pattern::from_normal(cur_pattern_lines));
    }

    result
}

fn day11_inner(input_fname: &str) -> (usize, Vec<usize>) {
    let data = std::fs::read_to_string(input_fname).unwrap();
    let patterns = parse_patterns(&data);

    println!("patterns: {}", patterns.len());
    let mut rvec = Vec::new();
    let mut result = 0;

    for p in patterns {
        p.print();
        if let Some(r) = p.find_reflection() {
            println!("Found reflection (normal): {}", r);
            rvec.push(r * 100);
            result += r * 100;
        } else if let Some(r) = p.find_transposed_reflection() {
            println!("Found reflection (transposed): {}", r);
            rvec.push(r);
            result += r;
        } else {
            panic!("No reflection found");
        }
    }

    (result, rvec)
}

fn main() {
    let (r, d) = day11_inner("inputs/day13-sample.txt");
    println!("Result: {}", r);

    let (r, d) = day11_inner("inputs/day13.txt");
    println!("Result: {}", r);
}