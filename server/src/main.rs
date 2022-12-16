use axum::extract::{DefaultBodyLimit};
use axum::{routing::post, Json, Router};
use headers::HeaderValue;
use std::sync::{Mutex};
use std::{net::SocketAddr};

use axum_extra::routing::SpaRouter;
use tracing::debug;

use serde_json::{json, Value};

use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use coqui_stt::Model;
use lazy_static::lazy_static;
use serde::{Deserialize};
use tower_http::cors::{Any, CorsLayer};

lazy_static! {
    static ref MODEL: Mutex<Model> = {
        let mut model = Model::new("./server/model.tflite").expect("Creating model");
        model
            .enable_external_scorer("./server/wiki-ru-6gram.scorer")
            .expect("Adding scorer");
        Mutex::new(model)
    };
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/process", post(process))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(DefaultBodyLimit::max(usize::MAX))
        .merge(SpaRouter::new("/", "./web/dist").index_file("index.html"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn process(speech: Json<Speech>) -> Json<Value> {
    let model = &mut *MODEL.lock().unwrap();

    let text = model.speech_to_text(speech.chunks.as_slice()).unwrap();

    debug!(text);

    Json(json!({ "text": text }))
}

#[derive(Deserialize)]
struct Speech {
    chunks: Vec<i16>,
}
