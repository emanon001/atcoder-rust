use cargo_snippet::snippet;

#[snippet("monoid")]
pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}
