# Axum Simple Authentication Backend

This is a Rust backend server built with the Axum web framework. It provides authentication functionality using JSON Web Tokens (JWTs).

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

## Getting Started

1. Clone the repository:

```bash
git clone git@github.com:kevmok/axum-auth-backend.git
```

2. Change into project directory

```bash
cd axum-auth-backend
```

3. Build and run the server

```bash
cargo run
```

The server will start running on localhost:3000

## API Endpoints

### Login

- URL: `/login`
- Method: POST
- Request Body:
  ```json
  {
  	"username": "your-username",
  	"password": "your-password"
  }
  ```
- Response:
  - Success (200 OK):
  ```json
  {
  	"token": "your-jwt-token"
  }
  ```
  - Unauthorized (401 Unauthorized):
  ```json
  {
  	"token": ""
  }
  ```
- Example curl command:
  ```bash
  curl -X POST -H "Content-Type: application/json" -d '{"username":"your-username","password":"your-password"}' http://localhost:3000/login
  ```

### Protected Route

- URL: `/protected`
- Method: GET
- Headers:

  - `Authorization: your-jwt-token`

- Response:
  - Success (200 OK): `Access granted`
  - Unauthorized (401 Unauthorized): `Invalid token` or `Missing token`

Example curl command:

```bash
curl -H "Authorization: your-jwt-token" http://localhost:3000/protected
```
