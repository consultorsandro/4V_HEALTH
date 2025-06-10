/// This module defines the structures and logic for Basal Metabolic Rate (BMR) calculation.
/// It follows the SOLID principles, especially Single Responsibility and Open/Closed.
// src/metabolism/tmb.rs

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}
// Represents the data required for TMB calculation
pub struct TmbData {
    pub weight: f32, // em kg
    pub height: f32, // em metros
    pub age: u32,    // em anos
    pub gender: Gender,
}

pub struct TmbCalculator; // This struct serves as a namespace for TMB calculation methods

#[derive(Debug)]
pub enum TmbCategory {
    VeryLow,
    Low,
    Normal,
    High,
    VeryHigh,
}
// Represents the categories of TMB based on the calculated value per kg of body weight (WHO guidelines)
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
    /// Classifies the TMB based on the calculated value per kg of body weight
    pub fn classify(tmb: f32, weight: f32, gender: &Gender) -> TmbCategory {
        let tmb_per_kg = tmb / weight;

        match gender {
            Gender::Male => match tmb_per_kg {
                x if x < 15.0 => TmbCategory::VeryLow,
                x if x < 20.0 => TmbCategory::Low,
                x if x < 25.0 => TmbCategory::Normal,
                x if x < 30.0 => TmbCategory::High,
                _ => TmbCategory::VeryHigh,
            },
            Gender::Female => match tmb_per_kg {
                x if x < 13.0 => TmbCategory::VeryLow,
                x if x < 18.0 => TmbCategory::Low,
                x if x < 23.0 => TmbCategory::Normal,
                x if x < 28.0 => TmbCategory::High,
                _ => TmbCategory::VeryHigh,
            },
        }
    }
    /// Generates a formatted string with the TMB result and classification
    pub fn evaluation_result(tmb: f32, weight: f32, category: &TmbCategory) -> String {
        let tmb_per_kg = tmb / weight;
        let classification = match category {
            TmbCategory::VeryLow => "Very low",
            TmbCategory::Low => "Low",
            TmbCategory::Normal => "Normal",
            TmbCategory::High => "High",
            TmbCategory::VeryHigh => "Very high",
        };
        // Return a formatted string with the TMB result and classification
        format!(
            "Your Basal Metabolic Rate (TMB) is {:.2} kcal/day.\n\
             Your TMB per kg is {:.2} kcal/kg/day.\n\
             Classification: {}",
            tmb, tmb_per_kg, classification
        )
    }
}
