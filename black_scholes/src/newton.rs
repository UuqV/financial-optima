pub fn newton(x0: f64, tol: f64, f: impl Fn(f64) -> f64, fprime: impl Fn(f64) -> f64) -> f64 {
    let mut xnew: f64 = x0;
    let mut xold: f64 = x0 - 1.0;
    while (xnew - xold).abs() > tol {
        xold = xnew;
        xnew = xold + f(xold) / fprime(xold);
        println!("{:#.6}", xnew);
    }
    return xnew;
}
