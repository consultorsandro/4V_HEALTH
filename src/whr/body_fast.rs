#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_male() {
        let data = BodyFastData {
            weight: 70.0,
            height: 1.75,
            gender: Gender::Male,
        };
        let bmi = BodyFastCalculator::calculate(&data);
        assert!((bmi - 22.86).abs() < 1e-2);
    }

    #[test]
    fn test_calculate_female() {
        let data = BodyFastData {
            weight: 60.0,
            height: 1.65,
            gender: Gender::Female,
        };
        let bmi = BodyFastCalculator::calculate(&data);
        assert!((bmi - 22.04).abs() < 1e-2);
    }

    #[test]
    fn test_evaluate_male_underweight() {
        let bmi = 17.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Male);
        assert!(result.contains("Underweight"));
    }

    #[test]
    fn test_evaluate_male_normal() {
        let bmi = 22.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Male);
        assert!(result.contains("Normal weight"));
    }

    #[test]
    fn test_evaluate_male_overweight() {
        let bmi = 27.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Male);
        assert!(result.contains("Overweight"));
    }

    #[test]
    fn test_evaluate_male_obese() {
        let bmi = 32.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Male);
        assert!(result.contains("Obese"));
    }

    #[test]
    fn test_evaluate_female_underweight() {
        let bmi = 17.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Female);
        assert!(result.contains("Underweight"));
    }

    #[test]
    fn test_evaluate_female_normal() {
        let bmi = 22.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Female);
        assert!(result.contains("Normal weight"));
    }

    #[test]
    fn test_evaluate_female_overweight() {
        let bmi = 27.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Female);
        assert!(result.contains("Overweight"));
    }

    #[test]
    fn test_evaluate_female_obese() {
        let bmi = 32.0;
        let result = BodyFastCalculator::evaluate(bmi, &Gender::Female);
        assert!(result.contains("Obese"));
    }
} 