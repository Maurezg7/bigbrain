+++
title = "Rust: manejo de errores"
summary = "Errores recuperables con Result, el operador ?, tipos de error propios, pánicos, y las bibliotecas thiserror y anyhow."
branch = "Rust"
order = 4
level = "Intermedio"
tags = ["Rust", "Result", "errores", "panic", "anyhow"]
+++

## Dos clases de errores

| Clase | Mecanismo | Ejemplo |
|---|---|---|
| **Recuperable** | `Result<T, E>` | no existe un archivo, entrada inválida, falla la red |
| **Irrecuperable** | `panic!` | un error de programación: índice fuera de rango, invariante rota |

Rust **no tiene excepciones**. Un error recuperable es un **valor** que la función devuelve, y su tipo lo anuncia en la firma.

## Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs;

match fs::read_to_string("config.toml") {
    Ok(texto) => println!("{texto}"),
    Err(error) => eprintln!("No se pudo leer: {error}"),
}
```

> [!IMPORTANT]
> El compilador no deja usar el valor sin tratar antes el posible error, y advierte si un `Result` se ignora.

## El operador ?

Propagar errores a mano es repetitivo. El operador `?` lo resume: si el resultado es `Ok`, entrega el valor; si es `Err`, **retorna** ese error de inmediato.

```rust
use std::fs;
use std::io;

fn leer_puerto(ruta: &str) -> Result<String, io::Error> {
    let texto = fs::read_to_string(ruta)?;
    Ok(texto.trim().to_string())
}
```

Equivale a:

```rust
let texto = match fs::read_to_string(ruta) {
    Ok(t) => t,
    Err(e) => return Err(e.into()),
};
```

`?` solo puede usarse en funciones que devuelven `Result` (u `Option`). Además convierte el error con `From`, lo que permite mezclar errores de distinto tipo.

`main` también puede devolver un `Result`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puerto = leer_puerto("puerto.txt")?;
    println!("{puerto}");
    Ok(())
}
```

## Métodos de Result

| Método | Qué hace |
|---|---|
| `map(f)` | transforma el valor de `Ok` |
| `map_err(f)` | transforma el error |
| `and_then(f)` | encadena otra operación que puede fallar |
| `unwrap_or(valor)` | el valor, o uno por defecto |
| `unwrap_or_else(f)` | el valor, o el resultado de una función |
| `ok()` | convierte en `Option`, descartando el error |
| `is_ok()`, `is_err()` | consultas |
| `unwrap()`, `expect(msg)` | el valor, o **pánico** |

```rust
let puerto: u16 = std::env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(8787);
```

## Errores propios

Una biblioteca debería definir su tipo de error, normalmente un enum:

```rust
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorConfig {
    Lectura(io::Error),
    PuertoInvalido(ParseIntError),
    PuertoReservado(u16),
}

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lectura(e) => write!(f, "no se pudo leer la configuración: {e}"),
            Self::PuertoInvalido(e) => write!(f, "el puerto no es un número: {e}"),
            Self::PuertoReservado(p) => write!(f, "el puerto {p} está reservado"),
        }
    }
}

impl std::error::Error for ErrorConfig {}

impl From<io::Error> for ErrorConfig {
    fn from(e: io::Error) -> Self {
        Self::Lectura(e)
    }
}

impl From<ParseIntError> for ErrorConfig {
    fn from(e: ParseIntError) -> Self {
        Self::PuertoInvalido(e)
    }
}
```

Con las conversiones `From`, `?` funciona con ambos orígenes:

```rust
fn cargar_puerto(ruta: &str) -> Result<u16, ErrorConfig> {
    let texto = std::fs::read_to_string(ruta)?;        // io::Error
    let puerto: u16 = texto.trim().parse()?;           // ParseIntError
    if puerto < 1024 {
        return Err(ErrorConfig::PuertoReservado(puerto));
    }
    Ok(puerto)
}
```

Quien llama puede distinguir cada caso con `match`.

## thiserror y anyhow

Dos bibliotecas eliminan el código repetitivo.

