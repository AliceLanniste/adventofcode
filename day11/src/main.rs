use std::io::{self, Write};

static GRID_NUM: i32 = 8561;

fn main() {
    let mut cell = Cell::new(300);
     for x in 1..=300 {
      for y in 1..=300 {
          cell.set(x, y, calculate_fuel(x, y));
      }
    }
    
    cell.summed_power();
    
    
    part1(&cell);
    part2(&cell);
     
}

fn part1(cell:&Cell)  {
  let (x,y,size)=cell.square_power(3);
   writeln!(
            io::stdout(),
            "Day 11, Problem 1 - [{}, {}]",
            x+ 1,
            y + 1
        )
        .unwrap();  
}

fn part2(cell:&Cell)  {
    let (x,y,size)=cell.square_power(300);
   writeln!(
            io::stdout(),
            "Day 11, Problem 2 - [{}, {}, {}]",
            x+ 1,
            y + 1,
            size
        )
        .unwrap();  
}

#[derive(Debug)]
struct Cell {
    power: Vec<Vec<i32>>
}


impl Cell {
    fn new(size: i32) -> Self {
        Cell{power: vec![vec![0; size as usize]; size as usize]}
    }

    fn get(&self, x:u32, y:u32) -> i32 {
        // let(x,y) = (x-1,y-1);
        let (x,y) =(x as usize, y as usize);
         if x < 300 &&  y < 300 {
            self.power[x][y]
        } else {
           0
        }
    }

    fn set(&mut self, x:i32, y:i32,power:i32)  {
        self.power[x as usize -1][y as usize -1] = power;
    }

    fn summed_power(&mut self) {
        for row in 0..300 {
            for col in 0..300 {
                let a = self.get(row,col);
                let b = if col > 0 { self.power[row as usize][(col-1) as usize]} else { 0 }; 
    
                let c = if row > 0 { self.power[(row-1) as usize][col as usize] } else { 0 };

                let d = if row > 0 && col > 0 {
                    self.power[(row-1) as usize][(col-1) as usize]
                } else {
                    0
                };

                let summed_power = a + b + c - d;
                self.power[row as usize][col as usize] = summed_power;
           
            }
        }

    }    

        fn square_power(&self, k_max:u32) -> (u32,u32,u32) {
             let mut best_power = std::i32::MIN;
             let mut best_cell = (0,0,0);
            for k in 1..=k_max {
               
                let offset = k - 1;
    
                for row in 0..300 - offset {
                    for col in 0..300 - offset {
                        let a = if row > 0 && col > 0 {
                            self.power[(row - 1) as usize][(col - 1) as usize]
                        } else {
                            0
                        };
                        let b = if row > 0 {
                            self.power[(row - 1) as usize][(col + offset) as usize]
                        } else {
                            0
                        };
                        let c = if col > 0 {
                            self.power[(row + offset) as usize][ (col - 1) as usize]
                        } else {
                            0
                        };
                        let d = self.power[(row + offset) as usize][ (col + offset) as usize];
    
                        let sum = a - b - c + d;
    
                        if sum > best_power {
                            best_power = sum;
                            best_cell = (row, col,k);      
                        }
                    }
                }

               
            }
                 best_cell
        }
    }



fn calculate_fuel(x:i32,y:i32) -> i32{
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += Grid_NUM;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;

    power
}