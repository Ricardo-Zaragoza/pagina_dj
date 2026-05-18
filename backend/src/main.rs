// Archivo: backend/src/main.rs
// Propósito: Arranque del servidor Axum, conexión a MongoDB Atlas, CORS y rutas.
// Resumen de cambios recientes:
// - Se corrigió el ping a MongoDB quitando argumentos inválidos en `run_command` (CORRECCIÓN 1).
// - Se eliminó el `None` en `insert_one` conforme a la API async actual de MongoDB (CORRECCIÓN 3).
// - Se añadió CORS abierto para desarrollo y cabeceras/métodos necesarios.
// - Se agregó la ruta `/api/auth/login` conectada a `auth::login_handler`.
// - Se incluyó `jwt_secret` en el `AppState` para firmar tokens JWT.
// - Se crea automáticamente el usuario admin si la colección `users` está vacía.

use axum::{routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use mongodb::{Client, Database, bson::doc};
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;

mod auth;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("Falta MONGO_URI en .env");
    let db_name = env::var("DB_NAME").expect("Falta DB_NAME en .env");
    let jwt_secret = env::var("JWT_SECRET").expect("Falta JWT_SECRET en .env");
    let admin_username = env::var("ADMIN_USERNAME").unwrap_or_else(|_| "djgustavo".to_string());
    let admin_password_hash = env::var("ADMIN_PASSWORD_HASH").expect("Falta ADMIN_PASSWORD_HASH en .env");

    println!(" Conectando a MongoDB Atlas...");

    let client = Client::with_uri_str(&mongo_uri).await.expect("Error al crear cliente");
    let db = client.database(&db_name);

    // CORRECCIÓN 1: Quitamos el 'None' del comando run_command
    db.run_command(doc! {"ping": 1})
        .await
        .expect("Falló el ping a Atlas");

    println!("Conexión exitosa a MongoDB Atlas.");

    let users_collection = db.collection::<mongodb::bson::Document>("users");
    let filter = doc! { "username": &admin_username };

    match users_collection.find_one(filter).await {
        Ok(None) => {
            println!("Colección vacía. Creando administrador en Atlas...");
            let admin_doc = doc! {
                "username": &admin_username,
                "password_hash": &admin_password_hash
            };
            // CORRECCIÓN 3: Quitamos el 'None' de insert_one
            if let Err(e) = users_collection.insert_one(admin_doc).await {
                println!("Fallo al insertar el usuario: {:?}", e);
            } else {
                println!("Cuenta '{}' creada con éxito en MongoDB", admin_username);
            }
        }
        Ok(Some(_)) => println!("El administrador '{}' ya existe en Atlas y está protegido.", admin_username),
        Err(e) => println!("Error de verificación: {:?}", e),
    }

    let cors = CorsLayer::new().allow_origin(Any).allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ]).allow_headers([axum::http::header::CONTENT_TYPE]);
    let state = AppState { db, jwt_secret };

    let app = Router::new()
        .route("/api/posts", get(|| async { "Feed de publicaciones" }))
        .route("/api/auth/login", post(auth::login_handler))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor Axum corriendo en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}