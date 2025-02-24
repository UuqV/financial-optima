use std::f64::consts::E;

fn disc(t: f64, cash_flow: f64, zero_rate: fn(f64) -> f64) -> f64 {
    return E.powf(-1.0 * t * zero_rate(cash_flow));
}

fn round(x: f64, power: f64) -> f64 {
    return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
}

#[cfg(test)]
mod simpsons_rule_tests {
    use super::*;

    #[test]
    fn disc_test() {
        assert_eq!(
            round(
                disc(0.5, 0.5, |x: f64| 0.05 + 0.005 * (1.0 + x).sqrt()),
                6.0
            ),
            0.972328
        );
    }
}
