use std::fs;
use std::io::{self, Write};
use std::str::FromStr;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<Error>>;

/** 
 *1. 把string 转成坐标
 *
 * */ 
fn main() {
    let input = fs::read_to_string("data/text.txt").unwrap();
    let coordinates = input.lines()
                        .map(|line| line.parse())
                        .collect::<Result<Vec<Coordinate>>>();

   
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
