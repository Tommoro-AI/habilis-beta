use leptos::prelude::*;

use crate::ui::{
    Animated, BulletList, Figure, Grid, InfoCard, RichItem, Section, SectionTitle, Spacer,
    SpacerSize, SubTitle, VideoCard,
};

#[component]
pub fn RealWorldPerf() -> impl IntoView {
    view! {
        <Section>
            <Animated>
                <SectionTitle>"Real-World Performance"</SectionTitle>

                <Figure src="/images/real_performance.svg" alt="Real-World Performance" />

                <Spacer size=SpacerSize::Lg />
                <SubTitle>"Habilis-β with Different CFG Values"</SubTitle>
                <Grid cols=2>
                    <VideoCard src="/videos/real_habilisbeta_cfg0.8_dbcp.mp4" title="Habilis-β w/ CFG 0.8" />
                    <VideoCard src="/videos/real_habilisbeta_cfg1.5_dbcp.mp4" title="Habilis-β w/ CFG 1.5" />
                </Grid>
                <InfoCard>
                    <BulletList>
                        <RichItem html="<b class='text-gray-800'>Lower guidance (CFG &lt; 1.0):</b> Prioritizes learned interaction priors, yielding smoother motion and improved stability." />
                        <RichItem html="<b class='text-gray-800'>Higher guidance (CFG &gt; 1.0):</b> Strengthens instruction adherence, increasing task speed and aggressiveness." />
                    </BulletList>
                </InfoCard>

                <Spacer />
                <SubTitle html="Baseline (π<sub>0.5</sub>) Failure Cases" />
                <Grid cols=2>
                    <VideoCard src="/videos/real_base_grab_x.mp4" caption="Over-grasping" />
                    <VideoCard src="/videos/real_base_recovery_x.mp4" caption="Recovery failure" />
                    <VideoCard src="/videos/real_base_stuck.mp4" caption="Success then stuck" />
                    <VideoCard src="/videos/real_base_timeout.mp4" caption="Timeout" />
                </Grid>
            </Animated>
        </Section>
    }
}
