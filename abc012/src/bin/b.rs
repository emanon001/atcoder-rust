use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    let h = n / (60 * 60);
    n = n % (60 * 60);
    let m = n / 60;
    n = n % 60;
    let s = n;
    println!("{0:02}:{1:02}:{2:02}", h, m, s);
}
