mod algo;
mod graph;



fn main() {
    // let mut test_list1: [i32; 5] = [4, 3, 1, 4, 5];
    // let mut test_list2: [i32; 5] = [1, 2, 3, 4, 5];
    let mut test_list3: [i32; 5] = [5, 2, 3, 2, 1];

    // let mut tes_list_iter = test_list3.iter();
    // let mut previous = tes_list_iter.next();
    // // println!("first elem {:?}", first);
    // for (i, item) in tes_list_iter.enumerate(){
    //     if previous.unwrap() < item {
    //         println!("smaller");
    //     }
    //     println!("{:?}:{:?}", i, item);
    //     previous = Some(item);
    // }

    // for (mut current, mut next) in test_list3.iter().zip(test_list3.iter().skip(1)){
    //     println!("{:?}, {:?}", current, next);
    //     if current < next {
    //         println!("smaller");
    // //         let temp = current;
    // //         current = next;
    // //         next = temp;
    // //     }
    // // }

    // // println!("{:?}", test_list3);
    // let mut s = String::from("hello world");

    // let word = &s;

    s.clear(); // error!

    println!("the first word is: {}", word);
    // swap(list[0], list[1])

    // println!("test_list1 {:?}", test_list1);
    // println!("test_list2 {:?}", test_list2);
    // println!("test_list3 {:?}", test_list3);
    
    // algo::sorting::insertion_sort(&mut test_list1);
    // algo::sorting::insertion_sort(&mut test_list2);
    // algo::sorting::insertion_sort(&mut test_list3);

    // println!("test_list1 {:?}", test_list1);
    // println!("test_list2 {:?}", test_list2);
    // println!("test_list3 {:?}", test_list3);

//     let test_vertices: [graph::Node;5] = [graph::Node::new(1), graph::Node::new(2), graph::Node::new(3), graph::Node::new(4), graph::Node::new(5),];
//     let test_graph: graph::Graph =  graph::Graph::new(&test_vertices, true);
//     println!("{:?}", test_graph);

    // let mut test_graph = graph::DirectedUnweightedGraph::new();
    // // test_graph.add_node();
    // // test_graph.add_node();
    // // test_graph.add_node();
    // // test_graph.add_node();
    // test_graph.add_edge(1, 0);

    // let mut n1 = graph::Node::new(String::from("a"));
    // println!("{:?}", n1);
    // println!("{:?}", n1);
    // let y = println!("pukpuk");
    // println!("{y}");

}
