fn mapnum(s: &str) -> (i64, &str) {
    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    if s.starts_with(|c: char| c.is_numeric()) {
        return (s.chars().nth(0).unwrap() as i64 - '0' as i64, &s[1..]);
    }

    for (i, n) in numbers.iter().enumerate() {
        if s.starts_with(n) {
            //return (i as i64, &s[n.len()..]);
            return (i as i64, &s[1..]);
        }
    }

    return (-1, &s[1..]);
}

fn main() {
    let data = std::fs::read_to_string("inputs/day1.txt").unwrap();

    let mut sum: i64 = 0;
    for line in data.lines() {
        // dynamic array
        let mut v = Vec::new();
        let mut s = line;
        while s.len() > 0 {
            let (n, rest) = mapnum(s);
            if n != -1 {
                v.push(n);
            }
            s = rest;
        }

        //let v = line.chars().filter(|c| c.is_numeric()).into_iter().collect::<Vec<char>>();
        //let a = *v.first().unwrap() as i64 - '0' as i64;
        //let b = *v.last().unwrap() as i64 - '0' as i64;
        println!("{}{} {}", v.first().unwrap(), v.last().unwrap(), line);
        sum += v.first().unwrap() * 10 + v.last().unwrap();
    }

    println!("sum: {}", sum);
}