**thiserror** genera `Display`, `Error` y `From`. Es para **bibliotecas**, donde el llamador necesita distinguir los errores:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum ErrorConfig {
    #[error("no se pudo leer la configuración")]
    Lectura(#[from] std::io::Error),

    #[error("el puerto no es un número")]
    PuertoInvalido(#[from] std::num::ParseIntError),

    #[error("el puerto {0} está reservado")]
    PuertoReservado(u16),
}
```

**anyhow** ofrece un tipo de error único que acepta cualquiera y permite agregar **contexto**. Es para **aplicaciones**, donde el error se informa y listo:

```rust
use anyhow::{bail, Context, Result};

fn cargar_puerto(ruta: &str) -> Result<u16> {
    let texto = std::fs::read_to_string(ruta)
        .with_context(|| format!("no se pudo leer {ruta}"))?;
    let puerto: u16 = texto.trim().parse().context("el puerto no es un número")?;
    if puerto < 1024 {
        bail!("el puerto {puerto} está reservado");
    }
    Ok(puerto)
}
```

| | thiserror | anyhow |
|---|---|---|
| Para | bibliotecas | aplicaciones |
| El llamador | distingue las variantes | solo informa |
| Aporta | tipos de error precisos | contexto y comodidad |

## Pánicos

Un **pánico** detiene el hilo: desenrolla la pila, libera los recursos e informa el mensaje.

```rust
panic!("estado imposible");
let x = vector[99];                  // pánico si el índice no existe
let n: i32 = "abc".parse().unwrap(); // pánico: el Result era Err
```

| Macro | Uso |
|---|---|
| `panic!` | detener con un mensaje |
| `assert!`, `assert_eq!` | verificar una condición |
| `unreachable!` | marcar código que nunca debería ejecutarse |
| `todo!` | marcar código pendiente |

> [!IMPORTANT]
> **`Result`** para lo que puede fallar por causas externas, aunque el programa esté bien escrito. **Pánico** solo para errores de programación, cuando seguir no tiene sentido.

### unwrap y expect

| Dónde | ¿Aceptable? |
|---|---|
| Ejemplos, prototipos y pruebas | sí |
| Cuando la lógica garantiza que no puede fallar | sí, con `expect` explicando por qué |
| Con datos del usuario, archivos o red | **no** |

```rust
let regex = Regex::new(r"^\d+$").expect("la expresión regular es constante y válida");
```

Para acceder a colecciones sin riesgo de pánico existen variantes que devuelven `Option`: `vector.get(i)`, `mapa.get(&clave)`, `a.checked_add(b)`.

## Option y Result juntos

```rust
let primero: Option<&str> = lista.first().copied();
let valor: Result<&str, &str> = primero.ok_or("la lista está vacía");
let quizas: Option<&str> = valor.ok();
```

Recolectar una secuencia de resultados: se detiene en el primer error.

```rust
let numeros: Result<Vec<i32>, _> = ["1", "2", "x"]
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();                       // Err: "x" no es un número
```

## En un servidor web

En Axum, un tipo de error propio se convierte en respuesta HTTP:

```rust
enum ErrorApi {
    NoEncontrado,
    Interno(anyhow::Error),
}

impl IntoResponse for ErrorApi {
    fn into_response(self) -> Response {
        match self {
            Self::NoEncontrado => (StatusCode::NOT_FOUND, "no existe").into_response(),
            Self::Interno(e) => {
                tracing::error!("{e:#}");
                (StatusCode::INTERNAL_SERVER_ERROR, "error interno").into_response()
            }
        }
    }
}
```

El detalle se **registra**; al cliente se le da un mensaje genérico, para no filtrar información interna.

## Buenas prácticas

| Hacer | Evitar |
|---|---|
| devolver `Result` y dejar que el llamador decida | `unwrap` sobre datos externos |
| agregar contexto al propagar | errores sin información de origen |
| tipos de error con variantes significativas | usar `String` como tipo de error en una biblioteca |
| manejar el error en el nivel que puede hacer algo | capturar y descartar (`let _ = ...`) sin motivo |
| mensajes en minúscula, sin punto final | mensajes que repiten el del error interno |

> [!TIP]
> Al escribir una función, preguntarse: ¿qué puede salir mal y quién puede hacer algo al respecto? La respuesta define el tipo de error.
