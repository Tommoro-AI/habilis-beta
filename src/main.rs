#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;
    use axum::Router;
    use habilis_beta::app::*;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.with_context(|| format!("failed to bind TCP listener on {addr}"))?;
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .context("axum server exited with error")?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
