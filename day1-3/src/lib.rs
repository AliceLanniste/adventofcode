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

    pub fn run() ->Vec<String>{
      
      let mut Pieces: Vec<Piece> = vec![];
      let file_strings = fs::read_to_string("data/day3.txt").unwrap();
      let lines = file_strings.lines();

      for line in lines {
          let p:Piece = line.parse().expect("failed to parse");
          Pieces.push(p);
      }
      

      let mut grid = Grid::new();
      for pie in &Pieces {
        for(x,y) in pie.iterPoint() {
          *grid.entry((x,y)).or_default() += 1;
        }
      }

      let part1 = part1(&grid);
      let part2 = part2(&grid, &Pieces);
      let result = vec![part1, part2];
      result
    }

    fn part1(grid:&Grid) -> String {
      let mut result = String::with_capacity(128);
      let count = grid.values().filter(|&&count| count > 1).count();
      writeln!(&mut result, "day 3,part 1 - {}",count);
    }

    fn part2(grid:&Grid, ps:&[Piece]) -> String {
      let mut result = String::with_capacity(128);
      for p in ps {
         if p.iterPoint().all(|i| grid[&i] == 1) {
            writeln!(&mut result, "day 3,part 2 - {}",p.id);
      
         }
      }
      result
    }

    #[derive(Debug)]
    struct Piece {
        id: u32,
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
             id: caps["id"].parse()?,
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
        self.py += 1;
        Some((x,y))
     }
   }
    
    
}

