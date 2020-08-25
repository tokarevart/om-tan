use om_tan::*;

fn main() {
    let f = |x: f64| x * x - 9.0 * x + 14.0;
    let der = |x: f64| 2.0 * x - 9.0;
    let range = 0.0..6.0;
    let eps = 1e-4;
    let x = search(range, eps, f, der);
    println!("x         : {}", x);
    println!("x^2-9*x+14: {}", f(x));
    println!("2*x-9     : {}", der(x));
    println!("");

    const FRAC_2_3: f64 = 2.0 / 3.0;
    let f = |x: f64| x * (FRAC_2_3 * x * x + 0.25 * x - 7.0);
    let der = |x: f64| 2.0 * x * x + 0.5 * x - 7.0;
    let range = 1.0..6.0;
    let eps = 1e-4;
    let x = search(range, eps, f, der);
    println!("x                  : {}", x);
    println!("2/3*x^3+1/4*x^2-7*x: {}", f(x));
    println!("2*x^2+1/2*x-7      : {}", der(x));
    println!("");

    let f = |x: f64| 5.0 * x * (x * x - x - 1.0) + 10.0;
    let der = |x: f64| 15.0 * x * x - 10.0 * x - 5.0;
    let range = 0.5..5.0;
    let eps = 1e-4;
    let x = search(range, eps, f, der);
    println!("x                 : {}", x);
    println!("5*x^3-5*x^2-5*x+10: {}", f(x));
    println!("15*x^2-10*x-5     : {}", der(x));
    println!("");
}
