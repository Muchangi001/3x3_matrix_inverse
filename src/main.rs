#[derive(Debug, Copy, Clone)]
struct Matrix3x3 {
    matrix: [[f64;3];3],
    transpose: [[f64;3];3],
    cofactor_matrix: [[f64;3];3],
    cofactor_matrix_transpose: [[f64;3];3],
    determinant: f64,
    inverse: Option<[[f64;3];3]>
}

impl Matrix3x3 {
    fn new(matrix: [[f64;3];3]) -> Self {
        let determinant = {
            matrix[0][0] * matrix[1][1] * matrix[2][2] +
            matrix[0][1] * matrix[1][2] * matrix[2][0] +
            matrix[0][2] * matrix[1][0] * matrix[2][1] -
            matrix[2][0] * matrix[1][1] * matrix[0][2] -
            matrix[2][1] * matrix[1][2] * matrix[0][0] -
            matrix[2][2] * matrix[1][0] * matrix[0][1]
        };

        let transpose = [
            [matrix[0][0], matrix[1][0], matrix[2][0]],
            [matrix[0][1], matrix[1][1], matrix[2][1]],
            [matrix[0][2], matrix[1][2], matrix[2][2]]
        ];

        let cofactor_matrix = {
            [
                [matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1], -1f64 * matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0], matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]],
                [-1f64 * matrix[0][1] * matrix[2][2] - matrix[0][2] * matrix[2][1], matrix[0][0] * matrix[2][2] - matrix[0][2] * matrix[2][0], -1f64 * matrix[0][0] * matrix[2][1] - matrix[0][1] * matrix[2][0]],
                [matrix[0][1] * matrix[1][2] - matrix[0][2] * matrix[1][1], -1f64 * matrix[0][0] * matrix[1][2] - matrix[0][2] * matrix[1][0], matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]]
            ]
        };

        let cofactor_matrix_transpose = [
            [cofactor_matrix[0][0], cofactor_matrix[1][0], cofactor_matrix[2][0]],
            [cofactor_matrix[0][1], cofactor_matrix[1][1], cofactor_matrix[2][1]],
            [cofactor_matrix[0][2], cofactor_matrix[1][2], cofactor_matrix[2][2]]
        ];

        let mut inverse = [[0f64;3];3]; 
        if determinant != 0f64 {
            for (i, &row) in cofactor_matrix_transpose.iter().enumerate() {
                for (j, &value) in row.iter().enumerate(){
                    inverse[i][j] = value / determinant;
                }
            }
        }
        

        Self {
            matrix,
            transpose,
            cofactor_matrix,
            cofactor_matrix_transpose,
            determinant,
            inverse: if determinant != 0f64 { Some(inverse) } else { None }
        }
    }
}

fn main() {
    let m1 = Matrix3x3::new(
        [
            [1f64, 3f64, 4f64],
            [3f64, 8f64, 6f64],
            [2f64, 5f64, 7f64]
            ]
        );

    println!("matrix : {:.2?}", m1.matrix);
    println!("determinant : {:.2}", m1.determinant);
    println!("transpose : {:.2?}", m1.transpose);
    println!("cofactor matrix : {:.2?}", m1.cofactor_matrix);
    println!("cofactor matrix transpose : {:.2?}", m1.cofactor_matrix_transpose);
    println!("inverse : {:.2?}", m1.inverse.expect("inverse does not exist"));
}