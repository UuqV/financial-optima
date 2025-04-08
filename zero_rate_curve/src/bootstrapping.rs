use crate::bond_price::{round, CFD};
use crate::newton::newton;
use std::f64::consts::E;

use ndarray::Array1;

pub struct Bond {
    pub maturity: f64,    // Maturity in years
    pub coupon_rate: f64, // Annual coupon rate as a decimal
    pub price: f64,       // Current market price
}

fn pricing(sum: f64, bond: &Bond) -> f64 {
    return newton(
        0.05,
        0.000001,
        |x| {
            return sum + 100.0 * single_price(&bond, x) - bond.price;
        },
        |x| {
            return -100.0 * bond.maturity * single_price(&bond, x);
        },
    );
}

fn single_price(bond: &Bond, rate: f64) -> f64 {
    return (bond.coupon_rate / 2.0) * E.powf(-rate * bond.maturity);
}

pub fn bootstrap_zero_rates(bonds: Vec<Bond>) -> Array1<f64> {
    let mut zero_rates = Vec::with_capacity(bonds.len());
    let mut sum: f64 = 0.0;

    for (i, bond) in bonds.iter().enumerate() {
        if i == 0 {
            let zero_rate = (bond.price / 100.0).ln() / -bond.maturity;
            zero_rates.push(zero_rate);
            sum += single_price(bond, zero_rate);
        } else {
            let zero_rate = pricing(sum, &bond);
            zero_rates.push(zero_rate);
            sum += single_price(bond, zero_rate)
        }
    }

    return Array1::from(zero_rates);
}

#[cfg(test)]
mod bootstrapping_tests {
    use super::*;
    #[test]
    fn six_mo_semiannual() {}
}
