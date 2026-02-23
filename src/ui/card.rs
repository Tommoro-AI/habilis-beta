use leptos::prelude::*;

// ---------------------------------------------------------------------------
// InfoCard  -- bordered card for annotations / explanatory notes
// ---------------------------------------------------------------------------

#[component]
pub fn InfoCard(children: Children) -> impl IntoView {
    view! {
        <div class="mt-4 bg-white rounded-xl px-5 py-4 border border-gray-100
                    shadow-sm text-sm text-text-light leading-relaxed">
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// CodeBlock  -- pre/code block with relative positioning (for overlay buttons)
// ---------------------------------------------------------------------------

#[component]
pub fn CodeBlock(
    children: Children,
) -> impl IntoView {
    view! {
        <div class="relative">
            {children()}
        </div>
    }
}

#[component]
pub fn CodePre(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into)] content: String,
) -> impl IntoView {
    view! {
        <pre class="rounded-xl bg-gray-50 p-5 border border-gray-100 text-xs md:text-sm
                    overflow-x-auto whitespace-pre-wrap break-words">
            <code id=id>{content}</code>
        </pre>
    }
}
