use std::f64::consts::E;
use std::f64::consts::PI;

use crate::simpsons::simpsons_rule_approximation;

pub fn round(x: f64, power: f64) -> f64 {
    return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
}

pub fn cumulative_distribution(t: f64, n: u64) -> f64 {
    return standard_constants(simpsons_rule_approximation(
        0.0,
        t,
        n,
        standard_normal_variable,
    ));
}

pub fn cumulative_distribution_in_tolerance(t: f64, n: u64, tol_factor: f64) -> f64 {
    let mut increase = n;
    let mut last = cumulative_distribution(t, n);
    let mut abs = last;
    let tol = 10.0_f64.powf(-tol_factor);
    println!("n={increase:#} : {last:#}");
    while abs > tol {
        increase = 2 * increase;
        println!("{abs:#.12} > {tol:#}\n");
        let current = cumulative_distribution(t, increase);
        abs = (current - last).abs();
        last = current;
        println!("n={increase:#.12} : {last:#.12}");
    }
    return last;
}

fn standard_constants(accumulation: f64) -> f64 {
    return 0.5 + (1.0 / (2.0 * PI)).sqrt() * accumulation;
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
}
