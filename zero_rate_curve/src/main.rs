mod bond_price;
mod newton;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}

fn pricing() {
    println!("Bond program output: ");
    println!(
        "10.\n i) {:#.6}",
        bond_price::bond_price_over_time(
            vec![
                bond_price::CFD {
                    t: (1.0 / 12.0),
                    cash_flow: 3.5,
                },
                bond_price::CFD {
                    t: (7.0 / 12.0),
                    cash_flow: 3.5,
                },
                bond_price::CFD {
                    t: (13.0 / 12.0),
                    cash_flow: 3.5,
                },
                bond_price::CFD {
                    t: (19.0 / 12.0),
                    cash_flow: 103.5,
                },
            ],
            |x: f64| 0.05 + 0.005 * (1.0 + x).sqrt(),
        )
    );
    println!(
        " ii) {:#.6}",
        bond_price::bond_price_over_time(
            vec![
                bond_price::CFD {
                    t: (7.0 / 12.0),
                    cash_flow: 4.0,
                },
                bond_price::CFD {
                    t: (19.0 / 12.0),
                    cash_flow: 104.0,
                },
            ],
            |x: f64| 0.02 + (x / (200.0 - x)),
        )
    );
    println!(
        "\n11.\n i) {:#.6}",
        bond_price::bond_price_over_time(
            vec![
                bond_price::CFD {
                    t: (1.0 / 12.0),
                    cash_flow: 2.0,
                },
                bond_price::CFD {
                    t: (7.0 / 12.0),
                    cash_flow: 2.0,
                },
                bond_price::CFD {
                    t: (13.0 / 12.0),
                    cash_flow: 2.0,
                },
                bond_price::CFD {
                    t: (19.0 / 12.0),
                    cash_flow: 102.0,
                },
            ],
            |x: f64| 0.02 + (x / (200.0 - x)),
        )
    );
    println!(
        " ii) {:#.6}",
        bond_price::bond_price_over_time(
            vec![
                bond_price::CFD {
                    t: (1.0 / 12.0),
                    cash_flow: 1.0,
                },
                bond_price::CFD {
                    t: (4.0 / 12.0),
                    cash_flow: 1.0,
                },
                bond_price::CFD {
                    t: (7.0 / 12.0),
                    cash_flow: 1.0,
                },
                bond_price::CFD {
                    t: (10.0 / 12.0),
                    cash_flow: 1.0,
                },
                bond_price::CFD {
                    t: (13.0 / 12.0),
                    cash_flow: 1.0,
                },
                bond_price::CFD {
                    t: (16.0 / 12.0),
                    cash_flow: 1.0,
                },
                bond_price::CFD {
                    t: (19.0 / 12.0),
                    cash_flow: 101.0,
                },
            ],
            |x: f64| 0.02 + (x / (200.0 - x)),
        )
    );
}
