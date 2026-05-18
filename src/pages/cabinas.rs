// Archivo: src/pages/cabinas.rs
// Propósito: Página de catálogo de cabinas con carrusel por tarjeta y banner de políticas de entrega.
// Fecha de última edición: 2026-05-16 16:45 (hora local).
// Notas: Comentarios añadidos para documentar funcionamiento y secciones clave. Sin cambios funcionales.
use leptos::*;

/// Modelo de datos mínimo para representar una cabina en la UI.
/// - Todos los campos son estáticos porque el catálogo es fijo en tiempo de compilación.
/// - `imagenes` puede contener 1..N rutas; si N>1 se habilita el carrusel en la tarjeta.
struct Cabina {
    id: usize,
    nombre: &'static str,
    descripcion: &'static str,
    precio: i32,
    alto: &'static str,
    largo: &'static str,
    ancho: &'static str,
    imagenes: Vec<&'static str>,
}

#[component]
/// Componente de catálogo de cabinas.
/// - Renderiza encabezado, banner de entrega y un grid de tarjetas.
/// - Cada tarjeta puede mostrar un carrusel si hay >1 imagen (controlado por señales locales).
pub fn Cabinas() -> impl IntoView {
    // Fuente de datos estática para el catálogo.
    // Nota: si en el futuro se desea consumir desde backend/API, sustituir este `vec![]`
    // por una llamada async + recurso (Resource) de Leptos.
    let lista_cabinas = vec![
        Cabina {
            id: 1,
            nombre: "CABINA 1",
            descripcion: "Estructura iluminada con patrones LED RGB audiorrítmicos de alta intensidad.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/tripleT.jpg", "/assets/11.jpeg", "/assets/111.jpeg"],
        },
        Cabina {
            id: 2,
            nombre: "CABINA 2",
            descripcion: "Acabado negro mate espejo con tubos de luz pixelados controlados por software.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/assets/2.jpeg"],
        },
        Cabina {
            id: 3,
            nombre: "CABINA 3",
            descripcion: "Efecto de túnel infinito mediante espejos templados y micro-LEDS cian.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/assets/3.jpeg"],
        },
        Cabina {
            id: 4,
            nombre: "CABINA 4",
            descripcion: "Diseño robusto envuelto en estructura de aluminio tipo truss, aspecto puramente escénico.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/assets/4.jpeg"],
        },
        Cabina {
            id: 5,
            nombre: "CABINA 5",
            descripcion: "Estética ochentera con acrílicos rosa neón brillantes y rejillas retroiluminadas.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/assets/5.jpeg", "/assets/55.jpeg", "/assets/555.jpeg"],
        },
        Cabina {
            id: 6,
            nombre: "CABINA 6",
            descripcion: "Diseño minimalista oscuro ideal para eventos corporativos VIP elegantes.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/assets/6.jpeg"],
        },
        Cabina {
            id: 7,
            nombre: "CABINA 7",
            descripcion: "Estructura blanca traslúcida que se ilumina por completo desde adentro.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/assets/7.jpeg"],
        },
        Cabina {
            id: 8,
            nombre: "CABINA 8",
            descripcion: "Equipada con micro-proyectores láser integrados que apuntan directo a la pista.",
            precio: 3000,
            alto: "1.02m",
            largo: "1.00m",
            ancho: "50cm",
            imagenes: vec!["/assets/8.jpeg"],
        },
    ];

    view! {
        <main class="min-h-screen bg-[#030213] text-white font-sans overflow-x-hidden overflow-y-auto pb-24">

            // --- Encabezado ---
            <section class="relative pt-16 pb-12 px-10 md:px-24">
                <div class="max-w-7xl mx-auto w-full text-center">
                    <h1 class="text-4xl md:text-6xl font-black italic tracking-tighter uppercase mb-4">
                        "NUESTRAS " <span class="text-[#00f2ff] drop-shadow-[0_0_15px_rgba(0,242,255,0.4)]">"CABINAS"</span>
                    </h1>
                    <p class="text-gray-400 text-sm md:text-base max-w-2xl mx-auto">
                        "Precios competitivos para CDMX. Reutiliza el estilo que prefieras para tu setup."
                    </p>
                </div>
            </section>

            // --- Banner de Políticas de Entrega ---
            <section class="max-w-5xl mx-auto px-6 mb-16">
                <div class="bg-gradient-to-r from-[#ff2d75]/10 to-transparent border border-[#ff2d75]/30 p-6 rounded-2xl backdrop-blur-md grid sm:grid-cols-2 gap-6 items-center shadow-[0_0_30px_rgba(255,45,117,0.05)]">
                    <div>
                        <h3 class="text-lg font-black tracking-tight text-[#ff2d75] uppercase italic mb-1">
                            "MÉTODOS DE ENTREGA"
                        </h3>
                        <p class="text-xs text-gray-300 leading-relaxed">
                            "Entrega a domicilio e instalación profesional en el lugar de tu evento."
                        </p>
                    </div>
                    <div class="space-y-2.5 text-xs font-semibold uppercase tracking-wide">
                        <div class="flex items-center gap-3 text-white">
                            <span class="text-green-500 font-bold text-sm">"✔"</span>
                            <span>"Sin costo a menos de 5km del Estadio Azteca"</span>
                        </div>
                        <div class="flex items-center gap-3 text-[#00f2ff]">
                            <span class="font-bold text-sm">"➔"</span>
                            <span>"$250 MXN dentro de cualquier otra zona de CDMX"</span>
                        </div>
                    </div>
                </div>
            </section>

            // --- GRID DE TARJETAS MODULARES ---
            <section class="max-w-7xl mx-auto px-6 md:px-24">
                <div class="grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 items-stretch">
                    {lista_cabinas.into_iter().map(|cabina| {
                        // Estado local e independiente por tarjeta para el índice de imagen del carrusel.
                        // - `img_idx`: señal con el índice actual (0..total_imgs-1).
                        // - `set_img_idx.update(...)`: actualiza in-place respetando el valor previo.
                        // Nota: cada iteración crea sus propias señales, por lo que los carruseles
                        // de distintas tarjetas no se afectan entre sí.
                        let (img_idx, set_img_idx) = create_signal(0);
                        let total_imgs = cabina.imagenes.len();

                        view! {
                            <div class="bg-black/40 border border-gray-800 p-5 rounded-2xl flex flex-col justify-between backdrop-blur-md hover:border-[#ff2d75]/40 transition-all group shadow-[0_0_20px_rgba(0,0,0,0.5)]">
                                <div>
                                    // Contenedor de Imagen o Carrusel
                                    <div class="relative w-full h-48 bg-gray-950 rounded-xl overflow-hidden mb-4 border border-gray-900">
                                        <img
                                            src=move || cabina.imagenes[img_idx.get()]
                                            alt=cabina.nombre
                                            class="w-full h-full object-cover transition-all duration-500"
                                        />

                                        // Controles del carrusel si hay más de una imagen:
                                        // - Se muestran solo si `total_imgs > 1`.
                                        // - Los botones izquierda/derecha actualizan `img_idx` con wrap-around (cíclico).
                                        // - `e.stop_propagation()` evita que el click burbujee y active otros manejadores del contenedor.
                                        // - `pointer-events-none` en el contenedor y `pointer-events-auto` en los botones
                                        //   permiten hacer clic solo en los botones, no en toda la superposición.
                                        {if total_imgs > 1 {
                                            view! {
                                                <div class="absolute inset-0 flex justify-between items-center px-2 pointer-events-none">
                                                    <button
                                                        on:click=move |e| {
                                                            e.stop_propagation();
                                                            set_img_idx.update(|idx| {
                                                                if *idx == 0 { *idx = total_imgs - 1; } else { *idx -= 1; }
                                                            });
                                                        }
                                                        class="pointer-events-auto bg-black/70 hover:bg-[#ff2d75] text-white font-black text-xs w-7 h-7 flex items-center justify-center rounded-full border border-gray-800 hover:border-[#ff2d75] transition-all cursor-pointer select-none"
                                                    >
                                                        "‹"
                                                    </button>
                                                    <button
                                                        on:click=move |e| {
                                                            e.stop_propagation();
                                                            set_img_idx.update(|idx| {
                                                                if *idx == total_imgs - 1 { *idx = 0; } else { *idx += 1; }
                                                            });
                                                        }
                                                        class="pointer-events-auto bg-black/70 hover:bg-[#ff2d75] text-white font-black text-xs w-7 h-7 flex items-center justify-center rounded-full border border-gray-800 hover:border-[#ff2d75] transition-all cursor-pointer select-none"
                                                    >
                                                        "›"
                                                    </button>
                                                </div>

                                                // Indicador de puntitos inferiores
                                                <div class="absolute bottom-2 left-0 right-0 flex justify-center gap-1.5">
                                                    {(0..total_imgs).map(|i| {
                                                        view! {
                                                            <span class=move || format!("h-1 rounded-full transition-all {}",
                                                                if img_idx.get() == i { "w-3 bg-[#ff2d75]" } else { "w-1 bg-gray-600" })
                                                            ></span>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </div>

                                    // Información de la Cabina
                                    <h3 class="text-base font-black tracking-tight group-hover:text-[#ff2d75] transition-colors mb-1 uppercase">
                                        {cabina.nombre}
                                    </h3>
                                    <p class="text-[11px] text-gray-400 leading-relaxed mb-4 min-h-[44px]">
                                        {cabina.descripcion}
                                    </p>

                                    // Especificaciones Técnicas (Medidas Reutilizadas)
                                    <div class="grid grid-cols-3 gap-1 bg-black/60 border border-gray-900 p-2 rounded-xl text-center text-[10px] uppercase font-bold tracking-tight mb-4">
                                        <div class="flex flex-col border-r border-gray-900">
                                            <span class="text-gray-500 text-[8px]">"Alto"</span>
                                            <span class="text-white">{cabina.alto}</span>
                                        </div>
                                        <div class="flex flex-col border-r border-gray-900">
                                            <span class="text-gray-500 text-[8px]">"Largo"</span>
                                            <span class="text-white">{cabina.largo}</span>
                                        </div>
                                        <div class="flex flex-col">
                                            <span class="text-gray-500 text-[8px]">"Ancho"</span>
                                            <span class="text-white">{cabina.ancho}</span>
                                        </div>
                                    </div>
                                </div>

                                // Footer de Tarjeta: Precio Estático y Selección
                                <div class="border-t border-gray-900 pt-3 mt-2">
                                    <div class="flex justify-between items-center mb-3">
                                        <span class="text-[9px] text-gray-500 uppercase font-black tracking-widest">"Precio Venta"</span>
                                        <div class="text-lg font-black text-white">
                                            // Formateo simple de miles: 3000 -> 3,000 (sin i18n); para formatos avanzados usar utilidades dedicadas.
                                            "$" {format!("{},{}", cabina.precio / 1000, format!("{:03}", cabina.precio % 1000))} " MXN"
                                        </div>
                                    </div>
                                    <button class="w-full bg-gray-900/60 hover:bg-[#ff2d75] text-gray-400 hover:text-white text-xs font-bold py-2.5 rounded-xl border border-gray-800 hover:border-[#ff2d75] shadow-sm transition-all duration-300">
                                        "Seleccionar Cabina"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>
        </main>
    }
}