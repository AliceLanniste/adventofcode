/**
 * day 6 part 1 要最大且是有限的距离
 * 1.无限的距离满足row == 0 || row == height-1 || col == 0 || col == width-1
 * 2.如果一个坐标和其他圈出来的坐标距离一致也不算在内
 *
 * 
 * part2 满足距离 < 10000,所有点的个数(面积)
 */

use std::fs;
use std::io::{self, Write};
use std::str::FromStr;
use std::error::Error;
use std::cmp::max;

type Result<T> = std::result::Result<T, Box<Error>>;

const EMPTY: i32 = -1;
const FULL: i32 = -2; 
const MAX_REGION:u32 = 10000;

/** 
 *1. 把string 转成坐标
 *
 * */ 
fn main() -> Result<()> {
    let input = fs::read_to_string("data/day6.txt").unwrap();
    let coordinates = input.lines()
                        .map(|line| line.parse().unwrap())
                        .collect::<Vec<Coordinate>>();

//    let mut temp :Vec<i32> = vec![];
  let mut width = 0;
  let mut height = 0;
  for c in coordinates.iter() {
      width = max(width, c.x+1);
      height = max(height,c.y+1);
  }

    

  let default = BestDistance {id:EMPTY, distance: std::i32::MAX};

  let mut distances:Vec<BestDistance> = vec![default; (width * height) as usize];
  let mut is_finite: Vec<bool> = vec![true; coordinates.len()];
  let counts: Vec<i32> = vec![0;coordinates.len()];  
 
//   1.填充distances,填充is_finite
  for row in 0..height {
      for col in 0..width {
          let entry = &mut distances[(row *width + col) as usize];

          for (id, c) in coordinates.iter().enumerate() {
             let dist = (col-c.x).abs() + (row-c.y).abs();
              if dist < entry.distance {
                  entry.id = id as i32;
                  entry.distance = dist;
              } else if dist == entry.distance {
                  entry.id = FULL;
              }
          }

          if (row == 0 || row == height-1 || col == 0 || col == width-1) 
                        && entry.id !=FULL {
                let idx = entry.id as usize;
                is_finite[idx] = false;
          }
      }
  }
  
 part1(distances, is_finite, counts)?;
part2(width, height, coordinates)?;
  Ok(())
}

fn part1(ds: Vec<BestDistance>, is_finite:Vec<bool>, 
                mut counts: Vec<i32>) -> Result<()> {
    
    for entry in &ds {
        if entry.id == FULL {
            continue;
        }
        let idc = entry.id as usize;
        counts[idc] +=1;
    }

    let mut max_finite = 0 ;
    for i in 0..counts.len() {
        if is_finite[i] && counts[i] > max_finite {
            max_finite = counts[i];
        }
    }
     writeln!(io::stdout(), "biggest area: {}", max_finite)?;

     Ok(())
}


fn part2(width:i32, height:i32,coordinates:Vec<Coordinate>) -> Result<()> {
    
    let output = help(width,height,coordinates);
    writeln!(io::stdout(), "part2 area: {}",output)?;
    Ok(())
}

fn help(width:i32,height:i32,coordinates:Vec<Coordinate>) -> i32 {
    let mut nums = 0;
    let mut distances = vec![0; (width * height) as usize];
    for row in 0..height {
        for col in 0..width {
            let entry = &mut distances[(row * width + col) as usize];

            for c in &coordinates {
                let dist = (c.x-col).abs() + (c.y - row).abs();
                let new_dist = *entry + dist;
                *entry = new_dist;
            }

            if (*entry as u32) < MAX_REGION {
                nums += 1;
            }
        }
        
    }
    nums
}


#[derive(Debug,Copy,Clone)]
struct BestDistance {
    id: i32,
    distance: i32,
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}


impl FromStr for Coordinate {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let comma = match s.find(",") {
            Some(i) => i,
            None => return Err(From::from("could not find comma")),
        };

        let (x1,y1) = (&s[..comma].trim(), &s[comma+1..].trim());
        Ok(Coordinate{ x: x1.parse()?, y: y1.parse()?})
    }
}
