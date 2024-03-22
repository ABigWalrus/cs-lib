#[derive(Debug)]
pub struct Node{
    name: String,
    // children: Vec<&Node>,
}

impl Node{ // <'_>
    pub fn new(name: &str) -> Node { // name: &str, 
        Node {
            name: String::from(name),
            // children: Vec::new(),
        }
    }

    // pub fn get_name(&self) -> String{
    //     self.name
    // }

    // pub fn add_child(&mut self, node: &Node){
    //     self.children.push(node);
    // }
}

// fn Graph(name: String, directed: bool, unweighted: bool){
//     if directed {
//         return 12;
//     }else{
//         return "hdh";
//     }
// }

#[derive(Debug)]
pub struct DirectedUnweightedGraph{
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    n_vert: usize,
}

impl DirectedUnweightedGraph{
    pub fn new()-> DirectedUnweightedGraph{
        DirectedUnweightedGraph{
            nodes: Vec::new(),
            edges: Vec::new(),
            n_vert: 0
        }
    }

    pub fn add_node(&mut self, node: Node){
        self.nodes.push(node);
        self.edges.push(Vec::new());
        self.n_vert += 1;
    }

    pub fn add_edge(&mut self, index1: usize, index2: usize){
        self.edges[index1].push(index2);
    }
}
