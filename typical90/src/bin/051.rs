#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub trait BinarySearchOk<T>: PartialEq + Copy {
    fn bs_needs_next_search(&self, ng: &T) -> bool;
    fn bs_mid_value(&self, ng: &T) -> Self;
    fn bs_into(&self) -> T;
}
impl BinarySearchOk<i64> for i64 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<i128> for i128 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<f64> for f64 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1.0
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2.0
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<isize> for isize {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (*self - *ng).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
pub fn bsearch<T, Num: BinarySearchOk<T>, F>(ok: Num, ng: T, pred: F) -> Option<Num>
where
    F: Fn(T) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while ok.bs_needs_next_search(&ng) {
        let mid = ok.bs_mid_value(&ng);
        if pred(mid.bs_into()) {
            ok = mid;
        } else {
            ng = mid.bs_into();
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, K: usize, P: i64,
        A: [i64; N],
    };

    let (a1, a2) = A.split_at(N / 2);

    let mut a1_price_with_count = vec![];
    for bits in 0..1 << a1.len() {
        let mut sum = 0;
        let mut count = 0_usize;
        for i in 0..a1.len() {
            if bits >> i & 1 == 1 {
                sum += a1[i];
                count += 1;
            }
        }
        a1_price_with_count.push((sum, count));
    }

    let mut a2_count_to_prices = HashMap::<usize, Vec<i64>>::new();
    for bits in 0..1 << a2.len() {
        let mut sum = 0;
        let mut count = 0_usize;
        for i in 0..a2.len() {
            if bits >> i & 1 == 1 {
                sum += a2[i];
                count += 1;
            }
        }
        a2_count_to_prices
            .entry(count)
            .or_insert(Vec::new())
            .push(sum);
    }
    let mut a2_count_to_sorted_prices = HashMap::<usize, Vec<i64>>::new();
    for (count, mut prices) in a2_count_to_prices {
        prices.sort();
        a2_count_to_sorted_prices.insert(count, prices);
    }

    let mut ans = 0_u64;
    for (a1_price, a1_count) in a1_price_with_count {
        if a1_price > P || a1_count > K {
            continue;
        }
        let rest_price = P - a1_price;
        let a2_count = K - a1_count;
        if let Some(a2_prices) = a2_count_to_sorted_prices.get(&a2_count) {
            ans += bsearch(-1_isize, a2_prices.len() as isize, |i| {
                a2_prices[i as usize] <= rest_price
            })
            .map(|i| i as u64 + 1)
            .unwrap_or(0);
        }
    }
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
