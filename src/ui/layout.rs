use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Section
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SectionVariant {
    #[default]
    Default,
    Alt,
    Gradient,
}

#[component]
pub fn Section(
    #[prop(into, optional)] variant: SectionVariant,
    #[prop(into, optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    let bg = match variant {
        SectionVariant::Default => "",
        SectionVariant::Alt => "bg-gray-50",
        SectionVariant::Gradient => {
            "bg-gradient-to-b from-white to-cyan/[0.03] border-y border-gray-100"
        }
    };

    view! {
        <section id=id class=format!("py-16 px-4 md:px-6 {bg}")>
            <div class="max-w-4xl mx-auto">
                {children()}
            </div>
        </section>
    }
}

// ---------------------------------------------------------------------------
// SectionTitle
// ---------------------------------------------------------------------------

#[component]
pub fn SectionTitle(children: Children) -> impl IntoView {
    view! {
        <h2 class="font-display font-bold text-gray-900 text-3xl md:text-4xl text-center mb-7 pb-3 relative">
            {children()}
        </h2>
    }
}

// ---------------------------------------------------------------------------
// Animated  -- wraps children in a fade-in-up container
// ---------------------------------------------------------------------------

#[component]
pub fn Animated(children: Children) -> impl IntoView {
    view! {
        <div class="fade-in-up">
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Narrow  -- constrains width to 80%, centred
// ---------------------------------------------------------------------------

#[component]
pub fn Narrow(children: Children) -> impl IntoView {
    view! {
        <div class="max-w-[80%] mx-auto">
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Grid  -- responsive column grid
// ---------------------------------------------------------------------------

#[component]
pub fn Grid(
    #[prop(default = 2)] cols: u8,
    children: Children,
) -> impl IntoView {
    let cls = match cols {
        1 => "grid grid-cols-1 gap-4",
        3 => "grid grid-cols-1 md:grid-cols-3 gap-4",
        4 => "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
        _ => "grid grid-cols-1 md:grid-cols-2 gap-4",
    };

    view! {
        <div class=cls>
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Spacer  -- vertical whitespace
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SpacerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Spacer(#[prop(into, optional)] size: SpacerSize) -> impl IntoView {
    let cls = match size {
        SpacerSize::Sm => "mt-4",
        SpacerSize::Md => "mt-8",
        SpacerSize::Lg => "mt-12",
    };
    view! { <div class=cls></div> }
}
