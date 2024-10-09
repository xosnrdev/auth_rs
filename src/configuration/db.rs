use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug)]
pub struct DbConfig {
    username: String,
    password: String,
    port: u16,
    host: String,
    name: String,
    ssl_mode: SslMode,
}

#[derive(Debug)]
pub enum SslMode {
    Require,
    Prefer,
    Disable,
}

impl DbConfig {
    pub fn new(
        username: String,
        password: String,
        port: u16,
        host: String,
        name: String,
        ssl_mode: SslMode,
    ) -> Self {
        Self {
            username,
            password,
            port,
            host,
            name,
            ssl_mode,
        }
    }

    pub fn to_pg_connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .host(&self.host)
            .database(&self.name)
            .ssl_mode(match self.ssl_mode {
                SslMode::Require => PgSslMode::Require,
                SslMode::Prefer => PgSslMode::Prefer,
                SslMode::Disable => PgSslMode::Disable,
            })
    }
}
