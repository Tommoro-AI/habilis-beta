use leptos::prelude::*;

#[island]
#[allow(unused)]
pub fn CopyButton(#[prop(into)] target_id: String) -> impl IntoView {
    let (label, set_label) = signal("Copy".to_string());
    let (copied, set_copied) = signal(false);

    let _tid = target_id.clone();
    #[allow(unused_variables)]
    let on_click = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            let tid = &_tid;
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            if let Some(el) = document.get_element_by_id(tid) {
                let text = el.text_content().unwrap_or_default();
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&text);

                set_label.set("Copied!".to_string());
                set_copied.set(true);

                let reset_label = set_label.clone();
                let reset_copied = set_copied.clone();
                let cb = Closure::once_into_js(move || {
                    reset_label.set("Copy".to_string());
                    reset_copied.set(false);
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    2000,
                );
            }
        }
    };

    let btn_class = move || {
        let base = "absolute top-2 right-2 z-10 inline-flex items-center gap-1.5 \
                    px-2.5 py-1 rounded-md text-xs font-medium transition-all duration-300 cursor-pointer";
        if copied.get() {
            format!("{base} bg-cyan text-dark border border-cyan opacity-100")
        } else {
            format!("{base} bg-gray-100 text-gray-600 border border-gray-200 opacity-50 hover:opacity-100")
        }
    };

    view! {
        <button class=btn_class on:click=on_click>
            <i class="fas fa-copy text-[10px]"></i>
            <span>{label}</span>
        </button>
    }
}
