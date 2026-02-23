use leptos::prelude::*;

use crate::ui::{Animated, Figure, Section, SectionTitle, SectionVariant};

#[component]
pub fn SystemOverview() -> impl IntoView {
    view! {
        <Section variant=SectionVariant::Gradient>
            <Animated>
                <SectionTitle>"System Overview"</SectionTitle>
                <Figure src="/images/system_overview.svg" alt="Habilis-β Model Overview" />
            </Animated>
        </Section>
    }
}
