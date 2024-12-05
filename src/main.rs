//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{extract::Query, response::Html, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/hello", get(handler_hello));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<html>
  <head>
    <title>HTMX with Axum Backend</title>
    <script src=\"https://unpkg.com/htmx.org@2.0.3\" integrity=\"sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq\" crossorigin=\"anonymous\"></script>

  </head>
  <body>
    <button hx-get=\"http://127.0.0.1:3000/hello\">
      Hello
    </button>
  </body>
</html>")
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}
