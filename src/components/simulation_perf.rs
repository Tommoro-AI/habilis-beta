use leptos::prelude::*;

use crate::islands::task_tabs::TaskTabs;
use crate::ui::{Animated, Figure, Section, SectionTitle, SectionVariant};

#[component]
pub fn SimulationPerf() -> impl IntoView {
    view! {
        <Section variant=SectionVariant::Alt>
            <Animated>
                <SectionTitle>"Simulation Performance"</SectionTitle>
                <Figure src="/images/simulation_performance.svg" alt="Simulation Performance" />
                <TaskTabs />
            </Animated>
        </Section>
    }
}
