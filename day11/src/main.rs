use std::error::Error;
use std::io::{self, Write};

type Result<T> = std::result::Result<T,Box<Error>>;
static GRID_NUMBER: i32 = 8561;

fn main() -> Result<()>{
  let mut cell = Cell::new(300);

  for x in 1..=300 {
      for y in 1..=300 {
          cell.set(x, y, calculate_fuel(x, y));
      }
      
  }

    part1(&cell)?;
    Ok(())
}



fn part1(cell: &Cell) -> Result<()> {
   let (mut max_left_x,mut max_left_y,mut max_power)=(-1,-1, std::i32::MIN);
   for x in 1..=300 {
       for y in 1..=300 {
           let power = cell.square_power(x, y);
           if power > max_power {
               max_left_x = x;
               max_left_y =y;
               max_power = power;
           }
       }
   }

 writeln!(io::stdout(), "most powerful 3x3 square: {},{}", max_left_x, max_left_y)?;
   Ok(())
}

part2(cell: &Cell) -> Result<()>
{
    
}


#[derive(Debug)]
struct Cell {
    power: Vec<Vec<i32>>
}


impl Cell {
    fn new(size: i32) -> Self {
        Cell{power: vec![vec![0; size as usize]; size as usize]}
    }

    fn get(&self, x:i32, y:i32) -> Option<i32> {
        let(x,y) = (x-1,y-1);
         if 0 <= x && x < 300 && 0 <= y && y < 300 {
            Some(self.power[x as usize][y as usize])
        } else {
           None
        }
    }

    fn set(&mut self, x:i32, y:i32,power:i32)  {
        self.power[x as usize -1][y as usize -1] = power;
    }

    fn square_power(&self,left_x:i32,left_y:i32) -> i32 {
        let mut power = 0;

        for x in left_x..left_x + 3 {
            for y in left_y..left_y+3 {
                power += self.get(x,y).unwrap_or_default();
            }
        }

        power
    }

}

fn calculate_fuel(x:i32, y:i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += GRID_NUMBER;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;
    power
}