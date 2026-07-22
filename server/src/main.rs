use std::{env, net::SocketAddr, path::PathBuf};

use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
};
use http::header::{self, HeaderName, HeaderValue};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

/// Security headers applied to every response (static assets, SPA fallback,
/// and 404s alike). Doesn't touch `Cache-Control`, which is set separately
/// per-route below.
async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; \
             img-src 'self' data:; font-src 'self' data:; connect-src 'self'; \
             object-src 'none'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'",
        ),
    );
    response
}

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let static_dir: PathBuf = env::var("STATIC_DIR")
        .unwrap_or_else(|_| "dist".to_string())
        .into();

    let index_html = static_dir.join("index.html");
    let assets_dir = static_dir.join("assets");

    // Everything else falls back to index.html so client-side routing keeps
    // working on a hard refresh (equivalent to nginx's `try_files ... /index.html`).
    let spa_service = ServeDir::new(&static_dir).fallback(ServeFile::new(&index_html));

    // /assets/* is content-hashed by the build, so it's safe to cache forever.
    // No fallback here: a missing asset (e.g. a stale hashed filename after a
    // redeploy) must 404, not get served index.html with an immutable cache
    // header stuck on it. index.html itself is served through `spa_service`
    // below and never gets this header. The closure only applies the header
    // to successful responses, so the 404 case stays uncached too.
    let assets_service = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            |response: &http::Response<_>| {
                response
                    .status()
                    .is_success()
                    .then(|| HeaderValue::from_static("public, max-age=31536000, immutable"))
            },
        ))
        .service(ServeDir::new(&assets_dir));

    let app = Router::new()
        .nest_service("/assets", assets_service)
        .fallback_service(spa_service)
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn(security_headers));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {addr}: {err}"));

    println!(
        "listening on http://{addr} (serving {})",
        static_dir.display()
    );

    axum::serve(listener, app).await.unwrap();
}
