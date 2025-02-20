use crate::standard_normal::standard_normal_variable;

pub fn simpsons_rule_approximation(a: f64, b: f64, n: f64, eval: fn(x: f64) -> f64) -> f64 {
    return eval(a);
}

#[cfg(test)]
mod simpsons_rule_tests {
    use super::*;

    #[test]
    fn simpsons_rule_test() {
        assert_eq!(
            simpsons_rule_approximation(0.0, 0.1, 10.0, standard_normal_variable),
            1.0
        );
    }
}
