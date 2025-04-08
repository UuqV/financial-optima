use crate::bond_price::{round, CFD};
use crate::newton::newton;
use std::f64::consts::E;

use ndarray::Array1;

pub struct Bond {
    pub maturity: f64,    // Maturity in years
    pub coupon_rate: f64, // Annual coupon rate as a decimal
    pub price: f64,       // Current market price
}

fn pricing(bonds: Vec<Bond>, zero_rates: &Vec<f64>, discount_factors: &Vec<f64>) -> f64 {
    return newton(
        x0,
        tol,
        |x| {
            return discount_factors.iter().enumerate().fold(0.0, |i, df| {
                let coupon_payment = if j < i {
                    bonds[j].coupon_rate * bonds[j].price
                } else {
                    0.0
                };
                return coupon_payment * df;
            });
        },
        |x| {},
    );
}

pub fn bootstrap_zero_rates(bonds: Vec<Bond>) -> Array1<f64> {
    let mut zero_rates = Vec::with_capacity(bonds.len());
    let mut discount_factors = Vec::new();

    for (i, bond) in bonds.iter().enumerate() {
        if i == 0 {
            // For the first bond (short-term), the zero rate is straightforward
            let zero_rate = (bond.price / 100.0).ln() / -bond.maturity;
            zero_rates.push(zero_rate);
            discount_factors.push(1.0 / (1.0 + zero_rate));
        } else {
            let zero_rate = pricing(bonds, &zero_rates, &discount_factors);
            zero_rates.push(zero_rate);
            discount_factors.push(1.0 / (1.0 + zero_rate));
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
