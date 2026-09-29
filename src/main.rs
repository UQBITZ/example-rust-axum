use axum::{response::Html, routing::get, Router};

const MARKER: &str = "uqbitz-example-rust-axum-v2";

async fn index() -> Html<String> {
    let probe = std::env::var("START_PROBE").unwrap_or_else(|_| "unset".to_string());
    Html(format!(
        r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>Rust on UQBITZ</title></head>
<body style="font-family: system-ui, sans-serif; max-width: 40rem; margin: 4rem auto; padding: 0 1rem">
<h1>Rust + axum on UQBITZ</h1>
<p>This page is rendered by a compiled Rust binary running in a Firecracker microVM.</p>
<p><code id="marker">{MARKER}</code></p>
<p>start probe: <code id="start-probe">{probe}</code></p>
</body>
</html>"#
    ))
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    // UQBITZ sets PORT (3000 for Rust). Bind 0.0.0.0 so the platform can reach it.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let app = Router::new()
        .route("/", get(index))
        .route("/healthz", get(health));
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    println!("listening on {addr}");
    axum::serve(listener, app).await.expect("serve");
}
