// Archivo: src/pages/login.rs
// Propósito: Pantalla de acceso para administradores. Envía credenciales al backend Axum y guarda el JWT.
// Resumen de cambios recientes:
// - Se creó una acción reactiva (`create_action`) para manejar el POST asincrónico hacia `/api/auth/login`.
// - Se añadió manejo de estados: `pending` para mostrar "Verificando..." y mensajes de error 401.
// - Al iniciar sesión exitosamente, se persiste `jwt_token` en `localStorage` y se navega a `/admin`.
// - Se mantuvo el formulario controlado mediante señales (`username`, `password`).
use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use leptos_router::{use_navigate, NavigateOptions};

// 1. Estructuras de datos compartidas con el Backend
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthResponse {
    pub token: String,
    pub message: String,
}

// 2. Función asíncrona nativa que dispara el POST hacia Axum (Puerto 3000)
async fn enviar_login_al_backend(payload: LoginPayload) -> Result<AuthResponse, String> {
    let response = Request::post("http://127.0.0.1:3000/api/auth/login")
        .json(&payload)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == 200 {
        let auth_data = response.json::<AuthResponse>().await.map_err(|e| e.to_string())?;
        Ok(auth_data)
    } else {
        // Captura el error 401 Unauthorized si la contraseña no coincide
        Err("Usuario o contraseña incorrectos.".to_string())
    }
}

#[component]
pub fn Login() -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(String::new());

    // 3. Crear la Acción Reactiva de Leptos
    let login_action = create_action(|payload: &LoginPayload| {
        enviar_login_al_backend(payload.clone())
    });

    // 4. Leer el resultado de la acción reactivamente
    let login_value = login_action.value();
    let login_pending = login_action.pending();

    create_effect(move |_| {
        if let Some(resultado) = login_value.get() {
            match resultado {
                Ok(data) => {
                    set_error_msg.set(String::new());
                    if let Some(window) = web_sys::window() {
                        if let Some(storage) = window.local_storage().unwrap() {
                            storage.set_item("jwt_token", &data.token).unwrap();
                        }
                    }
                    logging::log!("{} Token guardado con éxito.", data.message);

                    let navigate = use_navigate();
                    navigate("/admin", NavigateOptions::default());
                }
                Err(err) => {
                    set_error_msg.set(err);
                }
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let user = username.get();
        let pass = password.get();

        if user.is_empty() || pass.is_empty() {
            set_error_msg.set("Por favor, llena todos los campos.".to_string());
            return;
        }

        // 5. Disparar la acción enviando los datos reales
        set_error_msg.set(String::new()); // Limpiamos errores previos
        login_action.dispatch(LoginPayload {
            username: user,
            password: pass,
        });
    };

    view! {
        <div class="login-container">
            <div class="login-card">
                <h2>"DJ GUSTAVO • ACCESS"</h2>
                <p class="subtitle">"Ingresa tus credenciales para administrar el feed"</p>

                <form on:submit=on_submit>
                    <div class="input-group">
                        <label for="username">"Usuario"</label>
                        <input
                            type="text"
                            id="username"
                            placeholder="Ej. djgustavo_admin"
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            prop:value=username
                            disabled=move || login_pending.get()
                        />
                    </div>

                    <div class="input-group">
                        <label for="password">"Contraseña"</label>
                        <input
                            type="password"
                            id="password"
                            placeholder="••••••••"
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            prop:value=password
                            disabled=move || login_pending.get()
                        />
                    </div>

                    // 6. Mostrar mensaje de carga o error dinámicamente
                    <Show when=move || login_pending.get()>
                        <p class="loading-text" style="color: #00f0ff;">"Verificando credenciales criptográficas..."</p>
                    </Show>

                    <Show when=move || !error_msg.get().is_empty() && !login_pending.get()>
                        <p class="error-text" style="color: #ff0055;">{move || error_msg.get()}</p>
                    </Show>

                    <button type="submit" class="btn-login" disabled=move || login_pending.get()>
                        {move || if login_pending.get() { "PROCESANDO" } else { "INICIAR SESIÓN" }}
                    </button>
                </form>
            </div>
        </div>
    }
}