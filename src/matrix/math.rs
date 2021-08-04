use crate::matrix::Matrix;
use num_traits::{FromPrimitive, Num};
use std::ops::AddAssign;

impl<T: FromPrimitive + Num> Matrix<T> {
    pub fn mult(m1: &Matrix<T>, m2: &Matrix<T>) -> Result<Matrix<T>, String>
    where
        T: Copy + AddAssign,
    {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim.1 != m2_dim.0 {
            return Err(String::from(
                "Number of columns in the first matrix must match number of rows in the second matrix",
            ));
        }

        let mut m3 = Matrix::new_empty(m1_dim.0, m2_dim.1).unwrap();
        for row in 0..m1_dim.0 {
            for col in 0..m2_dim.1 {
                let mut acc: T = FromPrimitive::from_u8(0).unwrap();

                for index in 0..m1_dim.1 {
                    acc += m1[row][index] * m2[index][col]
                }
                m3[row][col] = acc;
            }
        }

        Ok(m3)
    }

    // hadamard product
    //
    pub fn prod(m1: Matrix<T>, m2: Matrix<T>) -> Result<Matrix<T>, String>
    where
        T: Copy,
    {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim != m2_dim {
            return Err(String::from("Both matrices must be identically shaped"));
        }

        let mut m3: Matrix<T> = Matrix::new_empty(m1_dim.0, m1_dim.1).unwrap();
        for row in 0..m1_dim.0 {
            for col in 0..m1_dim.1 {
                m3[row][col] = m1[row][col] * m2[row][col];
            }
        }

        Ok(m3)
    }

    // vector based functions
    //
    pub fn dot(m1: Matrix<T>, m2: Matrix<T>) -> Result<T, String>
    where
        T: AddAssign + Copy,
    {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim != m2_dim {
            return Err(String::from("Both matrices must be identically shaped"));
        }

        let mut acc: T = FromPrimitive::from_u8(0).unwrap();
        for row in 0..m1_dim.0 {
            for col in 0..m1_dim.1 {
                acc += m1[row][col] * m2[row][col];
            }
        }

        Ok(acc)
    }

    pub fn add(m1: &Matrix<T>, m2: &Matrix<T>) -> Result<Matrix<T>, String>
    where
        T: Copy,
    {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim != m2_dim {
            return Err(String::from("Both matrices must be identically shaped"));
        }

        let mut m3: Matrix<T> = Matrix::new_empty(m1_dim.0, m1_dim.1).unwrap();
        for row in 0..m1_dim.0 {
            for col in 0..m1_dim.1 {
                m3[row][col] = m1[row][col] + m2[row][col];
            }
        }

        Ok(m3)
    }

    pub fn sub(m1: &Matrix<T>, m2: &Matrix<T>) -> Result<Matrix<T>, String>
    where
        T: Copy,
    {
        let m1_dim = m1.dim();
        let m2_dim = m2.dim();

        if m1_dim != m2_dim {
            return Err(String::from("Both matrices must be identically shaped"));
        }

        let mut m3: Matrix<T> = Matrix::new_empty(m1_dim.0, m1_dim.1).unwrap();
        for row in 0..m1_dim.0 {
            for col in 0..m1_dim.1 {
                m3[row][col] = m1[row][col] - m2[row][col];
            }
        }

        Ok(m3)
    }
}
