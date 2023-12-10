#![allow(unused_imports, unused_variables, dead_code)]
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd};
use itertools::Itertools;
use regex::Regex;

// bits: 0bNESW

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
enum Direction {
    North = 0b1000,
    East = 0b0100,
    South = 0b0010,
    West = 0b0001,
}

impl Direction {
    fn flip(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl std::convert::TryFrom<i64> for Direction {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0b1000 => Ok(Direction::North),
            0b0100 => Ok(Direction::East),
            0b0010 => Ok(Direction::South),
            0b0001 => Ok(Direction::West),
            _ => Err(())
        }
    }
}

// start: 0b1111
// ground: 0b0000
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
enum MapTile {
    Ground = 0b0000,
    Start = 0b1111,
    EastAndSouth = 0b0110,
    EastAndNorth = 0b1100,
    EastAndWest = 0b0101,
    WestAndSouth = 0b0011,
    WestAndNorth = 0b1001,
    NorthAndSouth = 0b1010,
}

impl MapTile {
    fn other_direction(&self, rhs: Direction) -> Direction {
        assert_ne!(self, &MapTile::Ground);
        assert_ne!(self, &MapTile::Start);
        Direction::try_from((*self as i64) & !(rhs as i64)).unwrap()
    }
}

impl std::convert::TryFrom<i64> for MapTile {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if MapTile::NorthAndSouth as i64 == value { return Ok(MapTile::NorthAndSouth); }
        if MapTile::EastAndWest as i64 == value { return Ok(MapTile::EastAndWest); }
        if MapTile::EastAndSouth as i64 == value { return Ok(MapTile::EastAndSouth); }
        if MapTile::EastAndNorth as i64 == value { return Ok(MapTile::EastAndNorth); }
        if MapTile::WestAndSouth as i64 == value { return Ok(MapTile::WestAndSouth); }
        if MapTile::WestAndNorth as i64 == value { return Ok(MapTile::WestAndNorth); }
        Err(())
    }
}

impl BitAnd<Direction> for MapTile {
    type Output = bool;

    fn bitand(self, rhs: Direction) -> Self::Output {
        if self == MapTile::Ground || self == MapTile::Start {
            return false;
        }
        self as i64 & rhs as i64 != 0
    }
}

