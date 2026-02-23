use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Prose  -- justified text container with vertical spacing between paragraphs
// ---------------------------------------------------------------------------

#[component]
pub fn Prose(children: Children) -> impl IntoView {
    view! {
        <div class="text-justify leading-relaxed space-y-4">
            {children()}
        </div>
    }
}

// ---------------------------------------------------------------------------
// Para  -- plain paragraph
// ---------------------------------------------------------------------------

#[component]
pub fn Para(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <p class=class.unwrap_or_default()>
            {children()}
        </p>
    }
}

// ---------------------------------------------------------------------------
// RichPara  -- paragraph rendered from raw HTML (for subscripts, etc.)
// ---------------------------------------------------------------------------

#[component]
pub fn RichPara(#[prop(into)] html: String) -> impl IntoView {
    view! {
        <p inner_html=html></p>
    }
}

// ---------------------------------------------------------------------------
// SubTitle  -- sub-section heading (h4)
// ---------------------------------------------------------------------------

#[component]
pub fn SubTitle(
    #[prop(into, optional)] html: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = "font-display font-bold text-gray-900 text-lg md:text-xl text-center mb-4";
    if let Some(h) = html {
        view! { <h4 class=class inner_html=h></h4> }.into_any()
    } else if let Some(c) = children {
        view! { <h4 class=class>{c()}</h4> }.into_any()
    } else {
        view! {}.into_any()
    }
}

// ---------------------------------------------------------------------------
// BulletList + RichItem  -- unordered list with HTML-capable items
// ---------------------------------------------------------------------------

#[component]
pub fn BulletList(children: Children) -> impl IntoView {
    view! {
        <ul class="list-disc pl-5 space-y-1">
            {children()}
        </ul>
    }
}

#[component]
pub fn RichItem(#[prop(into)] html: String) -> impl IntoView {
    view! {
        <li inner_html=html></li>
    }
}
