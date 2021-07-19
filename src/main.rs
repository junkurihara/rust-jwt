mod config;
mod constants;
mod globals;
mod jwt;
use config::parse_opt;
use env_logger;
use globals::{Globals, Mode};
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
  let mode = match parse_opt(&mut globals) {
    Err(e) => {
      eprintln!("Invalid options: {:?}", e);
      return;
    }
    Ok(m) => m,
  };
  debug!("{:?}", globals);

  match mode {
    Mode::GENERATE => match generate(&globals) {
      Err(e) => eprintln!("Failed to generate JWT: {:?}", e),
      Ok(token) => {
        println!("[Generated JWT]\n{}", &token);
      }
    },
    Mode::VERIFY => match verify(&mut globals) {
      Err(e) => eprintln!("[Validation]: Failed\n{:?}", e),
      Ok(_) => {
        println!("[Validation] Success",);
      }
    },
    _ => (),
  }
}
