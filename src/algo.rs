pub mod sorting{

    pub fn swap<T:Copy>(list: &mut [T], index1: usize, index2: usize) {
        let t = list[index1];
        list[index1] = list[index2];
        list[index2] = t;
    }

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
                    swap(list, i, i + 1);
                    flag = true;
                }
            }
        }
    }

    pub fn selection_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        let mut n = list.len();
        let mut max;
        for i in 0..(list.len() - 1){
            max = find_max_interval(list, 0, n - i);
            swap(list, max, n - i - 1)
        }
    }

    pub fn insertion_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        if list[0] > list[1] {
            swap(list, 0, 1);
        }
        for i in 1..(list.len() - 1){
            if list[i] > list[i + 1] {
                swap(list, i, i + 1);
                for j in 0..i{
                    if list[i - j] < list[i - j - 1]{
                        swap(list, i - j, i - j - 1);
                    }
                }
            }
        }
    }

    pub fn build_max_heap<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        for j in 0..list.len(){
            let i = list.len() - j - 1;
            sink(list, i, list.len());
        }
    }

    fn sink<T: std::cmp::PartialOrd + Copy>(list: &mut [T], index: usize, limit: usize){
        let mut max = index;
        if 2 * index + 1 < limit {
            if list[2 * index + 1] > list[max] {
                max = 2 * index + 1;
            }
        }
        if 2 * index + 2 < limit {
            if list[2 * index + 2] > list[max] {
                max = 2 * index + 2;
            }
        }
        if max == index {
            return;
        } else {
            swap(list, index, max);
            sink(list, max, limit - 1);
        }
    }

    pub fn heap_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        build_max_heap(list);
        for i in 0..(list.len() - 1) {
            swap(list, 0, list.len() - 1 - i);
            sink(list, 0, list.len() - 1 - i);
        }
    }

    pub fn quick_sort<T: std::cmp::PartialOrd + Copy + std::fmt::Debug>(list: &mut [T]){
        quick_sort_indexed(list, 0, list.len() - 1);
    }

    fn quick_sort_indexed<T: std::cmp::PartialOrd + Copy + std::fmt::Debug>(list: &mut [T], left: usize, right: usize){
        if left < right  {
            let pivot_pos = divide(list, left, right);
            if pivot_pos != 0 {
                quick_sort_indexed(list, left, pivot_pos - 1);
            }
            quick_sort_indexed(list, pivot_pos + 1, right);
        }   
    }

    fn divide<T: std::cmp::PartialOrd + Copy>(list: &mut [T], left: usize, right: usize) -> usize {
        let middle = (left + right) / 2;
        let pivot = list[middle].clone();
        swap(list, middle, right);
        let mut pivot_pos = left;

        for i in left..right {
            if list[i] <= pivot {
                swap(list, pivot_pos, i);
                pivot_pos += 1;
            }
        }

        swap(list, pivot_pos, right);

        pivot_pos
    }

    pub fn merge_sort<T: std::cmp::PartialOrd + Copy + std::default::Default>(list: &mut [T]) {
        merge_sort_indexed(list, 0, list.len() - 1);
    }
    fn merge_sort_indexed<T: std::cmp::PartialOrd + Copy + std::default::Default>(list: &mut [T], from: usize, to: usize) {
        if (to - from) > 0 {
            let middle: usize = (from + to) / 2;

            merge_sort_indexed(list, from, middle);
            merge_sort_indexed(list, middle + 1, to);

            merge(list, from, middle, to);
        }
    }
    fn merge<T: std::cmp::PartialOrd + Copy + std::default::Default>(list: &mut [T], left: usize, middle: usize, right: usize) {
        let n: usize =  middle - left + 1; 
        let mut tmp_list: Vec<T> = vec![Default::default(); n];
        for i in 0..n {
            tmp_list[i] = list[i].clone();
        }
        let mut index_left: usize = 0;
        let mut index_right: usize = middle + 1;
        let mut index_result: usize = left;
        
        while (index_left < n) && (index_right <= right) {
            if tmp_list[index_left] <= list[index_right] {
                list[index_result] = tmp_list[index_left].clone();
                index_left += 1;
            } else {
                list[index_result] = tmp_list[index_right].clone();
                index_right += 1;
            }

            index_result += 1;
        }

        while index_left < tmp_list.len() {
            list[index_result] = tmp_list[index_left];
            index_result += 1;
            index_left += 1;
        }
    }
}

pub mod graph{
    
}