mod black_scholes;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    black_scholes::cdf(0.5);

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
