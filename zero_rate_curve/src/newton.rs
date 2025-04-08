use crate::bond_price::{round, CFD};
use std::f64::consts::E;
use std::num;

pub fn newton(x0: f64, tol: f64, f: impl Fn(f64) -> f64, fprime: impl Fn(f64) -> f64) -> f64 {
    let mut xnew: f64 = x0;
    let mut xold: f64 = x0 - 1.0;
    while (xnew - xold).abs() > tol {
        xold = xnew;
        xnew = xold + f(xold) / fprime(xold);
        println!("{:#.6}", xnew);
    }
    return xnew;
}

pub fn newton_bond_yield(b: f64, cash_flow_dates: &[CFD], x0: f64, tol: f64) -> f64 {
    return newton(
        x0,
        tol,
        |x: f64| -> f64 { upper_sum(cash_flow_dates, x) - b },
        |x: f64| -> f64 { lower_sum(cash_flow_dates, x) },
    );
}

fn upper_sum(cash_flow_dates: &[CFD], xold: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |i, c| {
        return i + (c.cash_flow * E.powf(-xold * c.t));
    });
}

fn lower_sum(cash_flow_dates: &[CFD], xold: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |i, c| {
        return i + (c.cash_flow * c.t * E.powf(-xold * c.t));
    });
}

pub fn modified_duration(cash_flow_dates: &[CFD], b: f64, y: f64) -> f64 {
    return (1.0 / b) * duration(cash_flow_dates, y);
}

pub fn dollar_duration(cash_flow_dates: &[CFD], b: f64, y: f64) -> f64 {
    return -1.0 * duration(cash_flow_dates, y);
}

fn duration(cash_flow_dates: &[CFD], y: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |v, c| {
        return v + c.cash_flow * c.t * E.powf(-y * c.t);
    });
}

pub fn dollar_convexity(cash_flow_dates: &[CFD], y: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |v, c| {
        return v + c.cash_flow * c.t.powf(2.0) * E.powf(-y * c.t);
    });
}
pub fn convexity(cash_flow_dates: &[CFD], b: f64, y: f64) -> f64 {
    return (1.0 / b) * dollar_convexity(cash_flow_dates, y);
}

pub fn dv01(cash_flow_dates: &[CFD], b: f64, y: f64) -> f64 {
    return dollar_duration(cash_flow_dates, b, y) / 10000.0;
}

pub fn taylor_duration_price(price: f64, duration: f64, delta: f64) -> f64 {
    return price * (1.0 - duration * delta);
}

pub fn taylor_duration_convexity_price(
    price: f64,
    duration: f64,
    convexity: f64,
    delta: f64,
) -> f64 {
    return price * (1.0 - duration * delta + convexity / 2.0 * delta.powf(2.0));
}

#[cfg(test)]
mod newton_bond_tests {
    use super::*;
    #[test]
    fn upper_sum_test_one() {
        const cash_flows: [CFD; 1] = [CFD {
            t: (4.0 / 12.0),
            cash_flow: 4.0,
        }];
        assert_eq!(round(upper_sum(&cash_flows, 0.1), 6.0), 3.868864);
    }
    fn upper_sum_test_two() {
        const cash_flows: [CFD; 2] = [
            CFD {
                t: (4.0 / 12.0),
                cash_flow: 4.0,
            },
            CFD {
                t: (10.0 / 12.0),
                cash_flow: 4.0,
            },
        ];
        assert_eq!(round(upper_sum(&cash_flows, 0.1), 6.0), 7.549042);
    }
    #[test]
    fn lower_sum_test() {
        let cash_flows: [CFD; 1] = [CFD {
            t: (4.0 / 12.0),
            cash_flow: 4.0,
        }];
        assert_eq!(round(lower_sum(&cash_flows, 0.1), 6.0), 1.289621);
    }
    #[test]
    fn book_ex() {
        let cash_flows: [CFD; 6] = [
            CFD {
                t: (4.0 / 12.0),
                cash_flow: 4.0,
            },
            CFD {
                t: (10.0 / 12.0),
                cash_flow: 4.0,
            },
            CFD {
                t: (16.0 / 12.0),
                cash_flow: 4.0,
            },
            CFD {
                t: (22.0 / 12.0),
                cash_flow: 4.0,
            },
            CFD {
                t: (28.0 / 12.0),
                cash_flow: 4.0,
            },
            CFD {
                t: (34.0 / 12.0),
                cash_flow: 104.0,
            },
        ];

        assert_eq!(
            round(newton_bond_yield(105.0, &cash_flows, 0.1, 0.000001), 6.0),
            0.064502
        );
    }

    static book_q1_fixture: [CFD; 10] = [
        CFD {
            t: (6.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (12.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (18.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (24.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (30.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (36.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (42.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (48.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (54.0 / 12.0),
            cash_flow: 1.6875,
        },
        CFD {
            t: (60.0 / 12.0),
            cash_flow: 101.6875,
        },
    ];

    #[test]
    fn book_1() {
        assert_eq!(
            round(
                newton_bond_yield(100.0 + (1.0 / 32.0), &book_q1_fixture, 0.1, 0.000001),
                6.0
            ),
            0.033401
        );
    }
    #[test]
    fn duration_test() {
        assert_eq!(
            round(
                modified_duration(
                    &book_q1_fixture,
                    100.0 + (1.0 / 32.0),
                    newton_bond_yield(100.0 + (1.0 / 32.0), &book_q1_fixture, 0.1, 0.000001)
                ),
                6.0
            ),
            4.642735
        );
    }
    #[test]
    fn convexity_test() {
        assert_eq!(
            round(
                convexity(
                    &book_q1_fixture,
                    100.0 + (1.0 / 32.0),
                    newton_bond_yield(100.0 + (1.0 / 32.0), &book_q1_fixture, 0.1, 0.000001)
                ),
                6.0
            ),
            22.573118
        );
    }
    #[test]
    fn taylor_duration_test() {
        assert_eq!(
            round(taylor_duration_price(101.0, 1.5, 0.001), 4.0),
            100.8485
        );
        assert_eq!(
            round(taylor_duration_price(101.0, 1.5, 0.005), 4.0),
            100.2425
        );
        assert_eq!(round(taylor_duration_price(101.0, 1.5, 0.01), 4.0), 99.4850);
        assert_eq!(round(taylor_duration_price(101.0, 1.5, 0.02), 4.0), 97.9700);
    }
    #[test]
    fn taylor_duration_convexity_test() {
        assert_eq!(
            round(taylor_duration_convexity_price(101.0, 1.5, 2.5, 0.001), 4.0),
            100.8486
        );
        assert_eq!(
            round(taylor_duration_convexity_price(101.0, 1.5, 2.5, 0.005), 4.0),
            100.2457
        );
        assert_eq!(
            round(taylor_duration_convexity_price(101.0, 1.5, 2.5, 0.01), 4.0),
            99.4976
        );
        assert_eq!(
            round(taylor_duration_convexity_price(101.0, 1.5, 2.5, 0.02), 4.0),
            98.0205
        );
    }
}
