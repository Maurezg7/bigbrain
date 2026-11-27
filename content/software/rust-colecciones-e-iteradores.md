+++
title = "Rust: colecciones, cadenas e iteradores"
summary = "Vec, String y &str, HashMap, slices, clausuras y la programación con iteradores y sus adaptadores."
branch = "Rust"
order = 5
level = "Intermedio"
tags = ["Rust", "Vec", "String", "HashMap", "iteradores", "closures"]
+++

## Vec

`Vec<T>` es un arreglo dinámico: crece según haga falta y guarda sus elementos contiguos en el montón.

```rust
let mut numeros: Vec<i32> = Vec::new();
numeros.push(3);
numeros.push(1);

let mut v = vec![3, 1, 2];           // macro para crearlo con valores
v.sort();
v.len();                             // 3
v[0];                                // 1; pánico si el índice no existe
v.get(10);                           // None: acceso seguro
v.pop();                             // Some(3)
v.contains(&2);                      // true
```

| Operación | Costo |
|---|---|
| `push`, `pop` (al final) | $O(1)$ amortizado |
| acceso por índice | $O(1)$ |
| `insert`, `remove` (en el medio) | $O(n)$ |
| `contains` | $O(n)$ |

Si se conoce el tamaño final, `Vec::with_capacity(n)` evita reasignaciones.

### Slices

Un **slice** `&[T]` es una vista prestada de una porción contigua de elementos. No es dueño de los datos.

```rust
let v = vec![1, 2, 3, 4, 5];
let medio: &[i32] = &v[1..4];        // [2, 3, 4]

fn suma(valores: &[i32]) -> i32 {
    valores.iter().sum()
}

suma(&v);                            // un &Vec<i32> se convierte en &[i32]
```

> [!IMPORTANT]
> Los parámetros conviene declararlos como `&[T]` y no como `&Vec<T>`: aceptan vectores, arreglos y porciones.

## String y &str

| | `String` | `&str` |
|---|---|---|
| Qué es | texto propio, en el montón | vista prestada de un texto |
| ¿Crece? | sí | no |
| Uso | guardar y modificar | leer, pasar como parámetro |
| Literal | — | `"hola"` es `&'static str` |

```rust
let mut s = String::from("hola");
s.push_str(", mundo");
s.push('!');

let vista: &str = &s[0..4];          // "hola"
let mensaje = format!("{s} ({} bytes)", s.len());

fn saludar(nombre: &str) -> String {         // recibe &str, devuelve String
    format!("Hola, {nombre}")
}
```

Las cadenas son siempre **UTF-8** válido. Un carácter puede ocupar de 1 a 4 bytes, y por eso **no se pueden indexar** con `s[0]`:

```rust
let texto = "ñandú";
texto.len();                 // 7 bytes
texto.chars().count();       // 5 caracteres

for c in texto.chars() { ... }
```

> [!WARNING]
> Cortar una cadena por bytes en medio de un carácter (`&texto[0..1]` con «ñ») produce un pánico.

Métodos útiles: `trim`, `split`, `lines`, `to_uppercase`, `replace`, `starts_with`, `contains`, `parse::<T>()`.

## HashMap

```rust
use std::collections::HashMap;

let mut notas: HashMap<String, u32> = HashMap::new();
notas.insert("Ana".to_string(), 9);
notas.insert("Luis".to_string(), 6);

if let Some(nota) = notas.get("Ana") {
    println!("{nota}");
}

for (nombre, nota) in &notas {
    println!("{nombre}: {nota}");
}
```

La API **entry** inserta o actualiza con una sola búsqueda:

```rust
let mut conteo: HashMap<&str, u32> = HashMap::new();
for palabra in texto.split_whitespace() {
    *conteo.entry(palabra).or_insert(0) += 1;
}
```

| Colección | Uso |
|---|---|
| `Vec<T>` | secuencia; la opción por defecto |
| `VecDeque<T>` | cola con inserción en ambos extremos |
| `HashMap<K, V>` | diccionario sin orden |
| `BTreeMap<K, V>` | diccionario ordenado por clave |
| `HashSet<T>`, `BTreeSet<T>` | conjuntos |
| `BinaryHeap<T>` | cola de prioridad |

## Clausuras

Una **clausura** (*closure*) es una función anónima que puede capturar variables de su entorno.

```rust
let factor = 3;
let triplicar = |x: i32| x * factor;       // captura «factor»
triplicar(5);                              // 15
```

Según cómo usa lo capturado, implementa uno de estos traits:

| Trait | Captura | Se puede llamar |
|---|---|---|
| `Fn` | por referencia compartida | muchas veces |
| `FnMut` | por referencia mutable | muchas veces, modificando su estado |
| `FnOnce` | por valor, consumiéndolo | una vez |

