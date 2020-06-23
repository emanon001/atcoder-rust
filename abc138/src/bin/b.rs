use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [f64; n]
    };

    let res = 1.0 / av.into_iter().map(|a| 1.0 / a).sum::<f64>();
    println!("{}", res);
}
