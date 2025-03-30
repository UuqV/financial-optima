use crate::bond_price::{round, CFD};
use crate::newton::newton;
use std::f64::consts::E;

fn pricing(bond: &CFD, price: f64) -> f64 {
    return newton(
        0.05,
        0.0000001,
        |x: f64| -> f64 { 100.0 * E.powf(-0.5 * x) - price },
        |x: f64| -> f64 { -50.0 * E.powf(-0.5 * x) },
    );
}

#[cfg(test)]
mod bootstrapping_tests {
    use super::*;
    #[test]
    fn six_mo_semiannual() {
        const cash_flows: CFD = CFD {
            t: (6.0 / 12.0),
            cash_flow: 0.0,
        };
        assert_eq!(round(pricing(&cash_flows, 99.0), 6.0), 0.020101);
    }
}
