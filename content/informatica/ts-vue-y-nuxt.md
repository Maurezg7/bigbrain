+++
title = "Vue y Nuxt"
summary = "Componentes de un solo archivo, Composition API, reactividad con ref y computed, Pinia, Vue Router y Nuxt."
branch = "Frameworks de TypeScript"
order = 4
level = "Intermedio"
tags = ["TypeScript", "Vue", "Nuxt", "Pinia", "frontend"]
+++

## Vue

Vue es un framework **progresivo**: puede usarse para una parte de una página o para una aplicación completa. Es conocido por su curva de aprendizaje suave y su documentación.

## Componentes de un solo archivo

Un archivo `.vue` reúne lógica, plantilla y estilos.

```vue
<script setup lang="ts">
import { computed, ref } from 'vue';

const props = defineProps<{ inicial?: number }>();
const emit = defineEmits<{ cambio: [valor: number] }>();

const cuenta = ref(props.inicial ?? 0);
const doble = computed(() => cuenta.value * 2);

function sumar() {
  cuenta.value++;
  emit('cambio', cuenta.value);
}
</script>

<template>
  <button @click="sumar">Clics: {{ cuenta }}</button>
  <p>El doble es {{ doble }}</p>
</template>

<style scoped>
button { padding: 0.5rem 1rem; }
</style>
```

`<script setup>` es la forma actual de la **Composition API**. `scoped` limita los estilos a este componente.

## Reactividad

| Primitiva | Uso |
|---|---|
| `ref(valor)` | valor reactivo; se lee y escribe con `.value` (en la plantilla no hace falta) |
| `reactive(objeto)` | objeto reactivo completo |
| `computed(() => ...)` | valor derivado, con caché |
| `watch(fuente, fn)` | reaccionar a un cambio |
| `watchEffect(fn)` | igual, detectando solo las dependencias |

Vue registra qué componentes leen cada valor y actualiza **solo esos**, sin volver a ejecutar todo.

> [!WARNING]
> Desestructurar un objeto `reactive` (`const { titulo } = estado`) rompe la reactividad. Usá `toRefs(estado)` o trabajá con `ref`.

## Plantillas

| Directiva | Función |
|---|---|
| `{{ valor }}` | mostrar |
| `:prop="valor"` | pasar un dato (`v-bind`) |
| `@evento="fn"` | escuchar (`v-on`) |
| `v-if`, `v-else` | condición |
| `v-for="t in temas" :key="t.id"` | lista |
| `v-model="campo"` | enlace en ambos sentidos |
| `v-show` | ocultar con CSS, sin quitar del DOM |

```vue
<template>
  <input v-model="texto" placeholder="Buscar" />
  <ul v-if="filtrados.length">
    <li v-for="tema in filtrados" :key="tema.id">{{ tema.titulo }}</li>
  </ul>
  <p v-else>Sin resultados.</p>
</template>
```

## Composables

Funciones que encapsulan lógica reactiva reutilizable; equivalen a los hooks propios de React.

```ts
import { ref, watch } from 'vue';

export function useBusqueda(texto: Ref<string>) {
  const resultados = ref<Tema[]>([]);
  const cargando = ref(false);

  watch(texto, async (valor) => {
    cargando.value = true;
    const res = await fetch(`/api/buscar?q=${encodeURIComponent(valor)}`);
    resultados.value = await res.json();
    cargando.value = false;
  });

  return { resultados, cargando };
}
```

## Pinia

Es el almacén de estado oficial.

```ts
import { defineStore } from 'pinia';

export const useProgreso = defineStore('progreso', () => {
  const leidos = ref<string[]>([]);
  const total = computed(() => leidos.value.length);

  function marcar(slug: string) {
    if (!leidos.value.includes(slug)) leidos.value.push(slug);
  }

  return { leidos, total, marcar };
});
```

## Vue Router

```ts
const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Inicio },
    { path: '/temas/:slug', component: () => import('./Tema.vue') },   // carga diferida
  ],
});
```

## Nuxt

Nuxt es a Vue lo que Next.js es a React: suma enrutado por archivos, renderizado en servidor y carga de datos.

```text
pages/
  index.vue             /
  temas/[slug].vue      /temas/ley-de-ohm
server/
  api/temas.get.ts      GET /api/temas
components/             se importan solos
composables/            se importan solos
```

```vue
<script setup lang="ts">
const route = useRoute();
const { data: tema, error } = await useFetch<Tema>(`/api/temas/${route.params.slug}`);
</script>
```

`useFetch` hace el pedido en el servidor durante el primer render y **no lo repite** en el navegador.

Rutas de API en el mismo proyecto:

```ts
// server/api/temas.get.ts
export default defineEventHandler(async () => {
  return await db.select().from(temas);
});
```

## Vue comparado con React

| | Vue | React |
|---|---|---|
| Plantilla | HTML con directivas | JSX |
| Reactividad | automática, por dependencias | se vuelve a ejecutar el componente |
| Estilos | incluidos y con alcance | a elección |
| Framework completo | Nuxt | Next.js |
| Ecosistema | oficial y unificado | amplio y variado |

## Buenas prácticas

- `<script setup>` y Composition API en código nuevo.
- `computed` para derivar; `watch` solo para efectos.
- `:key` única en cada `v-for`, y no combinar `v-if` con `v-for` en el mismo elemento.
- Props hacia abajo y eventos hacia arriba; estado compartido en Pinia.
- Tipar props y eventos con `defineProps` y `defineEmits`.
