+++
title = "Pruebas automatizadas"
summary = "Pirámide de pruebas, unitarias, de integración y de punta a punta, dobles de prueba, TDD, cobertura e integración continua."
branch = "Buenas prácticas"
order = 2
level = "Intermedio"
tags = ["pruebas", "testing", "TDD", "integración continua", "calidad"]
+++

## Para qué sirven

Una prueba automatizada es código que comprueba que otro código hace lo esperado. Su valor principal es permitir **cambiar el programa sin miedo**: si algo se rompe, se sabe en segundos y no cuando lo descubre un usuario.

## La pirámide

![Pirámide de pruebas: muchas unitarias, menos de integración y pocas de punta a punta](/media/informatica/piramide-de-pruebas.svg)

| Tipo | Qué prueba | Velocidad | Cantidad |
|---|---|---|---|
| **Unitaria** | una función o clase aislada | milisegundos | muchas |
| **Integración** | varias partes juntas: código con base de datos, API | segundos | algunas |
| **De punta a punta** (E2E) | el sistema entero como un usuario | lenta | pocas |

Cuanto más arriba, más realismo y también más lentitud y fragilidad.

## Anatomía de una prueba

Tres pasos: **preparar, actuar, verificar**.

```rust
#[test]
fn promedia_las_notas() {
    let notas = [8.0, 6.0, 10.0];        // preparar

    let resultado = promedio(&notas);    // actuar

    assert_eq!(resultado, Some(8.0));    // verificar
}

#[test]
fn sin_notas_no_hay_promedio() {
    assert_eq!(promedio(&[]), None);
}
```

```ts
import { describe, expect, it } from 'vitest';

describe('promedio', () => {
  it('promedia las notas', () => {
    expect(promedio([8, 6, 10])).toBe(8);
  });

  it('rechaza una lista vacía', () => {
    expect(() => promedio([])).toThrow();
  });
});
```

## Qué casos probar

- El caso normal.
- Los **bordes**: vacío, cero, uno, el máximo, negativos.
- Las entradas inválidas y los errores.
- Cada error ya corregido: una prueba que lo reproduzca, para que no vuelva.

## Propiedades de una buena prueba

| Propiedad | Significa |
|---|---|
| Rápida | se ejecuta todo el tiempo |
| Independiente | no depende de otras ni del orden |
| Repetible | da siempre el mismo resultado |
| Clara | al fallar, dice qué se rompió |
| De comportamiento | verifica qué hace el código, no cómo |

> [!WARNING]
> Una prueba **intermitente** (a veces pasa, a veces no) destruye la confianza en todas las demás. Las causas típicas son el tiempo, el orden, la red y el estado compartido. Se arregla o se elimina.

## Dobles de prueba

Reemplazan a una dependencia real.

| Doble | Qué hace |
|---|---|
| *Stub* | devuelve respuestas fijas |
| *Mock* | además verifica cómo fue llamado |
| *Fake* | implementación simple que funciona (un repositorio en memoria) |
| *Spy* | envuelve al real y registra las llamadas |

Se reemplaza lo **lento o impredecible**: red, reloj, azar, servicios de terceros. El abuso de dobles da pruebas que pasan aunque el sistema esté roto.

## Pruebas de integración

Verifican las uniones, que es donde aparecen muchos errores: la consulta SQL, la serialización, la configuración. Conviene usar la **misma base de datos** que en producción, en un contenedor (Testcontainers), y dejar los datos limpios entre pruebas.

## Pruebas de punta a punta

Manejan un navegador real con herramientas como **Playwright** o Cypress:

```ts
import { expect, test } from '@playwright/test';

test('buscar un tema', async ({ page }) => {
  await page.goto('/');
  await page.getByPlaceholder('Buscar').fill('ohm');
  await page.getByRole('link', { name: 'Ley de Ohm' }).click();
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Ley de Ohm');
});
```

Se reservan para los recorridos críticos: registrarse, pagar, buscar.

## TDD

El desarrollo guiado por pruebas repite un ciclo corto:

1. **Rojo**: escribir una prueba que falla.
2. **Verde**: escribir el código mínimo para que pase.
3. **Refactorizar**: mejorar el código con las pruebas en verde.

Obliga a pensar primero **cómo se va a usar** el código y deja como resultado un diseño fácil de probar.

## Cobertura

Mide qué porcentaje del código se ejecuta durante las pruebas.

> [!NOTE]
> La cobertura dice qué código **no** está probado; no dice que el resto esté bien probado. Un 100 % sin buenas verificaciones no vale nada. Sirve como alarma, no como objetivo.

## Integración continua

Un servidor ejecuta las verificaciones en **cada cambio**, antes de fusionarlo:

```yaml
name: ci
on: [push, pull_request]
jobs:
  verificar:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test
```

Si algo falla, el cambio no entra a la rama principal.

## Otras técnicas

| Técnica | Idea |
|---|---|
| Pruebas basadas en propiedades | generar cientos de entradas al azar y comprobar una propiedad general |
| Pruebas de instantánea | comparar la salida con una versión guardada |
| Pruebas de carga | medir el comportamiento con muchos usuarios (k6, JMeter) |
| Pruebas de mutación | alterar el código y ver si alguna prueba lo detecta |

## Buenas prácticas

- Probar el comportamiento público, no los detalles internos.
- Nombres que cuenten qué se espera.
- Mantener las pruebas tan cuidadas como el código.
- Si algo es difícil de probar, suele ser señal de un diseño acoplado.
- Toda corrección de un error empieza por una prueba que lo reproduce.
