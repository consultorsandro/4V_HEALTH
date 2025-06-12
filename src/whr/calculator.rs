/// Enum representing the gender of the user.
#[derive(Clone)]
pub enum Gender {
    Male,
    Female,
}

/// Struct to hold the input data for WHR calculation.
pub struct WhrData {
    pub waist_circumference: f32, // in centimeters
    pub hip_circumference: f32,   // in centimeters
    pub gender: Gender,
}

/// Trait for WHR calculation and evaluation.
pub trait WhrCalculatorTrait {
    fn calculate(data: &WhrData) -> f32;
    fn evaluate(whr: f32, gender: &Gender) -> String;
}

/// Implementation of the WHR calculator.
pub struct WhrCalculator;

impl WhrCalculatorTrait for WhrCalculator {
    /// Calculates the Waist-to-Hip Ratio (WHR).
    fn calculate(data: &WhrData) -> f32 {
        data.waist_circumference / data.hip_circumference
    }

    /// Evaluates the WHR result and returns a metabolic risk message.
    fn evaluate(whr: f32, gender: &Gender) -> String {
        match gender {
            Gender::Male => {
                if whr > 0.90 {
                    format!(
                        "WHR: {:.2}\nCondition: Higher risk for cardiovascular diseases (men, WHR > 0.90).",
                        whr
                    )
                } else {
                    format!(
                        "WHR: {:.2}\nCondition: Lower risk for cardiovascular diseases (men, WHR ≤ 0.90).",
                        whr
                    )
                }
            }
            Gender::Female => {
                if whr > 0.85 {
                    format!(
                        "WHR: {:.2}\nCondition: Higher risk for cardiovascular diseases (women, WHR > 0.85).",
                        whr
                    )
                } else {
                    format!(
                        "WHR: {:.2}\nCondition: Lower risk for cardiovascular diseases (women, WHR ≤ 0.85).",
                        whr
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_male() {
        let data = WhrData {
            waist_circumference: 90.0,
            hip_circumference: 100.0,
            gender: Gender::Male,
        };
        let whr = WhrCalculator::calculate(&data);
        assert!((whr - 0.9).abs() < 1e-6);
    }

    #[test]
    fn test_calculate_female() {
        let data = WhrData {
            waist_circumference: 85.0,
            hip_circumference: 100.0,
            gender: Gender::Female,
        };
        let whr = WhrCalculator::calculate(&data);
        assert!((whr - 0.85).abs() < 1e-6);
    }

    #[test]
    fn test_evaluate_male_high_risk() {
        let whr = 0.95;
        let result = WhrCalculator::evaluate(whr, &Gender::Male);
        assert!(result.contains("Higher risk"));
        assert!(result.contains("WHR: 0.95"));
    }

    #[test]
    fn test_evaluate_male_low_risk() {
        let whr = 0.89;
        let result = WhrCalculator::evaluate(whr, &Gender::Male);
        assert!(result.contains("Lower risk"));
        assert!(result.contains("WHR: 0.89"));
    }

    #[test]
    fn test_evaluate_female_high_risk() {
        let whr = 0.90;
        let result = WhrCalculator::evaluate(whr, &Gender::Female);
        assert!(result.contains("Higher risk"));
        assert!(result.contains("WHR: 0.90"));
    }

    #[test]
    fn test_evaluate_female_low_risk() {
        let whr = 0.80;
        let result = WhrCalculator::evaluate(whr, &Gender::Female);
        assert!(result.contains("Lower risk"));
        assert!(result.contains("WHR: 0.80"));
    }
}