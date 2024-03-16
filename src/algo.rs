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

    pub fn is_sorted<T: std::cmp::PartialOrd + Copy>(list: &mut [T]) -> bool{
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

    pub fn selection_sort<T: std::cmp::PartialOrd + Copy>(list: &mut [T]){
        let mut n_sorted = 0;
        let mut n = list.len();
        let mut max;
        for i in 0..(list.len() - 1){
            max = find_max_interval(list, 0, n - i);
            let t = list[max];
            list[max] = list[n - i - 1];
            list[n - i - 1] = t;
        }
    }
}