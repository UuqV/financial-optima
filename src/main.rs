mod simpsons;
mod standard_normal;

fn main() {
    println!(
        "N(0.1): {}",
        standard_normal::cumulative_distribution(0.1, 128)
    );
    println!(
        "N(0.5): {}",
        standard_normal::cumulative_distribution(0.5, 128)
    );
    println!(
        "N(1.0): {}",
        standard_normal::cumulative_distribution(1.0, 128)
    );
}
