use leptos::prelude::*;

use super::section_wrapper::{Section, SectionTitle};
use crate::islands::copy_button::CopyButton;

const BIBTEX_CONTENT: &str = r#"@article{tr2026habilisbeta,
  author  = {Jesoon Kang and Taegeon Park and Jisu An and Soo Min Kimm and Jaejoon Kim and Jinu Pahk and Byungju Kim and Junseok Lee and Namheon Baek and Sungwan Ha and Hojun Baek and Eduardo Ayerve Cruz and Wontae Kim and Junghyeon Choi and Yousuk Lee and Joonmo Han and Sunghyun Cho and Sunghyun Kwon and Soyoung Lee and Jun Ki Lee and Seung-Joon Yi and Byoung-Tak Zhang and Theo Taeyeong Kim},
  title   = {{Habilis-$\beta$: A Fast-Motion and Long-Lasting On-Device Vision-Language-Action Model}},
  year    = {2026},
}"#;

#[component]
pub fn BibTexSection() -> impl IntoView {
    view! {
        <Section id="BibTeX".to_string()>
            <div class="max-w-[80%] mx-auto fade-in-up">
                <SectionTitle>"BibTeX"</SectionTitle>
                <div class="relative">
                    <CopyButton target_id="bibtex-code" />
                    <pre class="rounded-xl bg-gray-50 p-5 border border-gray-100 text-xs md:text-sm
                                overflow-x-auto whitespace-pre-wrap break-words">
                        <code id="bibtex-code">{BIBTEX_CONTENT}</code>
                    </pre>
                </div>
            </div>
        </Section>
    }
}
