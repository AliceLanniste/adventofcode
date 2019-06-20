use std::io::{self, Write};

static Grid_NUM: i32 = 8561;
static CELL_SIZE: i32 = 2;
fn main() {
    let mut cell = Cell::new(300);
     for x in 1..=300 {
      for y in 1..=300 {
          cell.set(x, y, calculate_fuel(x, y));
      }
    }
    
   
    let (WIDTH,HEIGHT) =(CELL_SIZE as u32, CELL_SIZE as u32);

    //  let mut summed_area: Vec<i32> = vec![0; WIDTH * HEIGHT];
    let mut summed_area:Vec<Vec<i32>> =vec![vec![0;  WIDTH as usize]; HEIGHT as usize];
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let idx = row * WIDTH + col;
            let a = cell.get(row,col);
            println!("a is {}",a);
            let b = if col > 0 { summed_area[row as usize][(col-1) as usize]} else { 0 }; 
            println!("b is {}",b);
  
            let c = if row > 0 { summed_area[(row-WIDTH) as usize][col as usize] } else { 0 };
            println!("c is {}",c);

            let d = if row > 0 && col > 0 {
                summed_area[(row-WIDTH) as usize][(col-1) as usize]
            } else {
                0
            };
            println!("d is {}",d);

            let summed_power = a + b + c - d;
            summed_area[row as usize][col as usize] = summed_power;
            // println!("{},{}",idx, summed_area[idx]);
           
        }
    }


    
            
     
}


// fn base(arg: Type) -> RetType {
//     let mut best_cell_overall = (0, 0, 0);
//             let mut best_power_overall = 0;
    
//             let k_max = WIDTH;
//             for k in 1..=k_max {
//                 let mut best_power = std::i32::MIN;
//                 let mut best_cell = (0, 0);
//                 let offset = k - 1;
    
//                 for row in 0..HEIGHT - offset {
//                     for col in 0..WIDTH - offset {
//                         // Rust + rustfmt makes this annoyingly verbose. C++ ternary would be so clean.
//                         let a = if row > 0 && col > 0 {
//                             summed_area[(row - 1) * WIDTH + (col - 1)]
//                         } else {
//                             0
//                         };
//                         let b = if row > 0 {
//                             summed_area[(row - 1) * WIDTH + (col + offset)]
//                         } else {
//                             0
//                         };
//                         let c = if col > 0 {
//                             summed_area[(row + offset) * WIDTH + (col - 1)]
//                         } else {
//                             0
//                         };
//                         let d = summed_area[(row + offset) * WIDTH + (col + offset)];
    
//                         let sum = a - b - c + d;
    
//                         if sum > best_power {
//                             best_power = sum;
//                             best_cell = (col, row);
//                         }
//                     }
//                 }
    
//                 if k == 3 {
//                     // Answer: 235, 31
//                     writeln!(
//                        io::stdout(),
//                         "Day 11, Problem 1 - [{}, {}]",
//                         best_cell.1+ 1,
//                         best_cell.0 + 1
//                     )
//                     .unwrap();
//                 }
    
//                 if best_power > best_power_overall {
//                     best_power_overall = best_power;
//                     best_cell_overall = (best_cell.0, best_cell.1, k);
//                 }
//             }
    
//             // Answer: 241, 65, 10
//             writeln!(
//                 io::stdout(),
//                 "Day 11, Problem 2 - [{}, {}, {}]",
//                 best_cell_overall.1 + 1,
//                 best_cell_overall.0 + 1,
//                 best_cell_overall.2
//             )
//             .unwrap();
    
// }

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