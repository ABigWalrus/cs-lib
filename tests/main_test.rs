#[cfg(test)]
mod simple_int_list_sorting{
    use cs_lib::algo::sorting;

    #[test]
    fn is_sorted_test() {
        let mut test_list1: [i32; 5] = [1, 2, 3, 4, 5];
        let mut test_list2: [i32; 5] = [4, 3, 1, 4, 5];
        let mut test_list3: [i32; 5] = [5, 2, 3, 2, 1];

        assert!(sorting::is_sorted(&test_list1));
        assert!(!sorting::is_sorted(&test_list2));
        assert!(!sorting::is_sorted(&test_list3));
    }

    #[test]
    fn bubble_sort_test() {
        let mut test_list1: [i32; 5] = [1, 2, 3, 4, 5];
        let mut test_list2: [i32; 5] = [4, 3, 1, 4, 5];
        let mut test_list3: [i32; 5] = [5, 2, 3, 2, 1];

        sorting::bubble_sort(&mut test_list1);
        sorting::bubble_sort(&mut test_list2);
        sorting::bubble_sort(&mut test_list3);

        assert!(sorting::is_sorted(&test_list1));
        assert!(sorting::is_sorted(&test_list2));
        assert!(sorting::is_sorted(&test_list3));
    }

    #[test]
    fn selection_sort_test() {
        let mut test_list1: [i32; 5] = [1, 2, 3, 4, 5];
        let mut test_list2: [i32; 5] = [4, 3, 1, 4, 5];
        let mut test_list3: [i32; 5] = [5, 2, 3, 2, 1];

        sorting::selection_sort(&mut test_list1);
        sorting::selection_sort(&mut test_list2);
        sorting::selection_sort(&mut test_list3);

        assert!(sorting::is_sorted(&test_list1));
        assert!(sorting::is_sorted(&test_list2));
        assert!(sorting::is_sorted(&test_list3));
    }

    #[test]
    fn insertion_sort_test() {
        let mut test_list1: [i32; 5] = [1, 2, 3, 4, 5];
        let mut test_list2: [i32; 5] = [4, 3, 1, 4, 5];
        let mut test_list3: [i32; 5] = [5, 2, 3, 2, 1];

        sorting::insertion_sort(&mut test_list1);
        sorting::insertion_sort(&mut test_list2);
        sorting::insertion_sort(&mut test_list3);

        assert!(sorting::is_sorted(&test_list1));
        assert!(sorting::is_sorted(&test_list2));
        assert!(sorting::is_sorted(&test_list3));
    }
}
// mod algo;
// mod graph;

// // enum Parent{
// //     Child1,
// //     Child2,
// //     Child3,
// // }

// // impl Parent::Child1{
// //     fn puk(){
// //         println!("puk1");
// //     }
// // }


// // impl Parent::Child2{
// //     fn puk(){
// //         println!("puk2");
// //     }
// // }

// #[derive(Debug)]
// enum Shape{
//     circle(i32),
//     square(i32, i32)
// }
// impl Shape{
//     fn create_Shape(x: i32) -> Shape{
//         if x > 10 {
//             return Shape::circle(x);
//         } else {
//             return Shape::square(x, 3);
//         }
//     }
// }

// fn main()   {
//     let mut test_list1: [i32; 5] = [4, 3, 1, 4, 5];
//     let mut test_list2: [i32; 5] = [1, 2, 3, 4, 5];
//     let mut test_list3: [i32; 5] = [5, 2, 3, 2, 1];
//     println!("{test_list1:?}");
//     algo::sorting::heap_sort(&mut test_list1, 5);
//     println!("{test_list1:?}");

//     // let node0 = graph::Node::new("node0");
//     // let node1 = graph::Node::new("node1");
//     // let node2 = graph::Node::new("node2");
//     // let node3 = graph::Node::new("node3");
//     // let node4 = graph::Node::new("node4");
//     // let node5 = graph::Node::new("node5");

//     // let nodes0 = [node0, node1, node2, node3, node4, node5, ];
//     // let mut graph1 = graph::new_directed_unweighted();
//     // // graph1.insert_node(node0);
//     // graph1.insert_node(node1);
//     // graph1.insert_node(node2);
//     // graph1.insert_node(node3);
//     // graph1.insert_node(node4);
//     // graph1.add_node(&node5);
//     // graph1.add_nodes(nodes0.iter());
//     // graph1.add_edge(1, 0);
//     // graph1.add_edge(1, 1);
//     // graph1.add_edge(1, 2);
//     // graph1.add_edge(2, 3);




//     // println!("{:?}", graph1);
//     // println!("{:?}", node5);
    
    
//     // let tes1 = Shape::create_Shape(2);

//     // println!("{:?}", tes1);


//     // println!("{:?}", node1);

// //     // let mut tes_list_iter = test_list3.iter();
// //     // let mut previous = tes_list_iter.next();
// //     // // println!("first elem {:?}", first);
// //     // for (i, item) in tes_list_iter.enumerate(){
// //     //     if previous.unwrap() < item {
// //     //         println!("smaller");
// //     //     }
// //     //     println!("{:?}:{:?}", i, item);
// //     //     previous = Some(item);
// //     // }

// //     // for (mut current, mut next) in test_list3.iter().zip(test_list3.iter().skip(1)){
// //     //     println!("{:?}, {:?}", current, next);
// //     //     if current < next {
// //     //         println!("smaller");
// //     // //         let temp = current;
// //     // //         current = next;
// //     // //         next = temp;
// //     // //     }
// //     // // }

// //     // // println!("{:?}", test_list3);
// //     // let mut s = String::from("hello world");

// //     // let word = &s;

// //     // s.clear(); // error!

// //     // println!("the first word is: {}", word);
// //     // swap(list[0], list[1])

// //     // println!("test_list1 {:?}", test_list1);
// //     // println!("test_list2 {:?}", test_list2);
// //     // println!("test_list3 {:?}", test_list3);
    
// //     // algo::sorting::insertion_sort(&mut test_list1);
// //     // algo::sorting::insertion_sort(&mut test_list2);
// //     // algo::sorting::insertion_sort(&mut test_list3);

// //     // println!("test_list1 {:?}", test_list1);
// //     // println!("test_list2 {:?}", test_list2);
// //     // println!("test_list3 {:?}", test_list3);

// // //     let test_vertices: [graph::Node;5] = [graph::Node::new(1), graph::Node::new(2), graph::Node::new(3), graph::Node::new(4), graph::Node::new(5),];
// // //     let test_graph: graph::Graph =  graph::Graph::new(&test_vertices, true);
// // //     println!("{:?}", test_graph);

// //     // let mut test_graph = graph::DirectedUnweightedGraph::new();
// //     // // test_graph.add_node();
// //     // // test_graph.add_node();
// //     // // test_graph.add_node();
// //     // // test_graph.add_node();
// //     // test_graph.add_edge(1, 0);

// //     // let mut n1 = graph::Node::new(String::from("a"));
// //     // println!("{:?}", n1);
// //     // println!("{:?}", n1);
// //     // let y = println!("pukpuk");
//     // println!("{y}");

// }
// // // #[derive(Debug)]
// // // enum Shape{
// // //     rec, 
// // //     circle,
// // // }

// // // impl Shape{
// // //     fn new(&self){
// // //         match 
// // //     }
// // // }

// // fn main() {
// //     let s1 = graph::Graph(String::from("sjdjsd"), true, false);
// //     println!("{:?}", s1);
// // }
