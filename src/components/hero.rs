use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="bg-gradient-to-br from-dark via-dark-mid to-dark-deep relative overflow-hidden">
            <div class="absolute -top-[40%] -right-[20%] w-[600px] h-[600px] bg-[radial-gradient(circle,rgba(0,254,255,0.25)_0%,transparent_70%)] pointer-events-none"></div>
            <div class="max-w-4xl mx-auto px-4 py-8 pb-8 text-center">
                <h1 class="font-display text-white text-2xl sm:text-3xl md:text-[2.6rem] font-extrabold leading-tight drop-shadow-[0_0_40px_rgba(0,254,255,0.25)]">
                    "Habilis-β: A Fast-Motion and Long-Lasting "
                    <br class="hidden md:inline" />
                    "On-Device Vision-Language-Action Model"
                </h1>

                <div class="mt-3 font-display">
                    <a
                        href="https://tommoro.ai"
                        class="text-cyan font-semibold hover:text-white hover:underline transition-opacity duration-300"
                    >
                        "Tommoro Robotics"
                    </a>
                </div>

                <div class="mt-4 flex flex-wrap justify-center gap-3">
                    <a
                        href="https://arxiv.org/pdf/placeholder"
                        class="inline-flex items-center gap-2 px-5 py-2 rounded-full border-2 border-cyan text-cyan font-semibold
                               text-sm md:text-base transition-all duration-300
                               hover:bg-cyan hover:text-dark hover:-translate-y-0.5 hover:shadow-cyan-hover"
                    >
                        <i class="fas fa-file-pdf"></i>
                        <span>"Paper"</span>
                    </a>
                    <a
                        href="https://arxiv.org/abs/placeholder"
                        class="inline-flex items-center gap-2 px-5 py-2 rounded-full border-2 border-cyan text-cyan font-semibold
                               text-sm md:text-base transition-all duration-300
                               hover:bg-cyan hover:text-dark hover:-translate-y-0.5 hover:shadow-cyan-hover"
                    >
                        <i class="ai ai-arxiv"></i>
                        <span>"arXiv"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}
