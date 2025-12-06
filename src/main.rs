#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::LeptosOptions;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use leptos_meta::MetaTags;
    use ppst_academy::app::App;
    use tower_http::compression::CompressionLayer;
    use tower_http::services::ServeDir;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ppst_academy=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get Leptos configuration
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // Generate route list from App component
    let routes = generate_route_list(App);

    // Shell function for rendering the HTML document
    fn shell(options: LeptosOptions) -> impl IntoView {
        view! {
            <!DOCTYPE html>
            <html lang="ko">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <MetaTags/>
                    <AutoReload options=options.clone()/>
                    <HydrationScripts options=options.clone()/>
                    <link rel="stylesheet" id="leptos" href="/pkg/ppst-academy.css"/>
                    <link rel="icon" type="image/x-icon" href="/favicon.ico"/>
                </head>
                <body>
                    <App/>
                </body>
            </html>
        }
    }

    // Build the application router
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let options = leptos_options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .nest_service("/pkg", ServeDir::new("target/site/pkg"))
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    tracing::info!("PPST Academy listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // No client-side main for this project
    // Client entry point is the hydrate function in lib.rs
}
