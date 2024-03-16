#[derive(Debug)]
pub struct Node{ // <'a>
    index: usize,
    // name: &'a str,
    // content: 
}

impl Node{ // <'_>
    pub fn new(index: usize) -> Node { // name: &str, 
        Node {
            index,
            // name,
        }
    }

    // pub fn get_name(&self) -> &str{
    //     self.name
    // }

    pub fn get_index(&self) -> usize{
        self.index
    }   
}

#[derive(Debug)]
pub struct DirectedUnweightedGraph{
    vertices: Vec<Node>,
    edges: Vec<Vec<usize>>,
    n_vert: usize,
}

impl DirectedUnweightedGraph{
    pub fn new()-> DirectedUnweightedGraph{
        DirectedUnweightedGraph{
            vertices: Vec::new(),
            edges: Vec::new(),
            n_vert: 0
        }
    }

    pub fn add_node(&mut self){
        self.vertices.push(Node::new(self.n_vert));
        self.edges.push(Vec::new());
        self.n_vert += 1;
    }

    pub fn add_edge(&mut self, index1: usize, index2: usize){
        self.edges[index1].push(index2);
    }
}