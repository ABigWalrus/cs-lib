// mod algo;
// fn main() {
//     let mut test_list = vec![1, 2, 3, 4, 5];
//     test_list.push(1);
//     algo::sorting::bubble_sort(&mut test_list);
// }

mod algo;
fn main() {
    let mut test_list: [i32; 5] = [4, 3, 1, 4, 5];
    println!("{:?}", test_list);
    println!("{}", algo::sorting::is_sorted(&mut test_list));
    // algo::sorting::swap(&mut test_list, 0, 1);
    algo::sorting::bubble_sort(&mut test_list);
    println!("{:?}", test_list);
    println!("{}", algo::sorting::is_sorted(&mut test_list));
}
