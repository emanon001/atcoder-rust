use proconio::input;

fn main() {
    input! {
      a: usize, b: usize,
    };

    let ans = if a + b == 15 {
        "+"
    } else if a * b == 15 {
        "*"
    } else {
        "x"
    };
    println!("{}", ans);
}
