use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Matrix_3x3 {
    data: [[i32; 3]; 3],
}

impl Matrix_3x3 {
    pub fn new(data: [[i32; 3]; 3]) -> Matrix_3x3 {
        Matrix_3x3 { data }
    }

    pub fn transpose(self) -> Matrix_3x3 {
        let mut data = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                data[i][j] = self.data[j][i];
            }
        }
        return Matrix_3x3 { data };
    }
    }
}

impl fmt::Display for Matrix_3x3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[\n");
        for i in &self.data {
            write!(f, "    [ ");
            for j in i {
                write!(f, "{} ", j);
            }
            write!(f, "]\n");
        }
        write!(f, "]\n")
    }
}

mod test {
    use super::Matrix_3x3;

    #[test]
    fn test_transpose() {
        let matrix = Matrix_3x3::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let matrix_t = Matrix_3x3::new([[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
        assert_eq!(matrix.transpose(), matrix_t)
    }
}
