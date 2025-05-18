/// This module defines the structures and logic for Basal Metabolic Rate (BMR) calculation.
/// It follows the SOLID principles, especially Single Responsibility and Open/Closed.
// src/metabolism/tmb.rs

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

pub struct TmbData {
    pub weight: f32, // em kg
    pub height: f32, // em metros
    pub age: u32,    // em anos
    pub gender: Gender,
}

pub struct TmbCalculator;

impl TmbCalculator {
    pub fn calculate(data: &TmbData) -> f32 {
        match data.gender {
            Gender::Male => {
                66.0 + (13.7 * data.weight) + (5.0 * data.height * 100.0) - (6.8 * data.age as f32)
            }
            Gender::Female => {
                655.0 + (9.6 * data.weight) + (1.8 * data.height * 100.0) - (4.7 * data.age as f32)
            }
        }
    }

    pub fn evaluation_result(tmb: f32) -> String {
        format!("Your Basal Metabolic Rate (TMB) is {:.2} kcal/day.", tmb)
    }
}
