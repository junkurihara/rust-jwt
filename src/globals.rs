use jsonwebtoken::Algorithm;
use std::str::FromStr;

pub enum AlgorithmType {
  HMAC,
  EC,
  RSA,
}

#[derive(Debug)]
pub struct Globals {
  pub algorithm: Algorithm,
  signing_key: String,
  validation_key: String,
  pub duration: usize,
}

impl Globals {
  pub fn new(
    algorithm: Algorithm,
    signing_key: &str,
    validation_key: &str,
    duration: usize,
  ) -> Globals {
    Globals {
      algorithm: algorithm,
      signing_key: signing_key.to_string(),
      validation_key: validation_key.to_string(),
      duration: duration,
    }
  }

  pub fn set_algorithm(&mut self, algorithm_str: &str) {
    if let Ok(a) = Algorithm::from_str(algorithm_str) {
      self.algorithm = a;
    } else {
      panic!("Invalid algorithm")
    }
  }

  pub fn set_signing_key(&mut self, secret_str: &str) {
    self.signing_key = secret_str.to_string();
  }

  pub fn get_signing_key(&self) -> &str {
    &self.signing_key
  }

  pub fn set_validation_key(&mut self, vk_str: &str) {
    self.validation_key = vk_str.to_string();
  }

  pub fn get_validation_key(&self) -> &str {
    &self.validation_key
  }

  pub fn set_expires_in(&mut self, years: usize) {
    self.duration = years * 365 * 24 * 60 * 60;
  }

  pub fn get_type(&self) -> AlgorithmType {
    if self.is_hmac() {
      AlgorithmType::HMAC
    } else if self.is_ec() {
      AlgorithmType::EC
    } else {
      AlgorithmType::RSA
    }
  }

  pub fn is_hmac(&self) -> bool {
    self.algorithm == Algorithm::HS512
      || self.algorithm == Algorithm::HS384
      || self.algorithm == Algorithm::HS256
  }

  pub fn is_ec(&self) -> bool {
    self.algorithm == Algorithm::ES256 || self.algorithm == Algorithm::ES384
  }

  pub fn is_rsa(&self) -> bool {
    self.algorithm == Algorithm::RS256
      || self.algorithm == Algorithm::RS384
      || self.algorithm == Algorithm::RS512
      || self.algorithm == Algorithm::PS256
      || self.algorithm == Algorithm::PS384
      || self.algorithm == Algorithm::PS512
  }
}
