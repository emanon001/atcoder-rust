use proconio::input;

fn main() {
    input! {
        n: usize,
        mut abv: [(usize, usize); n]
    };

    abv.sort_by_key(|(_, b)| *b);
    let mut cur = 0;
    for (a, b) in abv {
        cur += a;
        if cur > b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
