#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}};

struct Thing {
    xmin: usize,
    xmax: usize,
    num: i32,
}

fn main() {
    let data = std::fs::read_to_string("inputs/day3.txt").unwrap();

    // we're going to prepend and append a '.'
    let mut engine = data.lines()
        .map(|line| {
            let mut l = line.trim().chars().collect::<Vec<_>>();
            l.push('.');
            l.insert(0, '.');
            l
        }).collect::<Vec<_>>();

    // and then pad out the top and bottom
    engine.push(vec!['.'; engine[0].len()]);
    engine.insert(0, vec!['.'; engine[0].len()]);

    let mut intervals: Vec<Vec<Thing>> = Vec::new();

    // compute the intervals
    for eline in engine.iter() {
        let mut i = 0;
        let len = eline.len();
        let mut line_intervals: Vec<Thing> = Vec::new();
        while i < len {
            //println!("{} {}", i, len);
            if eline[i].is_numeric() {
                let numstr = &eline[i..].into_iter().take_while(|c| c.is_numeric()).collect::<String>();
                //println!("{:?}", numstr);
                let num = numstr.parse::<i32>().unwrap();

                line_intervals.push(Thing { xmin: i, xmax: i + numstr.len(), num: num });
                i += numstr.len();
            } else {
                i += 1;
            }
        }

        intervals.push(line_intervals);
    }

    let mut sum: i64 = 0;

    // let's find stars
    for (y, eline) in engine.iter().enumerate() {
        if y == 0 || y == engine.len() - 1 {
            continue
        }

        for x in 1..eline.len()-1 {
            if eline[x] != '*' {
                continue
            }

            //println!("star: {} {}", x, y);
            let xmin = x-1;
            let xmax = x+2;
            let ymin = y-1;
            let ymax = y+1;

            let mut a: i32 = 0;
            let mut b: i32 = 0;
            let mut count = 0;

            'SEARCH: for ey in ymin..ymax+1 {
                //println!("ey {}", ey);
                for interval in intervals[ey].iter() {
                    //println!("({} {})? ({} {})", xmin, xmax, interval.xmin, interval.xmax);
                    // starts after the target ends
                    if xmin >= interval.xmax {
                        continue
                    }
                    // ends before the target starts
                    if xmax <= interval.xmin {
                        continue
                    }

                    //println!("yes");
                    if count == 0 {
                        a = interval.num;
                        count += 1;
                    } else if count == 1 {
                        b = interval.num;
                        count += 1;
                    } else {
                        count += 1;
                        break 'SEARCH;
                    }
                }
            }

            //println!("count {} {} {}", count, a, b);

            if count == 2 {
                sum += (a * b) as i64;
            }
        }
    }

    println!("sum: {}", sum);
}
