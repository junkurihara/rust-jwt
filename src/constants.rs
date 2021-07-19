use jsonwebtoken::Algorithm;

pub const ALGORITHM: Algorithm = Algorithm::HS256;
pub const SIGNING_KEY: &str = "secret";
