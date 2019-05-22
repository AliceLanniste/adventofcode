use std::fs;

// 466 players; last marble is worth 71436 points

fn main() {
    

}


#[derive(Debug)]
struct Player {
    points: u32,
}


#[derive(Debug)]
struct Circle {
    marbles: Vec<Marble>,
    current: usize,
}

#[derive(Debug)]
struct Marble {
    value: u32,
    prev: usize,
    next:  usize,
}

impl Circle {
    fn new() -> Circle {
        let first = Marble {value: 0, prev: 0,next:0 };

        Circle {marbles: vec![first], current: 0}
    }

    fn play(&mut self, player: &mut Player, value: u32)  {
        let marble_id = self.add_marble(value);
        if value % 23 != 0 {
            let insert = self.clockwise(1);
            self.insert_after(marble_id, insert_at);
            self.current = marble_id;
        }
    }

    fn add_marble(&mut self, value: u32)  {
        let id = self.marbles.len();
        self.marbles.push(Marble::new(value));
        id
    }

    fn insert_after(&mut self, to: usize,after:usize)  {
        let temp = self.marbles[after].next;
        self.marbles[after].next = to;
        self.marbles[temp].prev = to;
        self.marbles[to].prev = after;
        self.marbles[to].next = temp;
        
    }

    fn remove(&mut self,id: usize)  {
        let(prev, next) = (self.marbles[id].prev, self.marbles[id].next);
        self.marbles[id].prev = next;
        self.marbles[id].next = prev;
    }


    fn clockwise(&mut self, mut i: usize) -> usize {
        let mut id = self.current;
        while i > 0 {
            id = self.marbles[id].next;
            i- = 1;
        }

        id
    }


    fn counter_clockwise(&mut self, mut i: usize) -> usize {
        let mut id = self.count;
        while  i > 0 {
            id = self.marbles[id].prev;
            i- = 1;
        }

        id
    }
}

impl Marble {

    fn new() -> Self {6
        Marble {value , prev: 0, next: 0}
    }
}