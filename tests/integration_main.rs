use fourv_saude::bmi::calculator::{BmiCalculator, BmiData};
use fourv_saude::body_fat::{BodyFatCalculator, BodyFatData};
use fourv_saude::metabolism::tmb::Gender;

#[test]
fn integration_bmi_and_body_fat_male() {
    // Dados de exemplo para um homem
    let data = BodyFatData {
        weight: 80.0,
        height: 1.80,
        age: 35,
        gender: Gender::Male,
    };

    // Calcula o BMI usando o módulo BMI
    let bmi = BodyFatCalculator::calculate_bmi(&data);
    assert!((bmi - BmiCalculator::calculate(&BmiData { weight: 80.0, height: 1.80 })).abs() < 1e-2);

    // Calcula o percentual de gordura corporal usando o BMI calculado
    let pgc = BodyFatCalculator::calculate_pgc(bmi, data.age, &data.gender);

    // Verifica se o valor do PGC está dentro de um intervalo esperado para o exemplo
    assert!(pgc > 10.0 && pgc < 30.0, "PGC fora do esperado para homem adulto");
}

#[test]
fn integration_bmi_and_body_fat_female() {
    // Dados de exemplo para uma mulher
    let data = BodyFatData {
        weight: 65.0,
        height: 1.65,
        age: 30,
        gender: Gender::Female,
    };

    // Calcula o BMI usando o módulo BMI
    let bmi = BodyFatCalculator::calculate_bmi(&data);
    assert!((bmi - BmiCalculator::calculate(&BmiData { weight: 65.0, height: 1.65 })).abs() < 1e-2);

    // Calcula o percentual de gordura corporal usando o BMI calculado
    let pgc = BodyFatCalculator::calculate_pgc(bmi, data.age, &data.gender);

    // Verifica se o valor do PGC está dentro de um intervalo esperado para o exemplo
    assert!(pgc > 15.0 && pgc < 40.0, "PGC fora do esperado para mulher adulta");
}

#[test]
fn integration_body_fat_classification() {
    // Exemplo para classificação por sexo e idade
    let data = BodyFatData {
        weight: 70.0,
        height: 1.75,
        age: 28,
        gender: Gender::Male,
    };
    let bmi = BodyFatCalculator::calculate_bmi(&data);
    let pgc = BodyFatCalculator::calculate_pgc(bmi, data.age, &data.gender);

    let sex_category = BodyFatCalculator::classify_by_sex(pgc, &data.gender);
    let age_category = BodyFatCalculator::classify_by_age(pgc, data.age, &data.gender);

    // Apenas verifica se as funções retornam algum valor válido
    use fourv_saude::body_fat::{BodyFatSexCategory, BodyFatAgeCategory};
    match sex_category {
        BodyFatSexCategory::Essential
        | BodyFatSexCategory::Athlete
        | BodyFatSexCategory::Fitness
        | BodyFatSexCategory::Acceptable
        | BodyFatSexCategory::Obesity => {}
    }
    match age_category {
        BodyFatAgeCategory::Low
        | BodyFatAgeCategory::Normal
        | BodyFatAgeCategory::High
        | BodyFatAgeCategory::VeryHigh => {}
    }
}