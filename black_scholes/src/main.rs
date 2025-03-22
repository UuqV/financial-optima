mod black_scholes;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!(
        "3. i)\n\tThe delta of the put option position is -1000N(-d1)\n\t(S = 100, K = 100, vol = 30%, T = 0.5, r = 5%):\n\n\t{:#.6}\n",
        -1000.0 * black_scholes::delta(100.0, 100.0, 0.30, 0.5, 0.05)
    );

    let put: f64 = black_scholes::black_scholes(100.0, 100.0, 0.30, 0.5, 0.05);
    println!(
        "\tPut option price based on B-S Formula (S = 100, K = 100, vol = 30%, T = 0.5, r = 5%):\n\n\t{:#.6}",
        put
    );

    let put: f64 = black_scholes::black_scholes(102.0, 100.0, 0.30, 125.0 / 252.0, 0.05);
    println!(
        "\n\tii) Put option price based on B-S Formula (S = 102, K = 100, vol = 30%, T = 125/252, r = 5%):\n\n\t{:#.6}",
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

    let delta: f64 = black_scholes::delta(25.0, 30.0, 0.30, 0.25, 0.02);

    println!(
        "ii)\n\tThe delta of the put option position is -1000N(-d1):\n\n\t{:#.6}\n",
        -1000.0 * delta
    );

    println!(
        "\tThe delta of the portfolio is the delta of the put option + 400:\n\n\t{:#.6}\n",
        -1000.0 * delta + 400.0
    );

    println!(
        "\tWe will purchase the above amount of shares for:\n\n\t{:#.6}\n",
        (1000.0 * delta + 400.0) * 20.0
    );

    println!(
        "\tUsing the 10,000 in cash we have and borrowing the rest at the risk-free rate:\n\n\t{:#.6}\n",
        (1000.0 * delta + 400.0) * 20.0
    );

    rebalance(
        vec![30.0, 26.0, 22.0, 27.0],
        25.0,
        0.30,
        0.5,
        0.04,
        1000.0,
        1265.8841,
        -15394.46221
    )

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
