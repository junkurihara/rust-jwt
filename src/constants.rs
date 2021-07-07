use jsonwebtoken::Algorithm;

pub const ALGORITHM: Algorithm = Algorithm::HS256;
pub const SIGNING_KEY: &str = "secret";
pub const VALIDATION_KEY: &str = "";
pub const DURATION: usize = 365 * 24 * 60 * 60;
