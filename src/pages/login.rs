use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let user = username.get();
        let pass = password.get();

        if user.is_empty() || pass.is_empty() {
            set_error_msg.set("Por favor, llena todos los campos.".to_string());
            return;
        }
        set_error_msg.set("Conectando con el backend...".to_string());
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
                        />
                    </div>

                    <Show when=move || !error_msg.get().is_empty()>
                        <p class="error-text">{move || error_msg.get()}</p>
                    </Show>

                    <button type="submit" class="btn-login">
                        "INICIAR SESIÓN"
                    </button>
                </form>
            </div>
        </div>
    }
}