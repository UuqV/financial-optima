use std::f64::consts::E;

// Cash flow by date
struct CFD {
    t: f64,
    cash_flow: f64,
}

fn disc(t: f64, zero_rate: fn(f64) -> f64) -> f64 {
    return E.powf(-1.0 * t * zero_rate(t));
}

// dtcfs: Dates to cash flows
fn bond_price_over_time(dtcfs: Vec<CFD>, zero_rate: fn(f64) -> f64) -> f64 {
    return dtcfs.into_iter().fold(0.0, |b, dtcf| {
        return b + dtcf.cash_flow * disc(dtcf.t, zero_rate);
    });
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
            round(disc(0.5, |x: f64| 0.05 + 0.005 * (1.0 + x).sqrt()), 6.0),
            0.972328
        );
    }
    #[test]
    fn iteration_test() {
        assert_eq!(
            round(
                bond_price_over_time(
                    vec![
                        CFD {
                            t: 0.5,
                            cash_flow: 3.5
                        },
                        CFD {
                            t: 1.0,
                            cash_flow: 3.5
                        },
                        CFD {
                            t: 1.5,
                            cash_flow: 3.5
                        },
                        CFD {
                            t: 2.0,
                            cash_flow: 103.5
                        },
                    ],
                    |x: f64| 0.05 + 0.005 * (1.0 + x).sqrt()
                ),
                6.0
            ),
            101.960379
        );
        assert_eq!(
            round(
                bond_price_over_time(
                    vec![
                        CFD {
                            t: (1.0 / 12.0),
                            cash_flow: 3.5
                        },
                        CFD {
                            t: (7.0 / 12.0),
                            cash_flow: 3.5
                        },
                        CFD {
                            t: (13.0 / 12.0),
                            cash_flow: 3.5
                        },
                        CFD {
                            t: (19.0 / 12.0),
                            cash_flow: 103.5
                        },
                    ],
                    |x: f64| 0.05 + 0.005 * (1.0 + x).sqrt()
                ),
                6.0
            ),
            104.573694
        );
    }
}
