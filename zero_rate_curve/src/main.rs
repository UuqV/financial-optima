mod bond_price;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("Run the tests");

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
