use crate::standard_normal::{round, standard_normal_variable};

pub fn simpsons_rule_approximation(a: f64, b: f64, n: u64, eval: fn(x: f64) -> f64) -> f64 {
    let h = h(a, b, n);
    let s = initial_s(a, b, eval);
    return eval(a);
}

fn h(a: f64, b: f64, n: u64) -> f64 {
    return (b - a) / n as f64;
}

fn initial_s(a: f64, b: f64, eval: fn(x: f64) -> f64) -> f64 {
    return (eval(a) / 6.0) + (eval(b) / 6.0);
}

fn simpsons_first_loop(a: f64, h: f64, n: u64, eval: fn(x: f64) -> f64) {
    for i in 1..n {}

    return;
}

fn interval(a: f64, h: f64, i: u64, eval: fn(x: f64) -> f64) -> f64 {
    return eval(a + ((i as f64) * h)) / 3.0;
}

#[cfg(test)]
mod simpsons_rule_tests {
    use super::*;

    #[test]
    fn h_test() {
        assert_eq!(h(0.0, 0.1, 10), 0.01);
        assert_eq!(round(h(0.5, 1.6, 10), 2.0), 0.11);
        assert_eq!(round(h(1.6, 0.5, 20), 3.0), -0.055);
    }
    #[test]
    fn initial_s_test() {
        assert_eq!(
            round(initial_s(0.0, 0.1, standard_normal_variable), 12.0),
            0.332502079865
        );
    }
    #[test]
    fn interval_test() {
        assert_eq!(1 as f64, 1.0);
        assert_eq!(
            round(interval(0.0, 0.1, 1, standard_normal_variable), 12.0),
            0.331670826398
        );
    }
    #[test]
    fn simpsons_rule_test() {
        assert_eq!(
            simpsons_rule_approximation(0.0, 0.1, 10, standard_normal_variable),
            1.0
        );
    }
}
