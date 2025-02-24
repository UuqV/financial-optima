use std::f64::consts::E;

struct CFD {
    t: f64,
    cash_flow: f64,
}

fn disc(t: f64, cash_flow: f64, zero_rate: fn(f64) -> f64) -> f64 {
    return E.powf(-1.0 * t * zero_rate(cash_flow));
}

fn get_disc_by_date(dtcf: CFD, zero_rate: fn(f64) -> f64) -> f64 {
    return disc(dtcf.t, dtcf.cash_flow, zero_rate);
}

fn round(x: f64, power: f64) -> f64 {
    return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
}

#[cfg(test)]
mod bond_price_tests {
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
    #[test]
    fn map_test() {
        assert_eq!(
            round(
                get_disc_by_date(
                    CFD {
                        t: 0.5,
                        cash_flow: 0.5
                    },
                    |x: f64| 0.05 + 0.005 * (1.0 + x).sqrt()
                ),
                6.0
            ),
            0.972328
        );
    }
}
