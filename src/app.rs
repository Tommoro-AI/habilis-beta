use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::pages::home::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="description" content="Habilis-β: A Fast-Motion and Long-Lasting On-Device Vision-Language-Action Model" />
                <meta name="keywords" content="Vision-Language-Action Models, Continuous-Run Evaluation, Productivity–Reliability Plane, Fast-Motion, Long-Lasting, On-Device, Rectified Flow, Play Data, Cyclic Data" />
                <meta name="google-site-verification" content="L0ES0rjxjdSRH4xoosR-cTh5IsGMU_kg0QJq3333d2Y" />

                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() islands=true />
                <HashedStylesheet id="leptos" options=options />

                <link rel="icon" href="/favicon.png" />
                <link href="https://fonts.googleapis.com/css?family=Google+Sans|Noto+Sans|Castoro" rel="stylesheet" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.0/css/all.min.css" />
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/jpswalsh/academicons@1/css/academicons.min.css" />

                <script async src="https://www.googletagmanager.com/gtag/js?id=G-PYVRSFMDRL"></script>
                <script>
                    "window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments)}gtag('js',new Date());gtag('config','G-PYVRSFMDRL');"
                </script>

                <title>"Habilis-β: A Fast-Motion and Long-Lasting On-Device Vision-Language-Action Model"</title>
            </head>
            <body class="font-sans text-text antialiased">
                <App />
                <script>
                    r#"document.addEventListener('DOMContentLoaded',function(){
                        const obs=new IntersectionObserver(e=>{
                            e.forEach(x=>{if(x.isIntersecting)x.target.classList.add('visible')})
                        },{threshold:0.1});
                        document.querySelectorAll('.fade-in-up').forEach(el=>obs.observe(el));
                    });"#
                </script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=HomePage />
            </Routes>
        </Router>
    }
}
