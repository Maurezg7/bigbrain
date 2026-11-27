+++
title = "Tokio: programación asíncrona en Rust"
summary = "Futures, async y await, el runtime de Tokio, tareas, canales, temporizadores y los errores típicos al bloquear el hilo."
branch = "Frameworks de Rust"
order = 1
level = "Intermedio"
tags = ["Rust", "Tokio", "async", "concurrencia", "futures"]
+++

## Por qué asíncrono

Un servidor pasa casi todo el tiempo **esperando**: una consulta a la base de datos, una respuesta de red, un archivo. Con un hilo del sistema por conexión, miles de conexiones significan miles de hilos casi siempre dormidos. El modelo asíncrono usa pocos hilos y reparte entre ellos muchas **tareas** livianas que ceden el control cuando tienen que esperar.

Casi todo el ecosistema web de Rust (Axum, Actix Web, SQLx, reqwest) se apoya en **Tokio**, el runtime asíncrono más usado.

## Futures, async y await

Una función `async` no ejecuta nada al llamarla: devuelve un **future**, un valor que representa un cálculo pendiente. Recién avanza cuando alguien lo espera con `.await`.

```rust
async fn obtener_usuario(id: u32) -> String {
    // simula una consulta lenta
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    format!("usuario-{id}")
}

#[tokio::main]
async fn main() {
    let nombre = obtener_usuario(7).await;
    println!("{nombre}");
}
```

> [!WARNING]
> Los futures de Rust son **perezosos**. Si llamás a una función `async` y no hacés `.await` (ni la entregás a `spawn`), no pasa nada. El compilador avisa con una advertencia.

La macro `#[tokio::main]` arranca el runtime y ejecuta `main` dentro de él. En `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## El runtime

| Parte | Función |
|---|---|
| Planificador (*scheduler*) | reparte las tareas entre un grupo de hilos, con robo de trabajo |
| Reactor de E/S | avisa cuando un socket o archivo está listo (epoll, kqueue, IOCP) |
| Temporizadores | despierta las tareas que esperan un plazo |
| Grupo de hilos bloqueantes | ejecuta el código que no puede ser asíncrono |

Por defecto usa tantos hilos como núcleos tiene la máquina.

## Tareas

`tokio::spawn` lanza una tarea que corre de forma concurrente y devuelve un `JoinHandle` para esperar su resultado.

```rust
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut tareas = JoinSet::new();
    for id in 1..=3 {
        tareas.spawn(async move { obtener_usuario(id).await });
    }
    while let Some(resultado) = tareas.join_next().await {
        println!("{}", resultado.unwrap());
    }
}
```

Las tres consultas avanzan a la vez: el total tarda lo que la más lenta, no la suma.

Lo que se pasa a `spawn` debe ser `Send + 'static`: la tarea puede cambiar de hilo y vivir más que la función que la creó. Por eso se usa `async move` y los datos compartidos van en un `Arc`.

## Esperar varias cosas

| Herramienta | Qué hace |
|---|---|
| `tokio::join!(a, b)` | espera a que terminen todos |
| `tokio::try_join!(a, b)` | igual, pero corta ante el primer error |
| `tokio::select!` | espera al primero que termine y cancela el resto |
| `tokio::time::timeout(d, f)` | falla si `f` tarda más que `d` |

```rust
use std::time::Duration;
use tokio::time::timeout;

match timeout(Duration::from_secs(2), obtener_usuario(7)).await {
    Ok(nombre) => println!("{nombre}"),
    Err(_) => eprintln!("la consulta tardó demasiado"),
}
```

En Rust, **cancelar** un future es simplemente dejar de esperarlo y soltarlo. Por eso el código entre dos `.await` debe dejar los datos en un estado coherente.

## Estado compartido

| Tipo | Uso |
|---|---|
| `Arc<T>` | compartir datos de solo lectura entre tareas |
| `Arc<std::sync::Mutex<T>>` | secciones críticas cortas, sin `.await` adentro |
| `Arc<tokio::sync::Mutex<T>>` | cuando hay que mantener el candado a través de un `.await` |
| `tokio::sync::RwLock<T>` | muchas lecturas, pocas escrituras |
| `tokio::sync::Semaphore` | limitar cuántas tareas hacen algo a la vez |

## Canales

Muchas veces es más simple **pasar mensajes** que compartir memoria.

| Canal | Forma |
|---|---|
| `mpsc` | muchos productores, un consumidor |
| `oneshot` | un solo valor, una sola vez |
| `broadcast` | cada receptor ve todos los mensajes |
| `watch` | los receptores ven solo el último valor |

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(32);

    for n in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("mensaje {n}")).await.unwrap();
        });
    }
    drop(tx); // sin emisores vivos, el canal se cierra

    while let Some(texto) = rx.recv().await {
        println!("{texto}");
    }
}
```

El canal `mpsc` es **acotado**: si se llena, `send` espera. Esa contrapresión evita que un productor rápido agote la memoria.

## No bloquear el runtime

Una tarea solo cede el control en un `.await`. Si hace un cálculo largo o una llamada bloqueante, **retiene el hilo** y las demás tareas de ese hilo se frenan.

| En lugar de | Usar |
|---|---|
| `std::thread::sleep` | `tokio::time::sleep` |
| `std::fs` | `tokio::fs` |
| `std::net` | `tokio::net` |
| cálculo pesado o biblioteca bloqueante | `tokio::task::spawn_blocking` |

```rust
let hash = tokio::task::spawn_blocking(move || calcular_hash_costoso(&clave))
    .await
    .unwrap();
```

## Apagado ordenado

```rust
tokio::select! {
    _ = servidor => {},
    _ = tokio::signal::ctrl_c() => println!("cerrando..."),
}
```

## Buenas prácticas

- Poner **límite de tiempo** a toda operación de red.
- No sostener un `std::sync::Mutex` a través de un `.await`.
- Limitar la concurrencia con un `Semaphore` o un canal acotado.
- Propagar errores con `?` y `Result`, no con `unwrap`, fuera de los ejemplos.
- Usar `tracing` para registrar eventos con contexto por tarea.
- Async sirve para E/S. Para cálculo paralelo puro conviene `rayon`.
