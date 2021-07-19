use crate::globals::{Globals, Mode};
use clap::{App, Arg, ArgGroup};
use std::fs;

pub fn parse_opt(globals: &mut Globals) -> Result<Mode, Box<dyn std::error::Error>> {
  //, subject: &mut Subject) {
  let _ = include_str!("../Cargo.toml");
  let options = app_from_crate!()
    .subcommand(
      App::new("generate")
        .about("Generate a JWT")
        .arg(Arg::with_name("claim").help("Claim JSON string"))
        .arg(
          Arg::with_name("claim_path")
            .short("F")
            .long("claim-path")
            .takes_value(true)
            .help("Claim JSON path like \"--claim-path=./sample_claim.json\""),
        )
        .groups(&[ArgGroup::with_name("claim_args")
          .args(&["claim", "claim_path"])
          .required(true)])
        .arg(
          Arg::with_name("algorithm")
            .short("A")
            .long("algorithm")
            .takes_value(true)
            .help("Signing algorithm: HS256|ES256 (default = \"HS256\")"),
        )
        .arg(
          Arg::with_name("signing_key")
            .short("s")
            .long("signing-key")
            .takes_value(true)
            .help("Signing key string like \"secret\""),
        )
        .arg(
          Arg::with_name("signing_key_path")
            .short("P")
            .long("signing-key-path")
            .takes_value(true)
            .help("Signing key file path like \"./secret_key.pem\""),
        )
        .groups(&[ArgGroup::with_name("secret")
          .args(&["signing_key", "signing_key_path"])
          .required(true)])
        .arg(
          Arg::with_name("add_iat")
            .short("I")
            .long("add-iat")
            .help("Append 'issued_at (iat)' of current unix time in JWT claim"),
        )
        .arg(
          Arg::with_name("expires_in")
            .short("E")
            .long("expires-in")
            .takes_value(true)
            .help("Days in which the jwt expires"),
        ),
    )
    .subcommand(
      App::new("verify")
        .about("Verify a JWT")
        .arg(Arg::with_name("token").help("JWT string"))
        .arg(
          Arg::with_name("token_path")
            .short("T")
            .long("token-path")
            .takes_value(true)
            .help("JWT path like \"--token-path=./token_es256.example\""),
        )
        .groups(&[ArgGroup::with_name("token_args")
          .args(&["token", "token_path"])
          .required(true)])
        .arg(
          Arg::with_name("validation_key")
            .short("v")
            .long("validation-key")
            .takes_value(true)
            .help("Validation key string like \"secret\""),
        )
        .arg(
          Arg::with_name("validation_key_path")
            .short("W")
            .long("validation-key-path")
            .takes_value(true)
            .help("Validation key file path like \"./public_key.pem\""),
        )
        .groups(&[ArgGroup::with_name("public")
          .args(&["validation_key", "validation_key_path"])
          .required(true)]),
    );

  let matches = options.get_matches();

  if let Some(ref matches) = matches.subcommand_matches("generate") {
    // set args for generation
    if matches.is_present("claim_args") {
      if let Some(c) = matches.value_of("claim") {
        globals.claim = serde_json::from_str(c)?;
      } else {
        if let Some(f) = matches.value_of("claim_path") {
          match fs::read_to_string(f) {
            Ok(content) => {
              globals.claim = serde_json::from_str(&content)?;
            }
            Err(_) => {
              return Err("Invalid claim path")?;
            }
          }
        }
      }
    }

    if let Some(a) = matches.value_of("algorithm") {
      globals.set_algorithm(a);
    }

    if matches.is_present("secret") {
      if let Some(s) = matches.value_of("signing_key") {
        globals.set_signing_key(s);
      } else {
        if let Some(p) = matches.value_of("signing_key_path") {
          match fs::read_to_string(p) {
            Ok(content) => {
              if globals.is_hmac() {
                let truncate_vec: Vec<&str> = content.split("\n").collect();
                assert_eq!(truncate_vec.len() > 0, true);
                globals.set_signing_key(truncate_vec[0]);
              } else {
                globals.set_signing_key(&content);
              }
            }
            Err(_) => {
              return Err("Invalid signing key path")?;
            }
          }
        };
      }
    }

    if matches.is_present("add_iat") {
      globals.add_iat = true;
    }
    if let Some(d) = matches.value_of("expires_in") {
      let duration = d.parse::<usize>()?;
      globals.set_expires_in(duration);
      globals.add_exp = true;
    }
    Ok(Mode::GENERATE)
  } else if let Some(ref matches) = matches.subcommand_matches("verify") {
    // Set args for verification
    if matches.is_present("token_args") {
      if let Some(t) = matches.value_of("token") {
        globals.token = Some(t.to_string());
      } else {
        if let Some(f) = matches.value_of("token_path") {
          match fs::read_to_string(f) {
            Ok(content) => {
              let truncate_vec: Vec<&str> = content.split("\n").collect();
              assert_eq!(truncate_vec.len() > 0, true);
              globals.set_validation_key(truncate_vec[0]);
              globals.token = Some(truncate_vec[0].to_string());
            }
            Err(_) => {
              return Err("Invalid claim path")?;
            }
          }
        }
      }
    }

    if matches.is_present("public") {
      if let Some(v) = matches.value_of("validation_key") {
        globals.set_validation_key(v);
      } else {
        if let Some(w) = matches.value_of("validation_key_path") {
          match fs::read_to_string(w) {
            Ok(content) => {
              globals.set_validation_key(&content); // TODO: HS256のときに１行目だけ読むようにしないとダメ。jwt.rsで対応？
            }
            Err(_) => {
              return Err("Invalid validation key path")?;
            }
          }
        };
      }
    }

    Ok(Mode::VERIFY)
  } else {
    Ok(Mode::NONE)
  }
}
