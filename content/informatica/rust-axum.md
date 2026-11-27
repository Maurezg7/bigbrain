+++
title = "Axum: APIs web en Rust"
summary = "Rutas, handlers y extractores, estado compartido, JSON con Serde, manejo de errores, middleware con Tower y pruebas."
branch = "Frameworks de Rust"
order = 2
level = "Intermedio"
tags = ["Rust", "Axum", "API REST", "Tower", "Serde"]
+++

## Qué es

Axum es un framework web mantenido por el equipo de Tokio. Es pequeño: se apoya en **Tokio** para la ejecución, en **Hyper** para HTTP y en **Tower** para el middleware. No usa macros para definir rutas; todo son funciones y tipos comunes. Es el framework con el que está hecho el servidor de BigBrain.

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
tower-http = { version = "0.6", features = ["trace", "cors", "compression-gzip"] }
```

## Primer servidor

```rust
use axum::{routing::get, Router};

async fn hola() -> &'static str {
    "Hola desde Axum"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hola));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Handlers y extractores

Un **handler** es una función `async` cuyos argumentos son **extractores**: tipos que saben obtener algo del pedido. Axum los resuelve por el tipo, y si alguno falla responde con el error adecuado sin llamar al handler.

| Extractor | Obtiene |
|---|---|
| `Path<T>` | parámetros de la ruta |
| `Query<T>` | parámetros de la URL (`?pagina=2`) |
| `Json<T>` | cuerpo JSON deserializado |
| `State<T>` | estado compartido de la aplicación |
| `HeaderMap` | encabezados |
| `Form<T>` | formulario |

```rust
use axum::{extract::{Path, Query}, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Paginacion { pagina: Option<u32> }

#[derive(Serialize)]
struct Tema { id: u32, titulo: String }

async fn ver_tema(Path(id): Path<u32>, Query(p): Query<Paginacion>) -> Json<Tema> {
    let _pagina = p.pagina.unwrap_or(1);
    Json(Tema { id, titulo: "Ley de Ohm".into() })
}
```

La ruta se declara con llaves: `.route("/temas/{id}", get(ver_tema))`. Hasta la versión 0.7 se escribía `/temas/:id`.

> [!TIP]
> El extractor que consume el cuerpo (`Json`, `Form`, `String`) debe ir **último** entre los argumentos.

## Respuestas

Un handler puede devolver cualquier tipo que implemente `IntoResponse`: un texto, `Json<T>`, un `StatusCode`, una tupla `(StatusCode, Json<T>)` o un `Result` de ellos.

```rust
use axum::http::StatusCode;

async fn crear_tema(Json(nuevo): Json<NuevoTema>) -> (StatusCode, Json<Tema>) {
    let tema = Tema { id: 1, titulo: nuevo.titulo };
    (StatusCode::CREATED, Json(tema))
}
```

## Estado compartido

Lo que comparten todos los pedidos (conexiones a la base, configuración, cachés) va en un estado que se clona por pedido; por eso se envuelve en `Arc` o se usan tipos baratos de clonar, como un `PgPool`.

```rust
use axum::extract::State;
use std::sync::Arc;

#[derive(Clone)]
struct App { db: sqlx::PgPool, config: Arc<Config> }

async fn listar(State(app): State<App>) -> Json<Vec<Tema>> {
    let temas = sqlx::query_as!(Tema, "SELECT id, titulo FROM temas")
        .fetch_all(&app.db).await.unwrap();
    Json(temas)
}

let app = Router::new().route("/temas", get(listar)).with_state(estado);
```

## Manejo de errores

Lo habitual es definir un tipo de error propio y enseñarle a convertirse en respuesta. Así los handlers usan `?` y nunca `unwrap`.

```rust
use axum::response::{IntoResponse, Response};

enum ApiError { NoEncontrado, Interno(anyhow::Error) }

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NoEncontrado => (StatusCode::NOT_FOUND, "no encontrado").into_response(),
            ApiError::Interno(e) => {
                tracing::error!("{e:#}");
                (StatusCode::INTERNAL_SERVER_ERROR, "error interno").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self { ApiError::Interno(e.into()) }
}

async fn ver(State(app): State<App>, Path(id): Path<i32>) -> Result<Json<Tema>, ApiError> {
    let tema = sqlx::query_as!(Tema, "SELECT id, titulo FROM temas WHERE id = $1", id)
        .fetch_optional(&app.db).await?
        .ok_or(ApiError::NoEncontrado)?;
    Ok(Json(tema))
}
```

> [!WARNING]
> Nunca devuelvas al cliente el texto de un error interno: puede revelar consultas, rutas o credenciales. Se registra en el servidor y se responde con un mensaje genérico.

## Organizar las rutas

```rust
fn rutas_temas() -> Router<App> {
    Router::new()
        .route("/", get(listar).post(crear_tema))
        .route("/{id}", get(ver).delete(borrar))
}

let app = Router::new()
    .nest("/api/temas", rutas_temas())
    .fallback(no_encontrado)
    .with_state(estado);
```

`nest` monta un grupo de rutas bajo un prefijo y `merge` combina dos routers.

## Middleware

Axum reutiliza el ecosistema **Tower**: cada capa envuelve al servicio y puede actuar antes y después del handler.

```rust
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

let app = app
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .layer(CorsLayer::permissive());
```

Para lógica propia, `axum::middleware::from_fn`:

```rust
use axum::{extract::Request, middleware::Next};

async fn exigir_token(req: Request, next: Next) -> Result<Response, StatusCode> {
    match req.headers().get("authorization") {
        Some(_) => Ok(next.run(req).await),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
```

Las capas agregadas con `.layer` afectan solo a las rutas definidas **antes** de la llamada.

## Pruebas

Un `Router` es un servicio de Tower, así que se lo prueba sin abrir un puerto:

```rust
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn responde_hola() {
    let app = Router::new().route("/", get(hola));
    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
```

## Buenas prácticas

- Handlers delgados: validan la entrada, llaman a la lógica y arman la respuesta.
- Un solo tipo de error por aplicación con `IntoResponse`.
- Validar los datos de entrada (por ejemplo con el crate `validator`).
- Límite de tamaño de cuerpo y de tiempo por pedido.
- Apagado ordenado con `with_graceful_shutdown`.
- Configuración por variables de entorno, nunca escrita en el código.
