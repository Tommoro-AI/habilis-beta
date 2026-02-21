use leptos::prelude::*;

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
        SectionVariant::Gradient => "bg-gradient-to-b from-white to-cyan/[0.03] border-y border-gray-100",
    };

    view! {
        <section id=id class=format!("py-16 px-4 md:px-6 {bg}")>
            <div class="max-w-4xl mx-auto">
                {children()}
            </div>
        </section>
    }
}

#[component]
pub fn SectionTitle(children: Children) -> impl IntoView {
    view! {
        <h2 class="font-display font-bold text-gray-900 text-3xl md:text-4xl text-center mb-7 pb-3 relative">
            {children()}
        </h2>
    }
}
