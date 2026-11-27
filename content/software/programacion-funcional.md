+++
title = "Programación funcional"
summary = "Funciones puras, inmutabilidad, funciones de orden superior, map, filter y reduce, clausuras y composición."
branch = "Fundamentos de programación"
order = 4
level = "Intermedio"
tags = ["programación funcional", "inmutabilidad", "map", "lambda", "clausuras"]
+++

## La idea

La **programación funcional** construye programas combinando **funciones** que transforman datos, y evita el estado mutable y los efectos secundarios.

| | Imperativo | Funcional |
|---|---|---|
| Describe | **cómo** hacerlo, paso a paso | **qué** se quiere obtener |
| Estado | variables que cambian | datos inmutables |
| Repetición | bucles | recursión y funciones de orden superior |
| Unidad | instrucciones | expresiones |

```python
# Imperativo
cuadrados = []
for n in numeros:
    if n % 2 == 0:
        cuadrados.append(n * n)

# Funcional
cuadrados = [n * n for n in numeros if n % 2 == 0]
```

## Funciones puras

Una función pura depende solo de sus argumentos y no modifica nada externo.

> [!IMPORTANT]
> **Transparencia referencial**: una llamada a una función pura puede reemplazarse por su resultado sin cambiar el programa.

Ventajas:

- Se prueban sin preparar ningún contexto.
- Se pueden ejecutar en paralelo sin riesgo.
- Sus resultados se pueden guardar en caché (**memoización**).
- Se razona sobre ellas de forma aislada.

## Inmutabilidad

Los datos no se modifican: se crean versiones nuevas.

```python
original = (1, 2, 3)               # tupla: inmutable
nueva = original + (4,)            # original no cambia

precios = {"pan": 100}
actualizado = {**precios, "leche": 200}   # copia con un cambio
```

Elimina una clase entera de errores: nadie puede cambiar un dato «por detrás» mientras otra parte del programa lo usa.

## Funciones como valores

En los lenguajes funcionales las funciones son **ciudadanos de primera clase**: se guardan en variables, se pasan como argumentos y se devuelven.

Una **función anónima** o **lambda** se define sin nombre:

```python
doble = lambda x: x * 2
```

```javascript
const doble = (x) => x * 2;
```

Una **función de orden superior** recibe o devuelve otras funciones.

## Map, filter y reduce

| Función | Qué hace | Entrada → salida |
|---|---|---|
| **map** | transforma cada elemento | $n$ elementos → $n$ elementos |
| **filter** | conserva los que cumplen una condición | $n$ → $n$ o menos |
| **reduce** (fold) | combina todos en un único valor | $n$ → 1 |

```python
from functools import reduce

numeros = [1, 2, 3, 4, 5, 6]

pares = filter(lambda n: n % 2 == 0, numeros)        # 2, 4, 6
cuadrados = map(lambda n: n * n, pares)              # 4, 16, 36
suma = reduce(lambda acc, n: acc + n, cuadrados, 0)  # 56
```

En JavaScript se encadenan como métodos:

```javascript
const suma = numeros
  .filter((n) => n % 2 === 0)
  .map((n) => n * n)
  .reduce((acc, n) => acc + n, 0);
```

En Rust, con iteradores:

```rust
let suma: i32 = numeros.iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * n)
    .sum();
```

Cada paso dice **qué** se hace, sin índices ni variables auxiliares.

## Clausuras

Una **clausura** (*closure*) es una función que recuerda las variables del ámbito donde fue creada, aunque ese ámbito ya haya terminado.

```python
def multiplicador(factor):
    def multiplicar(x):
        return x * factor      # recuerda «factor»
    return multiplicar

triple = multiplicador(3)
triple(5)                      # 15
```

Sirven para crear funciones configuradas, mantener estado privado y escribir callbacks.

## Composición

Construir funciones encadenando otras: la salida de una es la entrada de la siguiente.

$$(f \circ g)(x) = f(g(x))$$

```python
def componer(f, g):
    return lambda x: f(g(x))

limpiar = componer(str.upper, str.strip)
limpiar("  hola  ")            # 'HOLA'
```

### Aplicación parcial y currificación

- **Aplicación parcial**: fijar algunos argumentos de una función para obtener otra con menos parámetros.
- **Currificación**: convertir una función de varios argumentos en una cadena de funciones de un argumento.

```python
from functools import partial

def potencia(base, exponente):
    return base ** exponente

cuadrado = partial(potencia, exponente=2)
cuadrado(5)                    # 25
```

## Recursión

Sin variables mutables no hay bucles tradicionales: la repetición se expresa con recursión.

```python
def suma(lista):
    if not lista:
        return 0
    return lista[0] + suma(lista[1:])
```

En la **recursión de cola**, la llamada recursiva es lo último que hace la función. Algunos lenguajes (Haskell, Scheme, Scala) la optimizan para que no consuma pila; Python y Java no.

## Evaluación perezosa

Con **evaluación perezosa** (*lazy*), un valor se calcula recién cuando se necesita. Permite trabajar con secuencias enormes o infinitas.

```python
def naturales():
    n = 0
    while True:
        yield n
        n += 1

from itertools import islice
list(islice((n * n for n in naturales()), 5))    # [0, 1, 4, 9, 16]
```

Los iteradores de Rust y los *streams* de Java también son perezosos: nada se ejecuta hasta la operación final.

## Manejo de la ausencia y los errores

En lugar de `null` y excepciones, se usan tipos que hacen explícita la posibilidad de fallo:

| Tipo | Representa | Lenguajes |
|---|---|---|
| `Option` / `Maybe` | un valor que puede faltar | Rust, Haskell, Scala, Java (`Optional`) |
| `Result` / `Either` | un resultado o un error | Rust, Haskell, Scala |

El compilador obliga a contemplar ambos casos.

## Coincidencia de patrones

Permite descomponer datos y elegir una rama según su forma:

```rust
match forma {
    Forma::Circulo { radio } => 3.14159 * radio * radio,
    Forma::Rectangulo { base, altura } => base * altura,
}
```

## Lenguajes

| Lenguaje | Estilo |
|---|---|
| Haskell | funcional puro, perezoso |
| Elixir, Erlang, Clojure, F#, OCaml | principalmente funcionales |
| Scala, Kotlin, Rust, Swift | multiparadigma con fuerte influencia funcional |
| JavaScript, Python, Java, C# | imperativos con herramientas funcionales |

> [!WARNING]
> No todo es más claro en estilo funcional. Una cadena larga de `map` y `reduce` anidados puede ser más difícil de leer que un bucle simple. El objetivo es la claridad.

> [!TIP]
> Un buen punto de partida en cualquier lenguaje: separar el núcleo de cálculo (funciones puras) de los bordes que hacen entrada y salida.
