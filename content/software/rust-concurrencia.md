+++
title = "Rust: concurrencia y async"
summary = "Hilos, canales, estado compartido con Arc y Mutex, los traits Send y Sync, y programación asíncrona con async/await y Tokio."
branch = "Rust"
order = 6
level = "Avanzado"
tags = ["Rust", "concurrencia", "hilos", "Arc", "Mutex", "async", "Tokio"]
+++

## Concurrencia sin miedo

| Concepto | Significado |
|---|---|
| **Concurrencia** | varias tareas avanzan intercaladas |
| **Paralelismo** | varias tareas se ejecutan a la vez, en distintos núcleos |

El problema clásico es la **carrera de datos**: dos hilos acceden al mismo dato, al menos uno escribe, y no hay sincronización.

> [!IMPORTANT]
> En Rust, las reglas de ownership hacen que las carreras de datos sean **errores de compilación**. Si el programa compila, no las tiene.

## Hilos

```rust
use std::thread;

let manejador = thread::spawn(|| {
    println!("desde otro hilo");
    42
});

let resultado = manejador.join().unwrap();     // espera y obtiene el valor
```

Para usar datos del entorno, la clausura debe tomar posesión de ellos con `move`:

```rust
let datos = vec![1, 2, 3];

let manejador = thread::spawn(move || {
    println!("{datos:?}");
});

// datos ya no se puede usar aquí: se movió al hilo
manejador.join().unwrap();
```

Sin `move` no compila: el hilo podría vivir más que `datos`.

### Hilos con ámbito

`thread::scope` garantiza que los hilos terminan antes de salir del bloque, y por eso pueden tomar datos **prestados**:

```rust
let datos = vec![1, 2, 3, 4];
let (izq, der) = datos.split_at(2);

let total = thread::scope(|s| {
    let a = s.spawn(|| izq.iter().sum::<i32>());
    let b = s.spawn(|| der.iter().sum::<i32>());
    a.join().unwrap() + b.join().unwrap()
});
```

## Paso de mensajes

> No comunicarse compartiendo memoria; compartir memoria comunicándose.

Un **canal** tiene un extremo emisor y uno receptor.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

for id in 0..3 {
    let tx = tx.clone();                   // un emisor por hilo
    thread::spawn(move || {
        tx.send(format!("resultado del hilo {id}")).unwrap();
    });
}
drop(tx);                                  // cierra el emisor original

for mensaje in rx {                        // termina cuando no quedan emisores
    println!("{mensaje}");
}
```

`mpsc` significa «múltiples productores, un consumidor». Al enviar un valor, su propiedad pasa al receptor: el emisor ya no puede tocarlo.

## Estado compartido

### Mutex

Un **mutex** (exclusión mutua) deja que un solo hilo por vez acceda al dato. En Rust, el mutex **contiene** el dato: no hay forma de usarlo sin tomar el cerrojo.

```rust
use std::sync::Mutex;

let contador = Mutex::new(0);

{
    let mut valor = contador.lock().unwrap();
    *valor += 1;
}                                          // el cerrojo se libera aquí
```

El cerrojo se libera solo, cuando la guarda sale de ámbito.

### Arc

Para que varios hilos sean dueños del mismo dato se usa `Arc` (conteo de referencias **atómico**):

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let contador = Arc::new(Mutex::new(0));
let mut manejadores = vec![];

for _ in 0..10 {
    let contador = Arc::clone(&contador);
    manejadores.push(thread::spawn(move || {
        *contador.lock().unwrap() += 1;
    }));
}

for m in manejadores {
    m.join().unwrap();
}

println!("{}", *contador.lock().unwrap());     // 10
```

> [!IMPORTANT]
> `Arc<Mutex<T>>` es el patrón estándar para estado mutable compartido entre hilos: `Arc` da la propiedad compartida y `Mutex` el acceso exclusivo.

| Tipo | Uso |
|---|---|
| `Rc<T>` | propiedad compartida en **un solo** hilo |
| `Arc<T>` | propiedad compartida entre hilos |
| `Mutex<T>` | un acceso por vez |
| `RwLock<T>` | muchos lectores **o** un escritor |
| `AtomicUsize`, `AtomicBool` | enteros y booleanos sin cerrojo |
| `OnceLock<T>` | inicialización única |

Si los datos compartidos **no cambian**, alcanza con `Arc<T>`, sin mutex.

## Send y Sync

Dos traits marcadores, que el compilador implementa automáticamente, sostienen todas estas garantías:

| Trait | Significado |
|---|---|
| `Send` | el valor puede **moverse** a otro hilo |
| `Sync` | el valor puede **compartirse** por referencia entre hilos |

Casi todos los tipos son ambos. Las excepciones son deliberadas: `Rc<T>` no es `Send` (su contador no es atómico), y `RefCell<T>` no es `Sync`.

`thread::spawn` exige que la clausura sea `Send`. Por eso intentar pasar un `Rc` a otro hilo es un error de compilación.

