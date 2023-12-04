use std::{cmp::{min,max}, os::linux::raw};

fn main() {
    let data = std::fs::read_to_string("inputs/day3.txt").unwrap();

    let engine = data.lines()
        .map(|line| { line.trim().chars().collect::<Vec<_>>() })
        .collect::<Vec<_>>();

    let mut sum: i64 = 0;

    for (y, eline) in engine.iter().enumerate() {
        let mut i = 0;
        let len = eline.len();
        while i < len {
            //println!("{} {}", i, len);
            if eline[i].is_numeric() {
                let numstr = &eline[i..].into_iter().take_while(|c| c.is_numeric()).collect::<String>();
                //println!("{:?}", numstr);
                let num = numstr.parse::<i32>().unwrap();

                let xmin = max(0, i as i32 - 1);
                let xmax = min(len, i + numstr.len() + 1) as i32;
                let ymin = max(0, y as i32 - 1);
                let ymax = min(engine.len(), y + 2) as i32;

                //println!("{}: {}..{} {}..{}", num, xmin, xmax, ymin, ymax);
                // check [xmin..xmax) [ymin..ymax) for non-number non-dot symbols
                'SEARCH: for yc in ymin..ymax {
                    for xc in xmin..xmax {
                        println!("{} {}", xc, yc);
                        let c = engine[yc as usize][xc as usize];
                        if !c.is_numeric() && c != '.' {
                            //println!("match: {:?}", num);
                            //println!("hit");
                            sum += num as i64;
                            break 'SEARCH;
                        }
                    }
                }

                i += numstr.len();
            } else {
                i += 1;
            }
        }
    }

    println!("sum: {}", sum);
}
