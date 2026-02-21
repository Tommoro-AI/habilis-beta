use leptos::prelude::*;

use super::section_wrapper::{Section, SectionTitle};
use super::video_card::VideoCard;

#[component]
pub fn RealWorldPerf() -> impl IntoView {
    view! {
        <Section>
            <div class="fade-in-up">
                <SectionTitle>"Real-World Performance"</SectionTitle>

                <figure class="my-8">
                    <img src="/images/real_performance.svg" alt="Real-World Performance" class="block w-full h-auto" />
                </figure>

                // CFG Comparison
                <div class="mt-12">
                    <h4 class="font-display font-bold text-gray-900 text-lg md:text-xl text-center mb-4">
                        "Habilis-β with Different CFG Values"
                    </h4>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <VideoCard src="/videos/real_habilisbeta_cfg0.8_dbcp.mp4" title="Habilis-β w/ CFG 0.8" />
                        <VideoCard src="/videos/real_habilisbeta_cfg1.5_dbcp.mp4" title="Habilis-β w/ CFG 1.5" />
                    </div>
                    <div class="mt-4 bg-white rounded-xl px-5 py-4 border border-gray-100 shadow-sm text-sm text-text-light leading-relaxed">
                        <ul class="list-disc pl-5 space-y-1">
                            <li inner_html="<b class='text-gray-800'>Lower guidance (CFG &lt; 1.0):</b> Prioritizes learned interaction priors, yielding smoother motion and improved stability."></li>
                            <li inner_html="<b class='text-gray-800'>Higher guidance (CFG &gt; 1.0):</b> Strengthens instruction adherence, increasing task speed and aggressiveness."></li>
                        </ul>
                    </div>
                </div>

                // Baseline Failure Cases
                <div class="mt-8">
                    <h4 class="font-display font-bold text-gray-900 text-lg md:text-xl text-center mb-4"
                        inner_html="Baseline (π<sub>0.5</sub>) Failure Cases">
                    </h4>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <VideoCard src="/videos/real_base_grab_x.mp4" caption="Over-grasping" />
                        <VideoCard src="/videos/real_base_recovery_x.mp4" caption="Recovery failure" />
                        <VideoCard src="/videos/real_base_stuck.mp4" caption="Success then stuck" />
                        <VideoCard src="/videos/real_base_timeout.mp4" caption="Timeout" />
                    </div>
                </div>
            </div>
        </Section>
    }
}
