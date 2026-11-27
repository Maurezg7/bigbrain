+++
title = "JavaScript y TypeScript"
summary = "Tipos, funciones, objetos y arreglos, asincronía con promesas y async/await, el DOM, módulos, y el tipado estático de TypeScript."
branch = "Lenguajes"
order = 4
level = "Intermedio"
tags = ["JavaScript", "TypeScript", "promesas", "async", "DOM"]
+++

## Características

**JavaScript** es el lenguaje de la web: lo ejecutan todos los navegadores. Con **Node.js**, Deno o Bun también corre en servidores. Es de tipado **dinámico** y débil, interpretado (con compilación JIT), y de un solo hilo con un **bucle de eventos**.

**TypeScript** es JavaScript con **tipos estáticos**: se compila a JavaScript y detecta errores antes de ejecutar.

```javascript
console.log("Hola, mundo");
```

## Variables y tipos

| Declaración | Reasignable | Ámbito |
|---|---|---|
| `const` | no | de bloque |
| `let` | sí | de bloque |
| `var` | sí | de función (evitar) |

Usar `const` por defecto y `let` solo si la variable cambia.

| Tipo | Ejemplo |
|---|---|
| `number` | `42`, `3.14` (todos son flotantes de 64 bits) |
| `bigint` | `123n` |
| `string` | `"hola"`, `` `plantilla` `` |
| `boolean` | `true`, `false` |
| `undefined` | variable sin valor asignado |
| `null` | ausencia intencional |
| `symbol` | identificador único |
| `object` | objetos, arreglos y funciones |

```javascript
const nombre = "Ana";
const saludo = `Hola, ${nombre}`;       // plantilla con interpolación
```

### Igualdad y valores falsos

> [!WARNING]
> Usar siempre `===` y `!==`. El operador `==` convierte tipos antes de comparar y da resultados sorprendentes: `0 == ""` y `null == undefined` son verdaderos.

Son **falsos** en un condicional: `false`, `0`, `""`, `null`, `undefined` y `NaN`. Todo lo demás es verdadero, incluidos `[]` y `{}`.

```javascript
const puerto = config.puerto ?? 3000;   // 3000 solo si es null o undefined
const ciudad = usuario?.direccion?.ciudad;   // undefined si falta algún eslabón
```

Como en otros lenguajes, `0.1 + 0.2 !== 0.3`: son números de punto flotante.

## Funciones

```javascript
function sumar(a, b) {
    return a + b;
}

const multiplicar = (a, b) => a * b;        // función flecha

function saludar(nombre = "mundo", ...resto) {   // valor por defecto y resto
    return `Hola, ${nombre}`;
}
```

Las funciones son valores: se pasan como argumentos y se devuelven. Las **clausuras** recuerdan las variables del ámbito donde se crearon:

```javascript
function contador() {
    let n = 0;
    return () => ++n;
}

const siguiente = contador();
siguiente();   // 1
siguiente();   // 2
```

## Objetos y arreglos

```javascript
const estudiante = {
    nombre: "Ana",
    edad: 20,
    saludar() {
        return `Soy ${this.nombre}`;
    },
};

estudiante.carrera = "Ingeniería";
const { nombre, edad } = estudiante;         // desestructuración
const copia = { ...estudiante, edad: 21 };   // copia con un cambio
```

```javascript
const numeros = [1, 2, 3, 4, 5, 6];

numeros.map((n) => n * 2);              // [2, 4, 6, 8, 10, 12]
numeros.filter((n) => n % 2 === 0);     // [2, 4, 6]
numeros.reduce((acc, n) => acc + n, 0); // 21
numeros.find((n) => n > 3);             // 4
numeros.some((n) => n > 5);             // true
numeros.every((n) => n > 0);            // true
numeros.includes(3);                    // true

const [primero, ...resto] = numeros;
const todos = [...numeros, 7, 8];
```

| Método | Modifica el original |
|---|---|
| `push`, `pop`, `shift`, `unshift`, `splice`, `sort`, `reverse` | sí |
| `map`, `filter`, `slice`, `concat`, `toSorted`, `toReversed` | no |

> [!WARNING]
> `sort()` sin argumentos ordena como **texto**: `[10, 9, 1].sort()` da `[1, 10, 9]`. Para números: `sort((a, b) => a - b)`.

Los objetos y arreglos se asignan **por referencia**: `const b = a` no copia. `const` impide reasignar la variable, no mutar el objeto.

`Map` y `Set` son las colecciones para diccionarios y conjuntos. **JSON** es el formato de intercambio: `JSON.stringify(obj)` y `JSON.parse(texto)`.

## Clases

```javascript
class Cuenta {
    #saldo = 0;                          // campo privado

    constructor(titular) {
        this.titular = titular;
    }

    depositar(monto) {
        if (monto <= 0) throw new Error("monto inválido");
        this.#saldo += monto;
    }

    get saldo() {
        return this.#saldo;
    }
}

class CajaDeAhorro extends Cuenta {}
```

