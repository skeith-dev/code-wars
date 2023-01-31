pub fn digital_root(n: i64) -> i64 {

    let mut num: i64 = n;
    let mut sum: i64 = 0;
    
    while num != 0 {
        let digit: i64 = num % 10;
        sum = sum + digit;
        num = num / 10;
        println!("num = {}, sum = {}", num, sum);
        
        if num == 0 && sum > 9 {
            num = sum;
            sum = 0;
        }
    }

    return sum;

}