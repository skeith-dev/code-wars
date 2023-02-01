pub fn tower_builder(n_floors: usize) -> Vec<String> {

    let mut tower: Vec<String> = vec![String::new(); n_floors];
    
    for i in 0..n_floors {
        let floor: String = create_floor(i + 1, n_floors);
        tower[i].push_str(&floor);
    }

    for i in 0..n_floors {
        println!("{}", tower[i]);
    }
    return tower;

}

fn create_floor(floor_number: usize, bottom_floor_number: usize) -> String {

    let mut floor: String = String::new();
    
    let length: usize = 2 * floor_number - 1;
    let bottom_length: usize = 2 * bottom_floor_number - 1;

    for _i in 0..((bottom_length - length) / 2) {
        floor.push(' ');
    }
    for _i in 0..length {
        floor.push('*');
    }
    for _i in 0..((bottom_length - length) / 2) {
        floor.push(' ');
    }

    println!("Floor #{} = {}", floor_number, floor);
    return floor;

}