mod lu;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    lu::decompose();

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
