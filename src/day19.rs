#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use std::collections::VecDeque;
use itertools::Itertools;
use regex::Regex;
use std::fmt::Debug;
use discrete_range_map::{DiscreteRangeMap, DiscreteFinite};
use discrete_range_map::interval::InclusiveInterval;

mod helpers;
use helpers::*;

pub fn ii(x1: i64, x2: i64) -> InclusiveInterval<i64> {
	InclusiveInterval { start: x1, end: x2, }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn new(x: i64, m: i64, a: i64, s: i64) -> Part {
        Part { x, m, a, s }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Condition {
    None,
    CheckLess((Option<i64>, Option<i64>, Option<i64>, Option<i64>)),
    CheckGreater((Option<i64>, Option<i64>, Option<i64>, Option<i64>)),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleAction {
    Jump(String),
    Accept,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleStep {
    cond: Condition,
    action: RuleAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    name: String,
    steps: Vec<RuleStep>,
}

fn optadd(vals: (Option<i64>, Option<i64>, Option<i64>, Option<i64>), add: i64) -> (Option<i64>, Option<i64>, Option<i64>, Option<i64>) {
    let mut res = (None, None, None, None);
    if let Some(x) = vals.0 { res.0 = Some(x + add); }
    if let Some(x) = vals.1 { res.1 = Some(x + add); }
    if let Some(x) = vals.2 { res.2 = Some(x + add); }
    if let Some(x) = vals.3 { res.3 = Some(x + add); }
    res
}

fn day19_inner(input_fname: &str) -> u128 {
    let data = std::fs::read_to_string(input_fname).unwrap();

    // rnm{s>2431:R,a<563:A,x<1798:A,R}
    let rule_re = Regex::new(r"^(?P<name>[a-zA-Z0-9]+)\{(?P<steps>[^}]+)\}$").unwrap();

    let lines = data.lines();

    let mut parts: Vec<Part> = vec![];
    let mut rules: Vec<Rule> = vec![];
    let mut rule_map: HashMap<String, usize> = HashMap::new();
    let mut second = false;

    for line in data.lines() {
        if line.trim().is_empty() {
            second = true;
            break;
        }

        if !second {
            let caps = rule_re.captures(line).unwrap();
            let name = caps.name("name").unwrap().as_str();
            let steps = caps.name("steps").unwrap().as_str().split(",")
                .map(|s| {
                    let s = s.trim();
                    match s {
                        "R" => RuleStep { cond: Condition::None, action: RuleAction::Reject },
                        "A" => RuleStep { cond: Condition::None, action: RuleAction::Accept },
                        _ => {
                            let cond_re = Regex::new(r"^((?P<var>[xmas])(?P<op>[<>])(?P<val>\d+):)?(?P<act>.+)?$").unwrap();
                            let caps = cond_re.captures(s).unwrap();
                            let var = caps.name("var");
                            let op = caps.name("op");
                            let val = caps.name("val");
                            let act = caps.name("act").unwrap().as_str();

                            let cond = if let (Some(var), Some(op), Some(val)) = (var, op, val) {
                                let var = var.as_str();
                                let op = op.as_str();
                                let val = val.as_str().parse::<i64>().unwrap();

                                let cval = match var {
                                    "x" => (Some(val), None, None, None),
                                    "m" => (None, Some(val), None, None),
                                    "a" => (None, None, Some(val), None),
                                    "s" => (None, None, None, Some(val)),
                                    _ => panic!(),
                                };

                                match op {
                                    "<" => Condition::CheckLess(cval),
                                    ">" => Condition::CheckGreater(cval),
                                    _ => Condition::None,
                                }
                            } else {
                                Condition::None
                            };

                            let act = match act {
                                "R" => RuleAction::Reject,
                                "A" => RuleAction::Accept,
                                _ => RuleAction::Jump(act.to_string()),
                            };

                            RuleStep { cond, action: act }
                        }
                    }
                }).collect_vec();

            rule_map.insert(name.to_string(), rules.len());
            rules.push(Rule { name: name.to_string(), steps });
        } else {
            // each line looks like
            // {x=39,m=19,a=96,s=217}
            let re = Regex::new(r"^\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}$").unwrap();
            let caps = re.captures(line).unwrap();
            let x = caps.name("x").unwrap().as_str().parse::<i64>().unwrap();
            let m = caps.name("m").unwrap().as_str().parse::<i64>().unwrap();
            let a = caps.name("a").unwrap().as_str().parse::<i64>().unwrap();
            let s = caps.name("s").unwrap().as_str().parse::<i64>().unwrap();
            parts.push(Part { x, m, a, s });
        }
    }

    let mut accept_paths = vec![];

    // the path is a set of (rule index, step index) pairs that lead to acceptance
    let mut cur_path = vec![];
    cur_path.push((rule_map["in"], 0 as usize));

    let mut work = VecDeque::new();
    work.push_back(cur_path);

    while let Some(item) = work.pop_front() {
        let last = item.last().unwrap();
        let last_rule = &rules[last.0];
        let last_step = &last_rule.steps[last.1];

        // if there was a condition, then put a work item back
        // for the next thing, if the condition fails
        if last_step.cond != Condition::None {
            let mut new_path = item.clone();
            new_path.last_mut().unwrap().1 += 1;
            work.push_back(new_path);
        }

        // now here, the condition passes. what's the action?
        match &last_step.action {
            RuleAction::Accept => {
                // this is a valid path to accepting a part
                accept_paths.push(item);
                continue;
            }
            RuleAction::Reject => {
                continue;
            }
            RuleAction::Jump(dest_name) => {
                let dest_idx = rule_map[dest_name];
                let mut new_path = item.clone();
                new_path.push((dest_idx, 0));
                work.push_back(new_path);
            }
        }
    }

    let mut res: u128 = 1;
    //for path in &accept_paths { println!("ACCEPT PATH: {:?}", path); }

    println!("ACCEPT PATHS: {}", accept_paths.len());
    for path in &accept_paths {
        let mut ivals = vec![DiscreteRangeMap::new(); 4];
        for i in 0..4 { ivals[i].insert_strict(ii(1, 4000), true).ok(); }

        println!("PATH: {:?}", path);

        for step_enc in path {
            let rule = &rules[step_enc.0];
            for stepi in 0..step_enc.1+1 {
                let step = &rule.steps[stepi];
                let mut cond = step.cond;
                if stepi != step_enc.1 {
                    // flip if not taken
                    cond = match cond {
                        Condition::CheckGreater(k) => Condition::CheckLess(optadd(k, 1)),
                        Condition::CheckLess(k) => Condition::CheckGreater(optadd(k, -1)),
                        Condition::None => panic!(),
                        _ => panic!(),
                    };
                }

                println!("  STEP: {:?} => {:?}", step.cond, cond);
                match &cond {
                    Condition::None => {}
                    Condition::CheckGreater((Some(x),None,None,None)) => { ivals[0].cut(ii(1, *x)).collect_vec(); },
                    Condition::CheckGreater((None,Some(m),None,None)) => { ivals[1].cut(ii(1, *m)).collect_vec(); },
                    Condition::CheckGreater((None,None,Some(a),None)) => { ivals[2].cut(ii(1, *a)).collect_vec(); },
                    Condition::CheckGreater((None,None,None,Some(s))) => { ivals[3].cut(ii(1, *s)).collect_vec(); },
                    Condition::CheckLess((Some(x),None,None,None)) => { ivals[0].cut(ii(*x, 4001)).collect_vec(); }, 
                    Condition::CheckLess((None,Some(m),None,None)) => { ivals[1].cut(ii(*m, 4001)).collect_vec(); }, 
                    Condition::CheckLess((None,None,Some(a),None)) => { ivals[2].cut(ii(*a, 4001)).collect_vec(); }, 
                    Condition::CheckLess((None,None,None,Some(s))) => { ivals[3].cut(ii(*s, 4001)).collect_vec(); }, 
                    _ => panic!()
                }
            }
        }

        let mut ires: u128 = 1;
        for i in 0..4 {
            let mut v = 0;
            for iv in ivals[i].iter() {
                v += (iv.0.end - iv.0.start) + 1;
                println!("{}:  {:?} -> {}", ["x","m","a","s"][i], iv.0, v);
            }
            ires *= v as u128;
        }

        res += ires;
    }

    res
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ival {
    chunks: Vec<(i64, i64)>,
}

impl Ival {
    fn new() -> Ival {
        Ival { chunks: vec![(1,4000)] }
    }

    fn include(&mut self, vmin: i64, vmax: i64) {
        let mut new_chunks = vec![];
        let mut found = false;
        for (a, b) in &self.chunks {
            if vmin > *b || vmax < *a {
                new_chunks.push((*a, *b));
            } else {
                new_chunks.push((min(*a, vmin), max(*b, vmax)));
                found = true;
            }
        }

        if !found {
            new_chunks.push((vmin, vmax));
        }

        self.chunks = new_chunks;
    }

    fn exclude(&mut self, vmin: i64, vmax: i64) {
        let mut new_chunks = vec![];
        for (a, b) in &self.chunks {
            if vmin > *b || vmax < *a {
                new_chunks.push((*a, *b));
            } else if vmin <= *a && vmax >= *b {
                // the whole chunk is excluded
            } else if vmin <= *a && vmax < *b {
                new_chunks.push((vmax+1, *b));
            } else if vmin > *a && vmax >= *b {
                new_chunks.push((*a, vmin-1));
            } else {
                new_chunks.push((*a, vmin-1));
                new_chunks.push((vmax+1, *b));
            }
        }

        self.chunks = new_chunks;
    }

    fn count(&self) -> i64 {
        let mut res = 0;
        for (a, b) in &self.chunks {
            res += b - a + 1;
        }
        res
    }
}

fn main() {
    let r = day19_inner("inputs/day19-sample-2.txt");
    println!("Result: {} max", 256000000000000 as u128);
    println!("Result: {}", r);

    let r = day19_inner("inputs/day19-sample.txt");
    assert_eq!(167409079868000, r-1);

    println!("===== Real =====");
    let r = day19_inner("inputs/day19.txt");
    println!("Result: {}", r-1);
}