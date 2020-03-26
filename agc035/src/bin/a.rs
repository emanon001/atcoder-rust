use proconio::input;

fn main() {
  input! {
    n: usize,
    a_l: [usize; n]
  };

  let mut table = std::collections::HashMap::new();
  for a in a_l {
    let x = table.entry(a).or_insert(0);
    *x += 1;
  }

  if table.iter().count() > 3 {
    println!("No");
    std::process::exit(0);
  }

  for (&k1, &v1) in &table {
    for (&k2, &v2) in &table {
      if k1 == k2 && v1 < 2 {
        continue;
      }
      let k3 = k1 ^ k2;
      if k1 == k2 && k2 == k3 && v1 < 3 {
        continue;
      }
      if let Some(&v3) = table.get(&k3) {
        let is_ok = if k1 == k2 && k2 == k3 {
          // 0, 0, 0
          n == v1
        } else if k1 == k2 {
          // 1, 1, 0
          v1 == v3 * 2 && v1 + v3 == n
        } else if k1 == k3 {
          // 1, 0, 1
          v1 == v2 * 2 && v1 + v2 == n
        } else if k2 == k3 {
          // 0, 1, 1
          v1 * 2 == v2 && v1 + v2 == n
        } else {
          // 1, 2, 3
          v1 == v2 && v2 == v3 && v1 + v2 + v3 == n
        };
        let res = if is_ok { "Yes" } else { "No" };
        println!("{}", res);
        std::process::exit(0);
      }
    }
  }
  println!("No");
}
