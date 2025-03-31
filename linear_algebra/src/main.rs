mod lu;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
