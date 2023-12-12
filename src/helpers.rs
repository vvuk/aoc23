#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::cmp::{min, max, PartialEq, Ordering, Ord, PartialOrd, Eq};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::{str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

pub fn expect_eq<Item: Debug, A: PartialEq + Debug>(item: &Item, expected: &A, actual: &A) {
    println!("{:?}: expected {:?} got {:?} -- {}", item, expected, actual, if expected == actual { "pass" } else { "FAIL" });
}

pub fn expect_map<Item: Debug + Eq + Hash, A: PartialEq + Debug>(d: &HashMap<Item, A>, item: Item, expected: A) {
    expect_eq(&item, &expected, &d[&item]);
}

pub fn expect_vec<A: PartialEq + Debug>(expected: &[A], actual: &[A]) {
    assert_eq!(expected.len(), actual.len());
    for (i, (e, a)) in expected.iter().zip(actual.iter()).enumerate() {
        expect_eq(&i, e, a);
    }
}


pub fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}
