#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;
use std::error::Error;
use std::cmp;
use std::io::{self, Write};
use std::str::{self, FromStr};

type Result<T> = std::result::Result<T,Box<Error>>;

//组成单词的面积是最小的，在之后面积会变大，因此要想获得最小的面积，
// 成立的条件是new_area > current_area,因此循环条件应为
//while new_area< current_area,
// 如果数据过大，width*height会导致数据溢出，无论是i32,i64还是usize
// 因此使用width+height取代width*height，可以运行正常


fn main() ->Result<()>{
    let mut input = fs::read_to_string("data/text.txt").unwrap();
    let mut Points_vec: Vec<Point> = vec![];
    for line in input.lines() {
        let point = line.parse().expect("error");
        Points_vec.push(point);
    }
    let mut points_struct = Points::new(Points_vec);
    let mut  current_area = std::i32::MAX;
    let mut  new_area =  points_struct.area();

    while new_area < current_area {
        points_struct.turning(true);
        current_area = new_area;
        new_area = points_struct.area();
    }
   
    points_struct.turning(false);
    writeln!(io::stdout(),"{}",points_struct.second)?;
    writeln!(io::stdout(), "{}", points_struct.make_string().trim())?;
    Ok(())
}


#[derive(Debug,Clone)]
struct Points {
    points: Vec<Point>,
    second: u32,
}

impl Points {
    fn new(points: Vec<Point>) -> Self {

        Points {points, second: 0}
    }

    fn turning(&mut self,cond:bool)  {
        if cond {
            for p in &mut self.points {
                p.x +=p.vx;
                p.y +=p.vy;
            }
            self.second += 1;
            }
        else {
            for p in &mut self.points {
                p.x -=p.vx;
                p.y -=p.vy;
            }
            self.second -= 1;
            }

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

    fn area(&self) -> i32{
        let bound =self.bound();
        let width = bound.width();
        let height = bound.height();

        width + height
    }

    fn width_height(&self) -> (usize, usize) {
        let bound =self.bound();
        
        (bound.width() as usize, bound.height() as usize)
    }


    fn make_string(&self) -> String {
        let (width,height) = self.width_height();
        let bounds = self.bound();
        let mut grid = vec![vec![b'.'; width]; height];
        for p in &self.points {
            let x = bounds.normal_x(p.x);
            let y = bounds.normal_y(p.y);
            grid[y as usize][x as usize] = b'#';
        }

        let mut buf = String::new();
        for row in grid {
            buf.push_str(str::from_utf8(&row).unwrap());
            buf.push('\n');
        }
        buf
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

    fn normal_x(&self, x: i32) -> u32 {
        if self.minX >= 0 {
            (x - self.minX) as u32
        } else {
            (x + self.minX.abs()) as u32
        }
    }

    fn normal_y(&self, y: i32) -> u32 {
        if self.minY >= 0 {
            (y - self.minY) as u32
        } else {
            (y + self.minY.abs()) as u32
        }
    }

    fn width(&self) -> i32 {
        (self.maxX - self.minX + 1) 
    }

    fn height(&self) -> i32 {
        (self.maxY - self.minY + 1) 
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




    
       



