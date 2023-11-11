use cargo_snippet::snippet;

#[snippet]
pub fn rotate90<T: Clone>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!grid.is_empty());

    (0..grid[0].len())
        .map(|col_i| {
            (0..grid.len())
                .rev()
                .map(|row_i| grid[row_i][col_i].clone())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate90() {
        let grid = vec![vec![1, 0, 1], vec![0, 1, 1], vec![1, 1, 0]];
        let grid90 = rotate90(grid);
        assert_eq!(grid90, vec![vec![1, 0, 1], vec![1, 1, 0], vec![0, 1, 1],]);
        let grid180 = rotate90(grid90);
        assert_eq!(grid180, vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 0, 1],]);
        let grid270 = rotate90(grid180);
        assert_eq!(grid270, vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 0, 1],]);
        let grid360 = rotate90(grid270);
        assert_eq!(grid360, vec![vec![1, 0, 1], vec![0, 1, 1], vec![1, 1, 0],]);
    }
    #[test]
    fn test_rotate90_col_row_size_different() {
        let grid = vec![vec![1, 0, 1], vec![0, 1, 1]];
        let grid90 = rotate90(grid);
        assert_eq!(grid90, vec![vec![0, 1], vec![1, 0], vec![1, 1],]);
        let grid180 = rotate90(grid90);
        assert_eq!(grid180, vec![vec![1, 1, 0], vec![1, 0, 1],]);
        let grid270 = rotate90(grid180);
        assert_eq!(grid270, vec![vec![1, 1], vec![0, 1], vec![1, 0]]);
        let grid360 = rotate90(grid270);
        assert_eq!(grid360, vec![vec![1, 0, 1], vec![0, 1, 1],]);
    }
}
