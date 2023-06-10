use cargo_snippet::snippet;

#[snippet("is_lear_year")]
pub fn is_leap_year(n: usize) -> bool {
    n % 4 == 0 && !(n % 100 == 0 && n % 400 != 0)
}

#[snippet("is_valid_date")]
#[snippet(include = "is_lear_year")]
pub fn is_valid_date(y: usize, m: usize, d: usize) -> bool {
    if !(1..=12).contains(&m) {
        return false;
    }
    let last_day_each_month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let last_day = if m == 2 && is_leap_year(y) {
        29
    } else {
        last_day_each_month[m - 1]
    };
    (1..=last_day).contains(&d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2004, true)]
    #[case(2005, false)]
    #[case(2006, false)]
    #[case(2007, false)]
    #[case(2008, true)]
    fn test_is_lear_year(#[case] year: usize, #[case] expected: bool) {
        assert_eq!(
            is_leap_year(year),
            expected,
            "is_lear_year({}) == {}",
            year,
            expected
        );
    }

    #[rstest]
    #[case(2000, true)]
    #[case(2100, false)]
    #[case(2200, false)]
    #[case(2300, false)]
    #[case(2400, true)]
    fn test_is_lear_year_exceptional(#[case] year: usize, #[case] expected: bool) {
        assert_eq!(is_leap_year(year), expected);
    }

    #[rstest]
    #[case(2001, 0, 1, false)]
    #[case(2001, 1, 0, false)]
    #[case(2001, 1, 1, true)]
    #[case(2001, 1, 31, true)]
    #[case(2001, 1, 32, false)]
    #[case(2001, 2, 1, true)]
    #[case(2001, 2, 28, true)]
    #[case(2001, 2, 29, false)]
    #[case(2001, 3, 1, true)]
    #[case(2001, 3, 31, true)]
    #[case(2001, 3, 32, false)]
    #[case(2001, 4, 1, true)]
    #[case(2001, 4, 30, true)]
    #[case(2001, 4, 31, false)]
    #[case(2001, 5, 1, true)]
    #[case(2001, 5, 31, true)]
    #[case(2001, 5, 32, false)]
    #[case(2001, 6, 1, true)]
    #[case(2001, 6, 30, true)]
    #[case(2001, 6, 31, false)]
    #[case(2001, 7, 1, true)]
    #[case(2001, 7, 31, true)]
    #[case(2001, 7, 32, false)]
    #[case(2001, 8, 1, true)]
    #[case(2001, 8, 31, true)]
    #[case(2001, 8, 32, false)]
    #[case(2001, 9, 1, true)]
    #[case(2001, 9, 30, true)]
    #[case(2001, 9, 31, false)]
    #[case(2001, 10, 1, true)]
    #[case(2001, 10, 31, true)]
    #[case(2001, 10, 32, false)]
    #[case(2001, 11, 1, true)]
    #[case(2001, 11, 30, true)]
    #[case(2001, 11, 31, false)]
    #[case(2001, 12, 1, true)]
    #[case(2001, 12, 31, true)]
    #[case(2001, 12, 32, false)]
    fn test_is_valid_date_not_lear_year(
        #[case] year: usize,
        #[case] month: usize,
        #[case] day: usize,
        #[case] expected: bool,
    ) {
        assert_eq!(
            is_valid_date(year, month, day),
            expected,
            "is_valid_date({}, {}, {}) == {}",
            year,
            month,
            day,
            expected
        );
    }

    #[rstest]
    #[case(2000, 2, 1, true)]
    #[case(2000, 2, 28, true)]
    #[case(2000, 2, 29, true)]
    #[case(2000, 2, 30, false)]
    #[case(2000, 3, 1, true)]
    #[case(2000, 3, 31, true)]
    #[case(2000, 3, 32, false)]
    fn test_is_valid_date_lear_year(
        #[case] year: usize,
        #[case] month: usize,
        #[case] day: usize,
        #[case] expected: bool,
    ) {
        assert_eq!(
            is_valid_date(year, month, day),
            expected,
            "is_valid_date({}, {}, {}) == {}",
            year,
            month,
            day,
            expected
        );
    }
}
