+++
title = "Rust: ownership y préstamos"
summary = "El sistema de propiedad con el que Rust garantiza seguridad de memoria sin recolector de basura."
branch = "Rust"
order = 1
level = "Avanzado"
tags = ["Rust", "ownership", "borrowing", "lifetimes", "memoria"]
+++

## El problema que resuelve

En C y C++ el programador administra la memoria a mano, y los errores (usar memoria ya liberada, liberarla dos veces, carreras de datos) son la fuente principal de fallas de seguridad. Java, Python o Go lo evitan con un **recolector de basura**, que cuesta rendimiento y pausas. Rust toma un tercer camino: un conjunto de reglas que el **compilador** verifica, sin costo en ejecución.

## Las tres reglas del ownership

> [!IMPORTANT]
> 1. Cada valor tiene un único **dueño** (*owner*).
> 2. Solo puede haber un dueño a la vez.
> 3. Cuando el dueño sale de su ámbito, el valor se libera.

```rust
{
    let s = String::from("hola");   // s es dueña del texto
    println!("{s}");
}                                   // s sale del ámbito: la memoria se libera sola
```

## Mover

Asignar un valor a otra variable, o pasarlo a una función, **mueve** la propiedad. La variable original deja de ser válida:

```rust
let a = String::from("hola");
let b = a;              // la propiedad se mueve de a hacia b
// println!("{a}");     // error de compilación: `a` fue movida
println!("{b}");        // correcto
```

![La propiedad de un String pasa de la variable a hacia la variable b](/media/software/ownership.svg)

*Al mover, solo se copia el puntero; el texto en el heap pasa a tener un único dueño nuevo.*

Así nunca hay dos variables que crean ser dueñas de la misma memoria. Los tipos simples que viven en la pila (`i32`, `f64`, `bool`, `char`) implementan `Copy` y se copian en lugar de moverse. Para duplicar datos del heap hay que pedirlo explícitamente con `.clone()`.

## Préstamos

Para usar un valor sin adueñarse de él se lo **toma prestado** con una referencia:

```rust
fn longitud(s: &String) -> usize {   // recibe una referencia: no es dueña
    s.len()
}

let texto = String::from("BigBrain");
let n = longitud(&texto);            // se presta
println!("{texto} tiene {n} letras"); // texto sigue siendo válido
```

Las referencias obedecen una regla central:

> [!IMPORTANT]
> En un momento dado puede haber **muchas referencias inmutables** (`&T`) **o una sola referencia mutable** (`&mut T`), pero no ambas cosas.

```rust
let mut v = vec![1, 2, 3];
let primero = &v[0];     // préstamo inmutable
// v.push(4);            // error: no se puede modificar mientras existe `primero`
println!("{primero}");
v.push(4);               // correcto: el préstamo anterior ya terminó
```

El ejemplo comentado es un error real en otros lenguajes: `push` puede reubicar el vector y dejar a `primero` apuntando a memoria liberada. Rust lo detecta al compilar. La misma regla elimina las **carreras de datos** entre hilos.

## Lifetimes

Una referencia nunca puede vivir más que el valor al que apunta. El compilador lo verifica con **lifetimes** (tiempos de vida), que casi siempre infiere solo. A veces hay que anotarlos para indicar cómo se relacionan las referencias de entrada y de salida:

```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

`'a` dice que el resultado vive, como mucho, lo que viva la más corta de las dos entradas.

## Resumen

| Quiero… | Uso | La variable original… |
|---|---|---|
| ceder el valor | `f(valor)` | deja de ser válida |
| solo leerlo | `f(&valor)` | sigue válida |
| modificarlo | `f(&mut valor)` | sigue válida; exclusiva durante el préstamo |
| una copia independiente | `f(valor.clone())` | sigue válida |

> [!TIP]
> Los mensajes del compilador de Rust explican qué regla se violó y suelen sugerir la corrección. Leerlos con atención es la forma más rápida de aprender el modelo.
