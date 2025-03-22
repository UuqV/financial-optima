mod simpsons;
mod standard_normal;
use std::f64::consts::E;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    print_n(
        "N(0.1)",
        30.0 * E.powf(-0.02 * 0.25)
            * standard_normal::cumulative_distribution_standard_normal(1.323813, 4, 6.0)
            - 25.0 * standard_normal::cumulative_distribution_standard_normal(1.1173813, 4, 6.0),
    );

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}

fn print_n(name: &str, output: f64) {
    println!("\n{name}: {output:#.12}\n----------------------\n");
}
