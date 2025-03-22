mod black_scholes;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    black_scholes::black_scholes(25.0, 30.0, 0.30, 0.25);

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
