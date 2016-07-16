pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // TODO
    let mut result : Matrix = Vec::new();

    let mat1_y = mat1.len();
    let mat1_x = mat1[0].len(); //Assuming all rows are the same len or its not a matrix

    //let mat2_y = mat2.len();
    let mat2_x = mat2[0].len();

    for y1 in 0..mat1_y {
        result.push(Vec::new());
        for x2 in 0..mat2_x {
            result[y1].push(0f32);
            for x1 in 0..mat1_x {
                result[y1][x2] += mat1[y1][x1] * mat2[x1][x2];
            }
        }
    }
    
    result
}


