+++
title = "Código limpio y principios SOLID"
summary = "Nombres, funciones pequeñas, comentarios, manejo de errores, los cinco principios SOLID y cuándo no aplicarlos."
branch = "Buenas prácticas"
order = 1
level = "Intermedio"
tags = ["código limpio", "SOLID", "refactorización", "diseño", "buenas prácticas"]
+++

## Para quién se escribe el código

El código se **lee** muchas más veces de las que se escribe. Código limpio es el que otra persona (o uno mismo en seis meses) entiende y modifica sin miedo. No es cuestión de estética: es lo que mantiene bajo el costo de cada cambio.

## Nombres

| En lugar de | Mejor |
|---|---|
| `d` | `diasHastaElExamen` |
| `lista2` | `temasPendientes` |
| `procesar()` | `calcularPromedio()` |
| `flag` | `estaAprobado` |
| `datos`, `info`, `manager` | el concepto concreto del dominio |

Un buen nombre dice **qué es** o **qué hace**, sin obligar a leer la implementación. Las funciones llevan verbos; los booleanos, una pregunta (`es`, `tiene`, `puede`).

## Funciones

- **Cortas** y con **una sola tarea**.
- **Pocos parámetros**: si son más de tres, probablemente falte un objeto.
- **Sin sorpresas**: una función llamada `obtenerUsuario` no debería además enviar un correo.
- Un solo nivel de abstracción por función.

Antes:

```ts
function procesar(alumnos: Alumno[]) {
  const r = [];
  for (const a of alumnos) {
    let s = 0;
    for (const n of a.notas) s += n;
    const p = s / a.notas.length;
    if (p >= 4 && a.asistencia >= 0.75) r.push({ nombre: a.nombre, p });
  }
  return r;
}
```

Después:

```ts
const NOTA_MINIMA = 4;
const ASISTENCIA_MINIMA = 0.75;

const promedio = (notas: number[]) => notas.reduce((a, b) => a + b, 0) / notas.length;

const regulariza = (a: Alumno) =>
  promedio(a.notas) >= NOTA_MINIMA && a.asistencia >= ASISTENCIA_MINIMA;

function alumnosRegulares(alumnos: Alumno[]) {
  return alumnos
    .filter(regulariza)
    .map((a) => ({ nombre: a.nombre, promedio: promedio(a.notas) }));
}
```

Los números sueltos pasaron a ser constantes con nombre y cada regla quedó en un lugar.

## Cláusulas de guarda

Tratar primero los casos especiales y salir evita el anidamiento.

```ts
function descuento(usuario: Usuario | null): number {
  if (!usuario) return 0;
  if (!usuario.activo) return 0;
  if (usuario.esEstudiante) return 0.5;
  return 0.1;
}
```

## Comentarios

Un comentario explica **por qué**, no **qué**. El qué debe decirlo el código.

```ts
// Mal: repite el código
i++; // incrementa i

// Bien: explica una decisión que el código no puede expresar
// La API del proveedor devuelve la hora en UTC-3 fija, sin horario de verano.
const hora = ajustarZona(respuesta.hora, -3);
```

Un comentario desactualizado es peor que no tener ninguno.

## Errores

- No ocultarlos: un `catch` vacío esconde el problema.
- Fallar **pronto y con un mensaje claro**.
- Validar en la frontera del sistema; adentro, confiar en datos ya validados.
- Usar el sistema de tipos: `Option` y `Result` en Rust, `Optional` en Java, uniones en TypeScript.

## Tres reglas cortas

| Sigla | Significa | Advertencia |
|---|---|---|
| **DRY** | no te repitas: cada conocimiento, en un solo lugar | dos códigos parecidos no siempre son el mismo concepto |
| **KISS** | mantenelo simple | simple no es lo mismo que fácil |
| **YAGNI** | no lo vas a necesitar | no programar para requisitos imaginarios |

## SOLID

Cinco principios de diseño orientado a objetos.

| Letra | Principio | En una frase |
|---|---|---|
| **S** | Responsabilidad única | una clase debe tener un solo motivo para cambiar |
| **O** | Abierto/cerrado | se extiende sin modificar lo existente |
| **L** | Sustitución de Liskov | un subtipo debe poder usarse donde se espera el tipo base |
| **I** | Segregación de interfaces | muchas interfaces chicas, no una enorme |
| **D** | Inversión de dependencias | depender de abstracciones, no de detalles |

### Responsabilidad única

Una clase `Informe` que calcula datos, los formatea en PDF y los envía por correo cambia por tres razones distintas. Se separa en tres.

### Abierto/cerrado

```ts
interface Forma { area(): number; }

class Circulo implements Forma {
  constructor(private r: number) {}
  area() { return Math.PI * this.r ** 2; }
}

class Rectangulo implements Forma {
  constructor(private b: number, private h: number) {}
  area() { return this.b * this.h; }
}

const areaTotal = (formas: Forma[]) => formas.reduce((s, f) => s + f.area(), 0);
```

Agregar un triángulo no obliga a tocar `areaTotal`.

### Sustitución de Liskov

Si `Cuadrado` hereda de `Rectangulo` pero cambiar el ancho también cambia el alto, el código que esperaba un rectángulo se rompe. La herencia expresa **comportamiento compatible**, no parecido.

### Segregación de interfaces

Una impresora simple no debería implementar `escanear()` y `enviarFax()` lanzando errores. Mejor interfaces separadas: `Impresora`, `Escaner`.

### Inversión de dependencias

```ts
interface RepositorioDeTemas { buscar(id: number): Promise<Tema | null>; }

class ServicioDeTemas {
  constructor(private repo: RepositorioDeTemas) {}   // no sabe si es Postgres o memoria
}
```

El servicio se prueba con un repositorio en memoria y en producción usa la base real.

> [!WARNING]
> Los principios son herramientas, no leyes. Aplicarlos a ciegas produce capas e interfaces que nadie necesita. Primero el código simple que funciona; la abstracción llega cuando aparece el segundo o tercer caso.

## Refactorizar

Es mejorar la estructura **sin cambiar el comportamiento**, en pasos chicos y con pruebas que lo respalden.

| Señal (*code smell*) | Remedio |
|---|---|
| función larga | extraer funciones |
| código duplicado | unificar |
| lista larga de parámetros | agrupar en un objeto |
| clase que hace de todo | dividir |
| `if` o `switch` repetido por tipo | polimorfismo o tabla |
| números mágicos | constantes con nombre |

## Buenas prácticas

- Formateador y *linter* automáticos (`rustfmt` y Clippy, Spotless, ESLint y Prettier): el estilo no se discute, se configura.
- Revisión de código entre pares, con cambios chicos.
- Dejar el código un poco mejor de lo que se lo encontró.
- Preferir composición a herencia y datos inmutables.
