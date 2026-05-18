// Archivo: backend/src/auth.rs
// Propósito: Endpoints y lógica de autenticación (login) para el backend Axum.
// Resumen de cambios recientes:
// - Se implementó verificación de contraseña usando Argon2 y `PasswordVerifier`.
// - Se maneja la ausencia de prefijo en `password_hash` añadiéndolo dinámicamente si falta.
// - Se generan JWT firmados con `JWT_SECRET` tomado del `AppState`.
// - Se mejoró el manejo de errores con códigos HTTP adecuados (401/500) y mensajes JSON.
use axum::{
    extract::State,
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::AppState;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

// Estructura para responderle al frontend
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub message: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let users_collection = state.db.collection::<mongodb::bson::Document>("users");

    // 1. Buscar al usuario administrador en MongoDB Atlas
    let filter = doc! { "username": &payload.username };
    let user_doc = match users_collection.find_one(filter).await {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Credenciales inválidas" }))).into_response();
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Error de comunicación con la base de datos" }))).into_response();
        }
    };

    // 2. Extraer el hash guardado
    let mut password_hash_str = match user_doc.get_str("password_hash") {
        Ok(hash) => hash.to_string(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Estructura de usuario corrupta" }))).into_response(),
    };

    if !password_hash_str.starts_with("$argon2id$") {
        password_hash_str = format!("$argon2id$v=19$m=19456,t=2,p=1{}", password_hash_str);
    }

    let parsed_hash = match PasswordHash::new(&password_hash_str) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Error al procesar el hash criptográfico" }))).into_response(),
    };

    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Credenciales inválidas" }))).into_response();
    }

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("Timestamp válido")
        .timestamp();

    let claims = serde_json::json!({
        "sub": payload.username,
        "exp": expiration
    });

    // Firmar criptográficamente el token usando la JWT_SECRET de tu .env
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "No se pudo firmar el token de acceso" }))).into_response(),
    };

    // 6. Enviar respuesta exitosa con la llave de acceso al frontend de Leptos
    (StatusCode::OK, Json(AuthResponse {
        token,
        message: format!("¡Bienvenido de vuelta, {}!", payload.username),
    })).into_response()
}