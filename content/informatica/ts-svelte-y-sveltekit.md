+++
title = "Svelte y SvelteKit"
summary = "Un framework que compila: runas, componentes, y SvelteKit con rutas, funciones load, acciones de formulario y adaptadores."
branch = "Frameworks de TypeScript"
order = 5
level = "Intermedio"
tags = ["TypeScript", "Svelte", "SvelteKit", "runas", "frontend"]
+++

## Un compilador, no una biblioteca

Svelte convierte los componentes, **al compilar**, en JavaScript que actualiza el DOM de forma directa. No usa DOM virtual ni envía un runtime grande al navegador, así que las aplicaciones resultan pequeñas y rápidas. La interfaz de BigBrain está hecha con SvelteKit.

## Componentes

Un archivo `.svelte` tiene lógica, marcado y estilos. Los estilos afectan solo a ese componente.

```svelte
<script lang="ts">
  let { inicial = 0 }: { inicial?: number } = $props();

  let cuenta = $state(inicial);
  let doble = $derived(cuenta * 2);
</script>

<button onclick={() => cuenta++}>Clics: {cuenta}</button>
<p>El doble es {doble}</p>

<style>
  button { padding: 0.5rem 1rem; }
</style>
```

## Runas

Desde Svelte 5 la reactividad se declara con **runas**, funciones especiales que empiezan con `$`.

| Runa | Uso |
|---|---|
| `$state(valor)` | estado reactivo; se lee y se asigna como una variable común |
| `$derived(expresión)` | valor calculado a partir de otros |
| `$effect(() => ...)` | efecto secundario cuando cambian sus dependencias |
| `$props()` | propiedades que recibe el componente |
| `$bindable()` | propiedad que el padre puede enlazar con `bind:` |

```svelte
<script lang="ts">
  let temas = $state<Tema[]>([]);
  let texto = $state('');

  let filtrados = $derived(
    temas.filter((t) => t.titulo.toLowerCase().includes(texto.toLowerCase()))
  );

  $effect(() => {
    document.title = `${filtrados.length} temas`;
  });
</script>
```

Los objetos y arreglos en `$state` son reactivos en profundidad: `temas.push(nuevo)` actualiza la página.

> [!TIP]
> Si un valor se puede calcular a partir de otro, usá `$derived`. `$effect` es para lo que está fuera de Svelte (el DOM, temporizadores, almacenamiento), no para copiar estado.

## Marcado

```svelte
<input bind:value={texto} placeholder="Buscar" />

{#if filtrados.length > 0}
  <ul>
    {#each filtrados as tema (tema.id)}
      <li>{tema.titulo}</li>
    {/each}
  </ul>
{:else}
  <p>Sin resultados.</p>
{/if}

{#await promesa}
  <p>Cargando...</p>
{:then datos}
  <p>{datos.length} resultados</p>
{:catch error}
  <p>{error.message}</p>
{/await}
```

El `(tema.id)` del `each` es la clave. `bind:value` enlaza en ambos sentidos.

Para pasar contenido a un componente se usan **snippets**: bloques de marcado reutilizables que el hijo dibuja con `{@render children()}`.

## Estado compartido

Las runas también funcionan en archivos `.svelte.ts`, lo que permite compartir estado entre componentes:

```ts
// progreso.svelte.ts
export const progreso = $state({ leidos: [] as string[] });

export function marcar(slug: string) {
  if (!progreso.leidos.includes(slug)) progreso.leidos.push(slug);
}
```

## SvelteKit

Es el framework de aplicación: enrutado, renderizado en servidor, carga de datos y compilación con Vite.

### Rutas por carpetas

```text
src/routes/
  +layout.svelte            marco común
  +page.svelte              /
  temas/[slug]/
    +page.svelte            /temas/ley-de-ohm
    +page.ts                carga datos (servidor y navegador)
    +page.server.ts         carga datos y acciones (solo servidor)
  api/buscar/+server.ts     ruta de API
```

### Funciones load

```ts
// src/routes/temas/[slug]/+page.server.ts
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
  const tema = await db.buscarTema(params.slug);
  if (!tema) error(404, 'Tema no encontrado');
  return { tema };
};
```

```svelte
<!-- +page.svelte -->
<script lang="ts">
  let { data } = $props();
</script>

<h1>{data.tema.titulo}</h1>
```

Los tipos de `data` se generan solos a partir de lo que devuelve `load`.

| Archivo | Dónde corre | Para qué |
|---|---|---|
| `+page.ts` | servidor y navegador | pedir datos a una API pública |
| `+page.server.ts` | solo servidor | base de datos, secretos |

### Acciones de formulario

```ts
export const actions = {
  default: async ({ request }) => {
    const datos = await request.formData();
    const titulo = String(datos.get('titulo') ?? '').trim();
    if (!titulo) return fail(400, { error: 'Falta el título' });
    await db.crearTema(titulo);
    return { ok: true };
  },
};
```

```svelte
<form method="POST" use:enhance>
  <input name="titulo" />
  <button>Guardar</button>
</form>
```

El formulario funciona **sin JavaScript**; con `use:enhance` se envía sin recargar la página. A eso se lo llama mejora progresiva.

### Adaptadores

| Adaptador | Resultado |
|---|---|
| `adapter-node` | servidor Node |
| `adapter-static` | sitio estático, servible desde cualquier servidor |
| `adapter-vercel`, `adapter-cloudflare` | despliegue en esas plataformas |

BigBrain usa `adapter-static`: los archivos generados los sirve el servidor en Rust.

## Buenas prácticas

- `$derived` antes que `$effect`.
- Cargar los datos en `load`, no dentro de los componentes.
- Todo lo sensible en archivos `.server.ts`; nunca importarlos desde código de cliente.
- Formularios con acciones y `use:enhance`.
- Validar en el servidor lo que llega del formulario.
- Ejecutar `svelte-check` en integración continua.
