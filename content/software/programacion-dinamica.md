+++
title = "Programación dinámica y algoritmos voraces"
summary = "Subproblemas superpuestos, memoización y tabulación, problemas clásicos de programación dinámica, algoritmos voraces y vuelta atrás."
branch = "Algoritmos y estructuras de datos"
order = 7
level = "Avanzado"
tags = ["programación dinámica", "memoización", "greedy", "backtracking", "mochila"]
+++

## La idea

La **programación dinámica** resuelve un problema combinando las soluciones de subproblemas más chicos, y **guarda** cada una para no recalcularla.

Se aplica cuando el problema tiene:

> [!IMPORTANT]
> 1. **Subestructura óptima**: la solución óptima se arma con soluciones óptimas de subproblemas.
> 2. **Subproblemas superpuestos**: los mismos subproblemas aparecen una y otra vez.

Si los subproblemas no se repiten, alcanza con divide y vencerás (como en merge sort).

## El ejemplo básico: Fibonacci

La versión recursiva directa recalcula lo mismo exponencialmente:

```python
def fib(n):                    # O(2^n)
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)
```

### Memoización (de arriba hacia abajo)

Se mantiene la recursión y se guarda cada resultado:

```python
from functools import lru_cache

@lru_cache(maxsize=None)
def fib(n):                    # O(n)
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)
```

### Tabulación (de abajo hacia arriba)

Se llena una tabla desde los casos base:

```python
def fib(n):
    if n < 2:
        return n
    tabla = [0] * (n + 1)
    tabla[1] = 1
    for i in range(2, n + 1):
        tabla[i] = tabla[i - 1] + tabla[i - 2]
    return tabla[n]
```

Como cada valor depende solo de los dos anteriores, bastan dos variables: memoria $O(1)$.

| | Memoización | Tabulación |
|---|---|---|
| Dirección | del problema a los casos base | de los casos base al problema |
| Forma | recursiva | iterativa |
| Calcula | solo los subproblemas necesarios | todos |
| Riesgo | desbordar la pila | ninguno |
| Optimizar memoria | difícil | más fácil |

## Método

1. **Definir el estado**: qué representa `dp[i]` (o `dp[i][j]`), con palabras precisas.
2. Escribir la **recurrencia**: cómo se obtiene un estado de otros más chicos.
3. Fijar los **casos base**.
4. Decidir el **orden** de cálculo.
5. Ubicar la **respuesta**.

El paso difícil es el primero.

## Problemas clásicos

### Cambio de monedas

Mínima cantidad de monedas para formar un monto.

- Estado: `dp[m]` = mínima cantidad de monedas para el monto `m`.
- Recurrencia: `dp[m] = 1 + min(dp[m - c])` para cada moneda `c ≤ m`.
- Base: `dp[0] = 0`.

```python
def cambio(monedas, monto):
    INF = float("inf")
    dp = [0] + [INF] * monto
    for m in range(1, monto + 1):
        for c in monedas:
            if c <= m and dp[m - c] + 1 < dp[m]:
                dp[m] = dp[m - c] + 1
    return dp[monto] if dp[monto] != INF else -1
```

Costo: $O(\text{monto} \times \text{cantidad de monedas})$.

> [!NOTE]
> Con monedas 1, 3 y 4, para el monto 6:
>
> | m | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
> |---|---|---|---|---|---|---|---|
> | dp | 0 | 1 | 2 | 1 | 1 | 2 | 2 |
>
> La respuesta es 2 (3 + 3). Elegir siempre la moneda más grande daría 4 + 1 + 1: tres monedas.

### Mochila 0/1

Hay objetos con peso y valor, y una mochila de capacidad $W$. Cada objeto se lleva entero o no se lleva. Maximizar el valor.

- Estado: `dp[i][w]` = mejor valor usando los primeros `i` objetos con capacidad `w`.
- Recurrencia: el máximo entre **no llevar** el objeto `i` (`dp[i-1][w]`) y **llevarlo** (`valor[i] + dp[i-1][w - peso[i]]`), si entra.

