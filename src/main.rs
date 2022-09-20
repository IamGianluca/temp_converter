use std::io;
use math::round;

fn main() {
    println!("Hello, world!");

    let mut f_temp = String::new();

    println!("Please enter temperature in Fahrenheit");
    io::stdin()
        .read_line(&mut f_temp)
        .expect("Error");

    let f_temp: f64 = f_temp
        .trim()
        .parse()
        .expect("Something went wrong");

    let c_temp = round::half_away_from_zero((f_temp - 32.) / 1.8, 2);
    println!("The temperature in Celsius is {c_temp}");
}
