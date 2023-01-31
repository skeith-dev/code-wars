pub fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    
    let mut arr: [i64; 3] = [a, b, c];
    arr.sort();
    println!("{:?}", arr);

    if arr[0] + arr[1] > arr[2] {
        return true;
    } else {
        return false;
    }

}