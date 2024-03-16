mod algo;

fn main() {
    let mut test_list1: [i32; 5] = [4, 3, 1, 4, 5];
    let mut test_list2: [i32; 5] = [1, 2, 3, 4, 5];
    let mut test_list3: [i32; 5] = [5, 4, 3, 2, 1];

    println!("test_list1 {:?}", test_list1);
    println!("test_list2 {:?}", test_list2);
    println!("test_list3 {:?}", test_list3);
    
    algo::sorting::selection_sort(&mut test_list1);
    algo::sorting::selection_sort(&mut test_list2);
    algo::sorting::selection_sort(&mut test_list3);
    
    println!("test_list1 {:?}", test_list1);
    println!("test_list2 {:?}", test_list2);
    println!("test_list3 {:?}", test_list3);
}
