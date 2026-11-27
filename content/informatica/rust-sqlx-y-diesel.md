+++
title = "Bases de datos en Rust: SQLx, Diesel y SeaORM"
summary = "Consultas verificadas en compilación, pool de conexiones, transacciones, migraciones y cuándo conviene un ORM."
branch = "Frameworks de Rust"
order = 4
level = "Intermedio"
tags = ["Rust", "SQLx", "Diesel", "SeaORM", "bases de datos"]
+++

## Tres enfoques

| Biblioteca | Enfoque | Asíncrona | Verificación en compilación |
|---|---|---|---|
| **SQLx** | se escribe SQL directo | sí | sí, contra una base real |
| **Diesel** | constructor de consultas y ORM | no (hay `diesel-async`) | sí, por el sistema de tipos |
| **SeaORM** | ORM sobre SQLx | sí | parcial |

## SQLx

No es un ORM: se escribe SQL y la biblioteca lo convierte en tipos de Rust.

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "macros", "migrate"] }
```

### Pool de conexiones

Abrir una conexión es caro. Un **pool** mantiene varias abiertas y las presta a quien las pide.

```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&std::env::var("DATABASE_URL")?)
    .await?;
```

El pool es barato de clonar y se comparte por toda la aplicación.

### Consultas

```rust
#[derive(sqlx::FromRow)]
struct Tema { id: i32, titulo: String, nivel: Option<String> }

// verificada en tiempo de ejecución
let temas: Vec<Tema> = sqlx::query_as("SELECT id, titulo, nivel FROM temas WHERE area = $1")
    .bind("fisica")
    .fetch_all(&pool)
    .await?;

// verificada en tiempo de compilación
let tema = sqlx::query_as!(Tema, "SELECT id, titulo, nivel FROM temas WHERE id = $1", 7)
    .fetch_optional(&pool)
    .await?;
```

Las macros `query!` y `query_as!` se conectan a la base **al compilar** y comprueban que el SQL sea válido y que los tipos coincidan. Un nombre de columna mal escrito es un error de compilación. Para compilar sin base (en integración continua), `cargo sqlx prepare` guarda los metadatos en el repositorio.

| Método | Devuelve |
|---|---|
| `fetch_one` | una fila; error si no hay |
| `fetch_optional` | `Option` de una fila |
| `fetch_all` | `Vec` con todas |
| `fetch` | un flujo, para resultados grandes |
| `execute` | cantidad de filas afectadas |

> [!WARNING]
> Nunca armes SQL concatenando texto del usuario. Usá siempre parámetros (`$1`, `?`): es lo que evita la **inyección SQL**.

### Transacciones

```rust
let mut tx = pool.begin().await?;

sqlx::query!("UPDATE cuentas SET saldo = saldo - $1 WHERE id = $2", monto, origen)
    .execute(&mut *tx).await?;
sqlx::query!("UPDATE cuentas SET saldo = saldo + $1 WHERE id = $2", monto, destino)
    .execute(&mut *tx).await?;

tx.commit().await?;
```

Si la función sale antes del `commit` (por un `?`, por ejemplo), la transacción se **deshace** sola al soltarse.

### Migraciones

Los cambios de esquema se guardan como archivos SQL numerados en `migrations/` y se aplican en orden:

```bash
sqlx migrate add crear_temas
sqlx migrate run
```

O al arrancar el programa: `sqlx::migrate!().run(&pool).await?;`

## Diesel

Diesel describe las tablas en Rust y construye las consultas con métodos. El compilador rechaza las que no tienen sentido.

```rust
diesel::table! {
    temas (id) {
        id -> Int4,
        titulo -> Text,
        area -> Text,
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = temas)]
struct Tema { id: i32, titulo: String, area: String }

use diesel::prelude::*;

let resultado = temas::table
    .filter(temas::area.eq("fisica"))
    .order(temas::titulo.asc())
    .limit(10)
    .select(Tema::as_select())
    .load(&mut conexion)?;
```

El archivo `schema.rs` lo genera la herramienta `diesel` a partir de las migraciones. Como es bloqueante, en un servidor asíncrono se lo llama dentro de `spawn_blocking` o se usa `diesel-async`.

## SeaORM

Es un ORM asíncrono con entidades, relaciones y un estilo parecido al de otros lenguajes:

```rust
let temas = Tema::find()
    .filter(tema::Column::Area.eq("fisica"))
    .order_by_asc(tema::Column::Titulo)
    .all(&db)
    .await?;
```

## Cuál elegir

| Si... | Conviene |
|---|---|
| sabés SQL y querés control total | SQLx |
| querés máxima seguridad de tipos y consultas componibles | Diesel |
| preferís entidades y relaciones al estilo ORM clásico | SeaORM |

## Buenas prácticas

- **Un pool** por aplicación, con tamaño acorde a la base.
- Pedir solo las columnas necesarias, no `SELECT *`.
- Evitar el problema **N+1**: no hacer una consulta por cada fila de otra; usar `JOIN` o `WHERE id = ANY($1)`.
- **Índices** en las columnas por las que se filtra y se ordena.
- Transacciones cortas, sin llamadas de red adentro.
- Migraciones versionadas en el repositorio; nunca cambios manuales en producción.
- Separar el acceso a datos en un módulo propio (patrón repositorio), para poder probar la lógica sin base.
