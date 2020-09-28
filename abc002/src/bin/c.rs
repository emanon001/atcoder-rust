use proconio::input;

fn main() {
    input! {
        a: (f64, f64),
        b: (f64, f64),
        c: (f64, f64),
    };

    let res = (((b.0 - a.0) * (c.1 - a.1)) - ((b.1 - a.1) * (c.0 - a.0))).abs() / 2.0;
    println!("{}", res);
}
