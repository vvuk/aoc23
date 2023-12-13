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

fn cntdiff(a: &Vec<bool>, b: &Vec<bool>) -> usize {
    assert_eq!(a.len(), b.len());
    let mut result = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            result += 1;
        }
    }
    result
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

    fn find_vert_reflection_inner(p: &Vec<Vec<bool>>, diffok: usize) -> Option<usize> {
        for i in 1..p.len() {
            if cntdiff(&p[i], &p[i-1]) <= diffok {
                let top_lines = i;
                let bot_lines = p.len() - i;

                let relevant = min(top_lines, bot_lines);

                let mut diffs = 0;

                println!("Found reflection at {} (diff {}), top: {} bot: {}, relevant: {}", i, cntdiff(&p[i], &p[i-1]), top_lines, bot_lines, relevant);

                for r in 0..relevant {
                    println!("Checking {} {}", i+r, i-r-1);
                    let diff = cntdiff(&p[i+r], &p[i-r-1]);
                    diffs += diff;
                }

                if diffs == diffok {
                    return Some(top_lines);
                }
            }
        }

        return None
    }

    fn find_reflection(&self, diffok: usize) -> Option<usize> {
        Pattern::find_vert_reflection_inner(&self.normal, diffok)
    }

    fn find_transposed_reflection(&self, diffok: usize) -> Option<usize> {
        Pattern::find_vert_reflection_inner(&self.transposed, diffok)
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

        let q = (
            p.find_reflection(1),
            p.find_transposed_reflection(1),
            p.find_reflection(0),
            p.find_transposed_reflection(0),
        );

        let mut qr;
        match q {
            (Some(r), _, _, _) => {
                println!("Found reflection (normal): {}", r);
                qr = r * 100;
            },
            (None, Some(r), _, _) => {
                println!("Found reflection (transposed): {}", r);
                qr = r;
            },
            (None, None, Some(r), _) => {
                println!("Found reflection (normal): {}", r);
                qr = r * 100;
            },
            (None, None, None, Some(r)) => {
                println!("Found reflection (transposed): {}", r);
                qr = r;
            },
            _ => panic!("No reflection found, q: {:?}", q),
        }

        rvec.push(qr);
        result += qr;
    }

    (result, rvec)
}

fn main() {
    let (r, d) = day11_inner("inputs/day13-sample.txt");
    println!("Result: {}", r);

    let (r, d) = day11_inner("inputs/day13.txt");
    println!("Result: {}", r);
}