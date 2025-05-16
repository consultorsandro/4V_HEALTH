mod bmi;

use std::io;
use bmi::calculator::{BmiCalculator, BmiData};

fn main() {
    // Prompt for weight
    println!("Please enter your weight in kilograms (e.g., 70.5): ");
    let weight = read_input_as_f32();

    // Prompt for height
    println!("Please enter your height in meters (e.g., 1.75): ");
    let height = read_input_as_f32();

    let data = BmiData { weight, height };
    let bmi = BmiCalculator::calculate(&data);
    let category = BmiCalculator::classify(bmi);
    let result = BmiCalculator::evaluation_result(bmi, &category);

    println!("{}", result);
}

/// Helper function to read user input and convert to f32
fn read_input_as_f32() -> f32 {
    loop {
        let mut input = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Error reading input. Please try again:");
            continue;
        }

        match input.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid number. Please enter a valid floating point number:");
                continue;
            }
        }
    }
}


