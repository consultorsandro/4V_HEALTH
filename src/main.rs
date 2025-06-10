mod bmi; 
mod metabolism;
mod body_fat;
mod whr;

use std::io; // Importing the standard input/output library
use bmi::calculator::{BmiCalculator, BmiData}; // Importing enum BMI calculator and data structures
use metabolism::tmb::{TmbCalculator, TmbData, Gender, TmbCategory}; 
use whr::calculator::{WhrCalculator, WhrCalculatorTrait, WhrData, Gender as WhrGender};

fn main() {
    loop { // Start of the main loop until the user decides to exit
        println!("\nRequest the health check you want to do:");
        println!("1 – BMI");
        println!("2 – TMB");
        println!("3 – Body Fat Percentage (PGC)");
        println!("4 – Waist-to-Hip Ratio (WHR)");
        println!("0 – Exit");

        let choice = read_input_as_u32(); // Read user input as u32

        match choice { // Match user input against available options
            1 => { // BMI Calculation
                println!("Please enter your weight in kilograms (e.g., 70.5): ");
                let weight = read_input_as_f32(); // Read user input as f32

                println!("Please enter your height in meters (e.g., 1.75): ");
                let height = read_input_as_f32();

                let data = BmiData { weight, height };
                let bmi = BmiCalculator::calculate(&data);
                let category = BmiCalculator::classify(bmi);
                let result = BmiCalculator::evaluation_result(bmi, &category);

                println!("{}", result);
            }
            2 => { // TMB Calculation
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
                    gender: gender.clone(), // gender is cloned to avoid ownership issues
                };

                let tmb = TmbCalculator::calculate(&data);
                let category = TmbCalculator::classify(tmb, weight, &gender);
                let result = TmbCalculator::evaluation_result(tmb, weight, &category);

                println!("{}", result);
            }
            3 => { // Body Fat Percentage (PGC) Calculation
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

                let data = body_fat::BodyFatData {
                    weight,
                    height,
                    age,
                    gender: gender.clone(),
                };

                let bmi = body_fat::BodyFatCalculator::calculate_bmi(&data);
                let pgc = body_fat::BodyFatCalculator::calculate_pgc(bmi, age, &gender);
                let sex_category = body_fat::BodyFatCalculator::classify_by_sex(pgc, &gender);
                let age_category = body_fat::BodyFatCalculator::classify_by_age(pgc, age, &gender);

                let result = body_fat::BodyFatCalculator::evaluation_result(
                    pgc,
                    &gender,
                    age,
                    &sex_category,
                    &age_category,
                );

                println!("{}", result);
            }
            4 => { // Waist-to-Hip Ratio (WHR) Calculation
                println!("Please enter your gender (M/F): ");
                let gender_input = read_input_as_string();
                let gender = match gender_input.to_lowercase().as_str() {
                    "m" => WhrGender::Male,
                    "f" => WhrGender::Female,
                    _ => {
                        println!("Invalid gender input. Please use 'M' or 'F'.");
                        continue;
                    }
                };

                println!("Please enter your waist circumference in centimeters (e.g., 85.0): ");
                let waist = read_input_as_f32(); // cintura

                println!("Please enter your hip circumference in centimeters (e.g., 95.0): ");
                let hip = read_input_as_f32(); // quadril

                let data = WhrData {
                    waist_circumference: waist,
                    hip_circumference: hip,
                    gender: gender.clone(),
                };

                let whr = WhrCalculator::calculate(&data);
                let result = WhrCalculator::evaluate(whr, &gender);

                println!("{}", result);
            }
            0 => {
                println!("Exiting application.");
                break;
            }
            _ => {
                println!("Invalid option. Please enter 1, 2, 3, 4, or 0.");
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

