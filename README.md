# rust-jwt-cli

JWT CLI tool

## Build

```:bash
$ cargo build
```

## Usage

```:bash
$ rjwt -h
rjwt 0.1.0
Jun Kurihara
JWT CLI tool

USAGE:
    rjwt [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate    Generate a JWT
    help        Prints this message or the help of the given subcommand(s)
    verify      Verify a JWT
```

- Generate a JWT:

  ```: bash
  USAGE:
      rjwt generate [FLAGS] [OPTIONS] <claim|--claim-path <claim_path>> <--signing-key <signing_key>|--signing-key-path <signing_key_path>>

  FLAGS:
      -I, --add-iat    Append 'issued_at (iat)' of current unix time in JWT claim
      -h, --help       Prints help information
      -V, --version    Prints version information

  OPTIONS:
      -A, --algorithm <algorithm>                  Signing algorithm: like "HS256" or "ES256" [default: HS256]
      -F, --claim-path <claim_path>                Claim JSON path like "--claim-path=./sample_claim.json"
      -E, --expires-in <expires_in>                Days in which the jwt expires
      -s, --signing-key <signing_key>              Signing key string like "secret"
      -P, --signing-key-path <signing_key_path>    Signing key file path like "./secret_key.pem"

  ARGS:
      <claim>    Claim JSON string
  ```

- Verify a JWT

  ```:bash
  USAGE:
      rjwt verify [OPTIONS] <--validation-key <validation_key>|--validation-key-path <validation_key_path>> <token|--token-path <token_path>>

  FLAGS:
      -h, --help       Prints help information
      -V, --version    Prints version information

  OPTIONS:
      -T, --token-path <token_path>                      JWT path like "--token-path=./token_es256.example"
      -v, --validation-key <validation_key>              Validation key string like "secret"
      -W, --validation-key-path <validation_key_path>    Validation key file path like "./public_key.pem"

  ARGS:
      <token>    JWT string
  ```

Example: Generate a jwt by ES256 with 10 days validity, and validate it.

```:bash
$ rjwt generate "{\"sub\": \"sample-subject\"}" \
 -A ES256 \
 -P ./secret_key_es256.example \
 -E 10 \
 -I

[Header to be signed]
{
  "typ": "JWT",
  "alg": "ES256"
}
[Claim to be signed]
{
  "exp": 1627598843,
  "iat": 1626734843,
  "sub": "sample-subject"
}
[Generated JWT]
eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJleHAiOjE2Mjc1OTg4NDMsImlhdCI6MTYyNjczNDg0Mywic3ViIjoic2FtcGxlLXN1YmplY3QifQ.LfheunfaXI-dcq-MOtkJGEBaUH-_R0Iw55Qrf9ucf1ng2u0cB5BRMrakbvgEajbk5_dx1llb-i8i5oa5AhdOcA

$ rjwt verify \
 -W ./public_key_es256.example \
  eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJleHAiOjE2Mjc1OTg4NDMsImlhdCI6MTYyNjczNDg0Mywic3ViIjoic2FtcGxlLXN1YmplY3QifQ.LfheunfaXI-dcq-MOtkJGEBaUH-_R0Iw55Qrf9ucf1ng2u0cB5BRMrakbvgEajbk5_dx1llb-i8i5oa5AhdOcA

[Validated header]
{
  "typ": "JWT",
  "alg": "ES256"
}
[Validated claims]
{
  "exp": 1627598843,
  "iat": 1626734843,
  "sub": "sample-subject"
}
[Validation] Success
```

Put `RUST_LOG=debug` as an environment variable to see the debug message.
You can use an arbitrarily JSON-formatted claims by passing it with `--claim-path`.
