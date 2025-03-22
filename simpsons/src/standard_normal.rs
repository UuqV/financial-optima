use std::f64::consts::E;
use std::f64::consts::PI;

use crate::simpsons::simpsons_rule_in_tolerance;

pub fn round(x: f64, power: f64) -> f64 {
    return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
}

pub fn cumulative_distribution_standard_normal(t: f64, n: u64, tol_factor: f64) -> f64 {
    return standard_constants(simpsons_rule_in_tolerance(
        t,
        n,
        tol_factor,
        standard_normal_variable,
    ));
}

fn standard_constants(accumulation: f64) -> f64 {
    return (1.0 / (2.0 * PI)).sqrt() * accumulation;
}

pub fn standard_normal_variable(x: f64) -> f64 {
    return E.powf(-1.0 * (x.powf(2.0) / 2.0));
}

#[cfg(test)]
mod standard_normal_tests {
    use super::*;

    #[test]
    fn round_6_dec_test() {
        assert_eq!(round(E, 6.0), 2.718282);
    }
    #[test]
    fn round_12_dec_test() {
        assert_eq!(round(E, 12.0), 2.718281828459);
    }
    #[test]
    fn standard_constants_test() {
        assert_eq!(round(standard_constants(E), 6.0), 1.584438);
    }
    #[test]
    fn standard_normal_variable_test() {
        assert_eq!(round(standard_normal_variable(2.0), 6.0), 0.135335);
        assert_eq!(standard_normal_variable(0.0), 1.0);
    }
    #[test]
    fn cumulative_test() {
        assert_eq!(
            round(cumulative_distribution_standard_normal(0.1, 4, 12.0), 12.0),
            0.539827837277
        );
    }
}
