# auth_rs

## Overview

`auth_rs` is an authentication service built using Actix Web and SQLx with PostgreSQL. It offers foundational authentication functions that adhere to RFC 6749 (OAuth 2.0) and RFC 6750 (Bearer Token) standards.

---

## Features

- **User Registration**: Registers users with hashed passwords.
- **User Login**: Authenticates users with email and password.
- **Token-based Auth**: Implements OAuth 2.0 flow with access and refresh tokens.
- **Secure Updates**: Protected endpoints for email and password changes.

## Installation

### Prerequisites

- Rust (latest stable)
- Docker (for PostgreSQL)
- `sqlx` CLI (for migrations)

### Setup Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/xosnrdev/auth_rs
   cd auth_rs
   ```

2. Start PostgreSQL with Docker:

   ```bash
   docker-compose up -d
   ```

3. Run database migrations:

   ```bash
   sqlx migrate run --source ./src/db/migrations
   ```

4. Create and configure `.env` file based on `.env.example`.

5. Start the application:

   ```bash
   cargo run
   ```

   Server runs on `localhost:<port>` default `50051`.

---

## API Response Format

All endpoints return responses in this format:

```json
{
  "status": "success" | "error",
  "message": "Human readable message",
  "tokenDetails": {
    "tokenType": "Bearer",
    "token": "string",
    "expiresIn": "number",
    "scope": "string",
    "refreshToken": "string"
  },
  "errorDetails": {
    "error": "error_code",
    "errorDescription": "Detailed error message",
    "errorUri": "https://auth.example.com/docs/errors"
  }
}
```

- `tokenDetails` appears only in successful authentication responses.
- `errorDetails` appears only in error responses.
- `scope` is optional in `tokenDetails`.
- `refreshToken` is omitted if empty.

---

## API Endpoints

### Public Endpoints (No Authentication Required)

#### **POST /api/v1/auth/register**

Registers a new user.

- **Request**:

  ```bash
  curl -X POST http://localhost:50051/api/v1/auth/register \
    -H "Content-Type: application/json" \
    -d '{"email": "user@example.com", "password": "$Password123"}'
  ```

- **Response**: Refer to the [API Response Format](#api-response-format) for response structure.

#### **POST /api/v1/auth/login**

Authenticates a user.

- **Request**:

  ```bash
  curl -X POST http://localhost:50051/api/v1/auth/login \
    -H "Content-Type: application/json" \
    -d '{"email": "user@example.com", "password": "$Password123"}'
  ```

- **Response**: Refer to the [API Response Format](#api-response-format) for response structure.

#### **GET /api/v1/auth/healthz**

Checks service status.

- **Request**:

  ```bash
  curl http://localhost:50051/api/v1/auth/healthz
  ```

- **Success Response** (`200 OK`):

  ```json
  {
    "status": "success",
    "message": "Service is operational"
  }
  ```

### Protected Endpoints (Access Token Required)

#### **PUT /api/v1/auth/password**

Updates the user’s password using the current access token.

- **Request**:

  ```bash
  curl -X PUT http://localhost:50051/api/v1/auth/password \
    -H "Authorization: Bearer <ACCESS_TOKEN>" \
    -H "Content-Type: application/json" \
    -d '{"newPassword": "new123"}'
  ```

- **Response**: Refer to the [API Response Format](#api-response-format) for response structure.

#### **PUT /api/v1/auth/email**

Updates the user’s email.

- **Request**:

  ```bash
  curl -X PUT http://localhost:50051/api/v1/auth/email \
    -H "Authorization: Bearer <ACCESS_TOKEN>" \
    -H "Content-Type: application/json" \
    -d '{"newEmail": "newemail@example.com"}'
  ```

- **Response**: Refer to the [API Response Format](#api-response-format) for response structure.

#### **POST /api/v1/auth/logout**

Logs out the user.

- **Request**:

  ```bash
  curl -X POST http://localhost:50051/api/v1/auth/logout \
    -H "Authorization: Bearer <ACCESS_TOKEN>"
  ```

#### **GET /api/v1/users/me**

Retrieves user details.

- **Request**:

  ```bash
  curl http://localhost:50051/api/v1/users/me \
    -H "Authorization: Bearer <ACCESS_TOKEN>"
  ```

#### **DELETE /api/v1/users/me**

Deletes the user account.

- **Request**:

  ```bash
  curl -X DELETE http://localhost:50051/api/v1/users/me \
    -H "Authorization: Bearer <ACCESS_TOKEN>"
  ```

### Token Refresh Endpoint

#### **POST /api/v1/auth/token/refresh**

Refreshes an access token using a refresh token.

- **Request**:

  ```bash
  curl -X POST http://localhost:50051/api/v1/auth/token/refresh \
    -H "Content-Type: application/json" \
    -d '{"token": "<REFRESH_TOKEN>"}'
  ```

## Security Notes

- **Access tokens**: Short-lived (15 minutes)
- **Refresh tokens**: Long-lived (7 days)
- **HTTPS**: Required for all requests in production
- **Password hashing**: Uses Argon2id for strong security

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
