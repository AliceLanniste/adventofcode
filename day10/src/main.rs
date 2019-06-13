#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;
use std::str::FromStr;
use std::error::Error;

type Result<T> = std::result::Result<T,Error>;

//组成单词的面积是最小的，在之后面积会变大，因此要想获得单词，成立的条件是new_area > current_area,

fn main() {
    let mut input = fs::read_to_string("data/text.txt").unwrap();
    let mut Points_vec = vec![];
    for line in input.lines() {
        let point = line.parse().expect("error");
        Points_vec.push(point);
    }

}



#[derive(Debug)]
struct Points {
    points: Vec<Point>,
    second: u32,
}

impl Points {
    fn new(points: Vec<Point>) -> Self {

        Points {points, second: 0}
    }

    fn turing(&mut self)  {
        for p in &mut self.points {
            p.x +=p.vx;
            p.y +=p.vy;
        }
        self.second += 1;
    }


    fn dimesion(arg: Type) -> RetType {
        unimplemented!();
    }
}


#[derive(Debug)]
struct Bound {
    minX: i32,
    maxX: i32,
    minY: i32,
    maxY: i32
}

impl Bound {

    fn width(arg: Type) -> usize {
        (self.maxX - self.minX + 1) as usize 
    }

    fn height(arg: Type) -> usize {
        (self.maxY - self.minY + 1) as usize
    }
}


#[derive(Debug,PartialEq)]
struct Point {
    Position:(i32, i32),
    Velocity:(i32, i32)
}


impl FromStr for Point {
    type Err =Box<>;
    fn from_str(s: &str) -> Result<Point> {
        lazy_static! {
             static ref RE: Regex = Regex::new(r"(?x)
                position=<\s*(?P<x>[-0-9]+),\s*(?P<y>[-0-9]+)>
                \s+
                velocity=<\s*(?P<vx>[-0-9]+),\s*(?P<vy>[-0-9]+)>
            ").unwrap();
        }

        let caps = RE.captures(s).expect("failed to regex");

        Ok(Point {
            Position:(caps["x"].parse()?, caps["y"].parse()?),
            Velocity:(caps["vx"].parse()?, caps["vy"].parse()?)
        })
    }
}



