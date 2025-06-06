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