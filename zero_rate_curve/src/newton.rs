use crate::bond_price::{round, CFD};
use std::f64::consts::E;
use std::num;

fn newton_bond_price(b: f64, cash_flow_dates: &Vec<CFD>, tol: f64) -> f64 {
    let x0: f64 = 0.1;
    let mut xnew: f64 = x0;
    let mut xold: f64 = x0 - 1.0;
    while (xnew - xold).abs() > tol {
        xold = xnew;
        xnew = xold + (upperSum(cash_flow_dates, xold) - b) / lowerSum(cash_flow_dates, xold);
        println!("{}", xnew);
    }
    return xnew;
}

fn upperSum(cash_flow_dates: &Vec<CFD>, xold: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |i, c| {
        return c.cash_flow * E.powf(-xold * c.t);
    });
}

fn lowerSum(cash_flow_dates: &Vec<CFD>, xold: f64) -> f64 {
    return cash_flow_dates.into_iter().fold(0.0, |i, c| {
        return c.cash_flow * c.t * E.powf(-xold * c.t);
    });
}

#[cfg(test)]
mod newton_bond_tests {
    use super::*;

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
            round(newton_bond_price(105.0, &cash_flows, 0.000001), 4.0),
            6.4502
        );
    }
}
