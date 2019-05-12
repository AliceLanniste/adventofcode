#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::io::{self, Write};
use std::fs;
use std::str::FromStr;
use std::error::Error;
use std::collections::{HashMap, HashSet};


type Result<T> = std::result::Result<T,Box<Error>>;
type RequiredFor = HashMap<char, HashSet<char>>;

fn main() ->Result<()>{
   let input = fs::read_to_string("data/day7.txt").unwrap();

    let mut instructions:Vec<Instruction> = vec![];

    for line in input.lines() {
        let ins = line.parse().expect("failed parse");
        instructions.push(ins);
    }

    let mut required_for: RequiredFor = HashMap::new();

    for dep in &instructions {
        required_for.entry(dep.cur).or_default().insert(dep.last);
        required_for.entry(dep.last).or_default();
    }
    

    part1(&required_for)?;
    Ok(())
}


fn part1(require: &RequiredFor) -> Result<()> {
    let mut taken : HashSet<char>= HashSet::new();
    let mut order: Vec<char> = vec![];
let mut next: Vec<char> = vec![];

    loop {
    next_step(require, &taken, &taken, &mut next);
    let next_step = match next.pop(){
        None => break,
        Some(s) => s,
    };
        taken.insert(next_step);
        order.push(next_step);
    }

    let answer: String = order.iter().cloned().collect();
     writeln!(io::stdout(), "step order: {}", answer)?;
    Ok(())
}


// 
fn next_step(required: &RequiredFor, taken: &HashSet<char>,
            done: &HashSet<char>, steps: &mut Vec<char>)
{
    for(&step, ins) in required {
        if taken.contains(&step)
        {
            continue;
        }

        if ins.iter().all(|e| done.contains(e)) {
            steps.push(step);
        }
    }
    steps.sort();
    steps.dedup();
    steps.reverse();
  
}

/* 
part 2
*/ 
#[derive(Debug)]
struct Worker {
    status: vec<Status>,
}

#[derive(Debug)]
enum Status {
    Idle,
    Working {name: char, time: u32}
}

impl Worker {
 fn new(count: usize) -> Self {
     Worker {status: vec![Status::Idle; count]}
 }

    fn available(&self) -> Vec<usize> {
        let mut idles = vec![];
        for (worker, &status) in self.status {
            if status == Status::include_str! {
                idles.push(worker);
                }
          }
            idles
        }

    fn all_idle(&self) -> bool {
        self.status.iter().all(|s| *s == Status::Idle)

    }

    fn work_on(&mut self, worker:usize, step:char) -> RetType {
        let status = &mut self.status[worker];
        let time =(step as u32) -b'A' as u32 + 1 + 60;

        *status = Status::Working{step, time}
    }

    fn run_one(&mut self, order: &mut Vec<char>, done: &mut HashSet<char>) -> RetType {
        for w in 0..self.status.len()  {
            let mut is_done = false;
            match self.status[w] {
                Status::Idle => {}
                Status::Working{name, ref mut time} => {
                    *time -=1;
                    if *time == 0 {
                        is_done = true;
                        order.push(name);
                        done.insert(name);
                    }
                }
            }

            if is_done {
                self.status[w] = Status::Idle;
            }
        }
    }
}

/* 
 part 1
*/ 
#[derive(Debug)]
struct Instruction {
    cur: char,
    last:  char,
}

impl FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
              r"Step ([A-Z]) must be finished before step ([A-Z]) can begin."
            ).unwrap();
        }

        let caps = RE.captures(s).expect("failed to regex");

        Ok(Instruction {
            cur: caps[2].as_bytes()[0] as char,
            last: caps[1].as_bytes()[0] as char,
        })
    }
}