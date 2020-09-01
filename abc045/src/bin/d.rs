use proconio::input;
use proconio::marker::*;
use std::collections::HashMap;

pub const ALL_DIRS: [(isize, isize); 8] = [
    // 時計回り
    (-1, 0),  // 上
    (-1, 1),  // 右上
    (0, 1),   // 右
    (1, 1),   // 右下
    (1, 0),   // 下
    (1, -1),  // 左下
    (0, -1),  // 左
    (-1, -1), // 左上
];

fn main() {
    input! {
        h: usize, w: usize, n: usize,
        abv: [(Usize1, Usize1); n]
    };

    let is_ok = |i: usize, j: usize| -> bool { i > 0 && i < h - 1 && j > 0 && j < w - 1 };
    let mut table = HashMap::new();
    for (a, b) in abv {
        let i = a;
        let j = b;
        if is_ok(i, j) {
            *table.entry((i, j)).or_insert(0) += 1;
        }
        for &(di, dj) in &ALL_DIRS {
            let new_i = i as isize + di;
            let new_j = j as isize + dj;
            if new_i < 0 || new_i >= h as isize || new_j < 0 || new_j >= w as isize {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if is_ok(new_i, new_j) {
                *table.entry((new_i, new_j)).or_insert(0) += 1;
            }
        }
    }
    let all_count = (w - 3 + 1) * (h - 3 + 1);
    let mut counts = vec![0; 10];
    for (_, c) in table {
        counts[c] += 1;
    }
    let zero_count = all_count - counts.iter().skip(1).sum::<usize>();
    counts[0] = zero_count;
    for c in counts {
        println!("{}", c);
    }
}
