// PBKDF2 < bcrypt < scrypt
fn passhash(name: &str, pass: &str) -> String {
    let namedpass = format!("{}{}", name, pass);
    let hash = bcrypt::hash(namedpass.as_bytes(), bcrypt::DEFAULT_COST).unwrap();
    // info!("{}{}: {}", name, pass, hash);
    hash
}
fn passhash_verify(name: &str, pass: &str, hash: &str) -> bool {
    let namedpass = format!("{}{}", name, pass);
    bcrypt::verify(namedpass.as_bytes(), hash).unwrap()
}

#[cfg(any(feature = "mysql"))]
type SqlID = u64;
#[cfg(any(feature = "sqlite", feature = "postgres"))]
type SqlID = i32;

// time_zone: https://github.com/launchbadge/sqlx/issues/329
#[cfg(any(feature = "mysql", feature = "postgres"))]
type SqlDateTime = chrono::DateTime<chrono::Utc>;
// type SqlDateTime = chrono::NaiveDateTime;

// Extend derive(FromRow): https://github.com/launchbadge/sqlx/issues/156
#[cfg(any(feature = "sqlite"))]
type SqlDateTime = String;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: SqlID,
    pub name: String,
    // pub phone: String,
    pub email: String,
    // not return password
    #[serde(skip_serializing)]
    pub pass: String,
    pub create_dt: SqlDateTime,
    pub update_dt: SqlDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub name: String,
    pub password: String,
    #[serde(default)]
    pub rememberme: bool,
}

impl Login {
    pub fn verify(&self, hash: &str) -> bool {
        passhash_verify(&self.name, &self.password, hash)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    // username
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Register {
    pub fn passhash(&self) -> String {
        passhash(&self.name, &self.password)
    }
}
