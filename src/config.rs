use crate::globals::Globals;
use crate::subject::Subject;
use clap::{Arg, ArgGroup};
use std::fs;

pub fn parse_opt(globals: &mut Globals, subject: &mut Subject) {
  let _ = include_str!("../Cargo.toml");
  let options = app_from_crate!()
    .arg(
      Arg::with_name("subject")
        .required(true)
        .help("subject in claim"),
    )
    .arg(
      Arg::with_name("algorithm")
        .short("A")
        .long("algorithm")
        .takes_value(true)
        .help("Signing algorithm: HS256|ES256 (default = \"HS256\")"),
    )
    .arg(
      Arg::with_name("signing_key")
        .short("S")
        .long("signing_key")
        .takes_value(true)
        .help("Signing key string like \"secret\""),
    )
    .arg(
      Arg::with_name("signing_key_path")
        .short("P")
        .long("signing_key_path")
        .takes_value(true)
        .help("Signing key file path like \"./secret_key.pm\""),
    )
    .groups(&[ArgGroup::with_name("secret").args(&["signing_key", "signing_key_path"])])
    .arg(
      Arg::with_name("validation_key_path")
        .short("V")
        .long("validation_key_path")
        .takes_value(true)
        .help("Validation key file path like \"./public_key.pm\""),
    )
    .arg(
      Arg::with_name("expires_in")
        .short("E")
        .long("expires_in")
        .takes_value(true)
        .help("Years in which the jwt expires (default = 1 (year))"),
    );

  let matches = options.get_matches();

  if let Some(s) = matches.value_of("subject") {
    subject.sub = s.to_string();
  }

  if let Some(a) = matches.value_of("algorithm") {
    globals.set_algorithm(a);
  }

  if matches.is_present("secret") {
    if let Some(s) = matches.value_of("signing_key") {
      globals.set_signing_key(s);
    } else {
      if let Some(p) = matches.value_of("signing_key_path") {
        if let Ok(content) = fs::read_to_string(p) {
          if globals.is_hmac() {
            let truncate_vec: Vec<&str> = content.split("\n").collect();
            assert_eq!(truncate_vec.len() > 0, true);
            globals.set_signing_key(truncate_vec[0]);
          } else {
            globals.set_signing_key(&content);
          }
        }
      }
    }
  }

  if let Some(vk) = matches.value_of("validation_key_path") {
    if let Ok(content) = fs::read_to_string(vk) {
      if globals.is_rsa() || globals.is_ec() {
        globals.set_validation_key(&content);
      }
    }
  }

  if let Some(y) = matches.value_of("expires_in") {
    globals.set_expires_in(y.parse::<usize>().unwrap());
  }
}
