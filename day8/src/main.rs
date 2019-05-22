use std::error::Error;
use std::fs;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<()> {
   let mut input = fs::read_to_string("data/text.txt").unwrap();

    let mut flat = vec![];
    for number in input.split_whitespace() {
        flat.push(number.parse()?);
    }
   
    Ok(())
}



#[derive(Debug)]
struct Node {
    metadata: Vec<u32>,
    childern:     Vec<Node>,
    len :  usize,

}


impl Node {
    // 根据数据流来构造一棵树
    fn build_tree( stream: vec<u32>) -> Node {
        let(child_num, meta_num) = (stream[0],stream[1]);
        let mut node = Node {childern: vec::new()};

    // 首先构造顶端节点 A (metadata:Vec<i32> , childeren: Vec<node>)
        for n in 0..child_num {
            let child = Node:: build_tree(stream[node.len..]);
            node.childern.push(child);
        } 

        for m in meta_num {
            unimplemented!();
        }
    }

    fn sum_metadata(arg: Type) -> u32 {
        
    }
}