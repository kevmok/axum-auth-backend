use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/login", post(login))
        .route("/protected", get(protected));

    let addr = "0.0.0.0:3000";
    println!("Server started at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn hello_world() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "message": "Hello crustacean!"
        })),
    )
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    if !payload.username.is_empty() && !payload.password.is_empty() {
        let token = generate_token(payload.username);
        (StatusCode::OK, Json(LoginResponse { token }))
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                token: String::new(),
            }),
        )
    }
}

fn generate_token(username: String) -> String {
    let secret = b"airtunes";
    let encoding_key = EncodingKey::from_secret(secret);
    let header = Header::new(Algorithm::HS256);
    let claims = json!({
        "sub": username,
        "exp": chrono::Utc::now().timestamp() + 3600, // Token expiration 3600 seconds
    });
    encode(&header, &claims, &encoding_key).unwrap()
}

async fn protected(token: Option<String>) -> impl IntoResponse {
    let token = match token {
        Some(token) => token,
        None => return (StatusCode::UNAUTHORIZED, "missing token").into_response(),
    };

    let secret = b"airtunes";
    let decoding_key = DecodingKey::from_secret(secret);
    let validation = Validation::default();

    match decode::<serde_json::Value>(&token, &decoding_key, &validation) {
        Ok(_) => (StatusCode::OK, "Access granted").into_response(),
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    }
}
