// Archivo: src/pages/servicios.rs
// Propósito: Página de "Servicios" para el sitio de DJ Gustavo construida con Leptos.
// Fecha de última edición: 2026-05-15 17:25 (hora local).
//
// Resumen de cambios/comentarios:
// - Se añadió una barra de navegación pegajosa (sticky) con desenfoque de fondo y botón de CTA.
// - Se estableció un layout de pantalla completa con fondo oscuro y tipografía sans-serif.
// - Se actualizó el título principal para destacar la sección de Servicios con estilos responsivos.
// - Se añadieron clases utilitarias (tipo Tailwind) para espaciados, colores y efectos de hover.

use leptos::*; // Importa las macros y tipos base de Leptos para construir vistas.
use leptos_router::A; // Importa el enlace de router "A" para navegación SPA sin recargar la página.

#[component]
pub fn Servicios() -> impl IntoView {
    // "view!" es la macro de Leptos que traduce JSX-like a nodos/efectos reactivos.
    view! {
        // Contenedor principal de la página:
        // - min-h-screen: asegura altura mínima de la altura de la ventana.
        // - bg-[#030213]: color de fondo personalizado (muy oscuro).
        // - overflow-x-hidden/overflow-y-auto: evita scroll horizontal y permite vertical.
        <main class="min-h-screen bg-[#030213] text-white font-sans overflow-x-hidden overflow-y-auto">

            // --- Barra de Navegación ---
            // - sticky top-0 z-50: la barra queda fija arriba durante el scroll y por encima del contenido.
            // - bg-black/20 + backdrop-blur-md: fondo semitransparente con desenfoque para efecto glassmorphism.
            // - px-10 py-6: espaciados horizontales/verticales.
            <nav class="flex justify-between items-center px-10 py-6 bg-black/20 backdrop-blur-md sticky top-0 z-50">
                <div class="flex items-center gap-2">
                    // Marca del sitio: tipografía muy marcada para identidad fuerte.
                    <span class="text-2xl font-black tracking-tighter text-white">"DJ Gustavo"</span>
                </div>
                // Menú de navegación (oculto en móviles con "hidden md:flex") para diseño responsive.
                <div class="hidden md:flex gap-8 text-sm font-medium text-gray-400">
                    // Enlaces SPA usando <A> del router para no recargar la página.
                    <A href="/" class="hover:text-white transition-colors">"Inicio"</A>
                    <A href="/servicios" class="hover:text-white transition-colors">"Servicios"</A>
                    // Enlaces placeholder (anclas normales) para secciones futuras.
                    <a href="#" class="hover:text-white transition-colors">"Cabinas"</a>
                    <a href="#" class="hover:text-white transition-colors">"Sobre Mí"</a>
                    <a href="#" class="hover:text-white transition-colors">"Contacto"</a>
                </div>
                // Botón de llamada a la acción (CTA) con color destacado y sombra brillante.
                <button class="bg-[#ff2d75] hover:bg-[#ff1a63] text-white px-6 py-2 rounded-md font-bold text-sm shadow-[0_0_20px_rgba(255,45,117,0.3)] transition-all">
                    "Reservar Ahora"
                </button>
            </nav>

            // --- Sección con el título actualizado ---
            // Sección héroe centrada verticalmente con contenedor ancho máximo.
            <section class="relative h-[90vh] flex items-center px-10 md:px-24">
                <div class="max-w-7xl mx-auto w-full text-center">
                    // Título responsivo:
                    // - text-5xl md:text-7xl: tamaños para móvil y escritorio.
                    // - font-black italic uppercase: estilo llamativo y en mayúsculas.
                    // - tracking-tighter: espaciado de letras más compacto.
                    <h1 class="text-5xl md:text-7xl font-black italic tracking-tighter uppercase mb-4">
                        "Página de " "Servicios"
                    </h1>
                </div>
            </section>

        </main>
    }
}