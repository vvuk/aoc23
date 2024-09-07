#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use core::num;
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd, borrow::BorrowMut, u8};
use std::collections::VecDeque;
use itertools::{Itertools, MinMaxResult};
use regex::Regex;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use typed_arena::Arena;
use bumpalo::Bump;

use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};

mod helpers;
use helpers::*;

use vecmath::*;

type Vec3i = Vector3<i64>;
type Vec2i = Vector2<i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    vmin: Vec3i,
    vmax: Vec3i,
}

impl Brick {
    fn xyrect(&self) -> Rect {
        Rect { vmin: [self.vmin[0], self.vmin[1]], vmax: [self.vmax[0], self.vmax[1]] }
    }

    fn move_down(&mut self, delta: i64) {
        self.vmin[2] -= delta;
        self.vmax[2] -= delta;
        assert!(self.vmin[2] > 0);
        assert!(self.vmax[2] > 0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rect {
    vmin: Vec2i,
    vmax: Vec2i,
}

impl Rect {
}

fn rect_overlap(a: &Rect, b: &Rect) -> bool {
    for q in 0..2 {
        if (a.vmax[q] < b.vmin[q]) || (b.vmax[q] < a.vmin[q]) {
            return false;
        }
    }
    true
}

fn cube_overlap(a: &Brick, b: &Brick) -> bool {
    for q in 0..3 {
        if (a.vmax[q] < b.vmin[q]) || (b.vmax[q] < a.vmin[q]) {
            return false;
        }
    }
    true
}
fn br(bn: usize) -> String {
    let letters = ('A'..='Z').collect_vec();
    letters[bn].to_string()
}

fn print_bmap(name: &str, bmap: &HashMap<usize, Vec<usize>>, bricks: &[Brick]) {
    let letters = ('A'..='Z').collect_vec();
    println!("{}:", name);

    let mut keys = bmap.keys().collect_vec();
    keys.sort();

    for k in keys {
        let v = bmap.get(k).unwrap();
        println!("{} [{}]: {:?}", letters[*k], bricks[*k].vmin[2],
            v.iter().map(|x| format!("{} [{}] ", letters[*x], bricks[*x].vmax[2])).sorted().collect_vec());
    }
}

fn day22_inner(input_fname: &str) -> usize {
    let data = std::fs::read_to_string(input_fname).unwrap();

    let mut bricks = vec![];

    for line in data.lines() {
        let line = line.trim();
        if let Some((astr, bstr)) = line.split("~").collect_tuple() {
            // unwrap to a [i64; 3]
            let vmin = astr.split(",").map(|s| s.parse::<i64>().unwrap()).collect_vec();
            let vmax = bstr.split(",").map(|s| s.parse::<i64>().unwrap()).collect_vec();

            for q in 0..3 { assert!(min(vmin[q], vmax[q]) == vmin[q]); }

            bricks.push(Brick { vmin: [vmin[0], vmin[1], vmin[2]], vmax: [vmax[0], vmax[1], vmax[2]] });
        } else {
            panic!();
        }
    }

    bricks.sort_by_key(|b| b.vmin[2]);

    let mut bricks_below = HashMap::new();
    let mut bricks_above = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        let mut below = vec![];
        let mut above = vec![];
        for (j, other) in bricks.iter().enumerate() {
            if i == j { continue; }
            if other.vmax[2] <= brick.vmin[2] && rect_overlap(&brick.xyrect(), &other.xyrect())
            {
                below.push(j);
            }
            if other.vmin[2] >= brick.vmax[2] && rect_overlap(&brick.xyrect(), &other.xyrect()) {
                above.push(j);
            }
        }
        bricks_below.insert(i, below);
        bricks_above.insert(i, above);
    }

        print_bmap("bricks_below", &bricks_below, &bricks);
    // drop all the bricks
    println!("dropping bricks");
    loop {
        let mut brick_moved = false;
        for i in 0..bricks.len() {
            let brick = bricks[i];
            let max_of_below = bricks_below[&i].iter().map(|&b| bricks[b].vmax[2]).max().unwrap_or(1);
            let delta = max_of_below - brick.vmin[2];
            assert!(delta >= 0, "delta {} for brick {} is negative -- {} - {}", delta, i, max_of_below, brick.vmin[2]);
            if delta != 0 {
                bricks[i].move_down(delta);
                brick_moved = true;
            }
        }
        if !brick_moved { break; }
    }

    println!("stacking up bricks");
    let mut bricks_stacked_up = HashMap::new();

    for this_brick in 0..bricks.len() {
        let mut above_this_brick = HashSet::new();
        above_this_brick.extend(bricks_above[&this_brick].iter());

        let mut aabsz = above_this_brick.len();
        loop {
            for a_a_idx in above_this_brick.clone() {
                let further_up = &bricks_above[&a_a_idx];
                for fu in further_up {
                    above_this_brick.insert(*fu);
                }
            }
            if above_this_brick.len() == aabsz { break; }
            aabsz = above_this_brick.len();
        }

        bricks_stacked_up.insert(this_brick, above_this_brick);
    }

    println!("bricks_supporting");

    let mut bricks_supporting = HashMap::new();

    for this_brick in 0..bricks.len() {
        //println!("{}", this_brick);
        let above_this_brick = bricks_above[&this_brick].clone();
        let mut directly_supporting = above_this_brick.clone();

        for above_brick in above_this_brick {
            let mut xc = true;

            for stacked_brick in &bricks_stacked_up[&above_brick] {
                if directly_supporting.contains(&stacked_brick) {
                    directly_supporting.remove(directly_supporting.iter().position(|x| *x == *stacked_brick).unwrap());
                }
            }
        }

        bricks_supporting.insert(this_brick, directly_supporting);
    }

/*
    for bn in 0..bricks.len() { bricks_above.insert(bn, vec![]); }
    for (top, below) in bricks_below.iter() {
        for b in below {
            let item = bricks_above.get_mut(b).unwrap();
            
            item.push(*top);
        }
    }
*/

    println!("bricks_supported_by");

    let mut bricks_supported_by = HashMap::new();
    for bn in 0..bricks.len() { bricks_supported_by.insert(bn, vec![]); }

    for brickidx in 0..bricks.len() {
        let supporting = &bricks_supporting[&brickidx];

        // brick brickidx is supporting all the supporting bricks

        for &sidx in supporting {
            let bs = bricks_supported_by.get_mut(&sidx).unwrap();
            bs.push(brickidx);
        }
    }

    if bricks.len() < 20 {
        print_bmap("bricks_below", &bricks_below, &bricks);
        print_bmap("bricks_above", &bricks_above, &bricks);
        print_bmap("bricks_supporting", &bricks_supporting, &bricks);
        print_bmap("bricks_supported_by", &bricks_supported_by, &bricks);
    }

    let mut result = 0;
    for bn in 0..bricks.len() {
        let mut can_nuke = true;
        for &supporting_brick in bricks_supporting[&bn].iter() {
            debug_println!("checking brick {} supporting {}; supported by {}", br(bn), br(supporting_brick), bricks_supported_by[&supporting_brick].len());
            // brick bn is supporting brick supporting_brick; is anything else?
            if bricks_supported_by[&supporting_brick].len() <= 1 {
                debug_println!("Nothing else is supporting brick {}, can't nuke {}!", br(supporting_brick), br(bn));
                can_nuke = false;
                break;
            }
        }

        if can_nuke {
            debug_println!("can nuke {}", br(bn));
            result += 1;
        }
    }

    result
}

fn main() {
    assert_eq!(true, rect_overlap(&Rect { vmin: [0, 0], vmax: [2, 2] },
                                  &Rect { vmin: [1, 1], vmax: [3, 3] }));
    assert_eq!(true, rect_overlap(&Rect { vmin: [0, 0], vmax: [1, 1] },
                                   &Rect { vmin: [0, 1], vmax: [1, 2] }));
    let r = day22_inner("inputs/day22-sample.txt");
    println!("Result: {}", r);

    println!("===== Real =====");
    let r = day22_inner("inputs/day22.txt");
    println!("Result: {}", r);
}