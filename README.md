# Authnorization

## Overview

This is a simple Authentication service built with the Actix Web framework with SQLx Postgres. The service handles user authentication, including registration and login functionalities. Although it currently does not include access and refresh token mechanisms, it establishes a robust foundation for future extensions like JWT-based token authentication.

## Purpose

The service is designed to offer basic authentication features:

- **User Registration**: Safely stores user information with hashed passwords.
- **User Login**: Verifies user credentials and prepares the system for future token-based authentication.

It adheres to industry-standard security practices by utilizing password hashing (via Argon2) and SQLx for interacting with a PostgreSQL database. This service is foundational to a broader authentication system that will later include more advanced features like JWT-based access and refresh tokens.

---

## Features

1. **User Registration**

   - Users can register by providing an email and a password which is validated upon request.
   - The password is hashed using Argon2 before storage, ensuring security even if the database is compromised.

2. **User Login**

   - Users can log in using their email and password.
   - Passwords are verified against the hashed values stored in the database.
   - The service currently authenticates users by matching credentials without issuing tokens (this will be extended).

3. **Modular Structure**
   - Built with flexibility and scalability in mind, the architecture supports future expansions like token generation (JWT) for user sessions, access control, and more.

---

## Installation

### Prerequisites

- Rust (stable version)
- Docker (for running PostgreSQL and development environment)
- `sqlx` CLI for migrations

### Steps

1. **Clone the repository**

   ```bash
   git clone https://github.com/xosnrdev/authnorization
   cd authnorization
   ```

2. **Setup the database**
   Run the PostgreSQL database using Docker Compose:

   ```bash
   docker-compose up -d
   ```

3. **Run database migrations**

   Use `sqlx` to set up the database schema:

   ```bash
   sqlx migrate run --source ./src/db/migrations
   ```

4. **Create and setup `.env` file in the root directory from the provided `.env.example` file.**

5. **Run the application**

   Start the Actix Web server:

   ```bash
   cargo run
   ```

   The server will be running on `localhost:8080`.

---

## Endpoints

### **POST /auth/register**

- **Description**: Register a new user.
- **Payload**:
  ```json
  {
    "email": "user@example.com",
    "password": "P@ssw0rd!123"
  }
  ```
- **Response**:
  - `201 Created` on successful registration.
  - `400 Bad Request` if email is already in use or validation fails.

### **POST /auth/login**

- **Description**: Log in an existing user.
- **Payload**:
  ```json
  {
    "email": "user@example.com",
    "password": "P@ssw0rd!123"
  }
  ```
- **Response**:
  - `200 OK` on successful login.
  - `401 Unauthorized` for incorrect credentials.

---

## Future Work

This MVP is designed as a foundation for future expansions, which include:

- **JWT-based Authentication**: Integrating access tokens for stateless authentication.
- **Refresh Tokens**: Adding support for session renewal using long-lived refresh tokens.
- **Account Management**: Adding user profile updates, password resets, and email verification.
- **Tests**: Writing more unit and integration tests to ensure the service's reliability.
- **Logging**: Implementing structured logging for better monitoring and debugging.
- **Error Handling**: Improving error responses and handling for better user experience.
- **Security Enhancements**: Adding more security features like rate limiting, fault tolerance and resilence, CORS, and HTTPS.
- **Tracing and Metrics**: Integrating tracing and metrics for performance monitoring.
- **Configuration**: Adding support for environment-based configuration and deployment.
- **Validation**: Enhancing input validation and error handling for better user experience.
- **Documentation**: Writing more detailed documentation for the service and its features.

It is quite a handful, but these features will make the service more robust and production-ready.

---

## Contributing

Contributions are welcome to improve the service and add more advanced features. To contribute:

1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request with a clear explanation of the improvements or fixes.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

For any questions or issues, feel free to open an issue on the repository.
