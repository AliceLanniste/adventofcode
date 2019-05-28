use std::error::Error;
use std::fs;
use std::result;
use std:: io::{self, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<Error>>;


fn main() -> Result<()> {
    
     let mut input = fs::read_to_string("data/text.txt").unwrap();

    let mut flat = vec![];
    for number in input.split_whitespace() {
       flat.push(number.parse::<i32>()?);
}    
    let tree = Node::build_tree(&flat);

    part1(&tree)?;
    Ok(())
}


fn part1(root: &Node) -> Result<()> {
 writeln!(io::stdout(), "{}", root.sum_metadata())?;
 Ok(())
}


#[derive(Debug)]
struct Node {
    metadata: Vec<i32>,
    childern:     Vec<Node>,
    len :  usize,

}


impl Node {
    // 根据数据流来构造一棵树
    fn build_tree( stream: &[i32]) -> Node {
        let(child_num, meta_num) = (stream[0],stream[1]);
        let mut node = Node {childern: Vec::new(),metadata: Vec::new(),len: 2};

    // 首先构造顶端节点 A (metadata:Vec<i32> , childeren: Vec<node>)
        for _ in 0..child_num {
            let child = Node:: build_tree(&stream[node.len..]);
            node.len +=child.len;
            node.childern.push(child);
        } 

        // 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
        // 先选10 11 12也就是 B的metadata

        for _ in 0..meta_num {
            let data = stream[node.len];
            node.metadata.push(data);
            node.len += 1;
        }

        node
    }

    // 构造顶点A， A有header2，首先node A 必须把B,C装进Node，Node B，NodeB的Node为0因此
    // 首先给NodeB装进metadata 10，11，12

    fn sum_metadata(&self) -> i32 {
        let mut sum = self.metadata.iter().cloned().sum();
        for child in &self.childern {
            sum += child.sum_metadata();
        }
        sum
    }
}
