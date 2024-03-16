// pub mod sorting{
//     pub fn bubble_sort(list: &mut Vec<i32>){
//         println!("fist element: {}", list[0]);
//         list[0] = 10;
//         println!("fist element: {}", list[0]);
//     }
// }

pub mod sorting{ 
    // pub fn swap<T>(first: T, second_index: i32){
    //     let mut temp = list[first_index];
    //     list[first_index] = list[second_index];
    //     list[second_index] = temp;
    // }
    pub fn bubble_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        let mut flag = true;
        while flag{
            flag = false;
            for i in 0..(list.len() - 1){
                if list[i] > list[i + 1] {
                    // std::mem::swap(&mut list[i], &mut list[i + 1]);
                    let t = list[i];
                    list[i] = list[i + 1];
                    list[i + 1] = t;
                    flag = true;
                    println!("I swapped");
                }
                println!("I did something");
            }
        }
        
    }
}