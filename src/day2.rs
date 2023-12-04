use std::{cmp::{min,max}, os::linux::raw};

#[derive(Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn new() -> Game {
        Game { red: 0, green: 0, blue: 0 }
    }

    fn record_draw(&mut self, r: i32, g: i32, b: i32) {
        self.red = max(self.red, r);
        self.green = max(self.green, g);
        self.blue = max(self.blue, b);
    }

    fn is_possible(&self, r: i32, g: i32, b: i32) -> bool {
        return self.red <= r && self.green <= g && self.blue <= b;
    }
}

fn main() {
    let data = std::fs::read_to_string("inputs/day2.txt").unwrap();

    let games = data.lines().map(|line| {
        let draws = line[line.find(": ").unwrap() + 2..].split("; ");
        let mut bag = Game::new();
        for draw in draws {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            for color in draw.split(", ") {
                let col = color.split(" ").collect::<Vec<&str>>();
                let n = col.first().unwrap().parse::<i32>().unwrap();
                println!("{:?}", col);
                match col[1] {
                    "red" => r += n,
                    "green" => g += n,
                    "blue" => b += n,
                    _ => (),
                }
            }
            bag.record_draw(r, g, b);
        }

        bag
    }).collect::<Vec<Game>>();

    let mut sum: i32 = 0;
    for (n, game) in games.iter().enumerate() {
        println!("{:?}", game);
        if game.is_possible(12, 13, 14) {
            sum += (n as i32)+1;
        }
    }

    println!("sum: {}", sum);
}
