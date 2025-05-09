use crate::newton::{taylor_duration_convexity_price, taylor_duration_price};
use std::f64::consts::E;

// Cash flow by date
pub struct CFD {
    pub t: f64,
    pub cash_flow: f64,
}

fn discount(t: f64, zero_rate: impl Fn(f64) -> f64) -> f64 {
    return E.powf(-1.0 * t * zero_rate(t));
}

// dtcfs: Dates to cash flows
pub fn bond_price_over_time(dtcfs: &[CFD], zero_rate: impl Fn(f64) -> f64) -> f64 {
    println!("Discount factors:");
    return dtcfs.into_iter().fold(0.0, |b, dtcf| {
        let discount = discount(dtcf.t, &zero_rate);
        println!("{:.6}", discount);
        return b + dtcf.cash_flow * discount;
    });
}

pub fn taylor_bond_price_comparison(
    price: f64,
    cash_flow_dates: &[CFD],
    duration: f64,
    convexity: f64,
    y: f64,
    delta: f64,
) -> f64 {
    let d = taylor_duration_price(price, duration, delta);
    let c = taylor_duration_convexity_price(price, duration, convexity, delta);
    let bond_price = bond_price_over_time(cash_flow_dates, |x: f64| y + delta);
    let derr = (d - bond_price).abs() / bond_price;
    let cerr = (c - bond_price).abs() / bond_price;
    println!(
        "{:width$.6} {:width$.6} {:width$.6} {:width$.6} {:width$.6} {:width$.6}",
        delta,
        d,
        c,
        bond_price,
        derr,
        cerr,
        width = 15
    );
    return bond_price;
}

pub fn taylor_bond_price_deltas(
    price: f64,
    cash_flow_dates: &[CFD],
    duration: f64,
    convexity: f64,
    y: f64,
    deltas: &[f64],
) {
    for delta in deltas.into_iter() {
        let d = taylor_duration_price(price, duration, *delta);
        let c = taylor_duration_convexity_price(price, duration, convexity, *delta);
        let bond_price = bond_price_over_time(cash_flow_dates, |x: f64| y + *delta);
        let derr = (d - bond_price).abs() / bond_price;
        let cerr = (c - bond_price).abs() / bond_price;
        println!(
            "{:width$.6} {:width$.6} {:width$.6} {:width$.6} {:width$.6} {:width$.6}",
            delta,
            d,
            c,
            bond_price,
            derr,
            cerr,
            width = 15
        );
    }
}

pub fn round(x: f64, power: f64) -> f64 {
    return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
}

#[cfg(test)]
mod bond_price_tests {
    use super::*;

    #[test]
    fn disc_test() {
        assert_eq!(
            round(discount(0.5, |x: f64| 0.05 + 0.005 * (1.0 + x).sqrt()), 6.0),
            0.972328
        );
    }
    #[test]
    fn number_10() {
        assert_eq!(
            round(
                bond_price_over_time(
                    &[
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
                    &[
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
    #[test]
    fn number_11() {
        assert_eq!(
            round(
                bond_price_over_time(
                    &[
                        CFD {
                            t: (7.0 / 12.0),
                            cash_flow: 4.0
                        },
                        CFD {
                            t: (19.0 / 12.0),
                            cash_flow: 104.0
                        },
                    ],
                    |x: f64| 0.02 + (x / (200.0 - x))
                ),
                6.0
            ),
            103.440082
        );
        assert_eq!(
            round(
                bond_price_over_time(
                    &[
                        CFD {
                            t: (1.0 / 12.0),
                            cash_flow: 2.0
                        },
                        CFD {
                            t: (7.0 / 12.0),
                            cash_flow: 2.0
                        },
                        CFD {
                            t: (13.0 / 12.0),
                            cash_flow: 2.0
                        },
                        CFD {
                            t: (19.0 / 12.0),
                            cash_flow: 102.0
                        },
                    ],
                    |x: f64| 0.02 + (x / (200.0 - x))
                ),
                6.0
            ),
            103.495539
        );
        assert_eq!(
            round(
                bond_price_over_time(
                    &[
                        CFD {
                            t: (1.0 / 12.0),
                            cash_flow: 1.0
                        },
                        CFD {
                            t: (4.0 / 12.0),
                            cash_flow: 1.0
                        },
                        CFD {
                            t: (7.0 / 12.0),
                            cash_flow: 1.0
                        },
                        CFD {
                            t: (10.0 / 12.0),
                            cash_flow: 1.0
                        },
                        CFD {
                            t: (13.0 / 12.0),
                            cash_flow: 1.0
                        },
                        CFD {
                            t: (16.0 / 12.0),
                            cash_flow: 1.0
                        },
                        CFD {
                            t: (19.0 / 12.0),
                            cash_flow: 101.0
                        },
                    ],
                    |x: f64| 0.02 + (x / (200.0 - x))
                ),
                6.0
            ),
            102.518910
        );
    }
}
