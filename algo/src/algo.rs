use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Algo {
  pub r#type: AlgoType,
}

#[derive(Debug, Clone, Copy)]
pub enum AlgoType {
  Blake3,
  Default,
}

impl Algo {
  pub fn new(r#type: AlgoType) -> Self {
    Self { r#type }
  }

  pub fn hash(&self, v: String) -> String {
    match self.r#type {
      AlgoType::Blake3 => {
        let hash = blake3::hash(v.as_bytes());
        hash.to_string()
      }
      AlgoType::Default => {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish().to_string()
      }
    }
  }

  pub fn get_name(&self) -> &str {
    match self.r#type {
      AlgoType::Blake3 => "blake3",
      AlgoType::Default => "default",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hash_should_work() {
    let algo = Algo::new(AlgoType::Blake3);
    assert_eq!(
      algo.hash("hello".to_string()),
      "b0a7f6e1bcb9e8f708e4c3c1b9d1e5b0b0a7f6e1bcb9e8f708e4c3c1b9d1e5b0"
    );

    let algo = Algo::new(AlgoType::Default);
    assert_eq!(algo.hash("hello".to_string()), "17468397271165231797");
  }
}
