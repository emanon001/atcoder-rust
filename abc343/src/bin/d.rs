#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, T: usize,
        AB: [(Usize1, u64); T],
    };

    let mut id_to_point = vec![0_u64; N];
    let mut point_to_ids = HashMap::new();
    let initial_ids = (0..N).collect::<HashSet<_>>();
    point_to_ids.insert(0_u64, initial_ids);

    for (a, b) in AB {
        let point = id_to_point[a];
        point_to_ids.get_mut(&point).unwrap().remove(&a);
        if point_to_ids[&point].is_empty() {
            point_to_ids.remove(&point);
        }
        let new_point = point + b;
        id_to_point[a] = new_point;
        point_to_ids
            .entry(new_point)
            .or_insert(HashSet::new())
            .insert(a);
        println!("{}", point_to_ids.len());
    }
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
