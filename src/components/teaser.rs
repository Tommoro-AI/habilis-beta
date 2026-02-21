use leptos::prelude::*;

#[component]
pub fn TeaserVideo() -> impl IntoView {
    view! {
        <section class="py-10 px-4 md:px-6">
            <div class="max-w-4xl mx-auto">
                <div class="fade-in-up">
                    <video
                        autoplay controls muted loop playsinline
                        class="w-full rounded-[14px] shadow-teaser border border-gray-100
                               md:rounded-[14px] rounded-[10px]"
                    >
                        <source src="/videos/real_habilisbeta_dbcp.mp4" type="video/mp4" />
                    </video>
                    <p class="text-center text-text-light text-sm md:text-[0.95rem] mt-3">
                        "1-hour successful continuous operation of Habilis-β on the dual-bin conveyor packing task"
                    </p>
                </div>
            </div>
        </section>
    }
}
