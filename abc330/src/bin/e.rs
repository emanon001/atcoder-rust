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
) -> std::collections::HashMap<T, usize> {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x.clone());
    }
    let mut map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x, i);
    }
    map
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, Q: usize,
        mut A: [usize; N],
        queries: [(Usize1, usize); Q],
    };

    let mut candidates = vec![0];
    let query_values = queries.iter().map(|(_, x)| x).collect::<Vec<_>>();
    for &a in A.iter().chain(query_values) {
        candidates.push(a);
        candidates.push(a + 1);
    }
    let zahyo = compress_zahyo(&candidates);
    let reversed_zahyo = zahyo
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect::<HashMap<_, _>>();
    let mut counts = vec![0; zahyo.len()];
    let mut set = (0..zahyo.len()).collect::<BTreeSet<_>>();
    for &a in &A {
        let i = *zahyo.get(&a).unwrap();
        counts[i] += 1;
        set.remove(&i);
    }
    for (i, x) in queries {
        let new_v = *zahyo.get(&x).unwrap();
        let prev_v = *zahyo.get(&A[i]).unwrap();
        counts[prev_v] -= 1;
        if counts[prev_v] == 0 {
            set.insert(prev_v);
        }
        A[i] = x;
        counts[new_v] += 1;
        set.remove(&new_v);
        let v = set.iter().next().unwrap();
        let ans = reversed_zahyo.get(v).unwrap();
        println!("{}", ans);
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
