# BigBrain

Enciclopedia universitaria en modo oscuro: matemáticas, física, química, ingeniería electromecánica, software, informática e idiomas, con teoría explicada, fórmulas, figuras y ejemplos resueltos.

- **`api/`** — servidor en Rust (Axum). Carga el contenido, lo convierte a HTML, expone la API JSON y la búsqueda, y sirve la web.
- **`web/`** — interfaz en SvelteKit + TypeScript, gestionada con Bun. Se compila como sitio estático.
- **`content/`** — el conocimiento: un archivo Markdown por tema y las figuras SVG en `content/media/`.

## Requisitos

- [Rust](https://rustup.rs) estable
- [Bun](https://bun.sh)

## Ejecutar

```sh
cd web && bun install && bun run build && cd ..
cargo run --release --manifest-path api/Cargo.toml
```

Abrí <http://localhost:8787>.

Variables de entorno opcionales: `PORT` (8787), `BIGBRAIN_CONTENT` (`content`) y `BIGBRAIN_WEB` (`web/build`).

## Desarrollo

Con el servidor Rust corriendo, en otra terminal:

```sh
cd web && bun run dev
```

Vite recarga la interfaz al instante y redirige `/api` y `/media` al servidor Rust.

Verificaciones:

```sh
cargo test --manifest-path api/Cargo.toml
cargo clippy --manifest-path api/Cargo.toml --all-targets
cd web && bun run check
```

## Agregar un tema

Creá `content/<área>/<tema>.md`. El nombre del archivo es la URL del tema.

```markdown
+++
title = "Derivadas"
summary = "Una oración que resume el tema."
branch = "Cálculo"
order = 2
level = "Intermedio"
tags = ["derivada", "tangente"]
+++

## Primera sección

Texto con fórmulas en línea $f'(x)$ y en bloque:

$$\int_a^b f(x)\,dx$$

![Descripción de la figura](/media/matematicas/derivada.svg)

> [!NOTE]
> Ejemplo resuelto.
```

- `branch` debe ser una de las ramas del área, definidas en `content/areas.toml`; `order` ordena los temas dentro de la rama.
- `level` es `Básico`, `Intermedio` o `Avanzado`.
- Los bloques `[!NOTE]`, `[!TIP]`, `[!IMPORTANT]` y `[!WARNING]` se muestran como recuadros destacados.
- Las figuras van en `content/media/<área>/`, en SVG con colores pensados para fondo oscuro.

Para agregar un área nueva, sumá una entrada `[[area]]` en `content/areas.toml` y creá su carpeta.

Reiniciá el servidor para que cargue el contenido nuevo.
