// Archivo: src/components/nav.rs
// Propósito: Componente de barra de navegación superior reutilizable para la SPA de "DJ Gustavo".
// Fecha de última edición: 2026-05-15 17:43 (hora local).
//
// Notas de documentación:
// - Navegación SPA con <A> de `leptos_router` para evitar recargas.
// - Diseño sticky con fondo semitransparente y desenfoque (glassmorphism).
// - Menú responsive: oculto en móviles (`hidden md:flex`).
// - CTA destacado "Reservar Ahora" con color neón y sombra.

use leptos::*; // Macros/tipos base para construir vistas en Leptos.
use leptos_router::*; // Enrutador de Leptos; usamos <A> para enlaces SPA sin recarga.

#[component]
/// Componente de navegación superior.
///
/// Estructura:
/// - Logo/Marca que enlaza a la raíz (`/`).
/// - Menú con enlaces SPA a "Inicio" y "Servicios"; placeholders para secciones futuras.
/// - Botón de llamada a la acción (CTA) "Reservar Ahora".
///
/// Estilos clave:
/// - `sticky top-0 z-50` mantiene la nav fija al hacer scroll.
/// - `bg-black/20` + `backdrop-blur-md` generan efecto glassmorphism.
/// - Tipografía y espaciados consistentes con el resto del sitio.
pub fn Nav() -> impl IntoView {
    view! {
        // Contenedor principal de la navbar con sticky y blur de fondo
        <nav class="flex justify-between items-center px-10 py-6 bg-black/20 backdrop-blur-md sticky top-0 z-50">
            // Marca/Logo: vuelve a Inicio de forma instantánea (SPA)
            <div class="flex items-center gap-2">
                <A href="/" class="text-2xl font-black tracking-tighter text-white select-none">
                    "DJ Gustavo"
                </A>
            </div>

            // Menú de navegación (oculto en pantallas pequeñas)
            <div class="hidden md:flex gap-8 text-sm font-medium text-gray-400">
                // Enlaces SPA con transiciones de color al pasar el cursor
                <A href="/" class="hover:text-white transition-colors">"Inicio"</A>
                <A href="/servicios" class="hover:text-white transition-colors">"Servicios"</A>
                // Placeholders para futuras rutas/secciones
                <A href="/cabinas" class="hover:text-white transition-colors">"Cabinas"</A>
                <a href="#" class="hover:text-white transition-colors">"Blog"</a>
                <a href="#" class="hover:text-white transition-colors">"Reseñas"</a>
            </div>

            // CTA con color primario y sombra luminosa
            <button class="bg-[#ff2d75] hover:bg-[#ff1a63] text-white px-6 py-2 rounded-md font-bold text-sm shadow-[0_0_20px_rgba(255,45,117,0.3)] transition-all">
                "Reservar Ahora"
            </button>
        </nav>
    }
}