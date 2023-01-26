use std::io;

fn main() {
    println!("Temperature Converter");
    loop {
        println!("Please input the temperature in Fahrenheit!");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature value");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let new_temp = (temp - 32.0) * (5.0 / 9.0);

        println!("The temperature in celsius is {new_temp} degrees");
    }
}
