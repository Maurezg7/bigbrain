+++
title = "Actix Web"
summary = "Aplicación, rutas con macros, extractores, estado con web::Data, middleware, errores y comparación con Axum."
branch = "Frameworks de Rust"
order = 3
level = "Intermedio"
tags = ["Rust", "Actix Web", "API REST", "middleware", "backend"]
+++

## Qué es

Actix Web es uno de los frameworks web más maduros y rápidos de Rust. Trae más cosas incluidas que Axum (su propio servidor HTTP, sesiones, archivos estáticos, WebSockets) y define las rutas con **macros de atributo**.

```toml
[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }
```

## Primer servidor

```rust
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hola() -> impl Responder {
    "Hola desde Actix Web"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hola))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
```

`HttpServer` recibe una **fábrica**: una función que construye la `App`. La ejecuta una vez por cada hilo de trabajo, y cada hilo atiende sus conexiones con su propia copia.

## Rutas y extractores

```rust
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Tema { id: u32, titulo: String }

#[derive(Deserialize)]
struct NuevoTema { titulo: String }

#[get("/temas/{id}")]
async fn ver_tema(id: web::Path<u32>) -> web::Json<Tema> {
    web::Json(Tema { id: id.into_inner(), titulo: "Ley de Ohm".into() })
}

#[post("/temas")]
async fn crear_tema(nuevo: web::Json<NuevoTema>) -> HttpResponse {
    let tema = Tema { id: 1, titulo: nuevo.into_inner().titulo };
    HttpResponse::Created().json(tema)
}
```

| Extractor | Obtiene |
|---|---|
| `web::Path<T>` | parámetros de la ruta |
| `web::Query<T>` | parámetros de la URL |
| `web::Json<T>` | cuerpo JSON |
| `web::Form<T>` | formulario |
| `web::Data<T>` | estado compartido |
| `HttpRequest` | el pedido completo |

## Estado compartido

Como la fábrica corre en cada hilo, el estado común se crea **afuera** y se clona hacia adentro. `web::Data<T>` es un `Arc<T>`.

```rust
struct Estado { db: sqlx::PgPool }

#[get("/temas")]
async fn listar(estado: web::Data<Estado>) -> actix_web::Result<web::Json<Vec<Tema>>> {
    let temas = consultar(&estado.db).await?;
    Ok(web::Json(temas))
}

let estado = web::Data::new(Estado { db });

HttpServer::new(move || {
    App::new().app_data(estado.clone()).service(listar)
})
```

> [!WARNING]
> Si creás el estado **dentro** de la fábrica, cada hilo tendrá el suyo y no se compartirá nada. Un contador en memoria, por ejemplo, daría valores distintos según qué hilo atienda.

## Organizar las rutas

```rust
fn configurar(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(listar)
            .service(ver_tema)
            .service(crear_tema),
    );
}

App::new().configure(configurar)
```

Cada módulo expone su función `configurar`, y la aplicación las reúne.

## Errores

Un handler puede devolver `Result<T, E>` si `E` implementa `ResponseError`:

```rust
use actix_web::{http::StatusCode, ResponseError};

#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("no encontrado")]
    NoEncontrado,
    #[error("error interno")]
    Interno(#[from] sqlx::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NoEncontrado => StatusCode::NOT_FOUND,
            ApiError::Interno(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
```

## Middleware

```rust
use actix_web::middleware::{Compress, Logger};

App::new()
    .wrap(Logger::default())
    .wrap(Compress::default())
```

Los que se agregan **últimos** se ejecutan **primeros**. Para CORS existe el crate `actix-cors`; para sesiones, `actix-session`; para archivos estáticos, `actix-files`.

## Pruebas

```rust
use actix_web::{test, App};

#[actix_web::test]
async fn responde_hola() {
    let app = test::init_service(App::new().service(hola)).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());
}
```

## Actix Web o Axum

| | Actix Web | Axum |
|---|---|---|
| Rutas | macros `#[get]`, `#[post]` | funciones y `Router` |
| Middleware | sistema propio | ecosistema Tower |
| Runtime | Tokio, un runtime por hilo | Tokio multihilo |
| Estado | `web::Data<T>` | `State<T>` |
| Incluido | sesiones, archivos, WebSockets | núcleo mínimo más crates |
| Madurez | desde 2017 | desde 2021 |

Los dos son rápidos y aptos para producción. Axum suele elegirse por su integración con Tower y el resto del ecosistema de Tokio; Actix Web, por traer más resuelto. Otras opciones son **Rocket**, muy cómodo y con mucha ayuda de macros, y **Warp**, basado en filtros que se combinan.

## Buenas prácticas

- Estado creado fuera de la fábrica y compartido con `web::Data`.
- No bloquear: el trabajo pesado va a `web::block`.
- Un tipo de error con `ResponseError` y mensajes genéricos hacia el cliente.
- Límites de tamaño con `web::JsonConfig` y `PayloadConfig`.
- Rutas agrupadas con `scope` y `configure` por módulo.
