pub mod sorting{

    pub fn find_min<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T{
        let mut min = list[0];
        for i in 1..list.len(){
            if list[i] < min{
                min = list[i];
            }
        }
        min
    }

    pub fn find_min_interval<T: std::cmp::PartialOrd + Copy>(list: &[T], from: usize, to: usize) -> usize{
        let mut min = from;
        for i in (from + 1)..to{
            if list[i] < list[min]{
                min = i;
            }
        }
        min
    }

    pub fn find_max_interval<T: std::cmp::PartialOrd + Copy>(list: &[T], from: usize, to: usize) -> usize{
        let mut max = from;
        for i in (from + 1)..to{
            if list[i] > list[max]{
                max = i;
            }
        }
        max
    }

    pub fn find_max<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T{
        let mut max = list[0];
        for i in 1..list.len(){
            if list[i] > max{
                max = list[i];
            }
        }
        max
    }

    pub fn is_sorted<T: std::cmp::PartialOrd>(list: &[T]) -> bool{
        for i in 0..(list.len() - 1){
            if list[i] > list[i + 1]{
                return false;
            }
        }
        true
    }

    pub fn bubble_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        let mut flag = true;
        while flag{
            flag = false;
            for i in 0..(list.len() - 1){
                if list[i] > list[i + 1] {
                    let t = list[i];
                    list[i] = list[i + 1];
                    list[i + 1] = t;

                    flag = true;
                }
            }
        }
    }
    // pub fn bubble_sort_iter<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
    //     let mut flag = true;
    //     while flag{
    //         flag = false;
    //         let mut list_iter = list.iter();
    //         let previous = list_iter.next();
    //         for i in 0..(list.len() - 1){
    //             if list[i] > list[i + 1] {
    //                 let t = list[i];
    //                 list[i] = list[i + 1];
    //                 list[i + 1] = t;
    //                 flag = true;
    //             }
    //         }
    //     }
    // }

    pub fn selection_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        let mut n = list.len();
        let mut max;
        for i in 0..(list.len() - 1){
            max = find_max_interval(list, 0, n - i);
            let t = list[max];
            list[max] = list[n - i - 1];
            list[n - i - 1] = t;
        }
    }

    pub fn insertion_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        if list[0] > list[1] {
            let t = list[0];
            list[0] = list[1];
            list[1] = t;
        }
        for i in 1..(list.len() - 1){
            if list[i] > list[i + 1] {

                let t = list[i];
                list[i] = list[i + 1];
                list[i + 1] = t;

                for j in 0..i{
                    if list[i - j] < list[i - j - 1]{
                        let t = list[i - j];
                        list[i - j] = list[i - j - 1];
                        list[i - j - 1] = t;
                    }
                }
            }
        }
    }

    pub fn build_max_heap<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        // let mut largest = i;
        // let l = 2 * i + 1;
        // let r = 2 * i + 2;
        // if l < n && list[l] > list[largest]{
        //     largest = l;
        // }
        // if r < n && list[r] > list[largest]{
        //     largest = r;
        // }
        // if largest != i {
        //     let t = list[i];
        //     list[i] = list[largest];
        //     list[largest] = t;

        //     build_max_heap(list, n, largest);
        // }
        for j in 0..(list.len() ) {
            let i = list.len() - j - 1;
            if list[i] > list[i/2] {
                let root = i/2;
                let t = list[i];
                list[i] = list[root];
                list[root] = t;
                while root != 0 {
                    if list[root/2] > list[root] {
                        break;
                    } else {
                        let t = list[root];
                        list[root] = list[root/2];
                        list[root/2] = t;
                        let root = root/2;
                    }
                }
            }
        }
    }

    pub fn heap_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
    //     let sorted = 0;
    //     build_max_heap(list, 0);
    //     for i in 0..(list.len() - 1) {
    //         let t = list[0];
    //         list[0] = list[list.len() - 1 - sorted];
    //         list[list.len() - 1 - sorted] = t;
    //         let sorted = sorted + 1;

    //         // println!("{list:?}");

    //         let mut root = 0;
    //         while root < list.len() - sorted {
    //             // for j in 0..10{
    //             // println!("{} and {}", 2 * root + 2, list.len() - sorted);
    //             if 2 * root + 2 <= list.len() - sorted {
    //                 // println!("it went in 1 if {} {} {}", list[root], list[2 * root + 1], list[2 * root + 1]);
    //                 if list[2 * root + 2] > list[root] {
    //                     // println!("it went in 2 if");

    //                     if list[2 * root + 1] > list[2 * root + 2]{
    //                         let t = list[2 * root + 1];
    //                         list[2 * root + 1] = list[root];
    //                         list[root] = t;

    //                         root = 2 * root + 1;
    //                     } else {
    //                         let t = list[2 * root + 2];
    //                         list[2 * root + 2] = list[root];
    //                         list[root] = t;

    //                         root = 2 * root + 2;
    //                     }
    //                 } else if list[2 * root + 1] > list[root]{
    //                     let t = list[2 * root + 1];
    //                     list[2 * root + 1] = list[root];
    //                     list[root] = t;

    //                     root = 2 * root + 1;
    //                 }
    //             } else if 2 * root + 1 <= list.len() - sorted{
    //                 if list[2 * root + 1] > list[root]{
    //                     let t = list[2 * root + 1];
    //                     list[2 * root + 1] = list[root];
    //                     list[root] = t;

    //                     root = 2 * root + 1;
    //                 }
    //             }
    //         }
    //     }
        // let mut i = n / 2 - 1;
        // while i >= 0 {
        //     build_max_heap(list, n, i);
        //     println!("{i}");
        //     if i != 0 {
        //         i = i - 1;
        //     }
        // }

        // for j in 0..=(n/2 - 1) {
        //     let i = n/2 - 1 - j;
        //     build_max_heap(list, n, i);
        // }

        // for j in 0..=(n - 1) {
        //     let i = n - 1 - j;
        //     build_max_heap(list, n, i);
        // }

        // let mut i = n - 1;
        // while i >= 0 {
        //     let t = list[0];
        //     list[0] = list[i];
        //     list[i] = t;

        //     build_max_heap(list, n, 0);
        //     if i != 0 {
        //         i = i - 1;
        //     }
        // }
    }
}