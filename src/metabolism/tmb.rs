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

#[derive(Debug, PartialEq)]
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
                88.36 + (13.4 * data.weight) + (4.8 * data.height * 100.0) - (5.7 * data.age as f32)
            }
            Gender::Female => {
                447.6 + (9.2 * data.weight) + (3.1 * data.height * 100.0) - (4.3 * data.age as f32)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_male() {
        let data = TmbData {
            weight: 70.0,
            height: 1.75,
            age: 25,
            gender: Gender::Male,
        };
        let tmb = TmbCalculator::calculate(&data);
        assert!((tmb - 1745.1).abs() < 0.1); // Valor esperado para um homem de 70kg, 1.75m e 25 anos
    }

    #[test]
    fn test_calculate_female() {
        let data = TmbData {
            weight: 60.0,
            height: 1.65,
            age: 30,
            gender: Gender::Female,
        };
        let tmb = TmbCalculator::calculate(&data);
        assert!((tmb - 1395.0).abs() < 0.1); // Valor esperado para uma mulher de 60kg, 1.65m e 30 anos
    }

    #[test]
    fn test_classify_male() {
        let tmb = 1745.1; // TMB calculado para o teste
        let weight = 70.0;
        let category = TmbCalculator::classify(tmb, weight, &Gender::Male);
        assert_eq!(category, TmbCategory::Normal);
    }

    #[test]
    fn test_classify_female() {
        let tmb = 1395.0; // TMB calculado para o teste
        let weight = 60.0;
        let category = TmbCalculator::classify(tmb, weight, &Gender::Female);
        assert_eq!(category, TmbCategory::Normal);
    }

    #[test]
    fn test_evaluation_result() {
        let tmb = 1745.1;
        let weight = 70.0;
        let category = TmbCategory::Normal;
        let result = TmbCalculator::evaluation_result(tmb, weight, &category);
        let expected = "Your Basal Metabolic Rate (TMB) is 1745.10 kcal/day.\n\
                        Your TMB per kg is 24.93 kcal/kg/day.\n\
                        Classification: Normal";
        assert_eq!(result, expected);
    }
}
