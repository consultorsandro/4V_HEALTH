mod bmi;
mod metabolism;

use std::io;
use bmi::calculator::{BmiCalculator, BmiData};
use metabolism::tmb::{TmbCalculator, TmbData, Gender};

fn main() {
    loop {
        println!("\nRequest the health check you want to do:");
        println!("1 – BMI");
        println!("2 – TMB");
        println!("0 – Exit");

        let choice = read_input_as_u32();

        match choice {
            1 => {
                println!("Please enter your weight in kilograms (e.g., 70.5): ");
                let weight = read_input_as_f32();

                println!("Please enter your height in meters (e.g., 1.75): ");
                let height = read_input_as_f32();

                let data = BmiData { weight, height };
                let bmi = BmiCalculator::calculate(&data);
                let category = BmiCalculator::classify(bmi);
                let result = BmiCalculator::evaluation_result(bmi, &category);

                println!("{}", result);
            }
            2 => {
                println!("Please enter your weight in kilograms (e.g., 70.5): ");
                let weight = read_input_as_f32();

                println!("Please enter your height in meters (e.g., 1.75): ");
                let height = read_input_as_f32();

                println!("Please enter your age in years (e.g., 30): ");
                let age = read_input_as_u32();

                println!("Please enter your gender (M/F): ");
                let gender_input = read_input_as_string();
                let gender = match gender_input.to_lowercase().as_str() {
                    "m" => Gender::Male,
                    "f" => Gender::Female,
                    _ => {
                        println!("Invalid gender input. Please use 'M' or 'F'.");
                        continue;
                    }
                };

                let data = TmbData {
                    weight,
                    height,
                    age,
                    gender,
                };

                let tmb = TmbCalculator::calculate(&data);
                let result = TmbCalculator::evaluation_result(tmb);

                println!("{}", result);
            }
            0 => {
                println!("Exiting application.");
                break;
            }
            _ => {
                println!("Invalid option. Please enter 1, 2, or 0.");
            }
        }
    }
}

/// Reads and returns user input as f32
fn read_input_as_f32() -> f32 {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(value) = input.trim().parse::<f32>() {
                return value;
            }
        }
        println!("Invalid number. Please enter a valid floating point number:");
    }
}

/// Reads and returns user input as u32
fn read_input_as_u32() -> u32 {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(value) = input.trim().parse::<u32>() {
                return value;
            }
        }
        println!("Invalid number. Please enter a valid whole number:");
    }
}

/// Reads and returns user input as trimmed String
fn read_input_as_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


