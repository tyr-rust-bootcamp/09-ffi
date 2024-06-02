use algo::{multiply, Matrix};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::fmt;

#[napi(js_name = "Matrix")]
pub struct JsMatrix {
  inner: Matrix<f64>,
}

#[napi]
impl JsMatrix {
  #[napi(constructor)]
  pub fn try_new(data: Vec<Vec<f64>>, _env: Env) -> Result<Self> {
    if data.is_empty() || data[0].is_empty() {
      return Err(Error::new(
        Status::InvalidArg,
        "row should not be empty or column should not be empty",
      ));
    }
    let row = data.len();
    let col = data[0].len();
    let data: Vec<_> = data.into_iter().flatten().collect();
    Ok(Self {
      inner: Matrix::new(data, row, col),
    })
  }

  #[napi]
  pub fn mul(&self, other: &JsMatrix) -> Result<Self> {
    let result = multiply(&self.inner, &other.inner).unwrap();
    Ok(Self { inner: result })
  }

  #[napi]
  pub fn multiply(&self, other: Vec<Vec<f64>>, env: Env) -> Result<Self> {
    let other = JsMatrix::try_new(other, env)?;
    self.mul(&other)
  }

  #[napi]
  pub fn display(&self) -> String {
    format!("{}", self.inner)
  }
}

impl fmt::Display for JsMatrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.inner)
  }
}
