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

#[cfg(test)]
mod tests {
    use super::*;

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
}
