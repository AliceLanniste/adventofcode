extern crate regex;

#[macro_use]
extern crate lazy_static;





pub mod day1 {
    use std::fs;
    use std::fmt::Write;
    use std::collections::HashSet;
    
    
  pub fn run() ->Vec<String>{
    let result_vec = vec![part1(), part2()];
    result_vec
   
  }

  fn part1() -> String {
    let mut result: String = String::with_capacity(128);

    let file_strings = fs::read_to_string("data/day1.txt").unwrap();
    let lines = file_strings.lines();

    let values = lines.map(|line| line.parse::<i32>().unwrap());

    let sum:i32 = values.clone().sum();

    writeln!(&mut result, "Day 1, Problem 1 - [{}]", sum).unwrap(); 

    result
  }

  fn part2() -> String {
    let mut result: String = String::with_capacity(128);

    let file_strings = fs::read_to_string("data/day1.txt").unwrap();
    let lines = file_strings.lines();

    let values = lines.map(|line| line.parse::<i32>().unwrap());


    let mut sum =0;
    let mut set = HashSet::<i32>::new();

    for v in values.clone().cycle() {
        sum +=v;
        let single = set.insert(sum);
        if !single 
          { break ; }
    }

    writeln!(&mut result, "Day 1, Problem 2 - [{}]", sum).unwrap(); 

    result
  }

}  


pub mod day2 {
  use std::fs;
  use std::fmt::Write;

  pub fn run() ->Vec<String> {
    let result_vec = vec![part1(), part2()];
    result_vec
    
  }

 pub fn part1() -> String {
      let mut result: String = String::with_capacity(128);
      let file_strings =fs::read_to_string("data/day2.txt").unwrap();
      let lines :Vec<&str> = file_strings.lines().collect();
      let mut total_two_count = 0;
      let mut total_three_count = 0;

      for line in &lines {
          let mut count: [i32; 26] = [0; 26];
            let mut line_two_count = 0;
            let mut line_three_count = 0;

      

          for letter in line.chars() {
              
              let index: usize = (letter as usize) - ('a' as usize);
              let new_count = count[index] + 1;

              
            match new_count {
                 2 => line_two_count  += 1,
                 3 =>{
                      line_three_count += 1;
                      line_two_count  -=1;
                 } 
                 4 => line_three_count -=1,
                 _ =>(),

            }
            count[index] = new_count;
           
         }
          


        if  line_two_count > 0
          {total_two_count += 1;}
        if line_three_count >0 {
            total_three_count += 1;
        }
      }
       writeln!(
            &mut result,
            "Day 2, Problem 1 - [{}]",total_two_count*total_three_count).unwrap();

    result
  }


  pub fn part2() -> String {
    let mut result_str = String::with_capacity(128);
    let file_strings = fs::read_to_string("data/day2.txt").unwrap();
    let lines: Vec<&str> = file_strings.lines().collect();

    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            if let Some(result) = common(&lines[i], &lines[j]) {
              writeln!(&mut result_str, "day2 {}",result);
            }
        }
    }

    result_str
  }


  fn common(line1: &str, line2:&str) -> Option<String> {
    if line1.len() != line2.len() {
        return None;
    }

    let mut differ = false;
    for (c1, c2) in line1.chars().zip(line2.chars()) {
      if c1 != c2 {
        if differ {
            return None;
        }

        differ = true;
      }
    }

    Some(line1.chars().zip(line2.chars())
              .filter(|&(c1, c2)| c1==c2)
              .map(|(c, _)| c)
              .collect()
    )
  }

}

// 
pub mod day3 {
 

  // use lazy_static;
  use regex::Regex;

  use std::str::FromStr;
  use std::collections::HashMap;
  use std::fs;
  use std::result;
  use std::error::Error;
  use std::fmt::Write;


  type Result<T> = result::Result<T,Box<Error>>;

  type Grid = HashMap<(u32,u32), u32>;

    pub fn run() ->String{
      
      let mut Pieces: Vec<Piece> = vec![];
      let file_strings = fs::read_to_string("data/day3.txt").unwrap();
      let lines = file_strings.lines();

      for line in lines {
          let p:Piece = line.parse().expect("failed to parse");
          Pieces.push(p);
      }

      let mut grid = Grid::new();
      for pie in Pieces {
        for(x,y) in pie.iterPoint() {
          *grid.entry((x,y)).or_default() += 1;
        }
      }

      let result =part1(&grid);
        result
    }

    fn part1(grid:&Grid) -> String {
      let mut result = String::with_capacity(128);
      let count = grid.values().filter(|&&count| count > 1).count();
      writeln!(&mut result, "day 2,part 1 - {}",count);
      result
    }

