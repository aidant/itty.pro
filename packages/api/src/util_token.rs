use {
    rand::{thread_rng, RngCore},
    sqlx::{sqlite::SqliteTypeInfo, Database, Decode, Encode, Sqlite, Type},
    std::{fmt::Display, str::FromStr},
    veil::Redact,
};

#[derive(Clone, Encode, Decode)]
#[sqlx(transparent)]
struct TokenInner(Vec<u8>);

impl Display for TokenInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base64_url::encode(&self.0))
    }
}

impl Type<Sqlite> for Token {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <Vec<u8> as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <Vec<u8> as Type<Sqlite>>::compatible(ty)
    }
}

#[derive(Redact, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct Token(#[redact(display, partial)] TokenInner);

impl Token {
    pub fn new() -> Self {
        let mut data = [0u8; 64];
        thread_rng().fill_bytes(&mut data);
        Self(TokenInner(data.into()))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl FromStr for Token {
    type Err = base64_url::base64::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Token(TokenInner(base64_url::decode(s)?)))
    }
}
