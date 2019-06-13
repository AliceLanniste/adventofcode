#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;
use std::str::FromStr;
use std::error::Error;
use std::cmp;

type Result<T> = std::result::Result<T,Box<Error>>;

//组成单词的面积是最小的，在之后面积会变大，因此要想获得单词，成立的条件是new_area > current_area,

fn main() {
    let mut input = fs::read_to_string("data/day10.txt").unwrap();
    let mut Points_vec: Vec<Point> = vec![];
    for line in input.lines() {
        let point = line.parse().expect("error");
        Points_vec.push(point);
    }
    let points_struct = Points::new(Points_vec);
    

    loop {
// 必须加上current points排列
        
    }

}
// 对比此时和下时刻的面积，当下时刻面积大于此时的面积就是组成字母
// 首先得把min和max确定

#[derive(Debug,Clone)]
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

    fn bound(&self) -> Bound {
        
       let mut minX = self.points[0].x;
       let mut maxX = self.points[0].x;
       let mut minY = self.points[0].y;
       let mut maxY = self.points[0].y;


        for p in &self.points {
            minX= cmp::min(minX, p.x);
            maxX= cmp::max(maxX,p.x);
            minY= cmp::min(minY, p.y);
            maxY= cmp::max(maxY,p.y);
        }

        Bound {minX, maxX, minY, maxY}
    }

    fn area(&self) -> u32{
        let bound =self.bound();
        let width = bound.width();
        let height = bound.height();

        width * height
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

    fn width(&self) -> u32 {
        (self.maxX - self.minX + 1) as u32 
    }

    fn height(&self) -> u32 {
        (self.maxY - self.minY + 1) as u32
    }
}


#[derive(Debug,PartialEq,Clone)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32
}


impl FromStr for Point {
    type Err =Box<Error>;

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
            x: caps["x"].parse()?, 
            y: caps["y"].parse()?,
            vx: caps["vx"].parse()?, 
            vy:caps["vy"].parse()?
        })
    }
}



