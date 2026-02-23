use leptos::prelude::*;

// ---------------------------------------------------------------------------
// HeroBanner  -- gradient section shell with decorative glow
// ---------------------------------------------------------------------------

#[component]
pub fn HeroBanner(children: Children) -> impl IntoView {
    view! {
        <section class="bg-gradient-to-br from-dark via-dark-mid to-dark-deep relative overflow-hidden">
            <div class="absolute -top-[40%] -right-[20%] w-[600px] h-[600px]
                        bg-[radial-gradient(circle,rgba(0,254,255,0.25)_0%,transparent_70%)]
                        pointer-events-none"></div>
            <div class="max-w-4xl mx-auto px-4 py-8 pb-8 text-center">
                {children()}
            </div>
        </section>
    }
}

// ---------------------------------------------------------------------------
// HeroTitle  -- responsive main heading
// ---------------------------------------------------------------------------

#[component]
pub fn HeroTitle(children: Children) -> impl IntoView {
    view! {
        <h1 class="font-display text-white text-2xl sm:text-3xl md:text-[2.6rem]
                    font-extrabold leading-tight
                    drop-shadow-[0_0_40px_rgba(0,254,255,0.25)]">
            {children()}
        </h1>
    }
}

// ---------------------------------------------------------------------------
// HeroAuthorLink  -- author/institution link below the title
// ---------------------------------------------------------------------------

#[component]
pub fn HeroAuthorLink(
    #[prop(into)] href: String,
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <div class="mt-3 font-display">
            <a
                href=href
                class="text-cyan font-semibold hover:text-white hover:underline transition-opacity duration-300"
            >
                {label}
            </a>
        </div>
    }
}

// ---------------------------------------------------------------------------
// HeroLinks  -- flex container for action buttons
// ---------------------------------------------------------------------------

#[component]
pub fn HeroLinks(children: Children) -> impl IntoView {
    view! {
        <div class="mt-4 flex flex-wrap justify-center gap-3">
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// HeroButtonLink  -- pill-shaped icon + text link
// ---------------------------------------------------------------------------

#[component]
pub fn HeroButtonLink(
    #[prop(into)] href: String,
    #[prop(into)] icon: String,
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <a
            href=href
            class="inline-flex items-center gap-2 px-5 py-2 rounded-full border-2 border-cyan
                   text-cyan font-semibold text-sm md:text-base transition-all duration-300
                   hover:bg-cyan hover:text-dark hover:-translate-y-0.5 hover:shadow-cyan-hover"
        >
            <i class=icon></i>
            <span>{label}</span>
        </a>
    }
}
