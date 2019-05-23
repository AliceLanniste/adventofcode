/// day 7的part 1问题是有向图，从顶点到终点的要经过哪些，当遇到多条路径，按字母大小选择
/// part2的问题是现有5个通道，从顶点开始，依次安排5个通道，每个点运行的时间是('X' -'A' + 1)运行的时间总共是多少


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
    part2(&required_for)?;

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


fn part2(require: &RequiredFor) -> Result<()> {
    let mut workers = Worker::new(5);
    let mut gg :HashSet<char> = HashSet::new();
    let mut done: HashSet<char> = HashSet::new();
    let mut store: Vec<char> = vec![];
    let mut next: Vec<char> = vec![];

    let mut second  = 0;
    loop {
        workers.run_one(&mut store, &mut done);

        next_step(require, &gg, &done, &mut next);

        if next.is_empty() && workers.all_idle() {
            break;
        }
        for worker in workers.available() {
            let next_step = match next.pop() {
                Some(next_step) => next_step,
                None => break,
            };
            gg.insert(next_step);
            workers.work_on(worker, next_step);
        }
        second += 1
    }

    let answer: String = store.iter().cloned().collect();
    writeln!(io::stdout(), "step order (part 2): {}", answer)?;
writeln!(io::stdout(), "total seconds: {}", second)?;
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
    status: Vec<Status>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Idle,
    Working {name: char, time: u32}
}

impl Worker {
 fn new(count: usize) -> Self {
     
    //  the trait std::clone::Clone is not implemented
     Worker {status: vec![Status::Idle; count]}  

 }

    fn available(&self) -> Vec<usize> {
        let mut idles = vec![];
        for (worker, &status) in self.status.iter().enumerate() {
            if status == Status::Idle {
                idles.push(worker);
            }
          }
            idles
        }

    fn all_idle(&self) -> bool {
        // cannot be applied to type'Status'
        self.status.iter().all(|s| *s == Status::Idle)

    }

    fn work_on(&mut self, worker:usize, step:char) {
        let status = &mut self.status[worker];
        let time =(step as u32) -b'A' as u32 + 1 + 60;

        *status = Status::Working{name:step, time}
    }

    fn run_one(&mut self, order: &mut Vec<char>, done: &mut HashSet<char>)  {
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