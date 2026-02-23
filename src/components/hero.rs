use leptos::prelude::*;

use crate::ui::{HeroAuthorLink, HeroBanner, HeroButtonLink, HeroLinks, HeroTitle};

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <HeroBanner>
            <HeroTitle>
                "Habilis-β: A Fast-Motion and Long-Lasting "
                <br class="hidden md:inline" />
                "On-Device Vision-Language-Action Model"
            </HeroTitle>
            <HeroAuthorLink href="https://tommoro.ai" label="Tommoro Robotics" />
            <HeroLinks>
                <HeroButtonLink href="https://arxiv.org/pdf/placeholder" icon="fas fa-file-pdf" label="Paper" />
                <HeroButtonLink href="https://arxiv.org/abs/placeholder" icon="ai ai-arxiv" label="arXiv" />
            </HeroLinks>
        </HeroBanner>
    }
}
