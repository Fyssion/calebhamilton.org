#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use axum::{extract::{Extension}, Router, routing::{get, post}};
    use tower_http::{trace::TraceLayer};
    use tower::ServiceBuilder;
    use std::sync::Arc;
    use fyssion_zone::app::*;
    use fyssion_zone::utils::post::{GetPost, GetPostMetadata};
    use fyssion_zone::pages::feed::feed;
    use fyssion_zone::pages::fallback::file_and_error_handler;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let _ = GetPost::register();
    let _ = GetPostMetadata::register();

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .route("/blog/feed.rss", get(feed))
        .fallback(file_and_error_handler)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(Arc::new(leptos_options)))
                .layer(TraceLayer::new_for_http())
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // HttpServer::new(move || {
    //     let leptos_options = &conf.leptos_options;
    //     let site_root = &leptos_options.site_root;

    //     App::new()
    //         .service(feed)
    //         .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
    //         .leptos_routes(
    //             leptos_options.to_owned(),
    //             routes.to_owned(),
    //             |cx| view! { cx, <App/> },
    //         )
    //         .service(Files::new("/", site_root))
    //     //.wrap(middleware::Compress::default())
    // })
    // .bind(&addr)?
    // .run()
    // .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
