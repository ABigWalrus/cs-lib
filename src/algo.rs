pub mod sorting{
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
}