pub fn rest_square_coordinates(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let c = (b.0 - b.1 + a.1, b.1 + b.0 - a.0);
    let d = (c.0 - c.1 + b.1, c.1 + c.0 - b.0);
    (c, d)
}

#[cfg(test)]
mod tests {
    use super::rest_square_coordinates;

    #[test]
    fn test_rest_square_coordinates() {
        assert_eq!(rest_square_coordinates((0, 0), (0, 1)), ((-1, 1), (-1, 0)));
        assert_eq!(rest_square_coordinates((2, 3), (6, 6)), ((3, 10), (-1, 7)));
        assert_eq!(
            rest_square_coordinates((31, -41), (-59, 26)),
            ((-126, -64), (-36, -131)),
        );
    }
}
