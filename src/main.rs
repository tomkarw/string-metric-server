use std::env;

use warp::Filter;

use string_metric_server::server::{user_connected, Connections};

#[tokio::main]
async fn main() {
    let connections = Connections::default();
    let connections = warp::any().map(move || connections.clone());

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("www/index.html"));

    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::fs::file("www/favicon.ico"));

    let statics = warp::path("static").and(warp::fs::dir("www/static/"));

    let string_metric =
        warp::path("ws")
            .and(warp::ws())
            .and(connections)
            .map(|ws: warp::ws::Ws, connections| {
                ws.on_upgrade(move |socket| user_connected(socket, connections))
            });

    let routes = index.or(favicon).or(statics).or(string_metric);

    let port = match env::args().nth(1) {
        Some(s) => s.parse::<u16>().unwrap(),
        None => 8080,
    };

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
