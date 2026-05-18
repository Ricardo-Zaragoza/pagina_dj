// Archivo: src/pages/admin.rs
// Propósito: Página de administración protegida. Valida JWT en localStorage y usa `Show`.
// Resumen de cambios recientes:
// - Se implementó un guard de ruta reactivo que verifica `jwt_token` en `localStorage`.
// - Se separó el `use_navigate()` en dos clones: uno para el guard y otro para el logout.
// - Se usó `<Show when=... fallback=...>` con closures ligeras para evitar capturas pesadas.
// - Se añadió botón flotante de "Cerrar Sesión" que elimina el token y navega a `/login`.
// - Se agregaron logs de depuración para estados de acceso y errores de recursos.
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
/// Componente de la página de administración protegida.
/// Implementa <Show> de forma limpia utilizando el estado reactivo puro.
pub fn Admin() -> impl IntoView {
    let (autenticado, set_autenticado) = create_signal(false);

    // 1. Clonamos navigate únicamente para la validación de la sesión
    let navigate_for_guard = use_navigate();
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().unwrap() {
                match storage.get_item("jwt_token").unwrap() {
                    Some(token) if !token.is_empty() => {
                        set_autenticado.set(true);
                        logging::log!(" Acceso autorizado al panel de administración.");
                    }
                    _ => {
                        logging::log!("Acceso denegado. Redirigiendo a /login...");
                        navigate_for_guard("/login", NavigateOptions::default());
                    }
                }
            }
        }
    });

    // 2. Clonamos navigate únicamente para el botón de logout
    let navigate_for_logout = use_navigate();

    view! {
        // 3. Usamos <Show> de Leptos pasando funciones lambda directas en las propiedades,
        // lo que evita que Rust cree closures FnOnce gigantescos.
        <Show
            when=move || autenticado.get()
            fallback=move || view! {
                <div class="min-h-screen bg-[#030213] flex items-center justify-center">
                    <p class="text-[#00f2ff] font-bold animate-pulse">"Verificando credenciales..."</p>
                </div>
            }
        >
            <main class="min-h-screen bg-[#030213] text-white font-sans overflow-x-hidden overflow-y-auto">

                // Botón flotante para Cerrar Sesión
 // Botón flotante para Cerrar Sesión (Esquina inferior izquierda)
                <div class="fixed bottom-6 left-6 z-50">
                    <button
                        on:click={
                            let nav = navigate_for_logout.clone();
                            move |_| {
                                if let Some(window) = web_sys::window() {
                                    if let Some(storage) = window.local_storage().unwrap() {
                                        storage.remove_item("jwt_token").unwrap();
                                        nav("/login", NavigateOptions::default());
                                    }
                                }
                            }
                        }
                        class="bg-[#ff2d75]/10 border border-[#ff2d75] hover:bg-[#ff2d75] text-[#ff2d75] hover:text-white text-xs font-bold tracking-widest px-4 py-3 rounded-sm transition-all duration-300 shadow-[0_0_15px_rgba(255,45,117,0.2)] backdrop-blur-sm"
                    >
                        "CERRAR SESIÓN"
                    </button>
                </div>

                <section class="relative h-[90vh] flex items-center px-10 md:px-24">

                    // --- Vinilo Giratorio Completo ---
                    <div class="absolute right-32 top-[15%] w-[500px] h-[500px] hidden lg:block z-0 overflow-hidden">
                        <div class="relative w-full h-full">
                            <div class="absolute inset-0 rounded-full border-[10px] border-[#ff2d75] shadow-[0_0_60px_rgba(255,45,117,0.4)] opacity-60"></div>
                            <div class="absolute inset-2 rounded-full overflow-hidden border-4 border-black animate-[spin_12s_linear_infinite]">
                                <img
                                    src="tripleT.jpg"
                                    alt="DJ Gus"
                                    class="w-full h-full object-cover"
                                    on:error=move |_| logging::log!("Error cargando la imagen del vinilo")
                                />
                            </div>
                        </div>
                    </div>

                    <div class="max-w-3xl z-10">
                        // Badge "En Vivo"
                        <div class="flex items-center gap-2 text-[#00f2ff] mb-6">
                            <div class="flex gap-1">
                                <span class="w-1 h-4 bg-[#00f2ff] animate-pulse"></span>
                                <span class="w-1 h-6 bg-[#00f2ff] animate-pulse delay-75"></span>
                                <span class="w-1 h-3 bg-[#00f2ff] animate-pulse delay-150"></span>
                            </div>
                            <span class="text-xs font-bold tracking-[0.2em]">"EN VIVO • DISPONIBLE"</span>
                        </div>

                        // Título Principal
                        <h1 class="text-7xl md:text-9xl font-black leading-[0.8] tracking-tighter mb-8 italic">
                            "Admin" <br/>
                            <span class="text-[#ff2d75] drop-shadow-[0_0_30px_rgba(255,45,117,0.5)]">"Gustavo"</span>
                        </h1>

                        // Descripción
                        <p class="text-lg md:text-xl text-gray-400 max-w-lg mb-10 leading-relaxed">
                            "Transformo cada evento en una experiencia que la gente recuerda para siempre. Música que mueve cuerpos, luces que incendian la noche."
                        </p>

                        // Botones de Acción
                        <div class="flex flex-wrap gap-4">
                            <button class="bg-[#ff2d75] hover:bg-[#ff1a63] text-white px-10 py-4 rounded-sm font-bold flex items-center gap-2 transition-all">
                                "Reservar mi fecha"
                            </button>
                            <button class="border-2 border-[#00f2ff] text-[#00f2ff] hover:bg-[#00f2ff]/10 px-10 py-4 rounded-sm font-bold transition-all">
                                "Ver cabinas"
                            </button>
                        </div>
                    </div>
                </section>
            </main>
        </Show>
    }
}