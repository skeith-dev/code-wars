pub fn sort_array(arr: &[i32]) -> Vec<i32> {

    let mut sorted_arr: Vec<i32> = vec!{};

    for i in 0..arr.len() {
        if arr[i] % 2 != 0 {
            sorted_arr.push(arr[i]);
        }
    }

    sorted_arr.sort();

    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            sorted_arr.insert(i, arr[i]);
        }
    }

    return sorted_arr;

}