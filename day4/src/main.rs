#[macro_use]
extern crate lazy_static;
extern crate regex;

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
    let mut cur_guardID = None;
    for e in events {
        if let EventKind::StartShift{ id } = e.eventKind {
            cur_guardID = Some(id);
        }

        if let Some(id) = cur_guardID {
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
     Ok(())
}

fn part1(arg: Type) -> RetType {
    unimplemented!();
}

// fn part2(arg: Type) -> RetType {
    // unimplemented!();
// }

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



// #[macro_use]
// extern crate lazy_static;
// extern crate regex;

// use std::collections::HashMap;
// use std::error::Error;
// use std::io::{self, Read, Write};
// use std::ops::Range;
// use std::result;
// use std::slice;
// use std::str::FromStr;
// use std::fs;
// use regex::Regex;

// macro_rules! err {
//     ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
// }

// type Result<T> = result::Result<T, Box<Error>>;

// fn main() -> Result<()> {
//     // let mut input = String::new();
//     // io::stdin().read_to_string(&mut input)?;
//     let input = fs::read_to_string("data/day4.txt").unwrap();

//     // collect events
//     let mut events: Vec<Event> = vec![];
//     for line in input.lines() {
//         let event = line.parse().or_else(|err| {
//             err!("failed to parse '{:?}': {}", line, err)
//         })?;
//         events.push(event);
//     }
//     if events.is_empty() {
//         return err!("found no events");
//     }

    // sort them by time and group them by guard
    // events.sort_by(|ev1, ev2| ev1.datetime.cmp(&ev2.datetime));
    // let mut events_by_guard = EventsByGuard::new();
    // let mut cur_guard_id = None;
    // for ev in events {
    //     if let EventKind::StartShift { guard_id } = ev.kind {
    //         cur_guard_id = Some(guard_id);
    //     }
    //     match cur_guard_id {
    //         None => return err!("no guard id set for event"),
    //         Some(id) => {
    //             events_by_guard.entry(id).or_default().push(ev);
    //         }
    //     }
    // }

    // // create a by-minute frequency map for each guard
    // let mut minutes_asleep: GuardSleepFrequency = HashMap::new();
    // for (&guard_id, events) in events_by_guard.iter() {
    //     // let mut freq = HashMap::new();
    //     for result in MinutesAsleepIter::new(events) {
    //         let u:u32 = result?;
    //         // for minute in result? {
    //         //     println!("{:?}", minute);
    //         //     // *freq.entry(minute).or_default() += 1;
    //         // }
            
    //     }
    //     // minutes_asleep.insert(guard_id, freq);
    // }

//     part1(&minutes_asleep)?;
//     part2(&minutes_asleep)?;
//     Ok(())
// }

// fn part1(minutes_asleep: &GuardSleepFrequency) -> Result<()> {
//     let (&sleepiest, _) = minutes_asleep
//         .iter()
//         .max_by_key(|&(_, ref freqs)| -> u32 {
//             freqs.values().cloned().sum()
//         })
//         // unwrap is OK since we're guaranteed to have at least one event
//         .unwrap();
//     let minute = match sleepiest_minute(minutes_asleep, sleepiest) {
//         None => return err!("guard {} was never asleep", sleepiest),
//         Some(minute) => minute,
//     };

//     writeln!(io::stdout(), "part 1, product: {}", sleepiest * minute)?;
//     Ok(())
// }

// fn part2(minutes_asleep: &GuardSleepFrequency) -> Result<()> {
//     let mut sleepiest_minutes: HashMap<GuardID, (u32, u32)> = HashMap::new();
//     for (&guard_id, freqs) in minutes_asleep.iter() {
//         let minute = match sleepiest_minute(minutes_asleep, guard_id) {
//             None => continue,
//             Some(minute) => minute,
//         };
//         let count = freqs[&minute];
//         sleepiest_minutes.insert(guard_id, (minute, count));
//     }
//     if sleepiest_minutes.is_empty() {
//         return err!("no guards slept");
//     }

//     let (&longest_asleep, &(minute, _)) = sleepiest_minutes
//         .iter()
//         .max_by_key(|&(_, (_, count))| count)
//         // unwrap is OK because sleepiest_minutes is non-empty
//         .unwrap();

//     writeln!(io::stdout(), "part 2, product: {}", longest_asleep * minute)?;
//     Ok(())
// }

// /// Return the minute that the given guard slept the most.
// fn sleepiest_minute(
//     minutes_asleep: &GuardSleepFrequency,
//     guard_id: GuardID,
// ) -> Option<u32> {
//     minutes_asleep[&guard_id]
//         .iter()
//         .max_by_key(|&(_, freq)| freq)
//         .map(|(&minute, _)| minute)
// }

// type GuardID = u32;

// type EventsByGuard = HashMap<GuardID, Vec<Event>>;

// // maps guard to minutes asleep frequency
// type GuardSleepFrequency = HashMap<GuardID, HashMap<u32, u32>>;

// /// An iterator that coalesces "asleep" and "wakeup" events into ranges of
// /// minutes slept.
// #[derive(Debug)]
// struct MinutesAsleepIter<'a> {
//     events: slice::Iter<'a, Event>,
//     fell_asleep: Option<u32>,
// }

// impl<'a> MinutesAsleepIter<'a> {
//     fn new(events: &'a [Event]) -> MinutesAsleepIter<'a> {
//         MinutesAsleepIter { events: events.iter(), fell_asleep: None }
//     }
// }

// impl<'a> Iterator for MinutesAsleepIter<'a> {
//     type Item = Result<Range<u32>>;

//     fn next(&mut self) -> Option<Result<Range<u32>>> {
//         loop {
//             let ev = match self.events.next() {
//                 Some(ev) => ev,
//                 None => {
//                     if self.fell_asleep.is_some() {
//                         return Some(err!("found sleep event without wake up"));
//                     }
//                     return None;
//                 }
//             };
//             match ev.kind {
//                 EventKind::StartShift { .. } => {}
//                 EventKind::Asleep => {
//                     self.fell_asleep = Some(ev.datetime.minute);
//                 }
//                 EventKind::WakeUp => {
//                     let fell_asleep = match self.fell_asleep.take() {
//                         Some(minute) => minute,
//                         None => {
//                             return Some(err!("found wakeup without sleep"));
//                         }
//                     };
//                     if ev.datetime.minute < fell_asleep {
//                         return Some(err!("found wakeup before sleep"));
//                     }
//                     return Some(Ok(fell_asleep..ev.datetime.minute));
//                 }
//             }
//         }
//     }
// }

// #[derive(Debug)]
// struct Event {
//     datetime: DateTime,
//     kind: EventKind,
// }

// #[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
// struct DateTime {
//     year: u32,
//     month: u32,
//     day: u32,
//     hour: u32,
//     minute: u32,
// }

// #[derive(Debug)]
// enum EventKind {
//     StartShift { guard_id: GuardID },
//     Asleep,
//     WakeUp,
// }

// impl FromStr for Event {
//     type Err = Box<Error>;

//     fn from_str(s: &str) -> Result<Event> {
//         lazy_static! {
//             static ref RE: Regex = Regex::new(r"(?x)
//                 \[
//                     (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
//                     \s+
//                     (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
//                 \]
//                 \s+
//                 (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))
//             ").unwrap();
//         }

//         let caps = match RE.captures(s) {
//             None => return err!("unrecognized event"),
//             Some(caps) => caps,
//         };
//         let datetime = DateTime {
//             year: caps["year"].parse()?,
//             month: caps["month"].parse()?,
//             day: caps["day"].parse()?,
//             hour: caps["hour"].parse()?,
//             minute: caps["minute"].parse()?,
//         };
//         let kind =
//             if let Some(m) = caps.name("id") {
//                 EventKind::StartShift { guard_id: m.as_str().parse()? }
//             } else if &caps["sleep"] == "falls asleep" {
//                 EventKind::Asleep
//             } else if &caps["sleep"] == "wakes up" {
//                 EventKind::WakeUp
//             } else {
//                 return err!("could not determine event kind")
//             };
//         Ok(Event { datetime, kind })
//     }
// }