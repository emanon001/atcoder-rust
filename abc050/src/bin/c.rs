use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [usize; n]
    };

    let mut counts = vec![0; n];
    for a in av {
        counts[a] += 1;
    }
    let is_ok = if n % 2 == 0 {
        (1..=n - 1).step_by(2).all(|x| counts[x] == 2)
    } else {
        (2..=n - 1).step_by(2).all(|x| counts[x] == 2) && counts[0] == 1
    };
    if !is_ok {
        println!("0");
        return;
    }
    let res = (1..=n / 2).fold(1_u64, |acc, _| acc * 2 % 1_000_000_007);
    println!("{}", res);
}
