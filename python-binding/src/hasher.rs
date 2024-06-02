use algo::{Algo, AlgoType};
use pyo3::prelude::*;

#[pyclass(name = "Algo")]
pub struct PyAlgo {
    inner: Algo,
}

#[pymethods]
impl PyAlgo {
    #[new]
    pub fn new(name: &str) -> Self {
        let algo = match name {
            "blake3" => Algo::new(AlgoType::Blake3),
            _ => Algo::new(AlgoType::Default),
        };
        Self { inner: algo }
    }

    pub fn hash(&self, v: &str) -> String {
        self.inner.hash(v)
    }

    pub fn get_name(&self) -> &str {
        self.inner.get_name()
    }
}
