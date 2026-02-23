use leptos::prelude::*;

use crate::islands::copy_button::CopyButton;
use crate::ui::{Animated, CodeBlock, CodePre, Narrow, Section, SectionTitle};

const BIBTEX_CONTENT: &str = r#"@article{tr2026habilisbeta,
  author  = {Jesoon Kang and Taegeon Park and Jisu An and Soo Min Kimm and Jaejoon Kim and Jinu Pahk and Byungju Kim and Junseok Lee and Namheon Baek and Sungwan Ha and Hojun Baek and Eduardo Ayerve Cruz and Wontae Kim and Junghyeon Choi and Yousuk Lee and Joonmo Han and Sunghyun Cho and Sunghyun Kwon and Soyoung Lee and Jun Ki Lee and Seung-Joon Yi and Byoung-Tak Zhang and Theo Taeyeong Kim},
  title   = {{Habilis-$\beta$: A Fast-Motion and Long-Lasting On-Device Vision-Language-Action Model}},
  year    = {2026},
}"#;

#[component]
pub fn BibTexSection() -> impl IntoView {
    view! {
        <Section id="BibTeX".to_string()>
            <Animated><Narrow>
                <SectionTitle>"BibTeX"</SectionTitle>
                <CodeBlock>
                    <CopyButton target_id="bibtex-code" />
                    <CodePre id="bibtex-code" content=BIBTEX_CONTENT />
                </CodeBlock>
            </Narrow></Animated>
        </Section>
    }
}
