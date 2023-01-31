pub fn high(input: &str) -> &str {

    let mut highest_score: i32 = 0;
    let mut highest_score_index: usize = 0;

    let str_split: std::str::Split<&str> = input.split(" ");
    let words: Vec<&str> = str_split.collect();

    for i in 0..words.len() {
        let score: i32 = word_score(words[i]);
        if score > highest_score {
            highest_score = score;
            highest_score_index = i;
        }
    }

    return words[highest_score_index];

}

fn word_score(input: &str) -> i32 {

    let mut score: i32 = 0;
    let word_chars: Vec<char> = input.chars().collect();

    for i in 0..word_chars.len() {
        score += (word_chars[i] as i32) - 96;
    }

    println!("\"{}\" = {}", input, score);
    return score;

}