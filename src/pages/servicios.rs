// Archivo: src/pages/servicios.rs
// Propósito: Página de "Servicios" para el sitio de DJ Gustavo construida con Leptos.
// Fecha de última edición: 2026-05-16 15:02 (hora local).
//
// Comentarios detallados de los cambios realizados (refactor sin cambiar comportamiento):
// 1) Se extrajeron helpers puros (negocio y formato) para eliminar duplicación y centralizar reglas:
//    - Constantes: HORAS_BASE=5, PRECIO_BASE_ESTANDAR=5500, PRECIO_BASE_PREMIUM=7500, COSTO_HORA_EXTRA=1200.
//      Razón: una única fuente de verdad para parámetros del simulador y precios base.
//    - Funciones de cálculo: 
//        cargo_por_personas(pax) → devuelve el cargo adicional por rangos (≤100: 0, ≤200: 3000, ≤300: 5500, >300: 7500).
//        costo_horas_extras(horas) → calcula (horas - HORAS_BASE) * COSTO_HORA_EXTRA si aplica.
//        total_paquete(base, horas, pax) → suma base + extras por horas + cargo por personas.
//    - Formateo de moneda: formatea_mxn(total) → "miles,XXX" para mostrar MXN con miles agrupados.
//    Impacto: se reemplazó lógica embebida/duplicada dentro de clausuras por llamadas a estos helpers.
//
// 2) Señales iniciales más expresivas:
//    - Antes: let (horas, set_horas) = create_signal(5)
//      Ahora: let (horas, set_horas) = create_signal(HORAS_BASE)
//      Razón: coherencia con la constante de negocio; si cambia HORAS_BASE, el simulador inicia acorde.
//
// 3) Clausuras de totales simplificadas y sin duplicación de reglas:
//    - calcular_total_estandar y calcular_total_premium ahora usan total_paquete(base, horas, pax)
//      en lugar de recalcular horas extras y cargo por personas en cada una.
//    - El formateo de precio se delega a formatea_mxn(total) para mantener un único estilo de salida.
//    Beneficio: menor repetición, menos riesgo de inconsistencias y funciones fácilmente testeables.
//
// 4) UI/Marcado:
//    - No se modificó el marcado Leptos ni las clases/estilos de las tarjetas, simulador o formulario.
//    - El comportamiento visual y las interacciones permanecen idénticos. Solo se tocó lógica interna.
//
// 5) Alcance:
//    - Cambios confinados a este archivo. No se alteraron rutas, componentes externos ni dependencias.
//
// Nota: Este refactor es 100% compatible; los totales, rangos y reglas de negocio producen los mismos
//       resultados que antes, pero con código más claro, reutilizable y mantenible.

use leptos::*;

// --- Helpers de negocio y formato ---
const HORAS_BASE: i32 = 5;
const PRECIO_BASE_ESTANDAR: i32 = 5500;
const PRECIO_BASE_PREMIUM: i32 = 7500;
const COSTO_HORA_EXTRA: i32 = 1200;

fn cargo_por_personas(pax: i32) -> i32 {
    if pax <= 100 { 0 }
    else if pax <= 200 { 3000 }
    else if pax <= 300 { 5500 }
    else { 7500 }
}

fn costo_horas_extras(horas: i32) -> i32 {
    if horas > HORAS_BASE { (horas - HORAS_BASE) * COSTO_HORA_EXTRA } else { 0 }
}

fn total_paquete(base: i32, horas: i32, pax: i32) -> i32 {
    base + costo_horas_extras(horas) + cargo_por_personas(pax)
}

fn formatea_mxn(total: i32) -> String {
    format!("{},{}", total / 1000, format!("{:03}", total % 1000))
}

