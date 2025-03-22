use statrs::distribution::{Continuous, Normal};
use statrs::statistics::Distribution;

pub fn dist_test() {
    let n = Normal::new(0.0, 1.0).unwrap();
    assert_eq!(n.mean().unwrap(), 0.0);
    assert_eq!(n.pdf(1.0), 0.2419707245191433497978);
}

#[cfg(test)]
mod black_scholes_test {
    use super::*;

    #[test]
    fn dist_test() {
        let n = Normal::new(0.0, 1.0).unwrap();
        assert_eq!(n.mean().unwrap(), 0.0);
        assert_eq!(n.pdf(1.0), 0.2419707245191433497978);
    }
}
