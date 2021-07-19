use crate::globals::{AlgorithmType, Globals};
use chrono::Local;
use jsonwebtoken::{decode, decode_header, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_json;

pub fn generate(globals: &Globals) -> Result<String, Box<dyn std::error::Error>> {
  let issued_at: usize = Local::now().timestamp() as usize;
  let expired_at: usize = issued_at + globals.duration;

  // header
  let header = Header::new(globals.algorithm.unwrap());
  println!(
    "[Header to be signed]\n{}",
    serde_json::to_string_pretty(&header)?
  );

  // key
  let key_str = match globals.get_type() {
    AlgorithmType::HMAC => globals.get_signing_key(),
    AlgorithmType::EC | AlgorithmType::RSA => globals.get_signing_key(),
  };
  let encoding_key = match globals.get_type() {
    AlgorithmType::HMAC => EncodingKey::from_secret(globals.get_signing_key().as_ref()),
    AlgorithmType::EC => EncodingKey::from_ec_pem(key_str.as_bytes())?,
    AlgorithmType::RSA => EncodingKey::from_rsa_pem(key_str.as_bytes())?,
  };

  // claim
  let mut claim_value: serde_json::Value = globals.claim.clone();
  if globals.add_iat {
    claim_value["iat"] = serde_json::Value::from(issued_at); // add ist here when option is enabled
  }
  if globals.add_exp {
    claim_value["exp"] = serde_json::Value::from(expired_at); // add exp here when option is enabled
  }
  println!(
    "[Claim to be signed]\n{}",
    serde_json::to_string_pretty(&claim_value)?
  );
  let jwt = encode(&header, &claim_value, &encoding_key)?;
  Ok(jwt)
}

pub fn verify(globals: &mut Globals) -> Result<(), Box<dyn std::error::Error>> {
  let token = globals.token.as_ref().unwrap();
  // header
  let parsed_header = decode_header(&token)?;
  let alg = parsed_header.alg;
  globals.algorithm = Some(alg);

  // key
  let key_str = match globals.get_type() {
    AlgorithmType::HMAC => globals.get_signing_key(),
    AlgorithmType::EC | AlgorithmType::RSA => match globals.get_validation_key() {
      Some(s) => s,
      None => return Err("No validation key is specified")?,
    },
  };
  let decoding_key = match globals.get_type() {
    AlgorithmType::HMAC => DecodingKey::from_secret(globals.get_signing_key().as_ref()),
    AlgorithmType::EC => DecodingKey::from_ec_pem(key_str.as_bytes())?,
    AlgorithmType::RSA => DecodingKey::from_rsa_pem(key_str.as_bytes())?,
  };

  let verified = decode::<serde_json::Value>(&token, &decoding_key, &Validation::new(alg))?;
  println!(
    "[Validated header]\n{}",
    serde_json::to_string_pretty(&verified.header)?
  );
  println!(
    "[Validated claims]\n{}",
    serde_json::to_string_pretty(&verified.claims)?
  );

  Ok(())
}
