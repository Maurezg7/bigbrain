+++
title = "Rust en el frontend y el escritorio: Leptos, Yew, Dioxus y Tauri"
summary = "WebAssembly, señales reactivas, componentes, funciones de servidor y aplicaciones de escritorio con Tauri."
branch = "Frameworks de Rust"
order = 5
level = "Avanzado"
tags = ["Rust", "WebAssembly", "Leptos", "Tauri", "frontend"]
+++

## Rust fuera del servidor

Rust compila a **WebAssembly** (Wasm), un formato binario que los navegadores ejecutan a velocidad cercana a la nativa. Eso permite escribir la interfaz en el mismo lenguaje que el servidor y compartir tipos entre ambos.

| Framework | Estilo | Destinos |
|---|---|---|
| **Leptos** | señales de grano fino, como SolidJS | web, con renderizado en servidor |
| **Yew** | componentes y DOM virtual, como React | web |
| **Dioxus** | parecido a React, con `rsx!` | web, escritorio, móvil |
| **Tauri** | envoltura nativa para una interfaz web | escritorio y móvil |

## Leptos

Un **componente** es una función que se ejecuta **una sola vez** y arma la vista. Lo que cambia después son las **señales**: valores reactivos que avisan a quien los leyó.

```rust
use leptos::prelude::*;

#[component]
fn Contador(inicial: i32) -> impl IntoView {
    let (cuenta, set_cuenta) = signal(inicial);
    let doble = move || cuenta.get() * 2;

    view! {
        <button on:click=move |_| set_cuenta.update(|n| *n += 1)>
            "Clics: " {cuenta}
        </button>
        <p>"El doble es " {doble}</p>
    }
}
```

Al hacer clic no se vuelve a ejecutar el componente ni se compara un árbol virtual: solo se actualizan los dos nodos de texto que dependen de `cuenta`.

| Primitiva | Para qué |
|---|---|
| `signal` | estado que cambia |
| Cierre o `Memo` | valor derivado de otras señales |
| `Effect` | efecto secundario cuando algo cambia |
| `Resource` | dato asíncrono (una consulta) integrado con `<Suspense>` |

### Funciones de servidor

Con renderizado en servidor, una función marcada con `#[server]` se ejecuta en el servidor, y el cliente la llama como si fuera local. Leptos genera el pedido HTTP.

```rust
#[server]
pub async fn buscar_temas(texto: String) -> Result<Vec<Tema>, ServerFnError> {
    let pool = usar_pool()?;            // solo existe en el servidor
    Ok(consultar(&pool, &texto).await?)
}
```

El tipo `Tema` es el mismo en ambos lados: si cambia un campo, el compilador marca todos los usos.

Leptos se integra con Axum y Actix Web. `cargo leptos watch` compila servidor y cliente y recarga el navegador.

## Yew

Usa el modelo de React: componentes con *hooks* y un DOM virtual que se compara en cada render.

```rust
use yew::prelude::*;

#[function_component]
fn Contador() -> Html {
    let cuenta = use_state(|| 0);
    let sumar = {
        let cuenta = cuenta.clone();
        Callback::from(move |_| cuenta.set(*cuenta + 1))
    };
    html! { <button onclick={sumar}>{ format!("Clics: {}", *cuenta) }</button> }
}
```

## Tauri

Tauri crea aplicaciones de escritorio cuya interfaz es una página web (hecha con cualquier framework: Svelte, React, Leptos) y cuyo núcleo es Rust. A diferencia de Electron, **no incluye un navegador**: usa el WebView del sistema operativo.

| | Tauri | Electron |
|---|---|---|
| Motor de interfaz | WebView del sistema | Chromium incluido |
| Lógica nativa | Rust | Node.js |
| Tamaño del instalador | pocos megabytes | del orden de 100 MB |
| Memoria | baja | alta |
| Aspecto idéntico en todos los sistemas | no garantizado | sí |

La interfaz llama a funciones de Rust llamadas **comandos**:

```rust
#[tauri::command]
fn leer_apunte(ruta: String) -> Result<String, String> {
    std::fs::read_to_string(ruta).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![leer_apunte])
        .run(tauri::generate_context!())
        .expect("no se pudo iniciar la aplicación");
}
```

```ts
import { invoke } from '@tauri-apps/api/core';

const texto = await invoke<string>('leer_apunte', { ruta: 'apuntes/ohm.md' });
```

> [!WARNING]
> Tauri funciona con **permisos**: la interfaz solo puede usar los comandos y recursos que la configuración habilita. Hay que conceder lo mínimo necesario, porque una página comprometida tendría acceso a lo mismo.

## Cuándo conviene

| Situación | Recomendación |
|---|---|
| Sitio con mucho contenido y poco equipo | un framework de TypeScript suele ser más productivo |
| Equipo que ya trabaja en Rust y quiere compartir tipos | Leptos o Dioxus |
| Cálculo pesado en el navegador | un módulo Wasm en Rust dentro de una interfaz común |
| Aplicación de escritorio liviana | Tauri |

## Buenas prácticas

- Compartir los tipos de datos entre cliente y servidor en un crate común.
- Vigilar el tamaño del binario Wasm: compilar con `opt-level = "z"`, `lto = true` y pasar `wasm-opt`.
- En Leptos, derivar valores con cierres o `Memo` en lugar de sincronizar señales con efectos.
- En Tauri, validar en Rust todo lo que llega desde la interfaz.
