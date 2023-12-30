use std::fmt::Error;



struct Graph{
    nodes: Vec<Node>,
    position: (u32, u32),
    width: u32,
    height: u32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    
    fn get_node_by_position(&self, position: (u32,u32))->Option<(Node,usize)>{
        for i in 0..self.nodes.len(){
            if self.nodes[i].position == position{
                let selected_node = self.nodes[i].clone();
                return Some((selected_node,i));
            }
        }
        None
    }

    fn Dijkstra(self){
        let mut dist: Vec<u32> = vec![];
        let mut prev: Vec<u32> = vec![];
        let mut unvisited_nodes: Vec<Node>= vec![];
        let mut current_node_index: usize = 0;
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i].clone();
            dist.insert(i, 99999999);
            prev.insert(i,0);
            unvisited_nodes.push(*node);
        }
        dist[current_node_index] = 0;
        
        while unvisited_nodes.len() > 0 {
            let cur_node = self.nodes[current_node_index];
            let next_node_1 = self.get_node_by_position((cur_node.position.0 + 1, cur_node.position.1));
            let next_node_2 = self.get_node_by_position((cur_node.position.0, cur_node.position.1 + 1 ));            
            match next_node_1 {
                Some(n)=> dist[n.1] = n.0.weight,
                None=>()
            }
            match next_node_2 {
                Some(n)=> dist[n.1] = n.0.weight,
                None=>()
            }
            match unvisited_nodes.iter().position(|&x| x.position == cur_node.position){
                Some(position) => {
                    unvisited_nodes.remove(position);
                },
                None => ()
            } 
            if next_node_1.is_some() && next_node_2.is_some(){
                if next_node_1.unwrap().0.weight < next_node_2.unwrap().0.weight {
                    current_node_index = next_node_1.unwrap().1
                }else{
                    current_node_index = next_node_2.unwrap().1
                }
            }
        }

        dbg!(dist);
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

fn nodes_from_vector(node_map: Vec<Vec<u32>>)->Vec<Node>{
    let mut nodes: Vec<Node> = vec![];
    for y in 0..node_map.len(){
        for x in 0..node_map[y].len(){
            let weight = node_map[y][x];
            let node = Node::new((x.try_into().unwrap(),y.try_into().unwrap()),weight);
            nodes.push(node);
        }
    }
    return nodes;
}

#[cfg(test)]
mod tests{
    use crate::{split_input, nodes_from_vector, Graph, Node};

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

    #[test]
    fn test_graph_first(){
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

        let nodes = nodes_from_vector(output);
        let graph = Graph::new(nodes);

        let found_node = graph.get_node_by_position((0,0)).unwrap();
        let node = Node::new((0,0),2);
        
        assert_eq!(found_node.0, node);
        let found_node = graph.get_node_by_position((12,12)).unwrap();
        let node = Node::new((12,12),3);
        
        assert_eq!(found_node.0, node);

        
    }
    #[test]
    fn test_dijkstra(){
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

        let nodes = nodes_from_vector(output);
        let graph = Graph::new(nodes);

        graph.Dijkstra();

        
    }

    #[test]
    fn test_remove(){
        let mut num = vec![10, 20, 30, 40, 50];

        // remove [2] (30)
        num.remove(2);

        println!("num[0]: {}", num[0]);
        println!("num[1]: {}", num[1]);
        println!("num[2]: {}", num[2]);
        println!("num[3]: {}", num[3]);
    }
}
