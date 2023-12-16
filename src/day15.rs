#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

mod helpers;
use helpers::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn hhash(s: &str) -> u64 {
    let mut result: u64 = 0;
    s.chars().for_each(|c| {
        result += c as u64;
        result *= 17;
        result %= 256;
    });
    result
}

fn day15_inner(input_fname: &str) -> (u64, Vec<usize>) {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let mut result: u64 = 0;
    let mut boxes: Vec<Vec<(&str,u64)>> = vec![Vec::new(); 256];

    for item in data.split(|c| c == '\n' || c == ',') {
        if item.trim().is_empty() {
            continue;
        }

        //println!("item: {}", item);
        if item.ends_with("-") {
            let label = &item[..item.len()-1];
            let hash = hhash(label);
            let mut bx = &mut boxes[hash as usize];

            //println!("Removing: {} hash: {}", label, hash);
            //go to the relevant box and remove the lens with the given label if it is present in the box.
            //Then, move any remaining lenses as far forward in the box as they can go without changing their order,
            //filling any space made by removing the indicated lens. (If no lens in that box has the given label, nothing happens.)

            if let Some(idx) = bx.iter().position(|(l,_)| l == &label) {
                bx.remove(idx);
            }
        } else {
            let mut parts = item.split('=');
            let label = parts.next().unwrap();
            let hash = hhash(label);
            let mut bx = &mut boxes[hash as usize];

            let fnum = parts.next().unwrap().parse::<u64>().unwrap();

            //println!("Inserting: {} hash: {} fnum: {}", label, hash, fnum);

            // If there is already a lens in the box with the same label, replace the old lens with the new lens:
            // remove the old lens and put the new lens in its place, not moving any other lenses in the box.
            // If there is not already a lens in the box with the same label, add the lens to the box immediately
            // behind any lenses already in the box. Don't move any of the other lenses when you do this.
            // If there aren't any lenses in the box, the new lens goes all the way to the front of the box.
            if bx.is_empty() {
                bx.push((label, fnum));
            } else if let Some(idx) = bx.iter().position(|(l,_)| l == &label) {
                bx[idx] = (label, fnum);
            } else {
                bx.push((label, fnum));
            }
        }
/*
        for (i, bx) in boxes.iter().enumerate() {
            if bx.is_empty() {
                continue;
            }
            println!("Box {}: {:?}", i, bx);
        }
        */
    }

    let mut result: u64 = 0;
    for (i, bx) in boxes.iter().enumerate() {
        let mut item: u64 = 0;
        for (li, lens) in bx.iter().enumerate() {
            item += (i as u64 + 1) * (li as u64 + 1) * lens.1;
        }
        result += item;
    }

    (result, vec![])
}

fn main() {
    let (r, d) = day15_inner("inputs/day15-sample.txt");
    println!("Result: {}", r);
    assert_eq!(145, r);

    let (r, d) = day15_inner("inputs/day15.txt");
    println!("Result: {}", r);
}