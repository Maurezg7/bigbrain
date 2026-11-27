+++
title = "Rust: structs, enums y pattern matching"
summary = "Tipos de datos propios con struct y enum, métodos con impl, Option, y coincidencia de patrones con match."
branch = "Rust"
order = 2
level = "Intermedio"
tags = ["Rust", "struct", "enum", "match", "Option"]
+++

## Structs

Un **struct** agrupa datos relacionados bajo un nombre.

```rust
struct Estudiante {
    nombre: String,
    edad: u32,
    promedio: f64,
}

let mut ana = Estudiante {
    nombre: String::from("Ana"),
    edad: 20,
    promedio: 8.5,
};

ana.edad += 1;
println!("{} tiene {} años", ana.nombre, ana.edad);
```

Atajos:

```rust
fn nuevo(nombre: String, edad: u32) -> Estudiante {
    Estudiante { nombre, edad, promedio: 0.0 }      // nombre: nombre
}

let luis = Estudiante { nombre: String::from("Luis"), ..ana };   // el resto, de ana
```

| Variante | Ejemplo | Uso |
|---|---|---|
| Con campos nombrados | `struct Punto { x: f64, y: f64 }` | el caso general |
| Tupla | `struct Color(u8, u8, u8);` | campos sin nombre |
| Envoltorio (*newtype*) | `struct Metros(f64);` | distinguir tipos con la misma representación |
| Unidad | `struct Marcador;` | sin datos |

El patrón *newtype* impide, al compilar, mezclar metros con segundos.

## Métodos

Se definen en un bloque `impl`:

```rust
struct Rectangulo {
    base: f64,
    altura: f64,
}

impl Rectangulo {
    fn new(base: f64, altura: f64) -> Self {       // función asociada
        Self { base, altura }
    }

    fn area(&self) -> f64 {                        // lee
        self.base * self.altura
    }

    fn escalar(&mut self, factor: f64) {           // modifica
        self.base *= factor;
        self.altura *= factor;
    }
}

let mut r = Rectangulo::new(3.0, 4.0);
r.escalar(2.0);
println!("{}", r.area());      // 48
```

| Primer parámetro | Significado |
|---|---|
| `&self` | toma prestado para leer |
| `&mut self` | toma prestado para modificar |
| `self` | consume el valor |
| (ninguno) | función asociada, se llama con `Tipo::funcion()` |

No hay constructores especiales: por convención se escribe una función asociada `new`.

### Derivar comportamiento

```rust
#[derive(Debug, Clone, PartialEq)]
struct Punto {
    x: f64,
    y: f64,
}

let p = Punto { x: 1.0, y: 2.0 };
println!("{p:?}");             // Punto { x: 1.0, y: 2.0 }
assert_eq!(p, p.clone());
```

| Trait | Aporta |
|---|---|
| `Debug` | impresión con `{:?}` |
| `Clone` | copia explícita con `.clone()` |
| `Copy` | copia implícita (solo tipos simples) |
| `PartialEq`, `Eq` | comparación con `==` |
| `PartialOrd`, `Ord` | orden |
| `Hash` | uso como clave de un `HashMap` |
| `Default` | valor por defecto |

## Enums

Un **enum** define un tipo que puede ser **una de varias variantes**. En Rust, cada variante puede llevar datos distintos.

```rust
enum Forma {
    Circulo { radio: f64 },
    Rectangulo { base: f64, altura: f64 },
    Triangulo(f64, f64, f64),
    Punto,
}
```

Es un **tipo suma**: un valor es un círculo **o** un rectángulo **o**... (un struct es un tipo producto: tiene un campo **y** otro).

> [!IMPORTANT]
> Modelar con enums hace que los **estados inválidos no se puedan representar**: no puede existir un rectángulo con radio.

## Match

`match` compara un valor contra patrones y ejecuta la rama que coincide. Es una **expresión**: devuelve un valor.

```rust
impl Forma {
    fn area(&self) -> f64 {
        match self {
            Forma::Circulo { radio } => std::f64::consts::PI * radio * radio,
            Forma::Rectangulo { base, altura } => base * altura,
            Forma::Triangulo(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
            Forma::Punto => 0.0,
        }
    }
}
```

