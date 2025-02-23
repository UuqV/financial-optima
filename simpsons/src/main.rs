mod simpsons;
mod standard_normal;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    print_N(
        "N(0.1)",
        standard_normal::cumulative_distribution_in_tolerance(0.1, 4, 12.0),
    );
    print_N(
        "N(0.5)",
        standard_normal::cumulative_distribution_in_tolerance(0.5, 4, 12.0),
    );
    print_N(
        "N(1.0)",
        standard_normal::cumulative_distribution_in_tolerance(1.0, 4, 12.0),
    );

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}

fn print_N(name: &str, output: f64) {
    println!("\n{name}: {output:#.12}\n----------------------\n");
}
