pub fn find_next_square(sq: u64) -> Option<u64> {

    let sq_rt: f64 = (sq as f64).sqrt();
    if sq_rt.fract() != 0.0 {
        println!("{} is NOT a perfect square!", sq);
        return None;
    }
    println!("{} IS a perfect square!", sq);

    let mut next_sq: u64 = sq + 1;
    loop {

        if (next_sq as f64).sqrt().fract() == 0.0 {
            println!("{} is the next perfect square!", next_sq);
            return Some(next_sq);
        }

        next_sq += 1;

    }

}