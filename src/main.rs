use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};

use structopt::StructOpt;
use tera::Tera;
use tower_http::services::ServeDir;

mod rest;

#[derive(Debug, StructOpt)]
#[structopt(name = "")]
struct Opt {
    /// .http file name
    #[structopt(long = "file")]
    file: String,
}

struct AppState {
    opt: Opt,
    tera: Tera,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let base_html = include_str!("../templates/base.html");
    let index_html = include_str!("../templates/index.html");

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("base.html", &base_html), ("index.html", &index_html)])?;
    let state = Arc::new(AppState {
        opt: Opt::from_args(),
        tera,
    });

    // Build App and routes
    let app = Router::new()
        .route("/", get(rest::index))
        .route("/:index", get(rest::index))
        .route("/:index/response", get(rest::response))
        .with_state(state)
        .nest_service("/static", ServeDir::new("static"));

    // Bind to port and start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    println!("\nServing at: {}\n", format!("http://{}", addr.to_string()));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
