use std::{cmp::{min,max}, os::linux::raw};

fn main() {
    let data = std::fs::read_to_string("inputs/day3-sample.txt").unwrap();

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

    let mut part1: i64 = 0;

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
                        //println!("{} {}", xc, yc);
                        let c = engine[yc as usize][xc as usize];
                        if !c.is_numeric() && c != '.' {
                            //println!("match: {:?}", num);
                            //println!("hit");
                            part1 += num as i64;
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

    println!("part1: {}", part1);

    let mut part2: i64 = 0;

    let check_star = |x: usize, y: usize| -> (i32, Option<i32>, Option<i32>) {
        // brute godamn force
        // abc
        // d.e
        // fgh
        let a = engine[y-1][x-1].is_numeric();
        let b = engine[y-1][x].is_numeric();
        let c = engine[y-1][x+1].is_numeric();
        let d = engine[y][x-1].is_numeric();
        let e = engine[y][x+1].is_numeric();
        let f = engine[y+1][x-1].is_numeric();
        let g = engine[y+1][x].is_numeric();
        let h = engine[y+1][x+1].is_numeric();

        let mut num_values = 0;

        if (a && b && c) || (a && b) || (b && c) {
            num_values += 1;
        } else if a && c {
            num_values += 2;
        } else if a || c {
            num_values += 1;
        }

        if d { num_values += 1; }
        if e { num_values += 1; }

        if (f && g && h) || (f && g) || (g && h) {
            num_values += 1;
        } else if f && h {
            num_values += 2;
        } else if f || h {
            num_values += 1;
        }

        //println!("{} {} {} {} {} {} {} {}", a, b, c, d, e, f, g, h);
        if num_values != 2 {
            return (0, None, None)
        }

        println!("star at {} {} is gear", x-1, y-1);
        return (0, None, None);
    };

    for (y, eline) in engine.iter().enumerate() {
        if y == 0 || y == engine.len() - 1 {
            continue
        }

        let mut i = 0;
        let len = eline.len();
        for i in 1..len-1 {
            if eline[i] != '*' {
                continue
            }

            println!("checking {} {}", i, y);

            let _ = check_star(i, y);
        }
    }

    println!("part2: {}", part2);
}
