mod perfect_squares;
mod population_growth;
mod is_triangle;
mod shortest_word;
mod sort_the_odd;
mod highest_scoring_word;
mod create_phone_number;
mod digital_root;
mod duplicate_encoder;
mod who_likes_it;


fn main() {
    
    //PERFECT SQUARES
    let ps_1: Option<u64> = perfect_squares::find_next_square(121);
    println!("{:?}\n", ps_1);
    let ps_2: Option<u64> = perfect_squares::find_next_square(625);
    println!("{:?}\n\n", ps_2);

    //POPULATION GROWTH
    let pg_1: i32 = population_growth::nb_year(1500, 5.0, 100, 5000);
    println!("{}\n", pg_1);
    let pg_2: i32 = population_growth::nb_year(1500000, 2.5, 10000, 2000000);
    println!("{}\n\n", pg_2);

    //IS TRIANGLE
    let it_1: bool = is_triangle::is_triangle(4, 2, 3);
    println!("A triangle can exist with side lengths {}, {}, and {}: {}\n", 4, 2, 3, it_1);
    let it_2: bool = is_triangle::is_triangle(7, 2, 2);
    println!("A triangle can exist with side lengths {}, {}, and {}: {}\n\n", 7, 2, 2, it_2);

    //SHORTEST WORD
    let sw_1: u32 = shortest_word::find_short("bitcoin take over the world maybe who knows perhaps");
    println!("The shortest word has {} letters\n", sw_1);
    let sw_2: u32 = shortest_word::find_short("turns out random test cases are easier than writing out basic ones");
    println!("The shortest word has {} letters\n\n", sw_2);

    //SORT THE ODD
    let sto_1: Vec<i32> = sort_the_odd::sort_array(&[7, 1]);
    println!("Odd-sorted array: {:?}\n", sto_1);
    let sto_2: Vec<i32> = sort_the_odd::sort_array(&[5, 8, 6, 3, 4]);
    println!("Odd-sorted array: {:?}\n", sto_2);
    let sto_3: Vec<i32> = sort_the_odd::sort_array(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    println!("Odd-sorted array: {:?}\n\n", sto_3);

    //HIGHEST SCORING WORD
    let hsw_1: &str = highest_scoring_word::high("man i need a taxi up to ubud");
    println!("The highest scoring word is: {}\n", hsw_1);
    let hsw_2: &str = highest_scoring_word::high("what time are we climbing up the volcano");
    println!("The highest scoring word is: {}\n", hsw_2);
    let hsw_3: &str = highest_scoring_word::high("massage yes massage yes massage");
    println!("The highest scoring word is: {}\n\n", hsw_3);

    //CREATE PHONE NUMBER
    let cpn_1: String = create_phone_number::create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    println!("{}", cpn_1);
    let cpn_2: String = create_phone_number::create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    println!("{}", cpn_2);
    let cpn_3: String = create_phone_number::create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]);
    println!("{}", cpn_3);

    //DIGITAL ROOT
    let dr_1: i64 = digital_root::digital_root(16);
    println!("The digital root of {} is {}\n", 16, dr_1);
    let dr_2: i64 = digital_root::digital_root(942);
    println!("The digital root of {} is {}\n", 942, dr_2);
    let dr_3: i64 = digital_root::digital_root(132189);
    println!("The digital root of {} is {}\n", 132189, dr_3);
    let dr_4: i64 = digital_root::digital_root(493193);
    println!("The digital root of {} is {}\n\n", 493193, dr_4);

    //DUPLICATE ENCODER
    let de_1: String = duplicate_encoder::duplicate_encode("din");
    println!("{} is encoded to {}\n", "din", de_1);
    let de_2: String = duplicate_encoder::duplicate_encode("recede");
    println!("{} is encoded to {}\n", "recede", de_2);
    let de_3: String = duplicate_encoder::duplicate_encode("Success");
    println!("{} is encoded to {}\n", "Success", de_3);
    let de_4: String = duplicate_encoder::duplicate_encode("(( @");
    println!("{} is encoded to {}\n\n", "(( @", de_4);

    //WHO LIKES IT
    let wli_1: String = who_likes_it::likes(&[]);
    println!("{}", wli_1);
    let wli_2: String = who_likes_it::likes(&["Peter"]);
    println!("{}", wli_2);
    let wli_3: String = who_likes_it::likes(&["Jacob", "Alex"]);
    println!("{}", wli_3);
    let wli_4: String = who_likes_it::likes(&["Max", "John", "Mark"]);
    println!("{}", wli_4);
    let wli_5: String = who_likes_it::likes(&["Alex", "Jacob", "Mark", "Max"]);
    println!("{}", wli_5);

}
