use cargo_snippet::snippet;

#[snippet]
pub fn rest_square_coordinates(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let c = (b.0 - b.1 + a.1, b.1 + b.0 - a.0);
    let d = (c.0 - c.1 + b.1, c.1 + c.0 - b.0);
    (c, d)
}

#[snippet]
pub fn distance_coordinates(a: (f64, f64), b: (f64, f64)) -> f64 {
    let h_distance = (a.0 - b.0).abs();
    let v_distance = (a.1 - b.1).abs();
    ((h_distance * h_distance) + (v_distance * v_distance)).sqrt()
}

#[snippet]
pub fn rotate_coordinate(x: f64, y: f64, radian: f64) -> (f64, f64) {
    let x2 = x * radian.cos() - y * radian.sin();
    let y2 = x * radian.sin() + y * radian.cos();
    (x2, y2)
}

#[snippet("circle")]
type Circle = (isize, isize, isize);

#[snippet("circle")]
pub fn is_inside_circle(c1: Circle, c2: Circle) -> bool {
    let dx = c1.0 - c2.0;
    let dy = c1.1 - c2.1;
    let dr = c1.2 - c2.2;
    dx * dx + dy * dy < dr * dr
}

#[snippet("circle")]
pub fn is_outside_circle(c1: Circle, c2: Circle) -> bool {
    let dx = c1.0 - c2.0;
    let dy = c1.1 - c2.1;
    let dr = c1.2 + c2.2;
    dx * dx + dy * dy > dr * dr
}

#[snippet]
pub fn is_sections_overlapping(
    l1: i64,
    r1: i64,
    l2: i64,
    r2: i64,
    include_side_by_side: bool,
) -> bool {
    assert!(l1 <= r1 && l2 <= r2);

    // l1 <= l2 <= r2 <= r1
    (l1 <= l2 && l2 <= r2 && r2 <= r1) ||
    // l1 <= l2 < r1 <= r2
    (l1 <= l2 && l2 < r1 && r1 <= r2) ||
    // l1 <= r1 == l2 <= r2
    (l1 <= r1 && (include_side_by_side && r1 == l2) && l2 <= r2) ||
    // l2 <= l1 <= r1 <= r2
    (l2 <= l1 && l1 <= r1 && r1 <= r2) ||
    // l2 <= l1 < r2 <= r1
    (l2 <= l1 && l1 < r2 && r2 <= r1) ||
    // l2 <= r2 == l1 <= r1
    (l2 <= r2 && (include_side_by_side && r2 == l1) && l1 <= r1)
}

#[snippet]
pub fn sections_overlapping_size(l1: i64, r1: i64, l2: i64, r2: i64) -> i64 {
    assert!(l1 <= r1 && l2 <= r2);

    // l1, l2, r2, r1
    if l1 <= l2 && l2 <= r2 && r2 <= r1 {
        return r2 - l2 + 1;
    }
    // l1, l2, r1, r2
    if l1 <= l2 && l2 <= r1 && r1 <= r2 {
        return r1 - l2 + 1;
    }
    // l2, l1, r1, r2
    if l2 <= l1 && l1 <= r1 && r1 <= r2 {
        return r1 - l1 + 1;
    }
    // l2, l1, r2, r1
    if l2 <= l1 && l1 <= r2 && r2 <= r1 {
        return r2 - l1 + 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_rest_square_coordinates() {
        assert_eq!(rest_square_coordinates((0, 0), (0, 1)), ((-1, 1), (-1, 0)));
        assert_eq!(rest_square_coordinates((2, 3), (6, 6)), ((3, 10), (-1, 7)));
        assert_eq!(
            rest_square_coordinates((31, -41), (-59, 26)),
            ((-126, -64), (-36, -131)),
        );
    }

    #[test]
    fn test_distance_coordinates() {
        assert_eq!(distance_coordinates((0.0, 0.0), (1.0, 2.0)), (5_f64).sqrt());
    }

    #[rstest]
    #[case(2.0, 2.0, 180_f64.to_radians(), (-2_f64, -2_f64))]
    #[case(15.0, 5.0, 360_f64.to_radians(), (15_f64, 4.999999_f64))]
    #[case(0.0, 0.0, 11_f64.to_radians(), (0_f64, 0_f64))]
    fn test_rotate_coordinate(
        #[case] x: f64,
        #[case] y: f64,
        #[case] radian: f64,
        #[case] expected: (f64, f64),
    ) {
        let (x2, y2) = rotate_coordinate(x, y, radian);
        let n = 1000000_f64;
        assert!(((x2 * n) as i64 - (expected.0 * n) as i64).abs() <= 1);
        assert!(((y2 * n) as i64 - (expected.1 * n) as i64).abs() <= 1);
    }

    #[test]
    fn test_is_sections_overlapping() {
        // l1 <= l2 <= r1 <= r2
        assert!(is_sections_overlapping(1, 3, 2, 4, false));
        // l1 <= l2 == r1 <= r2
        assert!(is_sections_overlapping(1, 3, 3, 4, true));
        assert!(!is_sections_overlapping(1, 3, 3, 4, false));
        // l2 <= l1 <= r2 <= r1
        assert!(is_sections_overlapping(2, 4, 1, 3, false));
        // l2 <= r2 == l1 <= r1
        assert!(is_sections_overlapping(3, 4, 1, 3, true));
        assert!(!is_sections_overlapping(3, 4, 1, 3, false));
        // l1 <= l2 <= r2 <= r1
        assert!(is_sections_overlapping(1, 4, 2, 3, false));
        assert!(is_sections_overlapping(1, 4, 1, 4, false));
        // l2 <= l1 <= r1 <= r2
        assert!(is_sections_overlapping(2, 3, 1, 4, false));

        assert!(!is_sections_overlapping(1, 3, 4, 5, false));
        assert!(!is_sections_overlapping(4, 5, 1, 3, false));
    }

    #[test]
    fn test_sections_overlapping_size() {
        // l1, l2, r1, r2
        assert_eq!(sections_overlapping_size(1, 3, 2, 4), 2);
        // l2, l1, r2, r1
        assert_eq!(sections_overlapping_size(2, 4, 1, 3), 2);
        assert_eq!(sections_overlapping_size(3, 4, 1, 3), 1);
        // l1, l2, r2, r1
        assert_eq!(sections_overlapping_size(1, 4, 2, 3), 2);
        assert_eq!(sections_overlapping_size(1, 4, 1, 4), 4);
        // l2, l1, r1, r2
        assert_eq!(sections_overlapping_size(2, 3, 1, 4), 2);

        assert_eq!(sections_overlapping_size(1, 3, 4, 5), 0);
    }
}
