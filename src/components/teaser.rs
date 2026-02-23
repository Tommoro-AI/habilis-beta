use leptos::prelude::*;

use crate::ui::{Animated, Section, VideoPlayer};

#[component]
pub fn TeaserVideo() -> impl IntoView {
    view! {
        <Section>
            <Animated>
                <VideoPlayer
                    src="/videos/real_habilisbeta_dbcp.mp4"
                    caption="1-hour successful continuous operation of Habilis-β on the dual-bin conveyor packing task"
                />
            </Animated>
        </Section>
    }
}
