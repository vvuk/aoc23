#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let data = include_str!("../inputs/day9.txt");

    let inputs: Vec<Vec<i64>> = data.lines()
        .map(|l| l.split_whitespace()
            .map(|v| v.parse::<i64>().unwrap()).collect_vec())
        .collect();

    let mut result: i64 = 0;

    for input in inputs {
        let mut sequence: Vec<Vec<i64>> = Vec::new();
        sequence.push(input.clone());
        loop {
            let cur = sequence.last().unwrap();
            let mut cseq: Vec<i64> = Vec::new();
            for i in 0..cur.len()-1 {
                cseq.push(cur[i+1] - cur[i]);
            }

            let done = cseq.iter().all(|v| *v == 0);
            sequence.push(cseq);
            if done { break; }
        }

        //for q in 0..sequence.len() { println!("{:?}", sequence[q]); }

        let mut item: i64 = 0;
        for q in sequence.iter().rev().skip(1) {
            item += q.last().unwrap();
        }
        //println!("{}", item);
        result += item;
        //println!("====");
    }

    println!("Result: {}", result);
}
