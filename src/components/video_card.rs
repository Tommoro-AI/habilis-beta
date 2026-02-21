use leptos::prelude::*;

#[component]
pub fn VideoCard(
    #[prop(into)] src: String,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] caption: Option<String>,
) -> impl IntoView {
    view! {
        <div class="rounded-xl overflow-hidden shadow-card border border-gray-100 bg-white
                    transition-all duration-300 hover:-translate-y-1 hover:shadow-card-hover hover:border-cyan">
            <video autoplay controls muted loop playsinline class="block w-full">
                <source src=src type="video/mp4" />
            </video>
            <div class="px-3 py-2 border-t border-gray-100">
                {title.map(|t| view! {
                    <h5 class="text-center text-sm text-gray-800 font-medium" inner_html=t></h5>
                })}
                {caption.map(|c| view! {
                    <p class="text-center font-semibold text-gray-500" inner_html=c></p>
                })}
            </div>
        </div>
    }
}
