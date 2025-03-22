use statrs::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;

pub fn black_scholes(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return k * E.powf(-r * t) * cdf(-d2(s, k, sigma, t, r)) - s * cdf(-d1(s, k, sigma, t, r));
}

pub fn delta(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return cdf(-d1(s, k, sigma, t, r));
}

fn d1(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return ((s / k).ln() + (r + (sigma.powf(2.0) / 2.0)) * t) / (sigma * t.sqrt());
}

fn d2(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return d1(s, k, sigma, t, r) - (sigma * t.sqrt());
}

fn cdf(x: f64) -> f64 {
    let n = Normal::new(0.0, 1.0).unwrap();
    return n.cdf(x);
}

pub fn rebalance(
    s: Vec<f64>,
    k: f64,
    sigma: f64,
    mut t: f64,
    t_interval: f64,
    r: f64,
    mut options: f64,
    mut asset: f64,
    mut cash: f64,
) -> f64 {
    let mut option_price = black_scholes(s[0], k, sigma, t, r);
    println!("            Options     Asset     Cash         Value");
    println!(
        "Week 0      {:width$.2} {:width$.2} {:width$.2} {:width$.2}",
        options,
        asset,
        cash,
        options * option_price + asset * s[0] + cash,
        width = 10
    );
    println!("----------------------------------------------------------");
    let mut week = 1;
    for price in s.iter() {
        cash = cash * E.powf(r);
        t = t - t_interval;
        option_price = black_scholes(*price, k, sigma, t, r);
        println!(
            "Week {:#} - BH {:width$.2} {:width$.2} {:width$.2} {:width$.2}",
            week,
            options,
            asset,
            cash,
            options * option_price + asset * price + cash,
            width = 10
        );
        let delta = delta(*price, k, sigma, t, r);

        let options_delta = delta * -options + asset;

        cash += -options_delta * price;
        asset += -options_delta;

        println!(
            "Week {:#} - HA            {:width$.2} {:width$.2}",
            week,
            options_delta,
            -options_delta * price,
            width = 10
        );

        println!(
            "Week {:#} - AH {:width$.2} {:width$.2} {:width$.2}",
            week,
            options,
            asset,
            cash,
            width = 10
        );

        println!("----------------------------------------------------------");

        week += 1;
    }
    return options * option_price + asset * s[week - 2] + cash;
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
        assert_eq!(round(delta(20.0, 25.0, 0.30, 0.5, 0.04), 3.0), 0.803);
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
                6.0
            ),
            23400.0
        );
    }
}
