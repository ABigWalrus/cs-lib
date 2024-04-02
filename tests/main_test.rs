use cs_lib::algo;

#[test]
fn sorting_test() {
    let mut test_list1: [i32; 5] = [1, 2, 3, 4, 5];
    let mut test_list2: [i32; 5] = [4, 3, 1, 4, 5];
    let mut test_list3: [i32; 5] = [5, 2, 3, 2, 1];

    assert!(algo::sorting::is_sorted(&test_list1));
    assert!(!algo::sorting::is_sorted(&test_list2));
    assert!(!algo::sorting::is_sorted(&test_list3));
}