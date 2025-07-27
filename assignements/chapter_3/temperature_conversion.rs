use std::io;

fn main() {
    println!("enter temperature in fahrenheit");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid Input");

    let temp_in_fahrenheit: f64 = input.trim().parse().expect("Invalid temperature");

    let temp_in_celsius = temp_convert(temp_in_fahrenheit);

    println!("converted temperature in celsius is, {temp_in_celsius}");
}

fn temp_convert(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}
