mod simpsons;
mod standard_normal;
use std::f64::consts::E;

fn main() {
    println!(
        "N(0.1): {}",
        standard_normal::cumulative_distribution(0.1, 4)
    );
    println!(
        "N(0.5): {}",
        standard_normal::cumulative_distribution(0.5, 4)
    );
}