#[component]
pub fn Servicios() -> impl IntoView {
    // --- ESTADOS GLOBALES DEL SIMULADOR ---
    let (horas, set_horas) = create_signal(HORAS_BASE);
    let (personas, set_personas) = create_signal(100);
    let (es_interior, set_es_interior) = create_signal(false);

    let calcular_total_estandar = move || {
        let total = total_paquete(PRECIO_BASE_ESTANDAR, horas.get(), personas.get());
        formatea_mxn(total)
    };

    let calcular_total_premium = move || {
        let total = total_paquete(PRECIO_BASE_PREMIUM, horas.get(), personas.get());
        formatea_mxn(total)
    };
    // --- ESTADOS DEL FORMULARIO---
    let (nombre, set_nombre) = create_signal(String::new());
    let (telefono, set_telefono) = create_signal(String::new());
    let (correo, set_correo) = create_signal(String::new());
    let (fecha, set_fecha) = create_signal(String::new());
    let (es_exterior, set_es_exterior) = create_signal(false);
    let (direccion, set_direccion) = create_signal(String::new());

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

            // Contenedor unificado para que todos los bloques queden alineados
            <div class="max-w-7xl mx-auto px-6 md:px-24 pb-24 space-y-12">

                // --- BLOQUE 1: GRID DE TARJETAS (STANDARD Y PREMIUM) ---
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
                                    <a href="https://wa.me/52155XXXXXXXX?text=Hola!%20Quiero%20cotizar%20el%20Servicio%20DJ%20Estandard%20para%20provincia."
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
                                    <a href="https://wa.me/52155XXXXXXXX?text=Hola!%20Quiero%20cotizar%20el%20paquete%20Premium%20Gustavo%20para%20provincia."
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

                // --- BLOQUE 2: SIMULADOR DE COSTOS ---
                <div class="bg-black/30 border border-gray-950 p-8 rounded-2xl max-w-4xl mx-auto backdrop-blur-md shadow-[0_0_50px_rgba(0,0,0,0.8)] border-t-[#ff2d75]/20">

                    <h2 class="text-xl font-black tracking-tight mb-8 text-center uppercase italic">
                        "Simulador de Costos " <span class="text-[#00f2ff] drop-shadow-[0_0_10px_rgba(0,242,255,0.3)]">"Interactivo"</span>
                    </h2>

                    // --- Selector Geográfico (Filtro CDMX / Interior) ---
                    <div class="flex justify-center gap-4 mb-8">
                        <button
                            on:click=move |_| set_es_interior.set(false)
                            class=move || format!("px-6 py-2.5 rounded-full font-black text-xs uppercase tracking-wider transition-all duration-300 {}",
                                if !es_interior.get() { "bg-[#00f2ff] text-black shadow-[0_0_20px_#00f2ff]" } else { "bg-black/50 border border-gray-800 text-gray-500 hover:text-gray-300" })
                        >
                            "CDMX"
                        </button>
                        <button
                            on:click=move |_| set_es_interior.set(true)
                            class=move || format!("px-6 py-2.5 rounded-full font-black text-xs uppercase tracking-wider transition-all duration-300 {}",
                                if es_interior.get() { "bg-[#ff2d75] text-white shadow-[0_0_20px_#ff2d75]" } else { "bg-black/50 border border-gray-800 text-gray-500 hover:text-gray-300" })
                        >
                            "Interior de la República"
                        </button>
                    </div>

                    // --- Grid de Controles Interactivos ---
                    <div class=move || format!("grid sm:grid-cols-2 gap-8 border-t border-gray-900 pt-8 transition-opacity duration-300 {}",
                        if es_interior.get() { "opacity-30 pointer-events-none" } else { "opacity-100" })
                    >
                        // Control de Horas (Slider)
                        <div class="flex flex-col justify-center">
                            <label class="block text-xs font-bold text-gray-400 uppercase tracking-wider mb-3">
                                "Duración del Evento: " <span class="text-white font-black text-sm ml-1">{move || horas.get()} " HRS"</span>
                            </label>
                            <input type="range"
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        set_horas.set(val);
                                    }
                                }
                                prop:value=move || horas.get()
                                min="5" max="12" step="1"
                                class="w-full accent-[#ff2d75] bg-gray-800 h-2 rounded-lg appearance-none cursor-pointer focus:outline-none"
                            />
                            <div class="flex justify-between text-[10px] text-gray-500 mt-2 uppercase font-semibold">
                                <span>"Mínimo: 5h"</span>
                                <span class="text-[#ff2d75] tracking-tight">"Hora Extra: $1,200"</span>
                                <span>"Máx: 12h"</span>
                            </div>
                        </div>

                        // Control de Invitados (Input Numérico)
                        <div>
                            <label class="block text-xs font-bold text-gray-400 uppercase tracking-wider mb-3">
                                "Cantidad de Invitados:"
                            </label>
                            <input type="number"
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<i32>().unwrap_or(10);
                                    set_personas.set(val);
                                }
                                prop:value=move || personas.get()
                                min="10" max="1000"
                                class="bg-black/60 text-white font-black px-4 py-2.5 rounded-xl w-full border border-gray-800 focus:border-[#00f2ff] outline-none text-sm transition-all focus:shadow-[0_0_15px_rgba(0,242,255,0.15)]"
                            />

                            <div class="grid grid-cols-3 gap-1 text-[9px] uppercase font-bold mt-3 text-center tracking-tighter">
                                <div class=move || format!("p-1.5 rounded border transition-all {}", if personas.get() <= 100 { "bg-[#00f2ff]/10 border-[#00f2ff]/30 text-[#00f2ff]" } else { "bg-transparent border-transparent text-gray-600" })>
                                    "10-100 Base"
                                </div>
                                <div class=move || format!("p-1.5 rounded border transition-all {}", if personas.get() > 100 && personas.get() <= 200 { "bg-[#00f2ff]/10 border-[#00f2ff]/30 text-[#00f2ff]" } else { "bg-transparent border-transparent text-gray-600" })>
                                    "101-200 +$3k"
                                </div>
                                <div class=move || format!("p-1.5 rounded border transition-all {}", if personas.get() > 200 { "bg-[#00f2ff]/10 border-[#00f2ff]/30 text-[#00f2ff]" } else { "bg-transparent border-transparent text-gray-600" })>
                                    "201-300+ +$5.5k+"
                                </div>
                            </div>
                        </div>
                    </div>

                    // Mensaje aclaratorio Provincia
                    {move || if es_interior.get() {
                        view! {
                            <div class="text-center text-xs text-[#ff2d75] font-bold mt-4 tracking-wide animate-pulse">
                                "ℹ Los costos extras de logística e instalación fuera de CDMX se calculan a medida mediante WhatsApp."
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>

                // --- BLOQUE 3: FORMULARIO DE CONTRATACIÓN ---
                <div class="bg-black/30 border border-gray-950 p-8 rounded-2xl max-w-4xl mx-auto backdrop-blur-md shadow-[0_0_50px_rgba(0,0,0,0.8)] border-t-[#ff2d75]/20 space-y-6">
                    <div>
                        <h2 class="text-xl font-black tracking-tight uppercase italic mb-1 text-left">
                            "Formulario de " <span class="text-[#ff2d75] drop-shadow-[0_0_10px_rgba(255,45,117,0.3)]">"Contratación"</span>
                        </h2>
                        <p class="text-xs text-gray-400">"Completa tus datos para congelar tu fecha con el anticipo."</p>
                    </div>

                    // Subsección: Contacto
                    <div class="space-y-4">
                        <h3 class="text-xs font-bold uppercase tracking-widest text-[#00f2ff] border-b border-gray-900 pb-2">"1. Datos de Contacto"</h3>

                        <div>
                            <label class="block text-[11px] font-bold text-gray-400 uppercase tracking-wider mb-1">"Nombre Completo"</label>
                            <input type="text"
                                on:input=move |ev| set_nombre.set(event_target_value(&ev))
                                prop:value=move || nombre.get()
                                placeholder="Ej. Ricardo Nuñez"
                                class="bg-black/50 text-white px-4 py-2.5 rounded-xl w-full border border-gray-800 focus:border-[#ff2d75] outline-none text-sm transition-all"
                            />
                        </div>

                        <div class="grid sm:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-[11px] font-bold text-gray-400 uppercase tracking-wider mb-1">"Teléfono Celular"</label>
                                <input type="tel"
                                    on:input=move |ev| set_telefono.set(event_target_value(&ev))
                                    prop:value=move || telefono.get()
                                    placeholder="55 1234 5678"
                                    class="bg-black/50 text-white px-4 py-2.5 rounded-xl w-full border border-gray-800 focus:border-[#ff2d75] outline-none text-sm transition-all"
                                />
                            </div>
                            <div>
                                <label class="block text-[11px] font-bold text-gray-400 uppercase tracking-wider mb-1">"Correo Electrónico"</label>
                                <input type="email"
                                    on:input=move |ev| set_correo.set(event_target_value(&ev))
                                    prop:value=move || correo.get()
                                    placeholder="correo@ejemplo.com"
                                    class="bg-black/50 text-white px-4 py-2.5 rounded-xl w-full border border-gray-800 focus:border-[#ff2d75] outline-none text-sm transition-all"
                                />
                            </div>
                        </div>
                    </div>

                    // Subsección: Logística
                    <div class="space-y-4 pt-2">
                        <h3 class="text-xs font-bold uppercase tracking-widest text-[#00f2ff] border-b border-gray-900 pb-2">"2. Logística del Evento"</h3>

                        <div class="grid sm:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-[11px] font-bold text-gray-400 uppercase tracking-wider mb-1">"Fecha del Evento"</label>
                                <input type="date"
                                    on:input=move |ev| set_fecha.set(event_target_value(&ev))
                                    prop:value=move || fecha.get()
                                    class="bg-black/50 text-white px-4 py-2.5 rounded-xl w-full border border-gray-800 focus:border-[#ff2d75] outline-none text-sm transition-all"
                                />
                            </div>
                            <div>
                                <label class="block text-[11px] font-bold text-gray-400 uppercase tracking-wider mb-1">"Tipo de Espacio"</label>
                                <div class="grid grid-cols-2 gap-2">
                                    <button
                                        on:click=move |_| set_es_exterior.set(false)
                                        class=move || format!("py-2.5 rounded-xl text-xs font-bold uppercase border tracking-wider transition-all {}",
                                            if !es_exterior.get() { "border-[#00f2ff] bg-[#00f2ff]/10 text-[#00f2ff]" } else { "border-gray-800 bg-transparent text-gray-500" })
                                    >
                                        "Interior"
                                    </button>
                                    <button
                                        on:click=move |_| set_es_exterior.set(true)
                                        class=move || format!("py-2.5 rounded-xl text-xs font-bold uppercase border tracking-wider transition-all {}",
                                            if es_exterior.get() { "border-[#00f2ff] bg-[#00f2ff]/10 text-[#00f2ff]" } else { "border-gray-800 bg-transparent text-gray-500" })
                                    >
                                        "Exterior"
                                    </button>
                                </div>
                            </div>
                        </div>

                        <div class="grid sm:grid-cols-3 gap-4">
                            <div>
                                <label class="block text-[11px] font-bold text-gray-500 uppercase tracking-wider mb-1">"Horas (Desde Simulador)"</label>
                                <input type="text" readonly prop:value=move || format!("{} HRS", horas.get()) class="bg-gray-900/40 text-gray-400 px-4 py-2.5 rounded-xl w-full border border-gray-900 text-sm outline-none cursor-not-allowed font-bold" />
                            </div>
                            <div>
                                <label class="block text-[11px] font-bold text-gray-500 uppercase tracking-wider mb-1">"Invitados (Desde Simulador)"</label>
                                <input type="text" readonly prop:value=move || format!("{} PAX", personas.get()) class="bg-gray-900/40 text-gray-400 px-4 py-2.5 rounded-xl w-full border border-gray-900 text-sm outline-none cursor-not-allowed font-bold" />
                            </div>
                            <div>
                                <label class="block text-[11px] font-bold text-gray-500 uppercase tracking-wider mb-1">"Ubicación Geográfica"</label>
                                <input type="text" readonly prop:value=move || if es_interior.get() { "PROVINCIA" } else { "CDMX / AREA METRO" } class="bg-gray-900/40 text-gray-400 px-4 py-2.5 rounded-xl w-full border border-gray-900 text-sm outline-none cursor-not-allowed font-bold text-[12px]" />
                            </div>
                        </div>

                        <div>
                            <label class="block text-[11px] font-bold text-gray-400 uppercase tracking-wider mb-1">"Dirección Completa del Evento"</label>
                            <textarea
                                on:input=move |ev| set_direccion.set(event_target_value(&ev))
                                prop:value=move || direccion.get()
                                placeholder="Calle, Número, Colonia, Delegación/Municipio, Código Postal"
                                rows="2"
                                class="bg-black/50 text-white px-4 py-2.5 rounded-xl w-full border border-gray-800 focus:border-[#ff2d75] outline-none text-sm transition-all resize-none"
                            ></textarea>
                        </div>
                    </div>
                </div>

            </div> // Cierre correcto del contenedor unificado
        </main>
    }
}