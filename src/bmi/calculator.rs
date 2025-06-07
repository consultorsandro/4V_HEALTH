/// This module provides functionality for calculating the Body Mass Index (BMI)
/// and evaluating the corresponding health classification based on standard ranges.

/// Struct representing a person with weight (kg) and height (m).
pub struct BmiData {
    pub weight: f32,
    pub height: f32,
}

/// Enum representing the different categories of BMI classification.
#[derive(Debug, PartialEq)]
pub enum BmiCategory {
    Underweight,
    NormalWeight,
    Overweight,
    ObesityGrade1,
    ObesityGrade2,
    ObesityGrade3,
}

/// This struct adheres to the Single Responsibility Principle (SRP),
/// being solely responsible for BMI calculation and classification.
pub struct BmiCalculator;

impl BmiCalculator {
    /// Calculates the Body Mass Index (BMI) given the user's weight and height.
    /// 
    /// Formula: BMI = weight (kg) / height² (m²)
    pub fn calculate(data: &BmiData) -> f32 {
        data.weight / (data.height * data.height)
    }

    /// Returns the BMI classification category according to the calculated BMI.
    pub fn classify(bmi: f32) -> BmiCategory {
        match bmi {
            b if b < 18.5 => BmiCategory::Underweight,
            b if b < 25.0 => BmiCategory::NormalWeight,
            b if b < 30.0 => BmiCategory::Overweight,
            b if b < 35.0 => BmiCategory::ObesityGrade1,
            b if b < 40.0 => BmiCategory::ObesityGrade2,
            _ => BmiCategory::ObesityGrade3,
        }
    }
    /// Returns a formatted human-readable string for the result.
    #[allow(unused_variables)]
    pub fn evaluation_result(bmi: f32, category: &BmiCategory) -> String {
        let classification = match category {
            BmiCategory::Underweight => "Underweight",
            BmiCategory::NormalWeight => "Normal weight",
            BmiCategory::Overweight => "Overweight",
            BmiCategory::ObesityGrade1 => "Obesity Grade 1",
            BmiCategory::ObesityGrade2 => "Obesity Grade 2",
            BmiCategory::ObesityGrade3 => "Obesity Grade 3 (morbid)",
        };
        // Return a formatted string with the BMI classification
        format!("Your BMI assessment is:: {}", classification)
    }
}