```python
def mochila(pesos, valores, capacidad):
    dp = [0] * (capacidad + 1)
    for peso, valor in zip(pesos, valores):
        for w in range(capacidad, peso - 1, -1):    # de mayor a menor
            dp[w] = max(dp[w], valor + dp[w - peso])
    return dp[capacidad]
```

Recorrer `w` de mayor a menor evita usar el mismo objeto dos veces. Costo: $O(nW)$.

### Subsecuencia común más larga

Dadas dos cadenas, la subsecuencia (no necesariamente contigua) más larga que aparece en ambas.

- Estado: `dp[i][j]` = longitud para los primeros `i` caracteres de `a` y los primeros `j` de `b`.
- Si `a[i-1] == b[j-1]`: `dp[i][j] = dp[i-1][j-1] + 1`.
- Si no: `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`.

```python
def lcs(a, b):
    dp = [[0] * (len(b) + 1) for _ in range(len(a) + 1)]
    for i in range(1, len(a) + 1):
        for j in range(1, len(b) + 1):
            if a[i - 1] == b[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
    return dp[len(a)][len(b)]
```

Es la base de `diff` y de la comparación de secuencias de ADN.

### Otros

| Problema | Estado | Costo |
|---|---|---|
| Subir una escalera de a 1 o 2 pasos | formas de llegar al escalón $i$ | $O(n)$ |
| Subsecuencia creciente más larga | la más larga que termina en $i$ | $O(n^2)$ |
| Distancia de edición | operaciones para convertir un prefijo en otro | $O(nm)$ |
| Caminos en una grilla | formas de llegar a la celda $(i, j)$ | $O(nm)$ |
| Suma máxima de un subarreglo (Kadane) | mejor suma que termina en $i$ | $O(n)$ |

## Algoritmos voraces

Un algoritmo **voraz** (*greedy*) elige en cada paso la mejor opción **local** y nunca la revisa. Es simple y rápido, pero solo es correcto en ciertos problemas.

| Problema | Elección voraz | ¿Óptimo? |
|---|---|---|
| Selección de actividades | la que termina antes | sí |
| Mochila fraccionaria | mayor valor por unidad de peso | sí |
| Árbol de expansión mínima | la arista más barata que no forma ciclo | sí |
| Códigos de Huffman | unir los dos símbolos menos frecuentes | sí |
| Cambio con monedas 1, 3, 4 | la moneda más grande | **no** |
| Mochila 0/1 | mayor valor por peso | **no** |

> [!WARNING]
> Que un algoritmo voraz dé la respuesta correcta en algunos ejemplos no prueba nada. Hay que demostrar que la elección local siempre lleva al óptimo, o buscar un contraejemplo.

## Vuelta atrás

La **vuelta atrás** (*backtracking*) explora todas las opciones construyendo la solución paso a paso, y **retrocede** apenas una opción parcial no puede completarse.

```python
def permutaciones(elementos, actual=None, usados=None):
    actual = actual or []
    usados = usados or set()
    if len(actual) == len(elementos):
        print(actual)
        return
    for e in elementos:
        if e not in usados:
            actual.append(e); usados.add(e)        # elegir
            permutaciones(elementos, actual, usados)
            actual.pop(); usados.remove(e)         # deshacer
```

Esquema: **elegir, explorar, deshacer**. La **poda** descarta ramas enteras y es lo que lo vuelve práctico.

Se usa en el problema de las $n$ reinas, el sudoku, la generación de combinaciones y la coloración de grafos. Su costo es exponencial en el peor caso.

## Elegir la técnica

| Técnica | Cuándo |
|---|---|
| Fuerza bruta | la entrada es chica |
| Divide y vencerás | subproblemas independientes |
| Voraz | una elección local es demostrablemente óptima |
| Programación dinámica | subproblemas superpuestos y subestructura óptima |
| Vuelta atrás | hay que explorar combinaciones con restricciones |

> [!TIP]
> Empezar por la solución recursiva directa. Si repite subproblemas, agregarle memoización ya la convierte en programación dinámica.
