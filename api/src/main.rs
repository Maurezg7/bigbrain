mod content;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use content::{AreaSummary, Heading, Library, TopicSummary};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
};

type Shared = Arc<Library>;

#[derive(Serialize)]
struct AreaDetail {
    #[serde(flatten)]
    area: AreaSummary,
    topics: Vec<TopicSummary>,
}

#[derive(Serialize)]
struct TopicDetail {
    #[serde(flatten)]
    summary: TopicSummary,
    area_title: String,
    color: String,
    html: String,
    headings: Vec<Heading>,
    prev: Option<TopicSummary>,
    next: Option<TopicSummary>,
}

#[derive(Deserialize)]
struct SearchParams {
    #[serde(default)]
    q: String,
}

async fn list_areas(State(lib): State<Shared>) -> Json<Vec<AreaSummary>> {
    Json(lib.areas.clone())
}

async fn get_area(
    State(lib): State<Shared>,
    Path(slug): Path<String>,
) -> Result<Json<AreaDetail>, StatusCode> {
    let area = lib.area(&slug).ok_or(StatusCode::NOT_FOUND)?.clone();
    let topics = lib.topics_of(&slug).map(|t| t.summary.clone()).collect();
    Ok(Json(AreaDetail { area, topics }))
}

async fn get_topic(
    State(lib): State<Shared>,
    Path((area, slug)): Path<(String, String)>,
) -> Result<Json<TopicDetail>, StatusCode> {
    let (topic, prev, next) = lib.topic(&area, &slug).ok_or(StatusCode::NOT_FOUND)?;
    let info = lib.area(&area).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(TopicDetail {
        summary: topic.summary.clone(),
        area_title: info.title.clone(),
        color: info.color.clone(),
        html: topic.html.clone(),
        headings: topic.headings.clone(),
        prev: prev.map(|t| t.summary.clone()),
        next: next.map(|t| t.summary.clone()),
    }))
}

async fn search(
    State(lib): State<Shared>,
    Query(params): Query<SearchParams>,
) -> Json<Vec<TopicSummary>> {
    Json(
        lib.search(&params.q, 30)
            .into_iter()
            .map(|t| t.summary.clone())
            .collect(),
    )
}

#[tokio::main]
async fn main() {
    let content_dir = PathBuf::from(env::var("BIGBRAIN_CONTENT").unwrap_or("content".into()));
    let web_dir = PathBuf::from(env::var("BIGBRAIN_WEB").unwrap_or("web/build".into()));
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8787);

    let library = Library::load(&content_dir).unwrap_or_else(|e| {
        eprintln!(
            "No se pudo cargar el contenido de {}: {e}",
            content_dir.display()
        );
        std::process::exit(1);
    });
    println!(
        "BigBrain: {} áreas, {} temas",
        library.areas.len(),
        library.topics.len()
    );

    let api = Router::new()
        .route("/areas", get(list_areas))
        .route("/areas/{slug}", get(get_area))
        .route("/topics/{area}/{slug}", get(get_topic))
        .route("/search", get(search))
        .fallback(|| async { StatusCode::NOT_FOUND })
        .with_state(Arc::new(library));

    // La web es una SPA: cualquier ruta desconocida devuelve su index.
    let spa = ServeDir::new(&web_dir).fallback(ServeFile::new(web_dir.join("index.html")));
    let app = Router::new()
        .nest("/api", api)
        .nest_service("/media", ServeDir::new(content_dir.join("media")))
        .fallback_service(spa)
        .layer(CompressionLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("puerto en uso");
    println!("Escuchando en http://localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}
