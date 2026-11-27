+++
title = "React y Next.js"
summary = "Componentes, props y estado, hooks, renderizado, y Next.js con enrutado por archivos, componentes de servidor y acciones."
branch = "Frameworks de TypeScript"
order = 2
level = "Intermedio"
tags = ["TypeScript", "React", "Next.js", "hooks", "frontend"]
+++

## La idea de React

React es una biblioteca para construir interfaces. Su idea central: **la interfaz es una función del estado**. No se le dice al navegador qué cambiar paso a paso; se describe cómo debe verse para cada estado y React actualiza la página.

## Componentes y props

Un **componente** es una función que recibe **props** (sus parámetros) y devuelve **JSX**, una sintaxis parecida a HTML dentro de TypeScript.

```tsx
interface TarjetaProps {
  titulo: string;
  resumen: string;
  nivel?: string;
}

function Tarjeta({ titulo, resumen, nivel = 'Básico' }: TarjetaProps) {
  return (
    <article className="tarjeta">
      <h3>{titulo}</h3>
      <p>{resumen}</p>
      <span>{nivel}</span>
    </article>
  );
}
```

Las props son de **solo lectura** y los datos bajan de padres a hijos. Para avisar hacia arriba, el padre pasa una función.

## Estado

```tsx
import { useState } from 'react';

function Contador() {
  const [cuenta, setCuenta] = useState(0);

  return <button onClick={() => setCuenta(cuenta + 1)}>Clics: {cuenta}</button>;
}
```

Al llamar a `setCuenta`, React **vuelve a ejecutar** la función del componente, compara el resultado con el anterior (el DOM virtual) y aplica solo las diferencias.

> [!WARNING]
> El estado no se modifica en el lugar. `lista.push(x)` no provoca un render; hay que crear un valor nuevo: `setLista([...lista, x])`.

## Listas y condiciones

```tsx
function Lista({ temas }: { temas: Tema[] }) {
  if (temas.length === 0) return <p>No hay temas.</p>;

  return (
    <ul>
      {temas.map((t) => (
        <li key={t.id}>{t.titulo}</li>
      ))}
    </ul>
  );
}
```

La `key` debe ser estable y única (un id, no el índice): con ella React reconoce cada elemento entre renders.

## Hooks

| Hook | Para qué |
|---|---|
| `useState` | estado local |
| `useReducer` | estado con transiciones complejas |
| `useEffect` | sincronizar con algo externo a React |
| `useRef` | valor mutable que no provoca render, o referencia a un nodo |
| `useMemo`, `useCallback` | recordar cálculos o funciones |
| `useContext` | leer un valor compartido por el árbol |

Reglas: los hooks se llaman solo en el nivel superior del componente (no dentro de condiciones ni bucles) y solo desde componentes u otros hooks.

### useEffect

```tsx
useEffect(() => {
  const id = setInterval(() => setSegundos((s) => s + 1), 1000);
  return () => clearInterval(id);        // limpieza
}, []);                                  // dependencias
```

Sirve para **sincronizar con el exterior**: temporizadores, suscripciones, el DOM. No hace falta para calcular un dato a partir de otro; eso se hace directamente en el render:

```tsx
const visibles = temas.filter((t) => t.titulo.includes(texto));   // sin efecto ni estado extra
```

### Hooks propios

```tsx
function useBusqueda(texto: string) {
  const [resultados, setResultados] = useState<Tema[]>([]);

  useEffect(() => {
    const control = new AbortController();
    fetch(`/api/buscar?q=${encodeURIComponent(texto)}`, { signal: control.signal })
      .then((r) => r.json())
      .then(setResultados)
      .catch(() => {});
    return () => control.abort();        // cancela el pedido anterior
  }, [texto]);

  return resultados;
}
```

En proyectos reales, para datos del servidor conviene **TanStack Query**, que resuelve caché, reintentos y estados de carga.

## Estado global

| Opción | Cuándo |
|---|---|
| Levantar el estado al padre común | pocos componentes cercanos |
| Context | datos que cambian poco: tema visual, usuario |
| Zustand, Redux Toolkit | estado de cliente grande y compartido |
| TanStack Query | datos que viven en el servidor |

## Next.js

React solo se ocupa de la interfaz. **Next.js** es el framework que agrega enrutado, renderizado en servidor, carga de datos y optimización.

### Enrutado por archivos

```text
app/
  layout.tsx            marco común
  page.tsx              /
  temas/
    page.tsx            /temas
    [slug]/page.tsx     /temas/ley-de-ohm
```

### Componentes de servidor

En el directorio `app`, los componentes se ejecutan **en el servidor** por defecto: pueden consultar la base directamente y no envían su código al navegador.

```tsx
// app/temas/[slug]/page.tsx
export default async function Pagina({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  const tema = await db.tema.findUnique({ where: { slug } });
  if (!tema) notFound();

  return <article><h1>{tema.titulo}</h1></article>;
}
```

Un componente que necesita estado o eventos lleva la directiva `'use client'` en la primera línea. Conviene que sean pequeños y estén en las hojas del árbol.

### Formas de renderizar

| Sigla | Cuándo se genera el HTML | Uso |
|---|---|---|
| SSG | al compilar | contenido que casi no cambia |
| ISR | al compilar, y se regenera cada cierto tiempo | catálogos, artículos |
| SSR | en cada pedido | datos personales o muy cambiantes |
| CSR | en el navegador | paneles muy interactivos |

### Acciones de servidor

```tsx
async function crearTema(datos: FormData) {
  'use server';
  await db.tema.create({ data: { titulo: String(datos.get('titulo')) } });
  revalidatePath('/temas');
}

<form action={crearTema}>...</form>
```

> [!WARNING]
> Una acción de servidor es una ruta pública. Hay que **validar los datos y comprobar permisos** adentro, igual que en cualquier API.

## Buenas prácticas

- Componentes chicos y con una sola tarea.
- Guardar el **mínimo** estado; todo lo que se pueda calcular, se calcula.
- No usar `useEffect` para derivar datos.
- Claves estables en las listas.
- No optimizar con `useMemo` sin haber medido.
- HTML semántico y accesible: `button` para acciones, `label` en los campos.
- Probar con **Vitest** y **React Testing Library**, consultando la página como lo haría un usuario.
