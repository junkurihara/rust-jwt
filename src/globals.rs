use crate::constants::*;
use jsonwebtoken::Algorithm;
use serde_json;
use std::str::FromStr;

pub enum AlgorithmType {
  HMAC,
  EC,
  RSA,
}

pub enum Mode {
  GENERATE,
  VERIFY,
  NONE,
}

#[derive(Debug)]
pub struct Globals {
  pub algorithm: Option<Algorithm>,
  signing_key: String,
  pub duration: usize,
  pub add_exp: bool,
  pub add_iat: bool,
  pub claim: serde_json::Value,
  validation_key: Option<String>,
  pub token: Option<String>,
}

impl Globals {
  pub fn new() -> Globals {
    Globals {
      algorithm: None,
      signing_key: SIGNING_KEY.to_string(),
      duration: 0,
      add_exp: false,
      add_iat: false,
      claim: serde_json::from_str(r#"{}"#).unwrap(),
      validation_key: None,
      token: None,
    }
  }

  pub fn set_algorithm(&mut self, algorithm_str: &str) {
    if let Ok(a) = Algorithm::from_str(algorithm_str) {
      self.algorithm = Some(a);
    } else {
      panic!("Invalid algorithm")
    }
  }

  pub fn set_signing_key(&mut self, secret_str: &str) {
    self.signing_key = secret_str.to_string();
  }

  pub fn get_signing_key(&self) -> &String {
    &self.signing_key
  }

  pub fn set_validation_key(&mut self, vk_str: &str) {
    self.validation_key = Some(vk_str.to_string());
  }

  pub fn get_validation_key(&self) -> Option<&String> {
    self.validation_key.as_ref()
  }

  pub fn set_expires_in(&mut self, days: usize) {
    self.duration = days * 24 * 60 * 60;
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
    if let Some(alg) = self.algorithm {
      alg == Algorithm::HS512 || alg == Algorithm::HS384 || alg == Algorithm::HS256
    } else {
      false
    }
  }

  pub fn is_ec(&self) -> bool {
    if let Some(alg) = self.algorithm {
      alg == Algorithm::ES256 || alg == Algorithm::ES384
    } else {
      false
    }
  }

  pub fn is_rsa(&self) -> bool {
    if let Some(alg) = self.algorithm {
      alg == Algorithm::RS256
        || alg == Algorithm::RS384
        || alg == Algorithm::RS512
        || alg == Algorithm::PS256
        || alg == Algorithm::PS384
        || alg == Algorithm::PS512
    } else {
      false
    }
  }
}