    #[derive(Debug)]
    struct Piece {
        x: u32,
        y: u32,
        width: u32,
        height: u32
    }

    impl FromStr for Piece {
        type Err = Box<Error>;
        
        fn from_str(s: &str) -> Result<Self> {
           lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):
                 \s+
                (?P<width>[0-9]+)x(?P<height>[0-9]+)
            ").unwrap();
          }

          let caps = RE.captures(s).expect("unrecognized claim");
          // parse 
          // pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err> 
          //   where  F: FromStr

           Ok(Piece {
              x: caps["x"].parse()?,
              y: caps["y"].parse()?,
              width: caps["width"].parse()?,
              height: caps["height"].parse()?
          })
        }
    }

    impl Piece {
        fn iterPoint(&self) -> Point {
          Point{
            piece: self,
            px: self.x,
            py:self.y
          }
        }
    }

   
   #[derive(Debug)]
   struct Point<'a> {
       piece: &'a Piece,
       px: u32,
       py: u32
   }

   impl<'a> Iterator for Point<'a> {
     type Item = (u32, u32);

     fn next(&mut self) -> Option<Self::Item> {
        if self.py >= self.piece.y+ self.piece.height {
          self.py = self.piece.y;
          self.px += 1;
        }

        if self.px >= self.piece.x+self.piece.width {
          return None;
        }

        let (x,y) = (self.px,self.py);
        self.px += 1;
        Some((x,y))
     }
   }
    
}

// pub mod day03 {
//     // use lazy_static::lazy_static;
//     use regex::Regex;
//     use std::cmp::max;
//     use std::fmt::Write;
//     use std::fs;

    // #[derive(Copy, Clone)]
    // struct Box {
    //     min_x: u16,
    //     max_x: u16,
    //     min_y: u16,
    //     max_y: u16,
    // }

    // impl Box {
    //     fn new(x: u16, y: u16, width: u16, height: u16) -> Box {
    //         Box {
    //             min_x: x,
    //             min_y: y,
    //             max_x: x + width - 1,
    //             max_y: y + height - 1,
    //         }
    //     }
    // }

//     pub fn run() -> String {
//         let mut result = String::with_capacity(128);

//         // Parse input
//         lazy_static! {
//             static ref file_string: String =
//                 fs::read_to_string("data/input_day3.txt").unwrap();
//         }

//         lazy_static! {
//             static ref re: Regex =
//                 Regex::new(r"#\d+ @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)")
//                     .unwrap();
//         }

//         let mut boxes: Vec<Box> = Vec::new();
//         let mut width = 0;
//         let mut height = 0;

//         for line in file_string.lines() {
//             let caps = re.captures(line).unwrap();
//             let x = caps["x"].parse().unwrap();
//             let y = caps["y"].parse().unwrap();
//             let w = caps["width"].parse().unwrap();
//             let h = caps["height"].parse().unwrap();

//             let new_box = Box::new(x, y, w, h);
//             boxes.push(new_box);

//             width = max(width, (new_box.max_x + 1) as usize);
//             height = max(height, (new_box.max_y + 1) as usize);
//         }

//         // Problem 1
//         let mut overlaps: Vec<u8> = vec![0; width * height];

//         // Write boxes into overlaps buffer
//         for b in &boxes {
//             for row in b.min_y..=b.max_y {
//                 for col in b.min_x..=b.max_x {
//                     let idx = row as usize * width + col as usize;
//                     unsafe {
//                         let value = overlaps.get_unchecked_mut(idx);
//                         *value = value.saturating_add(1);
//                     }
//                 }
//             }
//         }

//         let overlapped = overlaps.iter().filter(|c| **c >= 2).count();
//         writeln!(&mut result, "Day 3, Problem 1 - [{}]", overlapped).unwrap(); // 100595

//         // Problem 2
//         let mut non_overlapped: Option<usize> = None;

//         // Iterate boxes
//         for (i, b) in boxes.iter().enumerate() {
//             let mut any_overlaps = false;

//             // Iterate tiles overlapped by box
//             'outer: for row in b.min_y..=b.max_y {
//                 for col in b.min_x..=b.max_x {
//                     let idx = row as usize * width + col as usize;

//                     // Check if tile has multiple overlaps
//                     let count = unsafe { overlaps.get_unchecked(idx) };
//                     if *count >= 2 {
//                         any_overlaps = true;
//                         break 'outer;
//                     }
//                 }
//             }

//             // Check if this box had zero overlaps
//             if !any_overlaps {
//                 non_overlapped = Some(i + 1);
//                 break;
//             }
//         }

//         writeln!(
//             &mut result,
//             "Day 3, Problem 2 - [{}]",
//             non_overlapped.unwrap()
//         )
//         .unwrap(); // 415

//         result
//     }
// }

pub mod day4 {

}