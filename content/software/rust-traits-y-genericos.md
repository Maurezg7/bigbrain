+++
title = "Rust: traits y genéricos"
summary = "Comportamiento compartido con traits, funciones y tipos genéricos, cotas, despacho estático y dinámico, y los traits de la biblioteca estándar."
branch = "Rust"
order = 3
level = "Avanzado"
tags = ["Rust", "traits", "genéricos", "dyn", "polimorfismo"]
+++

## Traits

Un **trait** define un conjunto de métodos que un tipo puede implementar. Es la forma en que Rust expresa comportamiento compartido; se parece a las interfaces de otros lenguajes.

```rust
trait Figura {
    fn area(&self) -> f64;

    fn describir(&self) -> String {                // método por defecto
        format!("figura de área {:.2}", self.area())
    }
}

struct Circulo { radio: f64 }
struct Rectangulo { base: f64, altura: f64 }

impl Figura for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }
}

impl Figura for Rectangulo {
    fn area(&self) -> f64 {
        self.base * self.altura
    }
}
```

Rust **no tiene herencia** de structs. El código se reutiliza con traits y composición.

Se puede implementar un trait propio para un tipo ajeno, o un trait ajeno para un tipo propio. No se puede implementar un trait ajeno para un tipo ajeno (**regla del huérfano**).

## Genéricos

Permiten escribir código que funciona con muchos tipos.

```rust
struct Par<T> {
    primero: T,
    segundo: T,
}

fn primero<T>(lista: &[T]) -> Option<&T> {
    lista.first()
}
```

`Option<T>`, `Vec<T>` y `HashMap<K, V>` son tipos genéricos.

### Cotas

Una función genérica solo puede usar las operaciones que sus **cotas** (*trait bounds*) garantizan:

```rust
fn maximo<T: PartialOrd + Copy>(lista: &[T]) -> Option<T> {
    let mut mayor = *lista.first()?;
    for &valor in lista {
        if valor > mayor {
            mayor = valor;
        }
    }
    Some(mayor)
}
```

Sin `PartialOrd`, la comparación `>` no compilaría.

Tres formas equivalentes de escribirlas:

```rust
fn imprimir_area(f: &impl Figura) { ... }

fn imprimir_area<T: Figura>(f: &T) { ... }

fn imprimir_area<T>(f: &T)
where
    T: Figura,
{ ... }
```

`where` es más legible cuando hay muchas cotas.

## Despacho estático y dinámico

### Estático

Con genéricos, el compilador genera una copia especializada de la función para cada tipo usado (**monomorfización**).

```rust
fn area_total<T: Figura>(figuras: &[T]) -> f64 {
    figuras.iter().map(|f| f.area()).sum()
}
```

No tiene costo en ejecución: es una **abstracción de costo cero**. Pero todos los elementos del arreglo deben ser del **mismo** tipo.

### Dinámico

Para mezclar tipos distintos se usa un **objeto trait**, `dyn Trait`, detrás de un puntero:

```rust
let figuras: Vec<Box<dyn Figura>> = vec![
    Box::new(Circulo { radio: 1.0 }),
    Box::new(Rectangulo { base: 2.0, altura: 3.0 }),
];

for f in &figuras {
    println!("{}", f.describir());
}
```

El método a llamar se busca en tiempo de ejecución, en una tabla (**vtable**).

| | Estático (`impl Trait`, `<T: Trait>`) | Dinámico (`dyn Trait`) |
|---|---|---|
| Se resuelve | al compilar | al ejecutar |
| Costo por llamada | ninguno; permite *inlining* | una indirección |
| Tamaño del binario | mayor (una copia por tipo) | menor |
| Colección con tipos mezclados | no | sí |

> [!IMPORTANT]
> Usar genéricos por defecto. Recurrir a `dyn` cuando hace falta guardar tipos distintos juntos o decidir el tipo en tiempo de ejecución.

Una alternativa cuando el conjunto de tipos es **cerrado** y conocido: un `enum` con `match`.

## Traits de la biblioteca estándar

| Trait | Para qué | ¿Derivable? |
|---|---|---|
| `Debug` | impresión para depurar (`{:?}`) | sí |
| `Display` | impresión para el usuario (`{}`) | no |
| `Clone`, `Copy` | copias | sí |
| `PartialEq`, `Eq` | igualdad | sí |
| `PartialOrd`, `Ord` | orden | sí |
| `Hash` | claves de `HashMap` | sí |
| `Default` | valor por defecto | sí |
| `From`, `Into` | conversiones que no fallan | no |
| `TryFrom`, `TryInto` | conversiones que pueden fallar | no |
| `Iterator` | recorrido con `for` | no |
| `Drop` | código al liberar el valor | no |
| `Add`, `Mul`, `Index`... | sobrecarga de operadores | no |

```rust
use std::fmt;

struct Punto { x: f64, y: f64 }

impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

println!("{}", Punto { x: 1.0, y: 2.0 });      // (1, 2)
```

### Conversiones

Implementar `From` da `Into` gratis:

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

let f: Fahrenheit = Celsius(100.0).into();     // 212
```

### Sobrecarga de operadores

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Vector { x: f64, y: f64 }

impl Add for Vector {
    type Output = Vector;

    fn add(self, otro: Vector) -> Vector {
        Vector { x: self.x + otro.x, y: self.y + otro.y }
    }
}

let v = Vector { x: 1.0, y: 2.0 } + Vector { x: 3.0, y: 4.0 };
```

## Tipos asociados

Un trait puede declarar un tipo que cada implementación define:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Contador { n: u32 }

impl Iterator for Contador {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n < 5 {
            self.n += 1;
            Some(self.n)
        } else {
            None
        }
    }
}

let suma: u32 = Contador { n: 0 }.sum();       // 15
```

Con implementar `next`, el tipo recibe todos los métodos de los iteradores.

## Implementaciones condicionales

```rust
impl<T: fmt::Display> Par<T> {
    fn mostrar(&self) {
        println!("{} y {}", self.primero, self.segundo);
    }
}
```

`mostrar` existe solo para los `Par<T>` cuyo `T` se puede imprimir.

Las **implementaciones generales** (*blanket*) cubren a todos los tipos que cumplen una cota. La biblioteca estándar implementa `ToString` para todo lo que implementa `Display`.

## Supertraits

Un trait puede exigir otro:

```rust
trait Entidad: fmt::Debug + Clone {
    fn id(&self) -> u64;
}
```

## Lifetimes genéricos

Un **lifetime** es un parámetro genérico que describe cuánto vive una referencia:

```rust
fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Indica que el resultado no vive más que la menor de las dos entradas. En la mayoría de las funciones el compilador los deduce.

## Diseñar con traits

| Necesidad | Herramienta |
|---|---|
| El mismo algoritmo para varios tipos | función genérica con cotas |
| Una colección de tipos distintos | `Vec<Box<dyn Trait>>` |
| Un conjunto cerrado de variantes | `enum` |
| Reemplazar una dependencia en las pruebas | un trait y una implementación falsa |
| Extender un tipo ajeno | un trait propio implementado para él |

> [!WARNING]
> No todo trait sirve como `dyn`. Para ser **compatible con objetos**, sus métodos no pueden ser genéricos ni devolver `Self`.

> [!TIP]
> Mantener los traits pequeños y enfocados: es más fácil implementar y combinar varios chicos que uno enorme.
