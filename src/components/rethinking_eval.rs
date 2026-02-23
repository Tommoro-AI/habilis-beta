use leptos::prelude::*;

use crate::ui::{Animated, BulletList, Para, Section, SectionTitle, SectionVariant};

#[component]
pub fn RethinkingEval() -> impl IntoView {
    view! {
        <Section variant=SectionVariant::Alt>
            <Animated>
                <SectionTitle>"Rethinking Evaluation"</SectionTitle>

                <figure class="block w-[70%] sm:w-[85%] max-w-[1000px] mx-auto my-6">
                    <img src="/images/prp_plane.svg" alt="PRP" class="block w-full h-auto" />
                </figure>

                <Para>
                    "Current VLA evaluation remains largely confined to single-trial success rates under curated resets, which fails to capture the fast-motion and long-lasting capabilities essential for practical operation."
                </Para>
                <Para>
                    "We introduce a "
                    <b>"continuous-run protocol"</b>
                    ", where a robot operates for a fixed wall-clock duration without manual re-initialization of the task environment or robot pose. Performance is measured along two axes:"
                </Para>
                <BulletList>
                    <li><b>"Productivity"</b>" — Tasks per Hour (TPH)"</li>
                    <li><b>"Reliability"</b>" — Mean Time Between Intervention (MTBI)"</li>
                </BulletList>
                <Para>
                    "Together, they form the "
                    <b>"Productivity–Reliability Plane (PRP)"</b>
                    ". Deployment readiness means moving up and to the right."
                </Para>
            </Animated>
        </Section>
    }
}
