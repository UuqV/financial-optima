mod black_scholes;
mod implied_vol;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    implied_vol::implied_vol(2.5, 30.0, 30.0, 0.5, 0.03, 0.01, 0.5);

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}

fn bs() {
    println!(
        "3. i)\n\tThe delta of the put option position is -1000N(-d1)\n\t(S = 100, K = 100, vol = 30%, T = 0.5, r = 5%):\n\n\t{:#.6}\n",
        -1000.0 * black_scholes::delta(100.0, 100.0, 0.30, 0.5, 0.05, 0.0)
    );

    let put: f64 = black_scholes::black_scholes(100.0, 100.0, 0.30, 0.5, 0.05);
    println!(
        "\tP1 Put option price based on B-S Formula (S = 100, K = 100, vol = 30%, T = 0.5, r = 5%):\n\n\t{:#.6}",
        put
    );

    let put: f64 = black_scholes::black_scholes(102.0, 100.0, 0.30, 125.0 / 252.0, 0.05);
    println!(
        "\n\tP2 Put option price based on B-S Formula (S = 102, K = 100, vol = 30%, T = 125/252, r = 5%):\n\n\t{:#.6}",
        put
    );

    // 5

    let put: f64 = black_scholes::black_scholes(25.0, 30.0, 0.30, 0.25, 0.02);
    println!(
        "5. i)\n\tPut option price based on B-S Formula (S = 25, K = 30, vol = 30%, T = 0.25, r = 2%):\n\n\t{:#.6}",
        put
    );

    println!(
        "\n\n\tPortfolio value (1000P + 400S + 10000):\n\n\t{:#.6}\n",
        1000.0 * put + 400.0 * 25.0 + 10000.0
    );

    println!("iii)");

    black_scholes::rebalance(
        vec![25.0, 30.0, 26.0, 22.0, 27.0],
        30.0,
        0.30,
        0.25,
        1.0 / 52.0,
        0.02,
        1000.0,
        400.0,
        10000.0,
    );
}
