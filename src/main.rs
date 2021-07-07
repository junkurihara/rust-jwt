mod config;
mod constants;
mod globals;
mod jwt;
mod jwt_claims;
mod subject;
use config::parse_opt;
use constants::*;
use env_logger;
use globals::Globals;
use subject::Subject;
// use jsonwebtoken::{decode, DecodingKey, Validation};
use jwt::{generate, verify};
use log::{debug, error, info, warn};
use std::env;

#[macro_use]
extern crate clap;

#[tokio::main]
async fn main() {
  env::set_var("RUST_LOG", "info");
  env_logger::init();

  let mut globals = Globals::new(ALGORITHM, SIGNING_KEY, VALIDATION_KEY, DURATION);
  let mut subject = Subject {
    sub: "".to_string(),
  };
  parse_opt(&mut globals, &mut subject);
  debug!("{:?}", globals);

  let generated_token = generate(&globals, &subject);
  if let Err(e) = generated_token {
    error!("Failed to generate JWT: {:?}", e);
  } else {
    let unwrapped = generated_token.unwrap();
    println!("\nGenerated JWT:\n{:?}", unwrapped);
    // check
    if globals.get_validation_key() != VALIDATION_KEY || globals.is_hmac() {
      match verify(&globals, &unwrapped) {
        Ok(_) => {
          debug!("\nSuccessfully verified");
        }
        Err(e) => {
          error!("\nFailed to verify, something wrong:\n{:?}", e);
        }
      };
    }
  }
}
