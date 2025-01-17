pub(crate) fn transpose_matrix(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }

    transposed
}

#[test]
fn test_transpose_matrix() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    let transposed_matrix = transpose_matrix(matrix);

    assert_eq!(
        transposed_matrix,
        [[101, 201, 301], [102, 202, 302], [103, 203, 303]]
    );
}
