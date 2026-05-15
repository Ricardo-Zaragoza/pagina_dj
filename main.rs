//! Aplicación web Leptos para la landing de "DJ Gustavo".
//! 
//! Este módulo define el componente raíz `App` y el punto de entrada `main`.
//! 
//! Fecha de última edición: 2026-05-15 17:27 (hora local).
//!
//! Notas rápidas:
//! - Renderizado: modo CSR (Client-Side Rendering) habilitado en `Cargo.toml` con `leptos = { features = ["csr"] }`.
//! - Errores en navegador: `console_error_panic_hook` muestra los `panic!` de Rust en la consola.
//! - Estilos: se apoyan en clases tipo Tailwind (utilizadas como utilidades), definidas en el HTML/CSS del proyecto.

use leptos::*;
use leptos_router::*;
mod pages {
    pub mod home;
    pub mod servicios;
}
use pages::home::Home;
use pages::servicios::Servicios;
#[component]



/// Componente raíz de la SPA (Single Page Application).
/// 
/// Estructura:
/// - Barra de navegación fija y semitransparente con botón de reserva.
/// - Sección "hero" con tipografía llamativa, badge "En Vivo" y botones de acción.
/// 
/// Devuelve `impl IntoView` para que Leptos pueda montarlo en el DOM.
fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="min-h-screen bg-[#030213] text-white font-sans overflow-x-hidden">
                <Routes>
                    <Route path="" view=Home />
                    <Route path="servicios" view=Servicios />
                </Routes>

            </main>
        </Router>
    }
}

/// Punto de entrada de la aplicación.
/// 
/// - Inicializa `console_error_panic_hook` para ver `panic!` en la consola del navegador.
/// - Monta el componente `App` en el `<body>` del documento mediante `mount_to_body`.
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}