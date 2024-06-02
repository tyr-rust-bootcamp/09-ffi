use algo::{multiply, Matrix};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::fmt;

#[pyclass(name = "Matrix")]
#[derive(Debug)]
pub struct PyMatrix {
    inner: Matrix<f64>,
}

#[pymethods]
impl PyMatrix {
    #[new]
    pub fn try_new(data: Vec<Vec<f64>>) -> PyResult<Self> {
        if data.is_empty() || data[0].is_empty() {
            return Err(PyValueError::new_err(
                "both row and col should not be empty",
            ));
        }
        let row = data.len();
        let col = data[0].len();
        let data: Vec<_> = data.into_iter().flatten().collect();
        Ok(Self {
            inner: Matrix::new(data, row, col),
        })
    }

    pub fn mul(&self, other: &PyMatrix) -> PyResult<Self> {
        let result = multiply(&self.inner, &other.inner).unwrap();
        Ok(Self { inner: result })
    }

    pub fn multiply(&self, other: Vec<Vec<f64>>) -> PyResult<Self> {
        let other = PyMatrix::try_new(other)?;
        self.mul(&other)
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}

impl fmt::Display for PyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
