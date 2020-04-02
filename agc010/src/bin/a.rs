use proconio::input;

fn main() {
  input! {
    n: usize,
    av: [usize; n]
  };

  let even_count = av.iter().filter(|&x| x % 2 == 0).count();
  let odd_count = n - even_count;
  let res = if odd_count % 2 == 0 { "YES" } else { "NO" };
  println!("{}", res);
}
