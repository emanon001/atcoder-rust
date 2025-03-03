#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [usize; N],
    };

    let mut ans = usize::max_value();
    let mut counts = vec![0; 10.pow(6) + 1];
    let mut l = 0;
    let mut r = 0;
    while r < N {
        // 重複するまで右端を伸ばす
        while r < N {
            counts[A[r]] += 1;
            if counts[A[r]] > 1 {
                break;
            }
            r += 1;
        }
        // 重複していない場合は、処理を打ち切る
        if r >= N {
            break;
        }
        chmin!(ans, r - l + 1);
        // 重複している場合は、重複しなくなるまで左端を右にずらす
        while l <= r {
            counts[A[l]] -= 1;
            if counts[A[l]] == 1 {
                l += 1;
                r += 1;
                break;
            } else {
                l += 1;
                chmin!(ans, r - l + 1);
            }
        }
    }
    let ans = if ans == usize::max_value() {
        -1
    } else {
        ans as isize
    };
    println!("{}", ans);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
