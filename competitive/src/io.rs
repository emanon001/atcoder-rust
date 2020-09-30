use cargo_snippet::snippet;

#[snippet]
pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
