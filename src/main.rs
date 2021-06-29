use std::f64::NAN;

fn main() {
    let solution = solve_bisection(problem, 0.0, 10.0, 0.00002, 20);
    println!("solution = {}", solution);
}

fn problem(x: f64) -> f64 {
    return 3.0 * x.powi(3) - 24.0;
}

fn solve_bisection(
    problem: fn(f64) -> f64,
    neg: f64,
    pos: f64,
    precision: f64,
    max_iter: i64,
) -> f64 {
    let mut a = neg;
    let mut b = pos;

    let c = NAN;
    for _ in 1..max_iter {
        let c = (a + b) / 2.0;
        let mid = (b - a) / 2.0;
        let fc = problem(c);
        if fc == 0.0 || mid < precision {
            return c;
        }
        let c_is_neg = fc.is_sign_negative();
        let fa = problem(a);
        let a_is_neg = fa.is_sign_negative();
        if c_is_neg == a_is_neg {
            a = c;
        } else {
            b = c;
        }
    }
    return c;
}
