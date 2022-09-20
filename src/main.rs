use math::round;
use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("Please enter temperature in Fahrenheit: ");
        io::stdout().flush().unwrap();

        let mut f_temp = String::new();
        io::stdin()
            .read_line(&mut f_temp)
            .expect("Failed to read line");

        let f_temp: f64 = match f_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a valid number\n");
                continue;
            }
        };

        let c_temp = round::half_away_from_zero((f_temp - 32.) / 1.8, 2);
        println!("The temperature in Celsius is {c_temp}\n");
    }
}