## Lo que el compilador no evita

| Problema | Descripción | Prevención |
|---|---|---|
| **Interbloqueo** (*deadlock*) | dos hilos esperan cada uno el cerrojo del otro | tomar los cerrojos siempre en el mismo orden |
| Condición de carrera lógica | el resultado depende del orden, aunque no haya carrera de datos | hacer atómica la operación completa |
| Inanición | un hilo nunca consigue el recurso | |
| Cerrojo envenenado | un hilo entró en pánico con el mutex tomado | por eso `lock()` devuelve un `Result` |

Mantener los cerrojos el menor tiempo posible.

## Paralelismo de datos

La biblioteca **rayon** paraleliza iteradores cambiando una palabra:

```rust
use rayon::prelude::*;

let suma: u64 = numeros.par_iter().map(|n| n * n).sum();
```

Reparte el trabajo entre todos los núcleos. Es ideal para cálculo intensivo.

## Programación asíncrona

Los hilos del sistema son costosos: miles de conexiones no pueden tener un hilo cada una. La programación **asíncrona** permite que muchas tareas compartan pocos hilos, cediendo el control cuando esperan.

| | Hilos | Async |
|---|---|---|
| Conviene para | cálculo intensivo (CPU) | muchas esperas de entrada y salida (red, disco) |
| Costo por tarea | alto (una pila propia) | muy bajo |
| Cantidad práctica | miles | cientos de miles |
| Cambio de tarea | lo decide el sistema operativo | en cada `.await` |

### async y await

```rust
async fn obtener(url: &str) -> Result<String, reqwest::Error> {
    let respuesta = reqwest::get(url).await?;
    respuesta.text().await
}
```

- Una función `async` devuelve un **`Future`**: un valor que representa un cómputo todavía no terminado.
- `.await` espera el resultado **sin bloquear el hilo**: mientras tanto, corren otras tareas.

> [!WARNING]
> Los futuros son **perezosos**: llamar a una función `async` no ejecuta nada hasta que se le aplica `.await` o se la entrega a un ejecutor.

### El ejecutor

Rust no incluye un ejecutor de tareas asíncronas: lo aporta una biblioteca. **Tokio** es la más usada.

```rust
#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(                 // las dos a la vez
        obtener("https://ejemplo.com/a"),
        obtener("https://ejemplo.com/b"),
    );
    println!("{:?} {:?}", a.is_ok(), b.is_ok());
}
```

| Herramienta | Qué hace |
|---|---|
| `tokio::spawn` | lanza una tarea independiente |
| `tokio::join!` | espera varios futuros a la vez |
| `tokio::select!` | continúa con el primero que termina |
| `tokio::time::sleep` | espera sin bloquear |
| `tokio::time::timeout` | limita la duración de una operación |
| `tokio::sync::mpsc` | canales asíncronos |
| `tokio::sync::Mutex` | mutex que se puede mantener a través de un `.await` |
| `tokio::task::spawn_blocking` | lleva trabajo bloqueante a otro grupo de hilos |

```rust
let tarea = tokio::spawn(async move {
    procesar(datos).await
});

let resultado = tarea.await?;
```

### No bloquear el ejecutor

> [!WARNING]
> Dentro de código asíncrono, nunca llamar a operaciones **bloqueantes** (`std::thread::sleep`, lectura de archivos síncrona, cálculos largos): detienen al hilo y a todas las tareas que dependen de él. Usar las versiones asíncronas o `spawn_blocking`.

### Estado compartido en un servidor

En un servidor web, el estado se comparte entre las peticiones con `Arc`:

```rust
#[derive(Clone)]
struct Estado {
    temas: Arc<Vec<Tema>>,                     // solo lectura: sin mutex
    visitas: Arc<AtomicU64>,
}

async fn listar(State(estado): State<Estado>) -> Json<Vec<Tema>> {
    estado.visitas.fetch_add(1, Ordering::Relaxed);
    Json(estado.temas.as_ref().clone())
}
```

Un `std::sync::Mutex` sirve en código asíncrono mientras el cerrojo **no se mantenga a través de un `.await`**.

### Cancelación

Un futuro se cancela simplemente **descartándolo**: deja de ejecutarse en su próximo `.await`. Así funcionan `timeout` y `select!`. Hay que cuidar que una operación a medio hacer no deje datos inconsistentes.

## Elegir el modelo

| Situación | Herramienta |
|---|---|
| Cálculo pesado sobre una colección | `rayon` |
| Pocas tareas de larga duración | `std::thread` |
| Servidor con muchas conexiones | `async` con Tokio |
| Etapas que se pasan datos | canales |
| Un contador compartido | tipos atómicos |
| Una estructura compartida y mutable | `Arc<Mutex<T>>` o `Arc<RwLock<T>>` |

> [!TIP]
> Empezar por el diseño más simple: datos inmutables compartidos con `Arc`, o mensajes por canales. Recurrir a cerrojos solo cuando haga falta estado mutable compartido.
