use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router, extract::Path};
use serde::Serialize;
use minijinja::render;

mod templates;

#[derive(Debug, Serialize)]
struct Items {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
struct Profile{
    full_name: String,
    items: Vec<Items>,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

async fn home() -> Html<&'static str> {
    println!("GET: /");
    Html("Hello, world!")
}

async fn get_profile(
        Path(name): Path<String>) 
        -> Html<String> {
    println!("GET: /p/{}", name);
    let orders_example = vec![
        Items {
            id: 1,
            name: "Scarpa Instinct VSR".to_string(),
        },
        Items {
            id: 2,
            name: "LaSprotiva Skwarma".to_string(),
        },
    ];
    let profile_example = Profile {
        full_name: name,
        items: orders_example,
    };
    
    let r = render!(templates::PROFILE_TEMPLATE, profile => profile_example);
    Html(r)
}

#[tokio::main]
async fn main() {
    let server_config = ServerConfig{
        host: "0.0.0.0".into(),
        port: "3000".into(),
    };
    println!("Starting server on {host}:{port}", host=server_config.host, port=server_config.port);

    let addr: SocketAddr = format!("{}:{}", server_config.host, server_config.port)
        .parse()
        .unwrap();

    // single router
    let app = Router::new() // state will be available to all the routes
        .route("/", get(home))
        .route("/p/:profile", get(get_profile))
        .with_state(server_config);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
