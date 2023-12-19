#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;
use std::fmt::Debug;

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
                    println!("STEP: {}", s);
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

    let mut accepted: Vec<Part> = vec![];
    for part in &parts {
        let mut rule_index = rule_map["in"];
        'RULES: loop {
            let rule = &rules[rule_index];

            for step in &rule.steps {
                let pass = match step.cond {
                    Condition::CheckLess((Some(x), None, None, None)) => part.x < x,
                    Condition::CheckLess((None, Some(m), None, None)) => part.m < m,
                    Condition::CheckLess((None, None, Some(a), None)) => part.a < a,
                    Condition::CheckLess((None, None, None, Some(s))) => part.s < s,
                    Condition::CheckGreater((Some(x), None, None, None)) => part.x > x,
                    Condition::CheckGreater((None, Some(m), None, None)) => part.m > m,
                    Condition::CheckGreater((None, None, Some(a), None)) => part.a > a,
                    Condition::CheckGreater((None, None, None, Some(s))) => part.s > s,
                    Condition::None => true,
                    _ => panic!(),
                };

                if !pass {
                    continue;
                }

                if let RuleAction::Jump(name) = &step.action {
                    rule_index = rule_map[name];
                    continue 'RULES;
                }

                if step.action == RuleAction::Accept {
                    accepted.push(part.clone());
                }

                break 'RULES;
            }
        }
    }

    let mut sum: i64 = 0;
    for acc in &accepted {
        sum += acc.x + acc.m + acc.a + acc.s;
    }

    (sum, vec![])
}

fn main() {
    let (r, d) = day19_inner("inputs/day19-sample.txt");
    println!("Result: {}", r);

    println!("===== Real =====");
    let (r, d) = day19_inner("inputs/day19.txt");
    println!("Result: {}", r);
}