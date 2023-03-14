use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let mut inputted_farenheit_temp = String::new();
       
        print!(
            "Enter a Farenheit temperature to convert to Celsius: "
        );

        stdout().flush().unwrap();

        stdin()
            .read_line(&mut inputted_farenheit_temp)
            .expect("Failed to read line");

        println!();
        println!("You input: {inputted_farenheit_temp}");
        if inputted_farenheit_temp.trim() == "quit" {
            break;
        }

        let inputted_farenheit_temp: f32 = match inputted_farenheit_temp.trim().parse() {
            Ok(float) => float,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            }
        };

        println!(
            "{inputted_farenheit_temp}°F is {}°C",
            convert_temp_from_f_to_c(inputted_farenheit_temp)
        );
        println!();
    }
}

fn convert_temp_from_f_to_c(farenheit_temp: f32) -> f32 {
    let celsius = (farenheit_temp - 32.0) * (5.0 / 9.0);
    celsius
}
