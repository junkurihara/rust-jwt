# rust-jwt
JWT commandline tool


## Build

```:bash
$ cargo build
```

## Usage

```:bash
USAGE:
    rjwt [OPTIONS] <subject>

FLAGS:
    -h, --help       Prints help information
        --version    Prints version information

OPTIONS:
    -A, --algorithm <algorithm>                        Signing algorithm: HS256|ES256 (default = "HS256")
    -E, --expires_in <expires_in>                      Years in which the jwt expires (default = 1 (year))
    -S, --signing_key <signing_key>                    Signing key string like "secret"
    -P, --signing_key_path <signing_key_path>          Signing key file path like "./secret_key.pm"
    -V, --validation_key_path <validation_key_path>    Validation key file path like "./public_key.pm"

ARGS:
    <subject>    subject in claim
```

Example: Generate a jwt by ES256 with 10 years validity.

```:bash
$ ./target/debug/rjwt sample-subject \
 -A ES256 \
 -P ./secret_key_es256.example \
 -V ./public_key_es256.example \
 -E 10
```

When the public key associated with the given private key is specified, it also validates the generated token internally.
