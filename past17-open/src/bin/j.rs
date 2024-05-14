#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn compress_zahyo<T: Ord + std::hash::Hash + Clone>(
    zahyo: &[T],
) -> (
    std::collections::HashMap<T, usize>,
    std::collections::HashMap<usize, T>,
) {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x.clone());
    }
    let mut map = std::collections::HashMap::new();
    let mut inverse_map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x.clone(), i);
        inverse_map.insert(i, x);
    }
    (map, inverse_map)
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        AB: [(usize, usize); N],
        Q: usize,
        T: [usize; Q],
    };

    let mut times = HashSet::new();
    for &(a, b) in &AB {
        times.insert(a);
        times.insert(b + 1);
    }
    for &t in &T {
        times.insert(t);
    }

    let (zahyo_map, _) = compress_zahyo(&times.into_iter().collect::<Vec<_>>());
    let mut imos = vec![0; zahyo_map.len()];
    for &(a, b) in &AB {
        let a = *zahyo_map.get(&a).unwrap();
        let b = *zahyo_map.get(&(b + 1)).unwrap();
        imos[a] += 1;
        imos[b] -= 1;
    }
    for i in 1..imos.len() {
        imos[i] += imos[i - 1];
    }
    for t in T {
        let t = *zahyo_map.get(&t).unwrap();
        println!("{}", imos[t]);
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
