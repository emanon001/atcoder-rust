use proconio::input;

fn main() {
    input! {
        n: usize,
        mut vv: [usize; n]
    };

    vv.sort();
    let mut cur = vv[0] as f64;
    for i in 1..n {
        cur = (cur + vv[i] as f64) / 2.0;
    }
    println!("{}", cur);
}
