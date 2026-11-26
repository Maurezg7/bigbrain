+++
title = "TypeScript esencial"
summary = "Tipos, interfaces, uniones, genéricos, estrechamiento, tipos utilitarios y la configuración estricta que conviene usar."
branch = "Frameworks de TypeScript"
order = 1
level = "Básico"
tags = ["TypeScript", "tipos", "genéricos", "JavaScript", "tsconfig"]
+++

## Qué agrega TypeScript

TypeScript es JavaScript con **tipos estáticos**. El compilador revisa el código antes de ejecutarlo y después borra los tipos: lo que corre es JavaScript común. Todos los frameworks de esta rama se usan con TypeScript.

```ts
function promedio(notas: number[]): number {
  return notas.reduce((a, b) => a + b, 0) / notas.length;
}

promedio([8, 6, 10]);   // 8
promedio('8, 6, 10');   // error de compilación
```

> [!NOTE]
> Los tipos **no existen en tiempo de ejecución**. TypeScript no valida los datos que llegan de afuera (una API, un formulario): eso hay que comprobarlo con código.

## Tipos básicos

| Tipo | Ejemplo |
|---|---|
| `string`, `number`, `boolean` | `let nombre: string = 'Ana'` |
| Arreglo | `number[]` |
| Tupla | `[string, number]` |
| Literal | `'claro' \| 'oscuro'` |
| `unknown` | valor desconocido; obliga a comprobar antes de usar |
| `any` | desactiva el control; evitarlo |
| `void`, `never` | sin valor; nunca ocurre |

Casi siempre el compilador **infiere** el tipo: `const n = 5` ya es un número.

## Objetos: `interface` y `type`

```ts
interface Tema {
  id: number;
  titulo: string;
  nivel?: 'Básico' | 'Intermedio' | 'Avanzado';   // opcional
  readonly area: string;
}

type Id = number | string;
```

Las dos formas describen objetos. `interface` puede extenderse y es habitual para objetos; `type` hace falta para uniones y tipos calculados.

## Uniones y estrechamiento

Una **unión** dice que un valor es de uno de varios tipos. Antes de usarlo hay que **estrechar** el tipo con una comprobación.

```ts
type Resultado =
  | { estado: 'cargando' }
  | { estado: 'listo'; temas: Tema[] }
  | { estado: 'error'; mensaje: string };

function mostrar(r: Resultado): string {
  switch (r.estado) {
    case 'cargando': return 'Cargando...';
    case 'listo':    return `${r.temas.length} temas`;   // acá existe r.temas
    case 'error':    return r.mensaje;
  }
}
```

Esta **unión discriminada** modela estados sin combinaciones imposibles, y el compilador avisa si falta un caso.

## `null` y `undefined`

Con el modo estricto, `string` no admite `null`. Hay que declararlo y tratarlo:

```ts
function largo(texto: string | null): number {
  return texto?.length ?? 0;
}
```

`?.` corta la cadena si el valor es nulo y `??` da un valor por defecto.

## Genéricos

Permiten escribir código que funciona con muchos tipos sin perder información.

```ts
function primero<T>(lista: T[]): T | undefined {
  return lista[0];
}

const n = primero([1, 2, 3]);       // number | undefined
const s = primero(['a', 'b']);      // string | undefined

interface Pagina<T> { items: T[]; total: number; }

async function obtener<T>(url: string): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`HTTP ${res.status}`);
  return res.json() as Promise<T>;
}
```

## Tipos utilitarios

| Utilitario | Resultado |
|---|---|
| `Partial<T>` | todas las propiedades opcionales |
| `Required<T>` | todas obligatorias |
| `Pick<T, K>` | solo algunas propiedades |
| `Omit<T, K>` | todas menos algunas |
| `Record<K, V>` | objeto con claves `K` y valores `V` |
| `Readonly<T>` | solo lectura |
| `ReturnType<F>` | el tipo que devuelve una función |

```ts
type NuevoTema = Omit<Tema, 'id'>;
type CambiosDeTema = Partial<NuevoTema>;
```

## Validar datos externos

Con una biblioteca como **Zod** se define un esquema una vez y se obtiene la validación y el tipo:

```ts
import { z } from 'zod';

const TemaSchema = z.object({
  id: z.number(),
  titulo: z.string().min(1),
});
type Tema = z.infer<typeof TemaSchema>;

const tema = TemaSchema.parse(await res.json());   // lanza un error si no cumple
```

## Configuración

```json
{
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler"
  }
}
```

`strict` activa todos los controles importantes. Empezar un proyecto sin él es renunciar a la mitad del beneficio.

## Buenas prácticas

- `strict: true` desde el primer día.
- `unknown` en lugar de `any`; si hay un `any`, que sea temporal y esté justificado.
- Dejar que el compilador infiera; anotar las fronteras (parámetros y valores de retorno públicos).
- Uniones discriminadas para modelar estados.
- Validar con un esquema todo dato que venga de afuera.
- Evitar las aserciones `as` y el operador `!`: le dicen al compilador que no revise.
