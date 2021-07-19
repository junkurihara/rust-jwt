# rust-jwt
JWT commandline tool


## Build

```:bash
$ cargo build
```

## Usage

```:bash
USAGE:
    rjwt [FLAGS] [OPTIONS] <claim|--claim-path <claim_path>>

FLAGS:
    -I, --add-iat    Append 'issued_at (iat)' of current unix time in JWT claim
    -h, --help       Prints help information
        --version    Prints version information

OPTIONS:
    -A, --algorithm <algorithm>                        Signing algorithm: HS256|ES256 (default = "HS256")
    -F, --claim-path <claim_path>                      Claim JSON path like "--claim-path=./sample_claim.json"
    -E, --expires-in <expires_in>                      Years in which the jwt expires
    -S, --signing-key <signing_key>                    Signing key string like "secret"
    -P, --signing-key-path <signing_key_path>          Signing key file path like "./secret_key.pm"
    -V, --validation-key-path <validation_key_path>    Validation key file path like "./public_key.pm"

ARGS:
    <claim>    Claim JSON string
```

Example: Generate a jwt by ES256 with 10 years validity.

```:bash
$ ./target/debug/rjwt "{\"sub\": \"sample-subject\"}" \
 -A ES256 \
 -P ./secret_key_es256.example \
 -V ./public_key_es256.example \
 -E 1 \
 -I

[Header to be signed]
{
  "typ": "JWT",
  "alg": "ES256"
}
[Claim to be signed]
{
  "exp": 1626814529,
  "iat": 1626728129,
  "sub": "sample-subject"
}
[Generated JWT]
eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJleHAiOjE2MjY4MTQ1MjksImlhdCI6MTYyNjcyODEyOSwic3ViIjoic2FtcGxlLXN1YmplY3QifQ.Ac29-dDISATdCOGxmdmTLpHbiV0o7t8bv40Cnm2i4o_E3D6koe2BtFH4OPowDw0ZhIQCxyQVf29FfCQghJdCkw
```

When the public key associated with the given private key is specified, it also validates the generated token internally. Put `RUST_LOG=debug` as an environment variable to see the debug message.
