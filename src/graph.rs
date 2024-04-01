#[derive(Debug)]
#[derive(Clone)]
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
}

pub fn new_directed_unweighted() -> DirectedUnweightedGraph {
    DirectedUnweightedGraph{
        nodes: Vec::new(),
        edges: Vec::new(),
        n: 0
    }
}

#[derive(Debug)]
pub struct DirectedUnweightedGraph{
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    n: usize,
}

impl DirectedUnweightedGraph{
    // pub fn new()-> DirectedUnweightedGraph{
    //     DirectedUnweightedGraph{
    //         nodes: Vec::new(),
    //         edges: Vec::new(),
    //         n_vert: 0
    //     }
    // }

    pub fn insert_node(&mut self, node: Node){
        self.nodes.push(node);
        self.edges.push(Vec::new());
        self.n += 1;
    }


    /// NOT RECOMMENDED
    pub fn add_node(&mut self, node: &Node){
        self.nodes.push((*node).clone());
        self.edges.push(Vec::new());
        self.n += 1;
    }
    
    // pub fn add_nodes(&mut self, nodes: impl Iterator<Item = &Node>){
    //     for node in nodes {
    //         self.nodes.push(node);
    //         self.edges.push(Vec::new());
    //         self.n += 1;
    //     }
    // }

    pub fn add_edge(&mut self, index1: usize, index2: usize){
        self.edges[index1].push(index2);
    }
}

#[derive(Debug)]
pub struct DirectedWeightedGraph{
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    n: usize,
}

#[derive(Debug)]
pub struct UndirectedUnweightedGraph{
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    n: usize,
}

#[derive(Debug)]
pub struct UndirectedWeightedGraph{
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    n: usize,
}