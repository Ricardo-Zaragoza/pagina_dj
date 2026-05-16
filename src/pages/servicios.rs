// Archivo: src/pages/servicios.rs
// Propósito: Página de "Servicios" para el sitio de DJ Gustavo construida con Leptos.
// Fecha de última edición: 2026-05-15 17:25 (hora local).
//
// Resumen de cambios/comentarios:
// - Se añadió una barra de navegación pegajosa (sticky) con desenfoque de fondo y botón de CTA.
// - Se estableció un layout de pantalla completa con fondo oscuro y tipografía sans-serif.
// - Se actualizó el título principal para destacar la sección de Servicios con estilos responsivos.
// - Se añadieron clases utilitarias (tipo Tailwind) para espaciados, colores y efectos de hover.

use leptos::*;

#[component]
pub fn Servicios() -> impl IntoView {
    // --- ESTADOS GLOBALES DEL SIMULADOR ---
    let (horas, _set_horas) = create_signal(5);
    let (personas, _set_personas) = create_signal(100);
    let (es_interior, _set_es_interior) = create_signal(false);

    const PRECIO_BASE_ESTANDAR: i32 = 5500;
    const PRECIO_BASE_PREMIUM: i32 = 7500;
    const COSTO_HORA_EXTRA: i32 = 1200;

    let cargo_personas = move || {
        let p = personas.get();
        if p <= 100 { 0 }
        else if p <= 200 { 3000 }
        else if p <= 300 { 5500 }
        else { 7500 }
    };

    let calcular_total_estandar = move || {
        let horas_extras = if horas.get() > 5 { (horas.get() - 5) * COSTO_HORA_EXTRA } else { 0 };
        let total = PRECIO_BASE_ESTANDAR + horas_extras + cargo_personas();
        format!("{},{}", total / 1000, format!("{:03}", total % 1000))
    };

    let calcular_total_premium = move || {
        let horas_extras = if horas.get() > 5 { (horas.get() - 5) * COSTO_HORA_EXTRA } else { 0 };
        let total = PRECIO_BASE_PREMIUM + horas_extras + cargo_personas();
        format!("{},{}", total / 1000, format!("{:03}", total % 1000))
    };

    view! {
        <main class="min-h-screen bg-[#030213] text-white font-sans overflow-x-hidden overflow-y-auto">

            // --- Encabezado de la Página ---
            <section class="relative pt-16 pb-8 px-10 md:px-24">
                <div class="max-w-7xl mx-auto w-full text-center">
                    <h1 class="text-4xl md:text-6xl font-black italic tracking-tighter uppercase mb-4">
                        "NUESTROS " <span class="text-[#ff2d75] drop-shadow-[0_0_15px_rgba(255,45,117,0.4)]">"SERVICIOS"</span>
                    </h1>
                    <p class="text-gray-400 text-sm md:text-base max-w-md mx-auto">
                        "Precios base para CDMX. Modifica los parámetros en el simulador de abajo."
                    </p>
                </div>
            </section>

            // --- Contenedor de las Tarjetas ---
            <div class="max-w-7xl mx-auto px-6 md:px-24 pb-16">

                // --- GRID DE TARJETAS (STANDARD Y PREMIUM) ---
                <div class="grid md:grid-cols-2 gap-8 items-stretch">

                    // 1. TARJETA: SERVICIO DJ STANDARD
                    <div class="bg-black/40 border border-gray-800 p-8 rounded-2xl flex flex-col justify-between backdrop-blur-md relative hover:border-[#00f2ff]/40 transition-all shadow-[0_0_30px_rgba(0,242,255,0.02)]">
                        <div>
                            <div class="inline-block bg-[#00f2ff]/10 text-[#00f2ff] text-[10px] font-black px-3 py-1 rounded-full uppercase tracking-widest mb-4">
                                "Live Set"
                            </div>
                            <h3 class="text-2xl font-black tracking-tight mb-2">"SERVICIO DJ STANDARD"</h3>
                            <p class="text-sm text-gray-400 mb-6">"Ideal para eventos medianos o espacios cerrados."</p>

                            <ul class="space-y-3.5 text-gray-300 text-sm mb-8">
                                <li class="flex items-center gap-3">
                                    <span class="text-[#00f2ff]">"✔"</span> "Servicio de DJ Profesional (5 Horas Base)"
                                </li>
                                <li class="flex items-center gap-3">
                                    <span class="text-[#00f2ff]">"✔"</span> "Cabina Física Iluminada"
                                </li>
                                <li class="flex items-center gap-3">
                                    <span class="text-[#00f2ff]">"✔"</span> "Bocina Profesional de Alta Fidelidad"
                                </li>
                                <li class="flex items-center gap-3">
                                    <span class="text-[#00f2ff]">"✔"</span> "Set de Luces Rítmicas para la Pista"
                                </li>
                            </ul>
                        </div>

                        <div class="border-t border-gray-900 pt-6">
                            {move || if es_interior.get() {
                                view! {
                                    <a href="https://wa.me/52155XXXXXXXX?text=Hola!%20Quiero%20cotizar%20el%20Servicio%20DJ%20Standard%20para%20provincia."
                                       target="_blank"
                                       class="block text-center bg-green-600 hover:bg-green-500 text-white font-bold py-3.5 rounded-xl transition-all shadow-[0_0_15px_rgba(22,163,74,0.3)]">
                                        "Cotizar por WhatsApp"
                                    </a>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="flex flex-col mb-4">
                                        <span class="text-xs text-gray-500 uppercase tracking-wider">"Precio Estimado"</span>
                                        <div class="text-3xl font-black text-white">"$" {move || calcular_total_estandar()} " MXN"</div>
                                        <span class="text-[11px] text-[#00f2ff] font-medium">"Incluye " {move || horas.get()} " hrs base + extras"</span>
                                    </div>
                                    <button class="w-full bg-gradient-to-r from-[#00f2ff]/20 to-[#00f2ff]/10 hover:from-[#00f2ff] hover:to-[#00f2ff] text-[#00f2ff] hover:text-black font-bold py-3.5 rounded-xl border border-[#00f2ff]/30 shadow-[0_0_20px_rgba(0,242,255,0.1)] transition-all">
                                        "Apartar con Anticipo ($1,500)"
                                    </button>
                                }.into_view()
                            }}
                        </div>
                    </div>

                    // 2. TARJETA: PREMIUM
                    <div class="bg-black/40 border border-[#ff2d75]/30 p-8 rounded-2xl flex flex-col justify-between backdrop-blur-md relative hover:border-[#ff2d75] transition-all shadow-[0_0_40px_rgba(255,45,117,0.05)]">
                        <div class="absolute -top-3.5 right-6 bg-[#ff2d75] text-white text-[10px] font-black px-4 py-1 uppercase tracking-widest rounded-full shadow-[0_0_15px_#ff2d75]">
                            "Recomendado"
                        </div>

                        <div>
                            <div class="inline-block bg-[#ff2d75]/10 text-[#ff2d75] text-[10px] font-black px-3 py-1 rounded-full uppercase tracking-widest mb-4">
                                "Producción VIP"
                            </div>
                            <h3 class="text-2xl font-black tracking-tight mb-2 text-[#ff2d75]">"PREMIUM VORTEX"</h3>
                            <p class="text-sm text-gray-400 mb-6">"Máxima potencia visual y auditiva para eventos masivos."</p>

                            <ul class="space-y-3.5 text-gray-300 text-sm mb-8">
                                <li class="flex items-center gap-3">
                                    <span class="text-[#ff2d75]">"✔"</span> "Servicio de DJ Profesional (5 Horas Base)"
                                </li>
                                <li class="flex items-center gap-3">
                                    <span class="text-[#ff2d75]">"✔"</span> "Refuerzo de Audio (4 Bocinas de Alta Potencia + Subs)"
                                </li>
                                <li class="flex items-center gap-3">
                                    <span class="text-[#ff2d75]">"✔"</span> "Estructuras Truss, Cabezas Robóticas y Lasers"
                                </li>
                                <li class="flex items-center gap-3">
                                    <span class="text-[#ff2d75]">"✔"</span> "Efectos de Impacto: Máquinas de CO2 y Humo"
                                </li>
                                <li class="flex items-center gap-3">
                                    <span class="text-[#ff2d75]">"✔"</span> "Pirotecnia Premium y Chisperos Controlados"
                                </li>
                            </ul>
                        </div>

                        <div class="border-t border-gray-900 pt-6">
                            {move || if es_interior.get() {
                                view! {
                                    <a href="https://wa.me/52155XXXXXXXX?text=Hola!%20Quiero%20cotizar%20el%20paquete%20Premium%20Vortex%20para%20provincia."
                                       target="_blank"
                                       class="block text-center bg-green-600 hover:bg-green-500 text-white font-bold py-3.5 rounded-xl transition-all shadow-[0_0_15px_rgba(22,163,74,0.3)]">
                                        "Cotizar por WhatsApp"
                                    </a>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="flex flex-col mb-4">
                                        <span class="text-xs text-gray-500 uppercase tracking-wider">"Precio Estimado"</span>
                                        <div class="text-3xl font-black text-[#ff2d75]">"$" {move || calcular_total_premium()} " MXN"</div>
                                        <span class="text-[11px] text-[#ff2d75] font-medium">"Incluye show de efectos completo"</span>
                                    </div>
                                    <button class="w-full bg-[#ff2d75] hover:bg-[#ff1a63] text-white font-bold py-3.5 rounded-xl shadow-[0_0_25px_rgba(255,45,117,0.25)] transition-all">
                                        "Apartar con Anticipo ($1,500)"
                                    </button>
                                }.into_view()
                            }}
                        </div>
                    </div>

                </div>

            </div>
        </main>
    }
}