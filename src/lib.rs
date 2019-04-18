

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
  extern crate regex;

  use regex::Regex;

  use std::str::FromStr;
  use std::num::ParseIntError;
  use std::fs;

  type Grid = HashMap<(u32,u32), u32>;

    pub fn run() {
      
      let mut Pieces: Vec[piece] = vec![];
      let file_strings = fs::read_to_string("data/day3.txt")?;
      let lines = file_strings.lines();

      for line in &lines {
          let p = line.parse();
      }

    }

    fn part1(arg: Type) -> RetType {
        // unimplemented!();
       
    }

    #[derive(Debug)]
    struct piece {
        x: u32,
        y: u32,
        width: u32,
        height: u32
    }

    impl FromStr for pieces {
        type Err = ParseIntError;
        
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new(r"#\d+ @ (?P<x>\d+),
                                        (?P<y>\d+):
                                        (?P<w>\d+)x(?P<h>\d+)")?;
            let groups =  re.capture(s).exepct("Not found match ");

            Ok(piece {
              x: &groups["x"],
              y: &groups["y"],
              width: &groups["width"],
              height: &groups["height"]
            })
        }
    }
}

pub mod day4 {

}