`move` obliga a la clausura a **tomar posesión** de lo que captura; hace falta al pasarla a otro hilo.

```rust
fn aplicar<F: Fn(i32) -> i32>(f: F, valor: i32) -> i32 {
    f(valor)
}
```

## Iteradores

Un **iterador** produce una secuencia de valores, de a uno, con el método `next`.

| Método | Entrega | La colección |
|---|---|---|
| `iter()` | `&T` | queda intacta |
| `iter_mut()` | `&mut T` | se puede modificar |
| `into_iter()` | `T` | se consume |

```rust
for x in &v { ... }          // equivale a v.iter()
for x in &mut v { ... }      // v.iter_mut()
for x in v { ... }           // v.into_iter(): v ya no se puede usar
```

> [!IMPORTANT]
> Los iteradores son **perezosos**: los adaptadores no hacen nada hasta que un **consumidor** pide los valores.

### Adaptadores

Transforman un iterador en otro:

| Adaptador | Qué hace |
|---|---|
| `map(f)` | transforma cada elemento |
| `filter(p)` | conserva los que cumplen |
| `filter_map(f)` | transforma y descarta los `None` |
| `enumerate()` | agrega el índice |
| `zip(otro)` | empareja con otro iterador |
| `take(n)`, `skip(n)` | los primeros `n`; saltea `n` |
| `take_while(p)` | mientras se cumpla |
| `chain(otro)` | concatena |
| `rev()` | invierte |
| `flat_map(f)`, `flatten()` | aplana secuencias anidadas |
| `windows(n)`, `chunks(n)` | ventanas y bloques (sobre slices) |
| `peekable()` | permite mirar el siguiente sin consumirlo |

### Consumidores

Recorren el iterador y producen un resultado:

| Consumidor | Devuelve |
|---|---|
| `collect()` | una colección |
| `sum()`, `product()` | el total |
| `count()` | la cantidad |
| `min()`, `max()` | un `Option` |
| `min_by_key(f)`, `max_by_key(f)` | según una clave |
| `fold(inicial, f)` | un acumulado |
| `any(p)`, `all(p)` | un booleano |
| `find(p)` | el primero que cumple |
| `position(p)` | su índice |
| `for_each(f)` | nada |

```rust
struct Estudiante { nombre: String, nota: f64 }

let aprobados: Vec<&str> = estudiantes
    .iter()
    .filter(|e| e.nota >= 6.0)
    .map(|e| e.nombre.as_str())
    .collect();

let promedio = estudiantes.iter().map(|e| e.nota).sum::<f64>() / estudiantes.len() as f64;

let mejor = estudiantes
    .iter()
    .max_by(|a, b| a.nota.total_cmp(&b.nota));

let cuadrados_pares: u32 = (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).sum();   // 220
```

### collect

Construye distintas colecciones según el tipo pedido:

```rust
let v: Vec<i32> = (1..=3).collect();
let conjunto: HashSet<char> = "banana".chars().collect();
let mapa: HashMap<&str, usize> = ["sol", "luna"].iter().map(|p| (*p, p.len())).collect();
let texto: String = vec!["a", "b"].concat();
let numeros: Result<Vec<i32>, _> = ["1", "2"].iter().map(|s| s.parse::<i32>()).collect();
```

## Rendimiento

Las cadenas de iteradores se compilan al mismo código máquina que un bucle escrito a mano: son **abstracciones de costo cero**. Además evitan las verificaciones de límites del acceso por índice.

## Ownership al iterar

| Situación | Solución |
|---|---|
| Modificar un vector mientras se lo recorre | no compila: recolectar los cambios y aplicarlos después, o usar `retain` |
| Usar la colección después de `into_iter()` | iterar por referencia (`&v`) |
| Hace falta un `String` propio a partir de `&str` | `.to_string()` o `.to_owned()` |
| Convertir `&&str` o `&i32` en valores | `.copied()` o `.cloned()` |

```rust
v.retain(|x| x % 2 == 0);        // quita los impares, en el lugar
v.dedup();                       // quita repetidos consecutivos
v.sort_by_key(|e| e.abs());
```

## Ordenar

```rust
v.sort();                                        // requiere Ord
v.sort_unstable();                               // más rápido, no estable
personas.sort_by(|a, b| a.edad.cmp(&b.edad));
flotantes.sort_by(|a, b| a.total_cmp(b));        // f64 no implementa Ord
```

Los `f64` no tienen orden total (por `NaN`), y por eso no admiten `sort()` directo.

> [!TIP]
> Cuando un bucle arma un vector con `push`, casi siempre puede escribirse como `iter().map(...).collect()`, que es más corto y dice mejor la intención.
