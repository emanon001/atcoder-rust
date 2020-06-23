use proconio::input;

fn main() {
    input! {
        a: usize,
        s: String
    };

    let res = if a >= 3200 { s } else { "red".to_string() };
    println!("{}", res);
}
