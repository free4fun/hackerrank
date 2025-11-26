use std::cmp::min;

fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    let magic_squares = vec![
        vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]],
        vec![vec![6, 1, 8], vec![7, 5, 3], vec![2, 9, 4]],
        vec![vec![4, 9, 2], vec![3, 5, 7], vec![8, 1, 6]],
        vec![vec![2, 9, 4], vec![7, 5, 3], vec![6, 1, 8]], 
        vec![vec![8, 3, 4], vec![1, 5, 9], vec![6, 7, 2]],
        vec![vec![4, 3, 8], vec![9, 5, 1], vec![2, 7, 6]], 
        vec![vec![6, 7, 2], vec![1, 5, 9], vec![8, 3, 4]], 
        vec![vec![2, 7, 6], vec![9, 5, 1], vec![4, 3, 8]],
    ];
    let mut result = i32::MAX;
    for magic_square in &magic_squares {
        let mut total_cost = 0;
        for (i, row) in magic_square.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if s[i][j] != *val {
                    total_cost += (*val - s[i][j]).abs();
                }
            }
        }
        result = min(result, total_cost);
    }
    return result;
}