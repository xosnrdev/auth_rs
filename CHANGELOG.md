# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Transition to Axum framework and restructure project

## [1.5.0] - 2024-11-01

### Added
- Add rate limiting configuration by @xosnrdev
- Add middlewares module and re-export the rate limit middleware by @xosnrdev
- Add Redis service to docker-compose.yml by @xosnrdev
- Add Redis configuration to .env.example by @xosnrdev

### Changed
- Release auth_rs version 1.5.0 by @xosnrdev
- Cleanup and weekly maintenance by @xosnrdev in [#16](https://github.com/xosnrdev/auth-rs/pull/16)
- Trim token before converting to string by @xosnrdev
- Merge pull request #15 from xosnrdev/feature/#12-session-management by @xosnrdev in [#15](https://github.com/xosnrdev/auth-rs/pull/15)
- Merge pull request #14 from xosnrdev/feature/#10-rate-limiting by @xosnrdev in [#14](https://github.com/xosnrdev/auth-rs/pull/14)
- Add rate limiting middleware and configuration by @xosnrdev
- Add rate limiting middleware by @xosnrdev
- Change token extraction to return a String instead of a &str by @xosnrdev
- Add RateLimitError conversion to AuthResponse by @xosnrdev
- Added actix limitation crate for ratelimit setup by @xosnrdev
- Add RateLimitConfig module and builder by @xosnrdev
- Merge pull request #13 from xosnrdev/feature/#12-session-management by @xosnrdev in [#13](https://github.com/xosnrdev/auth-rs/pull/13)
- Add RedisConfig module and builder. by @xosnrdev
- Add RedisConfig module and builder by @xosnrdev
- Update Cargo.toml to add redis crate dependency by @xosnrdev
- Remove unnecessary EXPOSE statement in Dockerfile by @xosnrdev
- Update SQLX_OFFLINE environment variable in Dockerfile by @xosnrdev
- Add Dockerfile for containerization by @xosnrdev
- Update docker-compose.yml for PostgreSQL configuration by @xosnrdev
- Update sqlx migrate command in README.md by @xosnrdev
- Update SSLMode to use PgSslMode enum by @xosnrdev
- Moved to sqlx default source file by @xosnrdev

[1.5.0]: https://github.com/xosnrdev/auth-rs/compare/v1.0.0..v1.5.0

