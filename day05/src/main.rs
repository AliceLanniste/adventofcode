/**
 * day 5 相邻的元素如果是同类但相反的则会起反应，如 `aA`起反应于是变成``
 * part1 ：把`String` 转成 vec<u8>,然后不断循环比较相邻两个元素，如果达成上述反应，则把这两个元素
 * 跳过
 *  */ 

use std::fs;
use std::mem;
use std::io::{self,Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;


fn main() -> Result<()>{
    let input = fs::read_to_string("data/day5.txt").unwrap();
    let input = input.trim();
    
    // part1(input)?;
    part2(input)?;

    Ok(())

}

fn part1(polymer: &str) -> Result<()> {
    writeln!(io::stdout(), "inert length: {}", react(polymer).len())?;
    Ok(())
}

// 1.首先得按26个字母依次删除同类，然后使用react函数，得到反应后，对比然后得到最小
fn part2(polymer: &str) -> Result<()> {
    let mut first = polymer.len();

    for i in b'A'..=b'Z' {
        let t1 = i as char;
        let t2 = (i+32) as char;
        let cleaned = polymer.replace(t1,"").replace(t2,"");
        let clean_str = react(&cleaned);
        if clean_str.len() < first {
            first = clean_str.len();
        }
    }

     writeln!(io::stdout(), "shortest length: {}", first)?;

    Ok(())
}

fn react(polymer: &str) -> String {
    let mut polymer_vec = polymer.as_bytes().to_vec();
    let mut leave = vec![];
   
  
        loop {
            let mut is_same = false;
            let mut i = 1;
            while i < polymer_vec.len() {
            if same_type(polymer_vec[i-1], polymer_vec[i]) {
                is_same = true;
                //跳过元素
                i += 2;
                continue;
            }
            leave.push(polymer_vec[i-1]);
            i += 1;
        }

        if i == polymer_vec.len() {
            
            leave.push(polymer_vec[i-1]);
        }

        mem::swap(&mut polymer_vec, &mut leave);
        leave.clear();

     
        if !is_same {
            
            break;
        }
    }

    String::from_utf8(polymer_vec).unwrap()
}


fn same_type(t1:u8, t2:u8) -> bool {
  if t1 < t2 {
      t2 - t1 == 32
  } else {
      t1 - t2 == 32
  }
   
}
