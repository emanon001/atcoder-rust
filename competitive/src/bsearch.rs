use cargo_snippet::snippet;

#[snippet("bsearch")]
#[snippet]
pub trait BinarySearchOk<T>: PartialEq + Copy {
    fn bs_needs_next_search(&self, ng: &T) -> bool;
    fn bs_mid_value(&self, ng: &T) -> Self;
    fn bs_into(&self) -> T;
}

#[snippet("bsearch")]
#[snippet]
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

#[snippet("bsearch")]
#[snippet]
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

#[snippet("bsearch")]
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

#[cfg(test)]
mod tests {
    use super::bsearch;

    #[test]
    fn bsearch_min() {
        let vec = (0..=5).collect::<Vec<i32>>();
        let res = bsearch(vec.len() as i64, -1, |i| vec[i as usize] > 3);
        assert_eq!(res, Some(4));
    }

    #[test]
    fn bsearch_max() {
        let vec = (0..=5).collect::<Vec<i32>>();
        let res = bsearch(-1, vec.len() as i64, |i| vec[i as usize] < 3);
        assert_eq!(res, Some(2));
    }

    #[test]
    fn bsearch_all_ng() {
        let vec = (0..=5).collect::<Vec<i32>>();
        let res = bsearch(vec.len() as i64, -1, |i| vec[i as usize] > 5);
        assert_eq!(res, None);
    }

    #[test]
    fn bsearch_all_ok() {
        let vec = (0..=5).collect::<Vec<i32>>();
        let res = bsearch(vec.len() as i64, -1, |i| vec[i as usize] >= 0);
        assert_eq!(res, Some(0));
    }
}
