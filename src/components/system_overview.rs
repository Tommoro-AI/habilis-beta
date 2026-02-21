use leptos::prelude::*;

use super::section_wrapper::{Section, SectionTitle, SectionVariant};

#[component]
pub fn SystemOverview() -> impl IntoView {
    view! {
        <Section variant=SectionVariant::Gradient>
            <div class="text-justify fade-in-up">
                <SectionTitle>"System Overview"</SectionTitle>
                <figure class="w-full my-6 mx-auto rounded-2xl shadow-seamless
                               transition-all duration-300 hover:-translate-y-1 hover:shadow-seamless-hover">
                    <img
                        src="/images/system_overview.svg"
                        alt="Habilis-β Model Overview"
                        class="block w-full rounded-2xl"
                    />
                </figure>
            </div>
        </Section>
    }
}
