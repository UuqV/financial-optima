mod black_scholes;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let put: f64 = black_scholes::black_scholes(25.0, 30.0, 0.30, 0.25, 0.02);
    println!(
        "10. i)\n\tPut option price based on B-S Formula (S = 25, K = 30, vol = 30%, T = 0.25, r = 2%):\n\n\t{:#.6}",
        put
    );

    println!(
        "\n\n\tPortfolio value (1000P + 400S + 10000):\n\n\t{:#.6}\n",
        1000.0 * put + 400.0 * 25.0 + 10000.0
    );

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
