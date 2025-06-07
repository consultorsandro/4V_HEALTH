/// This module provides functionality to calculate and classify Body Fat Percentage (PGC)
/// using the Deurenberg formula, with classification by sex and by sex+age.
/// It follows SOLID principles for maintainability and extensibility.

use crate::bmi::calculator::{BmiCalculator, BmiData};
use crate::metabolism::tmb::Gender;

/// Data structure for Body Fat calculation input.
pub struct BodyFatData {
    pub weight: f32,
    pub height: f32,
    pub age: u32,
    pub gender: Gender,
}

/// Enum for general body fat classification by sex.
pub enum BodyFatSexCategory {
    Essential,
    Athlete,
    Fitness,
    Acceptable,
    Obesity,
}

/// Enum for body fat classification by sex and age.
pub enum BodyFatAgeCategory {
    Low,
    Normal,
    High,
    VeryHigh,
}

/// Main calculator for Body Fat Percentage.
pub struct BodyFatCalculator;

impl BodyFatCalculator {
    /// Calculates BMI using the existing BMI module.
    pub fn calculate_bmi(data: &BodyFatData) -> f32 {
        let bmi_data = BmiData { weight: data.weight, height: data.height };
        BmiCalculator::calculate(&bmi_data)
    }

    /// Calculates Body Fat Percentage using the Deurenberg formula.
    /// sex: 1 for male, 0 for female
    pub fn calculate_pgc(bmi: f32, age: u32, gender: &Gender) -> f32 {
        let sex = match gender {
            Gender::Male => 1.0,
            Gender::Female => 0.0,
        };
        (1.20 * bmi) + (0.23 * age as f32) - (10.8 * sex) - 5.4
    }

    /// Classifies Body Fat Percentage by sex.
    pub fn classify_by_sex(pgc: f32, gender: &Gender) -> BodyFatSexCategory {
        match gender {
            Gender::Male => {
                match pgc {
                    x if x < 6.0 => BodyFatSexCategory::Essential,
                    x if x < 14.0 => BodyFatSexCategory::Athlete,
                    x if x < 18.0 => BodyFatSexCategory::Fitness,
                    x if x < 25.0 => BodyFatSexCategory::Acceptable,
                    _ => BodyFatSexCategory::Obesity,
                }
            }
            Gender::Female => {
                match pgc {
                    x if x < 14.0 => BodyFatSexCategory::Essential,
                    x if x < 21.0 => BodyFatSexCategory::Athlete,
                    x if x < 25.0 => BodyFatSexCategory::Fitness,
                    x if x < 32.0 => BodyFatSexCategory::Acceptable,
                    _ => BodyFatSexCategory::Obesity,
                }
            }
        }
    }

    /// Classifies Body Fat Percentage by sex and age.
    pub fn classify_by_age(pgc: f32, age: u32, gender: &Gender) -> BodyFatAgeCategory {
        let (low, normal, high, very_high) = match gender {
            Gender::Male => match age {
                20..=29 => (7.0, 19.0, 24.0, 25.0),
                30..=39 => (8.0, 20.0, 25.0, 26.0),
                40..=49 => (10.0, 22.0, 27.0, 28.0),
                50..=59 => (11.0, 23.0, 28.0, 29.0),
                _ => (13.0, 25.0, 30.0, 31.0), // 60+ 
            },
            Gender::Female => match age {
                20..=29 => (16.0, 27.0, 32.0, 33.0),
                30..=39 => (17.0, 28.0, 33.0, 34.0),
                40..=49 => (18.0, 29.0, 34.0, 35.0),
                50..=59 => (19.0, 30.0, 35.0, 36.0),
                _ => (20.0, 31.0, 36.0, 37.0), // 60+
            },
        };
        // Classify based on the calculated PGC and the defined ranges
        if pgc < low {
            BodyFatAgeCategory::Low
        } else if pgc <= normal {
            BodyFatAgeCategory::Normal
        } else if pgc <= high {
            BodyFatAgeCategory::High
        } else {
            BodyFatAgeCategory::VeryHigh
        }
    }

    /// Returns a string with the result for the user.
    pub fn evaluation_result(
        pgc: f32,
        gender: &Gender,
        age: u32,
        sex_category: &BodyFatSexCategory,
        age_category: &BodyFatAgeCategory,
    ) -> String {
        let sex_str = match gender {
            Gender::Male => "male",
            Gender::Female => "female",
        };
        let sex_class = match sex_category {
            BodyFatSexCategory::Essential => "Essential to life",
            BodyFatSexCategory::Athlete => "Athlete",
            BodyFatSexCategory::Fitness => "Fitness",
            BodyFatSexCategory::Acceptable => "Acceptable",
            BodyFatSexCategory::Obesity => "Obesity",
        };
        let age_class = match age_category {
            BodyFatAgeCategory::Low => "Low",
            BodyFatAgeCategory::Normal => "Normal",
            BodyFatAgeCategory::High => "High",
            BodyFatAgeCategory::VeryHigh => "Very High",
        };
        format!(
            "Your Body Fat Percentage (PGC), sex [{}] is: {}\n\
             Your PGC for sex [{}] and age group [{}] (OMC Standard) is: {}",
            sex_str, sex_class, sex_str, age, age_class
        )
    }
} 