Por debajo, la herencia en JavaScript funciona con **prototipos**: cada objeto tiene un enlace a otro del que hereda propiedades.

## Asincronía

JavaScript ejecuta el código en **un solo hilo**. Las operaciones lentas (red, archivos, temporizadores) no lo bloquean: se inician, y su resultado se atiende más tarde, cuando el **bucle de eventos** encuentra el hilo libre.

### Promesas

Una **promesa** representa un valor que estará disponible en el futuro. Tiene tres estados: pendiente, cumplida o rechazada.

```javascript
fetch("/api/temas")
    .then((respuesta) => respuesta.json())
    .then((temas) => console.log(temas))
    .catch((error) => console.error(error));
```

### async y await

Permiten escribir código asíncrono con forma secuencial:

```javascript
async function cargarTemas() {
    try {
        const respuesta = await fetch("/api/temas");
        if (!respuesta.ok) {
            throw new Error(`HTTP ${respuesta.status}`);
        }
        return await respuesta.json();
    } catch (error) {
        console.error("No se pudo cargar:", error);
        return [];
    }
}
```

Una función `async` siempre devuelve una promesa.

Para lanzar varias operaciones **en paralelo**:

```javascript
const [temas, areas] = await Promise.all([
    fetch("/api/temas").then((r) => r.json()),
    fetch("/api/areas").then((r) => r.json()),
]);
```

> [!WARNING]
> Olvidar el `await` deja una promesa sin resolver en lugar del valor. Y hacer `await` dentro de un bucle ejecuta las operaciones de a una: si son independientes, usar `Promise.all`.

### Orden de ejecución

```javascript
console.log("1");
setTimeout(() => console.log("2"), 0);
Promise.resolve().then(() => console.log("3"));
console.log("4");
// Imprime: 1, 4, 3, 2
```

Primero termina el código síncrono; luego las **microtareas** (promesas); después las **tareas** (temporizadores, eventos).

## El navegador

El **DOM** es la representación en árbol de la página, que JavaScript puede leer y modificar.

```javascript
const boton = document.querySelector("#enviar");
const lista = document.querySelector("ul");

boton.addEventListener("click", (evento) => {
    evento.preventDefault();
    const item = document.createElement("li");
    item.textContent = "Nuevo elemento";
    lista.append(item);
});
```

> [!WARNING]
> Insertar texto del usuario con `innerHTML` permite inyectar código (XSS). Usar `textContent`.

Otras interfaces del navegador: `fetch` (red), `localStorage` (datos persistentes), `URL`, `history`.

## Módulos

```javascript
// matematica.js
export const PI = 3.14159;
export function areaCirculo(r) {
    return PI * r ** 2;
}

// principal.js
import { areaCirculo } from "./matematica.js";
```

Los paquetes se instalan desde el registro **npm** con `npm`, `pnpm` o `bun`, y se listan en `package.json`.

## TypeScript

Agrega tipos que se verifican al compilar y luego se borran:

```typescript
interface Estudiante {
    nombre: string;
    edad: number;
    carrera?: string;                    // opcional
}

function promedio(notas: number[]): number {
    return notas.reduce((a, n) => a + n, 0) / notas.length;
}

type Estado = "pendiente" | "aprobado" | "rechazado";   // unión de literales

function primero<T>(lista: T[]): T | undefined {       // genérico
    return lista[0];
}
```

Las **uniones discriminadas** modelan variantes de forma segura:

```typescript
type Resultado<T> =
    | { ok: true; valor: T }
    | { ok: false; error: string };

function mostrar(r: Resultado<number>) {
    if (r.ok) {
        console.log(r.valor);            // aquí el tipo se estrecha
    } else {
        console.error(r.error);
    }
}
```

| Concepto | Significado |
|---|---|
| Inferencia | el compilador deduce los tipos sin anotarlos |
| Estrechamiento | un `if` refina el tipo dentro de su rama |
| `unknown` | valor de tipo desconocido que hay que verificar antes de usar |
| `any` | desactiva la verificación (evitar) |
| `strict` | activa todas las comprobaciones (recomendado) |

> [!IMPORTANT]
> Los tipos de TypeScript **no existen al ejecutar**. Los datos que llegan de afuera (una API, un formulario) deben validarse en tiempo de ejecución, por ejemplo con una biblioteca de esquemas como Zod.

## Ecosistema

| Área | Herramientas |
|---|---|
| Entornos de ejecución | Node.js, Deno, Bun |
| Interfaces | React, Vue, Svelte, Angular |
| Frameworks completos | Next.js, SvelteKit, Nuxt |
| Servidor | Express, Fastify, NestJS |
| Empaquetado | Vite, esbuild |
| Pruebas | Vitest, Jest, Playwright |
| Calidad | ESLint, Prettier, Biome |

> [!TIP]
> En un proyecto nuevo, empezar directamente con TypeScript en modo `strict`: el costo es bajo y evita una clase entera de errores.
