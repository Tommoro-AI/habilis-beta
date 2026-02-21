use leptos::prelude::*;

#[island]
pub fn TaskTabs() -> impl IntoView {
    let (active, set_active) = signal("dbb".to_string());

    let btn_class = move |task: &'static str| {
        let base = "px-5 py-2 md:px-7 md:py-2.5 rounded-full font-semibold text-sm md:text-[0.95rem] \
                    border transition-all duration-300 cursor-pointer select-none";
        if active.get() == task {
            format!(
                "{base} bg-dark text-white border-dark shadow-pill-active -translate-y-0.5"
            )
        } else {
            format!(
                "{base} bg-white text-text-light border-black/5 shadow-sm \
                 hover:border-black/10 hover:text-dark hover:-translate-y-0.5 hover:shadow-md"
            )
        }
    };

    let tasks: Vec<(&'static str, &'static str)> = vec![
        ("dbb", "Dump Bin Bigbin"),
        ("pds", "Place Dual Shoes"),
        ("sbt", "Stack Bowls Three"),
    ];

    view! {
        <div class="flex flex-wrap justify-center gap-2 md:gap-3 p-2.5 mt-6">
            {tasks.into_iter().map(|(id, label)| {
                let set = set_active.clone();
                view! {
                    <button
                        class=move || btn_class(id)
                        on:click=move |_| set.set(id.to_string())
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>

        // Dump Bin Bigbin
        <div style:display=move || if active.get() == "dbb" { "" } else { "none" }>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                <VideoCardInline src="/videos/sim_habilisbeta_w_espada_dbb.mp4" title="Habilis-β" />
                <VideoCardInline src="/videos/sim_habilisbeta_wo_espada_dbb.mp4" title="Habilis-β (w/o ESPADA)" />
                <VideoCardInline src="/videos/sim_pi05_dbb.mp4" title_html="π<sub>0.5</sub>" />
            </div>
        </div>

        // Place Dual Shoes
        <div style:display=move || if active.get() == "pds" { "" } else { "none" }>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                <VideoCardInline src="/videos/sim_habilisbeta_w_espada_pds.mp4" title="Habilis-β" />
                <VideoCardInline src="/videos/sim_habilisbeta_wo_espada_pds.mp4" title="Habilis-β (w/o ESPADA)" />
                <VideoCardInline src="/videos/sim_pi05_pds.mp4" title_html="π<sub>0.5</sub>" />
            </div>
        </div>

        // Stack Bowls Three
        <div style:display=move || if active.get() == "sbt" { "" } else { "none" }>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                <VideoCardInline src="/videos/sim_habilisbeta_w_espada_sbt.mp4" title="Habilis-β" />
                <VideoCardInline src="/videos/sim_habilisbeta_wo_espada_sbt.mp4" title="Habilis-β (w/o ESPADA)" />
                <VideoCardInline src="/videos/sim_pi05_sbt.mp4" title_html="π<sub>0.5</sub>" />
            </div>
        </div>
    }
}

#[island]
fn VideoCardInline(
    #[prop(into)] src: String,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] title_html: Option<String>,
) -> impl IntoView {
    view! {
        <div class="rounded-xl overflow-hidden shadow-card border border-gray-100 bg-white
                    transition-all duration-300 hover:-translate-y-1 hover:shadow-card-hover hover:border-cyan">
            <video autoplay controls muted loop playsinline class="block w-full">
                <source src=src type="video/mp4" />
            </video>
            <div class="px-3 py-2 border-t border-gray-100">
                {if let Some(html) = title_html {
                    view! { <h5 class="text-center text-xs md:text-sm text-gray-800 font-medium" inner_html=html></h5> }.into_any()
                } else if let Some(t) = title {
                    view! { <h5 class="text-center text-xs md:text-sm text-gray-800 font-medium">{t}</h5> }.into_any()
                } else {
                    view! { }.into_any()
                }}
            </div>
        </div>
    }
}
