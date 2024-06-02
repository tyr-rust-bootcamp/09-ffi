use algo::{Algo, AlgoType};
use napi_derive::napi;

#[napi(js_name = "Algo")]
pub struct JsAlgo {
  inner: Algo,
}

#[napi]
impl JsAlgo {
  #[napi(constructor)]
  pub fn new(name: String) -> Self {
    let algo = match name.as_str() {
      "blake3" => Algo::new(AlgoType::Blake3),
      _ => Algo::new(AlgoType::Default),
    };
    Self { inner: algo }
  }

  #[napi]
  pub fn hash(&self, v: String) -> String {
    self.inner.hash(&v)
  }

  #[napi]
  pub fn get_name(&self) -> &str {
    self.inner.get_name()
  }
}
