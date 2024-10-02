pub struct JwtConfig {
    jwt_secret: String,
    jwt_expiration: i64,
}

impl JwtConfig {
    pub fn new(jwt_secret: String, jwt_expiration: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration,
        }
    }

    pub fn jwt_secret(&self) -> &String {
        &self.jwt_secret
    }

    pub fn jwt_expiration(&self) -> i64 {
        self.jwt_expiration
    }
}
