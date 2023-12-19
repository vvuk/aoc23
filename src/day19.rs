#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use std::collections::VecDeque;
use itertools::Itertools;
use regex::Regex;
use std::fmt::Debug;
use intervals_general::bound_pair::BoundPair;
use intervals_general::interval::Interval;

mod helpers;
use helpers::*;

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

fn day19_inner(input_fname: &str) -> (i64, Vec<i64>) {
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
            continue;
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

    let mut intervals = vec![Ival::new(); 4];

    //for path in &accept_paths { println!("ACCEPT PATH: {:?}", path); }
    println!("ACCEPT PATHS: {}", accept_paths.len());
    for path in &accept_paths {
        let mut vmin = vec![None, None, None, None];
        let mut vmax = vec![None, None, None, None];

        for step_enc in path {
            let step = &rules[step_enc.0].steps[step_enc.1];
            match &step.cond {
                Condition::None => {}
                Condition::CheckGreater((Some(x),None,None,None)) => {
                    // the condition is that the value has to be bigger than x. So there's a minimum.
                    if let Some(v) = vmin[0] { vmin[0] = Some(max(v, *x)); } else { vmin[0] = Some(*x); }
                },
                Condition::CheckGreater((None,Some(m),None,None)) => {
                    if let Some(v) = vmin[1] { vmin[1] = Some(max(v, *m)); } else { vmin[1] = Some(*m); }
                },
                Condition::CheckGreater((None,None,Some(a),None)) => {
                    if let Some(v) = vmin[2] { vmin[2] = Some(max(v, *a)); } else { vmin[2] = Some(*a); }
                },
                Condition::CheckGreater((None,None,None,Some(s))) => {
                    if let Some(v) = vmin[3] { vmin[3] = Some(max(v, *s)); } else { vmin[3] = Some(*s); }
                },
                Condition::CheckLess((Some(x),None,None,None)) => {
                    // the actual value has to be less than x, so there's a maximum
                    if let Some(v) = vmax[0] { vmax[0] = Some(min(v, *x)); } else { vmax[0] = Some(*x); }
                },
                Condition::CheckLess((None,Some(m),None,None)) => {
                    if let Some(v) = vmax[1] { vmax[1] = Some(min(v, *m)); } else { vmax[1] = Some(*m); }
                },
                Condition::CheckLess((None,None,Some(a),None)) => {
                    if let Some(v) = vmax[2] { vmax[2] = Some(min(v, *a)); } else { vmax[2] = Some(*a); }
                },
                Condition::CheckLess((None,None,None,Some(s))) => {
                    if let Some(v) = vmax[3] { vmax[3] = Some(min(v, *s)); } else { vmax[3] = Some(*s); }
                },
                _ => panic!()
            }
        }

        for i in 0..4 {
            match (vmin[i], vmax[i]) {
                (Some(vmin), Some(vmax)) => {
                    if vmin > vmax {
                        intervals[i].exclude(vmax, vmin);
                    } else {
                        intervals[i].include(vmin, vmax);
                    }
                },
                (Some(vmin), None) => {
                    intervals[i].include(vmin+1, 4000);
                },
                (None, Some(vmin)) => {
                    intervals[i].include(1, vmin-1);

                },
                _ => {}
            }
        }
    }

    let mut res = 1;
    for i in 0..4 {
        println!("{:?}", intervals[i]);
        res *= intervals[i].count();
    }

    (res, vec![])
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
    let (r, d) = day19_inner("inputs/day19-sample.txt");
    println!("Result: {}", r);

    println!("===== Real =====");
    //let (r, d) = day19_inner("inputs/day19.txt");
    //println!("Result: {}", r);
}