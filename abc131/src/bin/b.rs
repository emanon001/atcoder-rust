use proconio::input;

fn main() {
    input! {
        n: isize, l: isize
    };

    let r = l + n - 1;
    let res: isize = if l >= 0 {
        ((l + 1)..(l + n)).sum()
    } else if r >= 0 {
        (l..(l + n)).sum()
    } else {
        (l..(l + n - 1)).sum()
    };

    println!("{}", res);
}
