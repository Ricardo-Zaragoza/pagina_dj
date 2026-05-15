//! Aplicación web Leptos para la landing de "DJ Gustavo".
//! 
//! Este módulo define el componente raíz `App` y el punto de entrada `main`.
//! 
//! Notas rápidas:
//! - Renderizado: modo CSR (Client-Side Rendering) habilitado en `Cargo.toml` con `leptos = { features = ["csr"] }`.
//! - Errores en navegador: `console_error_panic_hook` muestra los `panic!` de Rust en la consola.
//! - Estilos: se apoyan en clases tipo Tailwind (utilizadas como utilidades), definidas en el HTML/CSS del proyecto.

use leptos::*;

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
        <main class="min-h-screen bg-[#030213] text-white font-sans overflow-x-hidden overflow-y-auto">

            // --- Barra de Navegación ---
            <nav class="flex justify-between items-center px-10 py-6 bg-black/20 backdrop-blur-md sticky top-0 z-50">
                <div class="flex items-center gap-2">
                    <span class="text-2xl font-black tracking-tighter text-white">"DJ Gustavo"</span>
                </div>
                <div class="hidden md:flex gap-8 text-sm font-medium text-gray-400">
                    <a href="#" class="hover:text-white transition-colors">"Servicios"</a>
                    <a href="#" class="hover:text-white transition-colors">"Cabinas"</a>
                    <a href="#" class="hover:text-white transition-colors">"Sobre Mí"</a>
                    <a href="#" class="hover:text-white transition-colors">"Galería"</a>
                    <a href="#" class="hover:text-white transition-colors">"Contacto"</a>
                </div>
                <button class="bg-[#ff2d75] hover:bg-[#ff1a63] text-white px-6 py-2 rounded-md font-bold text-sm shadow-[0_0_20px_rgba(255,45,117,0.3)] transition-all">
                    "Reservar Ahora"
                </button>
            </nav>

            <section class="relative h-[90vh] flex items-center px-10 md:px-24">

                // --- Vinilo Giratorio Completo ---
<div class="absolute right-32 top-[15%] w-[500px] h-[500px] hidden lg:block z-0 overflow-hidden">
    <div class="relative w-full h-full">

        // 1. Borde de neón estático (Marco del disco)
        <div class="absolute inset-0 rounded-full border-[10px] border-[#ff2d75] shadow-[0_0_60px_rgba(255,45,117,0.4)] opacity-60"></div>

        // 2. El Disco que gira (Contenedor de la imagen)
        <div class="absolute inset-2 rounded-full overflow-hidden border-4 border-black animate-[spin_12s_linear_infinite]">
            <img
                // PRUEBA ESTA RUTA: Si tu imagen está en assets
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
                        "DJ" <br/>
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