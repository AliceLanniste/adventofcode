
mod lib;

fn main() {
    
   
    let days =  [ lib::day1::run ,
                lib::day2::run ];
    
   let result = lib::day3::run();
   println!("{}", result);

}




