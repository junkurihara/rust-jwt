use crate::globals::{AlgorithmType, Globals};
use crate::jwt_claims::Claims;
use crate::subject::Subject;
use chrono::Local;
use jsonwebtoken::{decode, decode_header, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error, info, warn};

pub fn generate(
  globals: &Globals,
  subject: &Subject,
) -> Result<String, jsonwebtoken::errors::Error> {
  let issued_at: usize = Local::now().timestamp() as usize;
  let my_claims = Claims {
    sub: subject.sub.clone(), // コマンド引数で与える
    iat: issued_at,
    exp: issued_at + globals.duration,
  };

  // header
  let header = Header::new(globals.algorithm);
  let encoding_key = match globals.get_type() {
    AlgorithmType::HMAC => EncodingKey::from_secret(globals.get_signing_key().as_ref()),
    AlgorithmType::EC => {
      let ec_key_bytes = globals.get_signing_key().as_bytes();
      EncodingKey::from_ec_pem(ec_key_bytes)?
    }
    AlgorithmType::RSA => {
      let rsa_key_bytes = globals.get_signing_key().as_bytes();
      EncodingKey::from_rsa_pem(rsa_key_bytes)?
    }
  };
  encode(&header, &my_claims, &encoding_key)
}

pub fn verify(
  globals: &Globals,
  token: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
  let parsed_header = decode_header(&token)?;
  let alg = parsed_header.alg;
  let decoding_key = match globals.get_type() {
    AlgorithmType::HMAC => DecodingKey::from_secret(globals.get_signing_key().as_ref()),
    AlgorithmType::EC => {
      let ec_key_bytes = globals.get_validation_key().as_bytes();
      DecodingKey::from_ec_pem(ec_key_bytes).unwrap()
    }
    AlgorithmType::RSA => {
      let rsa_key_bytes = globals.get_validation_key().as_bytes();
      DecodingKey::from_rsa_pem(rsa_key_bytes).unwrap()
    }
  };
  let verified = decode::<Claims>(&token, &decoding_key, &Validation::new(alg));
  debug!("{:?}", verified);
  return verified;
}
