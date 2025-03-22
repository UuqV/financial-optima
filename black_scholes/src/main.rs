mod black_scholes;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    black_scholes::dist_test();

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
