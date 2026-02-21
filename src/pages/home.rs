use leptos::prelude::*;

use crate::components::abstract_section::AbstractSection;
use crate::components::bibtex::BibTexSection;
use crate::components::hero::Hero;
use crate::components::real_world_perf::RealWorldPerf;
use crate::components::rethinking_eval::RethinkingEval;
use crate::components::simulation_perf::SimulationPerf;
use crate::components::system_overview::SystemOverview;
use crate::components::teaser::TeaserVideo;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Hero />
        <TeaserVideo />
        <AbstractSection />
        <RethinkingEval />
        <SystemOverview />
        <SimulationPerf />
        <RealWorldPerf />
        <BibTexSection />
    }
}
