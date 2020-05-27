pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
  F: Fn(i64) -> bool,
{
  let orig_ok = ok;
  let mut ok = ok;
  let mut ng = ng;
  while (ok - ng).abs() > 1 {
    let mid = (ok + ng) / 2;
    if pred(mid) {
      ok = mid;
    } else {
      ng = mid;
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
