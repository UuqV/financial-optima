mod simpsons;
mod standard_normal;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    print_n(
        "N(0.1)",
        standard_normal::cumulative_distribution_in_tolerance(0.1, 4, 6.0),
    );
    print_n(
        "N(0.5)",
        standard_normal::cumulative_distribution_in_tolerance(0.5, 4, 6.0),
    );
    print_n(
        "N(1.0)",
        standard_normal::cumulative_distribution_in_tolerance(1.0, 4, 6.0),
    );

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}

fn print_n(name: &str, output: f64) {
    println!("\n{name}: {output:#.12}\n----------------------\n");
}
