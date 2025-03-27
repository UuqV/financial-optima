mod bond_price;
mod newton;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!(
        "Use Newton’s method to find the yield of a three year semiannual coupon bond with 4%
coupon rate and price 101. What are the modified duration and the convexity of the bond?"
    );

    const q1: [bond_price::CFD; 6] = [
        bond_price::CFD {
            t: (6.0 / 12.0),
            cash_flow: 2.0,
        },
        bond_price::CFD {
            t: (12.0 / 12.0),
            cash_flow: 2.0,
        },
        bond_price::CFD {
            t: (18.0 / 12.0),
            cash_flow: 2.0,
        },
        bond_price::CFD {
            t: (24.0 / 12.0),
            cash_flow: 2.0,
        },
        bond_price::CFD {
            t: (30.0 / 12.0),
            cash_flow: 2.0,
        },
        bond_price::CFD {
            t: (36.0 / 12.0),
            cash_flow: 102.0,
        },
    ];

    println!("\nNewton's method iterations:\n");

    let n: f64 = newton::newton_bond_yield(101.0, &q1, 0.1, 0.000001);

    println!("\nYield via Newton's method: {:#.6}", n);

    println!("\nDuration: {:#.6}", newton::duration(&q1, 101.0, n));

    println!("\nConvexity: {:#.6}", newton::convexity(&q1, 101.0, n));

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
