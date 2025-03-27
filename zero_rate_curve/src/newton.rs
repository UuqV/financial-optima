use crate::bond_price::{round, CFD};

fn newton_bond_price(b: f64, cash_flow_dates: Vec<CFD>, tol: f64) -> f64 {
    return b;
}

#[cfg(test)]
mod newton_bond_tests {
    use super::*;

    #[test]
    fn book_ex() {
        assert_eq!(
            round(
                newton_bond_price(
                    105.0,
                    vec![
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
                    ],
                    0.000001
                ),
                4.0
            ),
            6.4502
        );
    }
}
