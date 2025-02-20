use crate::standard_normal::{round, standard_normal_variable};

pub fn simpsons_rule_approximation(a: f64, b: f64, n: f64, eval: fn(x: f64) -> f64) -> f64 {
    let h = h(a, b, n);
    return eval(a);
}

fn h(a: f64, b: f64, n: f64) -> f64 {
    return (b - a) / n;
}

#[cfg(test)]
mod simpsons_rule_tests {
    use super::*;

    #[test]
    fn h_test() {
        assert_eq!(h(0.0, 0.1, 10.0), 0.01);
        assert_eq!(round(h(0.5, 1.6, 10.0), 2.0), 0.11);
        assert_eq!(round(h(1.6, 0.5, 20.0), 3.0), -0.055);
    }
    #[test]
    fn simpsons_rule_test() {
        assert_eq!(
            simpsons_rule_approximation(0.0, 0.1, 10.0, standard_normal_variable),
            1.0
        );
    }
}
