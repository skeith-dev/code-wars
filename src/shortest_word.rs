pub fn find_short(s: &str) -> u32 {
    
    let str_split: std::str::Split<&str> = s.split(" ");
    let words: Vec<&str> = str_split.collect();
    let mut shrt_word_len: u32 = std::u32::MAX;

    for i in 0..words.len() {
        if (words[i].len() as u32) < shrt_word_len {
            println!("{} < {}...", words[i], shrt_word_len);
            shrt_word_len = words[i].len() as u32;
        }
    }

    return shrt_word_len;

}