> [!IMPORTANT]
> `match` debe ser **exhaustivo**: si falta una variante, no compila. Al agregar una variante nueva, el compilador señala todos los lugares que hay que actualizar.

### Patrones

```rust
match numero {
    0 => println!("cero"),
    1 | 2 | 3 => println!("chico"),
    4..=9 => println!("mediano"),
    n if n < 0 => println!("negativo: {n}"),     // guarda
    _ => println!("grande"),                     // comodín
}

let (x, y) = (3, 4);                             // desestructurar una tupla
let Punto { x, y } = p;                          // y un struct
```

## Option

Rust **no tiene `null`**. La posible ausencia de un valor se expresa con el tipo `Option`:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
fn buscar(nombres: &[&str], objetivo: &str) -> Option<usize> {
    for (i, n) in nombres.iter().enumerate() {
        if *n == objetivo {
            return Some(i);
        }
    }
    None
}

match buscar(&["Ana", "Luis"], "Luis") {
    Some(i) => println!("está en la posición {i}"),
    None => println!("no está"),
}
```

Un `Option<usize>` no es un `usize`: el compilador obliga a tratar el caso `None` antes de usar el valor. Se elimina así toda la familia de errores de referencia nula.

### Formas breves

```rust
if let Some(i) = buscar(&nombres, "Ana") {       // solo interesa un caso
    println!("{i}");
}

let Some(i) = buscar(&nombres, "Ana") else {     // o salir temprano
    return;
};

while let Some(tarea) = pila.pop() {             // repetir mientras haya
    procesar(tarea);
}
```

### Métodos de Option

| Método | Qué hace |
|---|---|
| `unwrap_or(valor)` | el contenido, o un valor por defecto |
| `unwrap_or_default()` | el contenido, o el valor por defecto del tipo |
| `map(f)` | transforma el contenido, si existe |
| `and_then(f)` | encadena otra operación que devuelve `Option` |
| `ok_or(error)` | convierte en `Result` |
| `is_some()`, `is_none()` | consultas |
| `?` | devuelve `None` de inmediato si no hay valor |
| `unwrap()`, `expect(msg)` | el contenido, o **pánico** si es `None` |

```rust
let longitud = nombre.map(|n| n.len()).unwrap_or(0);
```

> [!WARNING]
> `unwrap()` hace caer el programa si el valor es `None`. Es aceptable en ejemplos y pruebas; en código real, tratar el caso o usar `expect` con un mensaje que explique por qué no puede fallar.

## Máquinas de estados

Los enums con datos modelan estados de forma natural:

```rust
enum Pedido {
    Pendiente,
    Pagado { monto: f64 },
    Enviado { seguimiento: String },
    Cancelado { motivo: String },
}

fn describir(p: &Pedido) -> String {
    match p {
        Pedido::Pendiente => "esperando el pago".into(),
        Pedido::Pagado { monto } => format!("pagado: ${monto:.2}"),
        Pedido::Enviado { seguimiento } => format!("en camino ({seguimiento})"),
        Pedido::Cancelado { motivo } => format!("cancelado: {motivo}"),
    }
}
```

El código de seguimiento solo existe cuando el pedido fue enviado.

## Ownership en structs

- Si un struct posee sus datos (`String`, `Vec`), es dueño de ellos y los libera al salir de ámbito.
- Si guarda **referencias**, necesita un parámetro de **lifetime** que garantice que los datos viven lo suficiente:

```rust
struct Fragmento<'a> {
    texto: &'a str,
}
```

En general conviene que los structs sean dueños de sus datos.

## Módulos y visibilidad

Todo es **privado** por defecto. `pub` lo hace visible fuera de su módulo.

```rust
mod banco {
    pub struct Cuenta {
        pub titular: String,
        saldo: f64,                    // privado
    }

    impl Cuenta {
        pub fn new(titular: String) -> Self {
            Self { titular, saldo: 0.0 }
        }

        pub fn saldo(&self) -> f64 {
            self.saldo
        }
    }
}
```

Con el campo `saldo` privado, solo los métodos del módulo pueden modificarlo: así se protegen las invariantes.

> [!TIP]
> Antes de escribir la lógica, definir los tipos. Si un `bool` y un `Option` juntos admiten combinaciones sin sentido, reemplazarlos por un enum.
