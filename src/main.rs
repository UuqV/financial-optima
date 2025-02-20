mod simpsons;
mod standard_normal;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!(
        "\nN(0.1): {}",
        standard_normal::cumulative_distribution_in_tolerance(0.1, 4)
    );
    println!(
        "\nN(0.5): {}",
        standard_normal::cumulative_distribution_in_tolerance(0.5, 4)
    );
    println!(
        "\nN(1.0): {}",
        standard_normal::cumulative_distribution_in_tolerance(1.0, 4)
    );

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
