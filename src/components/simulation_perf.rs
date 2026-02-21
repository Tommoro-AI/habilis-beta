use leptos::prelude::*;

use super::section_wrapper::{Section, SectionTitle, SectionVariant};
use crate::islands::task_tabs::TaskTabs;

#[component]
pub fn SimulationPerf() -> impl IntoView {
    view! {
        <Section variant=SectionVariant::Alt>
            <div class="fade-in-up">
                <SectionTitle>"Simulation Performance"</SectionTitle>

                <figure class="my-10">
                    <img
                        src="/images/simulation_performance.svg"
                        alt="Simulation Performance"
                        class="block w-full h-auto"
                    />
                </figure>

                <TaskTabs />
            </div>
        </Section>
    }
}
