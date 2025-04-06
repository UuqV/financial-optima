use statrs::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;

pub fn black_scholes(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return k * E.powf(-r * t) * cdf(-d2(s, k, sigma, t, r, 0.0))
        - s * cdf(-d1(s, k, sigma, t, r, 0.0));
}

pub fn black_scholes_call(s: f64, k: f64, sigma: f64, t: f64, r: f64, q: f64) -> f64 {
    return s * E.powf(-q * t) * cdf(d1(s, k, sigma, t, r, q))
        - k * E.powf(-r * t) * cdf(d2(s, k, sigma, t, r, q));
}

pub fn delta(s: f64, k: f64, sigma: f64, t: f64, r: f64, q: f64) -> f64 {
    return cdf(-d1(s, k, sigma, t, r, q));
}

pub fn d1(s: f64, k: f64, sigma: f64, t: f64, r: f64, q: f64) -> f64 {
    return ((s / k).ln() + ((r - q) + (sigma.powf(2.0) / 2.0)) * t) / (sigma * t.sqrt());
}

fn d2(s: f64, k: f64, sigma: f64, t: f64, r: f64, q: f64) -> f64 {
    return d1(s, k, sigma, t, r, q) - (sigma * t.sqrt());
}

fn cdf(x: f64) -> f64 {
    let n = Normal::new(0.0, 1.0).unwrap();
    return n.cdf(x);
}

pub fn rebalance(
    s: Vec<f64>,
    k: f64,
    sigma: f64,
    mut big_t: f64,
    t_interval: f64,
    r: f64,
    mut options: f64,
    mut asset: f64,
    mut cash: f64,
) -> f64 {
    let mut option_price = 0.0;
    let mut options_delta = 0.0;
    let mut week = 0;
    for price in s.iter() {
        option_price = black_scholes(*price, k, sigma, big_t, r);
        println!(
            "Week {:#} - BH {:width$.6} {:width$.6} {:width$.6} {:width$.6}",
            week,
            options,
            asset,
            cash,
            options * option_price + asset * price + cash,
            width = 15
        );
        let delta = delta(*price, k, sigma, big_t, r, 0.0);

        options_delta = delta * -options + asset;

        cash += options_delta * price;
        asset += -options_delta;

        println!(
            "Week {:#} - HA                 {:width$.6} {:width$.6}",
            week,
            options_delta,
            -options_delta * price,
            width = 15
        );

        println!(
            "Week {:#} - AH {:width$.6} {:width$.6} {:width$.6}",
            week,
            options,
            asset,
            cash,
            width = 15
        );

        println!("----------------------------------------------------------------------------");

        cash = cash * E.powf(r);
        big_t = big_t - t_interval;
        week += 1;
    }
    return options_delta;
}

#[cfg(test)]
mod black_scholes_test {
    use super::*;

    fn round(x: f64, power: f64) -> f64 {
        return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
    }

    #[test]
    fn put_test() {
        assert_eq!(
            round(black_scholes(20.0, 25.0, 0.30, 0.5, 0.04), 6.0),
            4.927351
        );
    }
    #[test]
    fn delta_test() {
        assert_eq!(round(delta(20.0, 25.0, 0.30, 0.5, 0.04, 0.0), 3.0), 0.803);
    }
    #[test]
    fn rebalance_test() {
        assert_eq!(
            round(
                rebalance(
                    vec![20.0, 24.0],
                    25.0,
                    0.30,
                    0.5,
                    1.0 / 12.0,
                    0.04,
                    1000.0,
                    400.0,
                    10000.0
                ),
                1.0
            ),
            291.6
        );
    }
    #[test]
    fn bs_call_test() {
        assert_eq!(
            round(black_scholes_call(30.0, 30.0, 0.30, 0.5, 0.03, 0.01), 6.0),
            2.660205
        );
    }
}
