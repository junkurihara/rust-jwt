mod config;
mod constants;
mod globals;
mod jwt;
use config::parse_opt;
use env_logger;
use globals::Globals;
use jwt::{generate, verify};
use log::debug;
// use std::env;

#[macro_use]
extern crate clap;

#[tokio::main]
async fn main() {
  //env::set_var("RUST_LOG", "debug");
  env_logger::init();

  let mut globals = Globals::new();
  match parse_opt(&mut globals) {
    Err(e) => {
      eprintln!("Invalid options: {:?}", e);
      return;
    }
    _ => (),
  };
  debug!("{:?}", globals);

  match generate(&globals) {
    Err(e) => eprintln!("Failed to generate JWT: {:?}", e),
    Ok(token) => {
      println!("[Generated JWT]\n{}", &token);
      // check
      if globals.is_hmac() || globals.get_validation_key() != None {
        match verify(&globals, &token) {
          Ok(_) => {
            debug!("Successfully verified");
          }
          Err(e) => {
            eprintln!("Failed to verify: {:?}", e);
          }
        }
      } else {
        debug!("no validation key");
      }
    }
  }
}
