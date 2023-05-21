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
pub fn rotate_coordinate(x: f64, y: f64, rad: f64) -> (f64, f64) {
    let x2 = x * rad.cos() - y * rad.sin();
    let y2 = x * rad.sin() + y * rad.cos();
    (x2, y2)
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
}
