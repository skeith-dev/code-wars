pub fn duplicate_encode(word: &str) -> String {

    let mut word_encoded: String = String::new();

    for i in 0..word.len() {

        let mut char_encoded: bool = false;

        for j in 0..word.len() {

            if j == i {
                continue;
            }

            let x: i32 = word.chars().nth(i).unwrap() as i32;
            let y: i32 = word.chars().nth(j).unwrap() as i32;
            if x == y {
                word_encoded.push(')');
                char_encoded = true;
                break;
            } else if x >= 65 && y >= 65 {
                if x + 32 == y || x - 32 == y {
                    word_encoded.push(')');
                    char_encoded = true;
                    break;
                }
            }

        }
        if char_encoded == false {
            word_encoded.push('(');
        }

    }

    return word_encoded;

}