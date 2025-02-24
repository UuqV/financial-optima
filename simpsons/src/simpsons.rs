pub fn simpsons_rule_approximation(a: f64, b: f64, n: u64, eval: fn(x: f64) -> f64) -> f64 {
    let h = h(a, b, n);
    return h
        * (initial_s(a, b, eval)
            + simpsons_first_loop(a, h, n, eval)
            + simpsons_second_loop(a, h, n, eval));
}

fn h(a: f64, b: f64, n: u64) -> f64 {
    return (b - a) / n as f64;
}

fn initial_s(a: f64, b: f64, eval: fn(x: f64) -> f64) -> f64 {
    return (eval(a) / 6.0) + (eval(b) / 6.0);
}

fn simpsons_first_loop(a: f64, h: f64, n: u64, eval: fn(x: f64) -> f64) -> f64 {
    return (1..n).fold(0.0, |acc, t| acc + interval(a, h, t, eval));
}

fn interval(a: f64, h: f64, i: u64, eval: fn(x: f64) -> f64) -> f64 {
    return eval(a + ((i as f64) * h)) / 3.0;
}

fn simpsons_second_loop(a: f64, h: f64, n: u64, eval: fn(x: f64) -> f64) -> f64 {
    return (1..=n).fold(0.0, |acc, t| acc + second_interval(a, h, t, eval));
}

fn second_interval(a: f64, h: f64, i: u64, eval: fn(x: f64) -> f64) -> f64 {
    return 2.0 * eval(a + (((i as f64) - 0.5) * h)) / 3.0;
}

#[cfg(test)]
mod simpsons_rule_tests {
    use super::*;
    use crate::standard_normal::{round, standard_normal_variable};

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
        assert_eq!(
            round(interval(0.0, 0.1, 1, standard_normal_variable), 12.0),
            0.331670826398
        );
    }
    #[test]
    fn second_interval_test() {
        assert_eq!(
            round(second_interval(0.0, 0.1, 1, standard_normal_variable), 12.0),
            0.66583385395
        );
    }
    #[test]
    fn simpsons_first_loop_test() {
        assert_eq!(
            round(
                simpsons_first_loop(0.0, 0.1, 2, standard_normal_variable),
                12.0
            ),
            round(interval(0.0, 0.1, 1, standard_normal_variable), 12.0)
        );
        assert_eq!(
            round(
                simpsons_first_loop(0.0, 0.1, 3, standard_normal_variable),
                12.0
            ),
            round(
                interval(0.0, 0.1, 1, standard_normal_variable)
                    + interval(0.0, 0.1, 2, standard_normal_variable),
                12.0
            )
        );
    }
    #[test]
    fn simpsons_rule_test() {
        assert_eq!(
            round(
                simpsons_rule_approximation(0.0, 0.5, 50, standard_normal_variable),
                1.0
            ),
            0.5
        );
    }
    #[test]
    fn easy_function_test() {
        assert_eq!(
            round(
                simpsons_rule_approximation(0.0, 1.0, 10000000, |x: f64| x.powf(2.0)),
                2.0
            ),
            0.33
        );
    }
}
