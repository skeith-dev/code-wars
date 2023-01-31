pub fn likes(names: &[&str]) -> String {

    if names.len() > 3 {
        let remainder: i32 = (names.len() as i32) - 2;
        return format!("{}, {} and {} others like this", names[0], names[1], remainder);
    } else if names.len() > 2 {
        return format!("{}, {} and {} like this", names[0], names[1], names[2]);
    } else if names.len() > 1 {
        return format!("{} and {} like this", names[0], names[1]);
    } else if names.len() > 0 {
        return format!("{} likes this", names[0]);
    } else {
        return "no one likes this".to_owned();
    }
    
}