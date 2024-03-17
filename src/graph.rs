#[derive(Debug)]
pub struct Node{ // <'a>
    index: usize,
    name: String,
    children: Vec<Node>,
}

impl Node{ // <'_>
    pub fn new(_name: String) -> Node { // name: &str, 
        Node {
            index: 0,
            name: _name,
            children: Vec::new(),
        }
    }

    // pub fn get_name(&self) -> String{
    //     self.name
    // }

    pub fn set_index(&mut self, new_index: usize){
        self.index = new_index;
    }

    pub fn get_index(&self) -> usize{
        self.index
    }

    pub fn add_cild(&mut self, node: Node){
        self.children.push(node);
    }
}

#[derive(Debug)]
pub struct DirectedUnweightedGraph{
    vertices: Vec<(Node, usize)>,
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

    // pub fn add_node(&mut self){
    //     self.vertices.push(Node::new(self.n_vert));
    //     self.edges.push(Vec::new());
    //     self.n_vert += 1;
    // }

    pub fn add_edge(&mut self, index1: usize, index2: usize){
        self.edges[index1].push(index2);
    }
}