type Coord = (usize, usize);

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<MapTile>>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Map {
    fn from_str(data: &str) -> Map {
        let mut map: Vec<Vec<MapTile>> = Vec::new();
        let mut start: (usize, usize) = (0, 0);
        for (y, line) in data.lines().enumerate() {
            let mut row: Vec<MapTile> = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let loc = match c {
                    'S' => MapTile::Start,
                    'F' => MapTile::EastAndSouth,
                    '-' => MapTile::EastAndWest,
                    '7' => MapTile::WestAndSouth,
                    '|' => MapTile::NorthAndSouth,
                    'J' => MapTile::WestAndNorth,
                    'L' => MapTile::EastAndNorth,
                    '.' => MapTile::Ground,
                    _ => panic!("Unknown location: {}", c)
                };
                if loc == MapTile::Start { start = (x, y); }
                row.push(loc);
            }
            map.push(row);
        }

        // fix the start character
        let start_tile = map[start.1][start.0];
        let dirmap: Vec<((i64, i64), Direction)> = vec![
            ((-1, 0), Direction::East),
            ((0, -1), Direction::South),
            ((1, 0), Direction::West),
            ((0, 1), Direction::North),
        ];

        let width = map[0].len();
        let height = map.len();

        let mut startchar: i64 = 0;
        for (offs, dir) in dirmap {
            if start.0 == 0 && offs.0 < 0 { continue; }
            if start.1 == 0 && offs.1 < 0 { continue; }
            if start.0 == width-1 && offs.0 > 0 { continue; }
            if start.1 == height-1 && offs.1 > 0 { continue; }

            let checkloc = (((start.0 as i64)+offs.0) as usize, ((start.1 as i64)+offs.1) as usize);
            let loc = map[checkloc.1][checkloc.0];
            println!("{:?} -> {:?} (the dir: {:?})", checkloc, loc, dir);
            if loc & dir {
                startchar |= dir.flip() as i64;
            }
        }

        let starttile = MapTile::try_from(startchar).unwrap();
        println!("st: {:?}", starttile);

        map[start.1][start.0] = starttile; 
        Map { width, height, map, start }
    }

    fn get(&self, loc: Coord) -> MapTile {
        self.map[loc.1][loc.0].clone()
    }

    // doesn't matter which way we go, just pick one
    fn pick_start_dir(&self) -> Direction {
        let sx = self.start.0;
        let sy = self.start.1;

        let dirmap: Vec<((i64, i64), Direction)> = vec![
            ((-1, 0), Direction::East),
            ((0, -1), Direction::South),
            ((1, 0), Direction::West),
            ((0, 1), Direction::North),
        ];

        for (offs, dir) in dirmap {
            if sx == 0 && offs.0 < 0 { continue; }
            if sy == 0 && offs.1 < 0 { continue; }
            if sx == self.width-1 && offs.0 > 0 { continue; }
            if sy == self.height-1 && offs.1 > 0 { continue; }

            let checkloc = (((sx as i64)+offs.0) as usize, ((sy as i64)+offs.1) as usize);
            let loc = self.get(checkloc);
            //println!("{:?} -> {:?} (the dir: {:?})", checkloc, loc, dir);
            if loc & dir {
                return dir.flip()
            }
        }
        panic!("Bad pick_start_dir");
    }

    fn go(&self, loc: Coord, dir: Direction) -> Coord {
        let mut x = loc.0 as i64;
        let mut y = loc.1 as i64;
        match dir {
            Direction::North => y -= 1,
            Direction::East => x += 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
        }
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            panic!("Bad go");
        }

        (x as usize, y as usize)
    }

    // we're now at location loc, and we just came from Direction.
    // return where we go next
    fn next_loc(&self, loc: Coord, come_from: Direction) -> Coord {
        let tile = self.get(loc);
        self.go(loc, tile.other_direction(come_from.flip()))
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() {
    let data = include_str!("../inputs/day10.txt");

    let map = Map::from_str(data);
    let mut dir = map.pick_start_dir();
    let mut loc  = map.go(map.start, dir);

    let mut wallmap: Vec<Vec<char>> = vec![vec!['.'; map.width]; map.height];

    wallmap[map.start.1][map.start.0] = '#';

    println!("start: {:?}", map.start);

    let mut result: i64 = 0;
    while loc != map.start {
        wallmap[loc.1][loc.0] = '#';
        // we just went "dir" to get to loc
        let tile = map.get(loc);
        //println!("loc: {:?} from {:?}, tile: {:?}", loc, dir, tile);
        dir = tile.other_direction(dir.flip());
        loc = map.go(loc, dir);
        result += 1;
    }

    println!("Result: {} -> {}", result, (result as f64 / 2.0).round());

    let mut ymap: Vec<Vec<char>> = vec![vec!['.'; map.width]; map.height];
    let mut xmap: Vec<Vec<char>> = vec![vec!['.'; map.width]; map.height];

    let mut enclosed: i64 = 0;

    for y in 0..map.height {
        let mut inside_n: bool = false;
        let mut inside_s: bool = false;
        for x in 0..map.width {
            let wall = wallmap[y][x] == '#';
            if wall {
                let tile = map.get((x, y));
                if tile & Direction::North {
                    ymap[y][x] = '|';
                    inside_n = !inside_n;
                }
                if tile & Direction::South {
                    ymap[y][x] = '|';
                    inside_s = !inside_s;
                }
            } else if inside_n || inside_s {
                ymap[y][x] = 'X';
            }
        }
    }
    print_map(&wallmap);
    println!("");

    for x in 0..map.width {
        let mut inside_e: bool = false;
        let mut inside_w: bool = false;
        for y in 0..map.height {
            let wall = wallmap[y][x] == '#';
            if wall {
                let tile = map.get((x, y));
                if tile & Direction::East {
                    xmap[y][x] = '-';
                    inside_e = !inside_e;
                }
                if tile & Direction::West {
                    xmap[y][x] = '-';
                    inside_w = !inside_w;
                }
            } else if inside_e || inside_w {
                xmap[y][x] = 'Y';
                if ymap[y][x] == 'X' {
                    ymap[y][x] = '$';
                    enclosed += 1;
                }
            }
        }
    }

    print_map(&ymap);
    println!("");
    print_map(&xmap);

    println!("Enclosed: {}", enclosed);
}
