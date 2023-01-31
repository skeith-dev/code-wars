pub fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {

    //p0 is the starting population
    //percent is the growth rate per year
    //aug is the number of inhabitants coming or going each year
    //p is the target population to equal or surpass

    let mut years: i32 = 0;
    let mut total: i32 = p0;

    loop {

        years += 1;
        total = total + (((total as f64) * (percent / 100.0)) as i32) + aug;

        if total >= p {
            println!("It takes the town {} years to achieve a population of {}, which equals or exceeds the target population of {}!", years, total, p);
            return years;
        }

    }

}