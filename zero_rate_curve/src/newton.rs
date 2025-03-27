use crate::bond_price::{round, CFD};
use std::f64::consts::E;
use std::num;

fn newton_bond_yield(b: f64, cash_flow_dates: &Vec<CFD>, x0: f64, tol: f64) -> f64 {
    let mut xnew: f64 = x0;
    let mut xold: f64 = x0 - 1.0;
    while (xnew - xold).abs() > tol {
        xold = xnew;
        xnew = xold + (upper_sum(cash_flow_dates, xold) - b) / lower_sum(cash_flow_dates, xold);
        println!("{}", xnew);
    }
    return xnew;
}

fn upper_sum(cash_flow_dates: &Vec<CFD>, xold: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |i, c| {
        return i + (c.cash_flow * E.powf(-xold * c.t));
    });
}

fn lower_sum(cash_flow_dates: &Vec<CFD>, xold: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |i, c| {
        return i + (c.cash_flow * c.t * E.powf(-xold * c.t));
    });
}

#[cfg(test)]
mod newton_bond_tests {
    use super::*;
    #[test]
    fn upper_sum_test_one() {
        let cash_flows = vec![CFD {
            t: (4.0 / 12.0),
            cash_flow: 4.0,
        }];
        assert_eq!(round(upper_sum(&cash_flows, 0.1), 6.0), 3.868864);
    }
    fn upper_sum_test_two() {
        let cash_flows = vec![
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
        let cash_flows = vec![CFD {
            t: (4.0 / 12.0),
            cash_flow: 4.0,
        }];
        assert_eq!(round(lower_sum(&cash_flows, 0.1), 6.0), 1.289621);
    }
    #[test]
    fn book_ex() {
        let cash_flows = vec![
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
    #[test]
    fn book_1() {
        let cash_flows = vec![
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

        assert_eq!(
            round(
                newton_bond_yield(100.0 + (1.0 / 32.0), &cash_flows, 0.1, 0.000001),
                6.0
            ),
            0.033401
        );
    }
}
