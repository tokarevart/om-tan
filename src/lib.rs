pub use std::ops::Range;

pub fn search(range: Range<f64>, eps: f64, f: impl Fn(f64) -> f64, der: impl Fn(f64) -> f64) -> f64 {
    assert!(eps > 0.0);

    let mut a = range.start;
    let mut b = range.end;
    let mut dera = der(a);
    let mut derb = der(b);
    let mut x = (f(a) - f(b) + b * derb - a * dera) / (derb - dera);
    let mut derx = der(x);

    while derx.abs() > eps {
        if derx < 0.0 {
            a = x;
            dera = derx;
        } else {
            b = x;
            derb = derx;
        }

        x = (f(a) - f(b) + b * derb - a * dera) / (derb - dera);
        derx = der(x);
    }

    x
}
