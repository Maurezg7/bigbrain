+++
title = "Recursión y ordenamiento"
summary = "Funciones que se llaman a sí mismas, la estrategia divide y vencerás y el algoritmo merge sort."
branch = "Algoritmos y estructuras de datos"
order = 3
level = "Intermedio"
tags = ["recursión", "divide y vencerás", "merge sort", "caso base"]
+++

## Recursión

Una función es **recursiva** cuando se llama a sí misma para resolver una versión más pequeña del mismo problema. Toda función recursiva necesita dos partes:

1. Un **caso base**, que se resuelve directamente y detiene la recursión.
2. Un **caso recursivo**, que reduce el problema y se acerca al caso base.

```python
def factorial(n):
    if n <= 1:                    # caso base
        return 1
    return n * factorial(n - 1)   # caso recursivo
```

Cada llamada queda en espera en la **pila de llamadas** hasta que la siguiente termina:

```text
factorial(4)
= 4 * factorial(3)
= 4 * (3 * factorial(2))
= 4 * (3 * (2 * factorial(1)))
= 4 * (3 * (2 * 1))
= 24
```

> [!WARNING]
> Sin caso base, o si el caso recursivo no se acerca a él, la pila crece hasta agotarse: es el error *stack overflow*.

## Cuando la recursión ingenua es lenta

La definición directa de Fibonacci recalcula los mismos valores una y otra vez, y resulta exponencial:

```python
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)   # O(2^n)
```

La solución es **memoizar**: guardar los resultados ya calculados. Con eso pasa a $O(n)$.

```python
from functools import lru_cache

@lru_cache(maxsize=None)
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)   # O(n)
```

## Divide y vencerás

Es una estrategia de diseño en tres pasos: **dividir** el problema en subproblemas más chicos, **resolver** cada uno recursivamente y **combinar** las soluciones.

## Merge sort

Para ordenar una lista: se parte por la mitad, se ordena cada mitad recursivamente y se **mezclan** las dos mitades ya ordenadas.

![Merge sort dividiendo una lista de ocho elementos y luego mezclando](/media/software/merge-sort.svg)

*Merge sort: primero se divide hasta llegar a listas de un elemento, después se mezcla de a pares.*

```python
def merge_sort(v):
    if len(v) <= 1:
        return v
    medio = len(v) // 2
    izq, der = merge_sort(v[:medio]), merge_sort(v[medio:])

    resultado, i, j = [], 0, 0
    while i < len(izq) and j < len(der):
        if izq[i] <= der[j]:
            resultado.append(izq[i]); i += 1
        else:
            resultado.append(der[j]); j += 1
    return resultado + izq[i:] + der[j:]
```

Hay $\log_2 n$ niveles de división y en cada nivel la mezcla recorre los $n$ elementos, así que el costo total es $O(n \log n)$, en todos los casos.

## Comparación de algoritmos de ordenamiento

| Algoritmo | Promedio | Peor caso | Memoria extra | Estable |
|---|---|---|---|---|
| Burbuja | $O(n^2)$ | $O(n^2)$ | $O(1)$ | sí |
| Inserción | $O(n^2)$ | $O(n^2)$ | $O(1)$ | sí |
| Merge sort | $O(n \log n)$ | $O(n \log n)$ | $O(n)$ | sí |
| Quicksort | $O(n \log n)$ | $O(n^2)$ | $O(\log n)$ | no |
| Heapsort | $O(n \log n)$ | $O(n \log n)$ | $O(1)$ | no |

Un ordenamiento es **estable** si conserva el orden relativo de los elementos iguales. Ningún algoritmo basado en comparaciones puede ser mejor que $O(n \log n)$ en el peor caso.

> [!TIP]
> En el trabajo real se usa el ordenamiento de la biblioteca estándar (`sort` en Python, `.sort()` en Rust), que ya es $O(n \log n)$ y está muy optimizado. Estudiar estos algoritmos sirve para aprender a diseñar y analizar los propios.
