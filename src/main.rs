use std::fmt::Error;



struct Graph{
    nodes: Vec<Node>,
    position: (u32, u32),
    width: u32,
    height: u32
}

#[derive(Clone, Copy)]
struct Node{
    position: (u32,u32),
    weight: u32
}

impl Node{
    fn new(poistion: (u32, u32), weight: u32)->Self{

        Self{
            position: poistion,
            weight: weight
        }
    }
}

impl Graph{
    fn new(nodes: Vec<Node>)-> Self{
        Self{
            nodes: nodes,
            width: 13,
            height: 13,
            position: (0,0)
        }
    }

    fn get_node_by_position(&self, position: (u32,u32))->Result<Node, Error>{
        for i in 0..self.nodes.len(){
            if self.nodes[i].position == position{
                let selected_node = self.nodes[i].clone();
                return Ok(selected_node)
            }
        }
        panic!("No valid node found")
    }

    fn Dijkstra(&self){

    }
}

fn main() {
    println!("Hello, world!");
    const INPUT: &str = 
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let output = split_input(INPUT);
    dbg!(&output);
}

fn split_input(input: &str) -> Vec<Vec<u32>>{
    let lines = input.lines();
    let mut output = vec![];
    let mut node_line: Vec<u32> = vec![];
    const RADIX : u32 = 10;
    for (y, line) in lines.enumerate(){
        let node_line_char = line.chars();
        node_line = vec![];
        for (x, node) in node_line_char.enumerate(){
            /*println!("[{},{}] -> {}", x,y,node);
            println!("{}",(node as u32));*/

            //println!("Node = {}; Node as u32 = {}; Node as u32 = {}, to radix 10 = {} ", node, node as u32, node as u32, node.to_digit(RADIX).unwrap());
            node_line.insert(x, node.to_digit(RADIX).unwrap());
        }

        output.insert(y, node_line.clone());
    }
    return output;
}


#[cfg(test)]
mod tests{
    use crate::split_input;

    #[test]
    fn test_input(){

        const INPUT: &str = 
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        
        
        let output = split_input(INPUT);
        
        assert_eq!(output.len(), 13);
        
        for n in 0 .. output.len()-1{
            assert_eq!(output[n].len(), 13);
        }
    }


    fn test_graph_first(){
        
    }
}
