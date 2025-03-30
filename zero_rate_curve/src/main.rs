mod bond_price;
mod newton;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    hw5_9();

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}

fn hw5_3() {
    println!(
        "3. Use Newton’s method to find the yield of a three year semiannual coupon bond with 4%
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
}

fn hw5_9() {
    println!("\n9. The yield of a two year quarterly coupon bond with coupon rate 8% is 9%.");

    const q9: [bond_price::CFD; 6] = [
        bond_price::CFD {
            t: (3.0 / 12.0),
            cash_flow: 4.0,
        },
        bond_price::CFD {
            t: (6.0 / 12.0),
            cash_flow: 4.0,
        },
        bond_price::CFD {
            t: (9.0 / 12.0),
            cash_flow: 4.0,
        },
        bond_price::CFD {
            t: (12.0 / 12.0),
            cash_flow: 4.0,
        },
        bond_price::CFD {
            t: (18.0 / 12.0),
            cash_flow: 4.0,
        },
        bond_price::CFD {
            t: (24.0 / 12.0),
            cash_flow: 104.0,
        },
    ];

    println!("\ni) What are the price B, duration D, and convexity C of the bond?");

    let y = 0.09;
    let price = bond_price::bond_price_over_time(&q9, |x: f64| 0.09);
    let duration = newton::duration(&q9, price, y);
    let convexity = newton::convexity(&q9, price, y);

    println!("\nBond price: {:#.6}", price);

    println!("\nDuration: {:#.6}", duration);

    println!("\nConvexity: {:#.6}", convexity);

    println!("\n       delta y      B(new, D)       B(new,D,C)      B(y + delta y)     Err(B(new, D))   Err(B(new,D,C))");
    bond_price::taylor_bond_price_deltas(
        price,
        &q9,
        duration,
        convexity,
        0.09,
        &[0.001, 0.005, 0.01, 0.02, 0.04],
    );
}

fn pricing() {
    println!("Bond program output: ");
    println!(
        "10.\n i) {:#.6}",
        bond_price::bond_price_over_time(
            &[
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
            &[
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
            &[
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
            &[
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
