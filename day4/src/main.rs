#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{Write,self};
use std::ops::Range;
use std::slice;
use std::result;
use regex:: Regex;
use std::str::FromStr;
use std::error::Error;
use std::fs;
use std::collections::HashMap;

type Result<T> = result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

fn main() ->Result<()> {
    let input = fs::read_to_string("data/text.txt").unwrap();
    let mut events:Vec<Event> = vec![];
    
    for line in input.lines() {
        let event = line.parse().unwrap();
        events.push(event);
    }

    events.sort_by(|ev1, ev2| ev1.dateTime.cmp(&ev2.dateTime));

    // 依照guardID把所有条目归类
    let mut events_by_guard:HashMap<u32,Vec<Event>> = HashMap::new();
    let mut cur_guard_id = None;
    for e in events {
        if let EventKind::StartShift{ id } = e.eventKind {
            cur_guard_id = Some(id);
        }

        if let Some(id) = cur_guard_id {
            events_by_guard.entry(id).or_default().push(e);
        }
    }

    // 过滤出sleep-wake的所有时间段
    let mut sleeptime :HashMap<u32,HashMap<u32,u32>> = HashMap::new();
    for (id, events) in events_by_guard.iter() {
        let mut counts :HashMap<u32,u32> = HashMap::new();
        for result in IterEvent::new(events) {
             
            for minute in result? {
                *counts.entry(minute).or_default() +=1;
            }
            
        }
        sleeptime.insert(*id,counts);

       
    }
    part1(&sleeptime)?;
    Ok(())
}
type TimeRange = HashMap<u32,HashMap<u32,u32>>;

fn part1(time_r: &TimeRange) -> Result<()> {
    let (&guard,_) = time_r.iter()
                        .max_by_key(|&(_,   freqs)|->u32 {
                                freqs.values().cloned().sum()
                        }).unwrap();
    
    let minute = match guard_minute(time_r, guard) {
        None => return err!("guard {} was never asleep", guard),
        Some(minute) => minute,
    };

    writeln!(io::stdout(), "part 1, product: {}", guard*minute )?;
    Ok(())
}

fn guard_minute(
    minutes_asleep: &TimeRange,
    guard_id: u32,
) -> Option<u32> {
    minutes_asleep[&guard_id]
        .iter()
        .max_by_key(|&(_, freq)| freq)
        .map(|(&minute, _)| minute)
}

#[derive(Debug)]
struct IterEvent <'a>{
    events: slice::Iter<'a, Event>,
    temp:Option<u32>,
}

impl<'a> IterEvent<'a> {
    fn new(events: &'a [Event]) -> IterEvent<'a> {
        IterEvent{events: events.iter(), temp:None}
    }

}

impl<'a> Iterator for IterEvent<'a> {
    type Item = Result<Range<u32>>;
     
    fn next(&mut self) -> Option<Self::Item> {
        loop {
        let  ev = match self.events.next() {
             Some(ev) => ev,
             None => return None,
            };   

        
        match ev.eventKind {
            EventKind::StartShift{ .. } => { }
            EventKind::Sleep => { self.temp = Some(ev.dateTime.minute);  }
            EventKind::WakeUp => {
                let start = match self.temp.take() {
                     Some(minute) => minute,
                        None => {
                            return Some(err!("found wakeup without sleep"));
                        }
                };
                    

                if ev.dateTime.minute < start {
                    return Some(err!("found wakeup before sleep"));
                }
                return Some(Ok(start..ev.dateTime.minute));    
            }
        }

      }
    }   //next scope
}


#[derive(Debug)]
struct Event {
    eventKind: EventKind,
    dateTime: Datetime
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Datetime {
    year: u32,
    month: u32,
    day:   u32,
    hour:  u32,
    minute: u32
}

#[derive(Debug)]
enum EventKind {
    StartShift{id: u32},
    Sleep,
    WakeUp
}

impl FromStr for Event {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
            \[
                (?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+)
                \s+
                (?P<hour>\d+):(?P<minute>\d+)
            \]
             \s+
            (?:Guard\ \#(?P<id>\d+)\ begins\ shift|(?P<sleep>.+))
            ").unwrap();
        }

        let caps = RE.captures(s).expect("failed to regex");

        let datetime = Datetime {
            year:  caps["year"].parse()?,
            month: caps["month"].parse()?,
            day:   caps["day"].parse()?, 
            hour:  caps["hour"].parse()?, 
            minute:caps["minute"].parse()?,
        };

        // 获取Kind，Kind有三种1.begin Shift 2.sleep 3.wake up
        let kind =  
                if let Some(k) = caps.name("id") {
                     EventKind::StartShift{id: k.as_str().parse()?}
                } else if &caps["sleep"] =="falls asleep" {
                    EventKind::Sleep

                } else if &caps["sleep"] == "wakes up" {
                    EventKind::WakeUp
                } else {
                    return err!("could not determine event kind")
                };

        Ok(Event {eventKind: kind, dateTime:datetime})        
    }
}




  