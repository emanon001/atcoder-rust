use proconio::input;

fn main() {
    input! {
        x: i64, y: i64
    };

    let res = x.max(y);
    println!("{}", res);
}
