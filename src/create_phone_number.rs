pub fn create_phone_number(numbers: &[u8]) -> String {

    let mut phone_number: String = String::new();

    phone_number.push('(');
    for i in 0..3 {
        phone_number.push_str(numbers[i].to_string().as_str());
    }
    phone_number.push_str(") ");
    for i in 3..6 {
        phone_number.push_str(numbers[i].to_string().as_str());
    }
    phone_number.push('-');
    for i in 6..numbers.len() {
        phone_number.push_str(numbers[i].to_string().as_str());
    }

    return phone_number;

}