use leptos::prelude::*;

use crate::ui::{Animated, Narrow, Para, Prose, RichPara, Section, SectionTitle};

#[component]
pub fn AbstractSection() -> impl IntoView {
    view! {
        <Section>
            <Animated><Narrow>
                <SectionTitle>"Abstract"</SectionTitle>
                <Prose>
                    <Para>
                        "We introduce Habilis-β, a fast-motion, long-lasting, and on-device vision-language-action (VLA) model designed for real-world deployment. "
                        "Current VLA evaluation remains largely confined to single-trial success rates under curated resets, which fails to capture the fast-motion and long-lasting capabilities essential for practical operation. "
                        "To address this, we introduce the Productivity–Reliability Plane (PRP) evaluating performance through Tasks per Hour (TPH) and Mean Time Between Intervention (MTBI) under a continuous-run protocol that demands both high-speed execution and sustained robustness."
                    </Para>
                    <Para>
                        "Habilis-β achieves high-performance by integrating language-free pre-training on large-scale play data for robust interaction priors with post-training on cyclic demonstrations that capture state drift across consecutive task iterations. "
                        "The system further employs ESPADA for phase-adaptive motion shaping to accelerate free-space transit, utilizes rectified-flow distillation to enable high-frequency control on edge devices, and incorporates classifier-free guidance (CFG) as a deployment-time knob to dynamically balance instruction adherence and learned interaction priors."
                    </Para>
                    <RichPara html=concat!(
                        "In 1-hour continuous-run evaluations, Habilis-β significantly outperforms π<sub>0.5</sub> in both simulation and real-world environments. ",
                        "In simulation, Habilis-β improves MTBI to 39.2s and TPH to 572.6 (from 30.5s and 120.5 for π<sub>0.5</sub>), ",
                        "while on a real-world humanoid logistics workflow, it achieves 137.4s MTBI and 124 TPH, surpassing π<sub>0.5</sub>'s 46.1s MTBI and 19 TPH. ",
                        "Finally, Habilis-β achieves the highest reported performance on the standard RoboTwin 2.0 leaderboard across representative tasks, validating its effectiveness in complex manipulation scenarios."
                    ) />
                </Prose>
            </Narrow></Animated>
        </Section>
    }
}
