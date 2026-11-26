+++
title = "Complejidad algorítmica y notación Big O"
summary = "Cómo medir la eficiencia de un algoritmo según crece el tamaño de la entrada."
branch = "Algoritmos y estructuras de datos"
order = 1
level = "Intermedio"
tags = ["Big O", "eficiencia", "búsqueda binaria", "análisis"]
+++

## Por qué no alcanza con medir el tiempo

El tiempo en segundos depende de la máquina, del lenguaje y de los datos. Lo que interesa es **cómo crece** el trabajo cuando crece el tamaño $n$ de la entrada. Eso es lo que expresa la notación **Big O**.

> [!IMPORTANT]
> $f(n) = O(g(n))$ si existen constantes $c > 0$ y $n_0$ tales que $f(n) \le c\,g(n)$ para todo $n \ge n_0$. Es decir, $g$ es una cota superior del crecimiento de $f$.

En la práctica: se cuenta la cantidad de operaciones, se conserva el término que más crece y se descartan las constantes. $3n^2 + 10n + 5$ es $O(n^2)$.

## Las clases más comunes

![Curvas de crecimiento de O(1), O(log n), O(n), O(n log n) y O(n²)](/media/software/big-o.svg)

*Cantidad de operaciones en función del tamaño de la entrada.*

| Notación | Nombre | Ejemplo | Operaciones para $n = 10^6$ |
|---|---|---|---|
| $O(1)$ | constante | acceder a `v[i]` | 1 |
| $O(\log n)$ | logarítmica | búsqueda binaria | ~20 |
| $O(n)$ | lineal | recorrer una lista | $10^6$ |
| $O(n \log n)$ | linearítmica | merge sort | ~$2 \times 10^7$ |
| $O(n^2)$ | cuadrática | dos bucles anidados | $10^{12}$ |
| $O(2^n)$ | exponencial | probar todos los subconjuntos | inviable |

## Cómo analizar código

- Instrucciones sueltas: $O(1)$.
- Un bucle que recorre $n$ elementos: $O(n)$.
- Bucles anidados: se multiplican. Dos bucles de $n$ dan $O(n^2)$.
- Bloques consecutivos: se suman, y queda el mayor.
- Si en cada paso el problema se reduce a la mitad: $O(\log n)$.

## Búsqueda lineal contra búsqueda binaria

Para encontrar un valor en una lista **desordenada** no queda otra que mirar elemento por elemento: $O(n)$. Pero si la lista está **ordenada**, se puede descartar la mitad en cada paso:

```rust
fn busqueda_binaria(v: &[i32], objetivo: i32) -> Option<usize> {
    let (mut izq, mut der) = (0, v.len());
    while izq < der {
        let medio = izq + (der - izq) / 2;
        if v[medio] == objetivo {
            return Some(medio);
        } else if v[medio] < objetivo {
            izq = medio + 1;
        } else {
            der = medio;
        }
    }
    None
}
```

> [!NOTE]
> ¿Cuántas comparaciones hacen falta, como máximo, para buscar en una lista ordenada de 1 000 000 de elementos?
>
> Cada paso divide el rango por 2, así que hacen falta $\lceil \log_2 10^6 \rceil = 20$ pasos. La búsqueda lineal podría necesitar un millón.

## Mejor caso, peor caso y espacio

Por defecto se informa el **peor caso**, que garantiza un límite. A veces se usa el **caso promedio** (quicksort es $O(n \log n)$ en promedio y $O(n^2)$ en el peor caso).

La misma notación mide la **memoria** adicional que usa un algoritmo (complejidad espacial). Es habitual poder cambiar una por otra: guardar resultados ya calculados gasta memoria pero ahorra tiempo.

> [!WARNING]
> Big O describe el comportamiento para $n$ grande. Con pocos datos, un algoritmo $O(n^2)$ simple puede ganarle a uno $O(n \log n)$ con constantes altas. Medí antes de optimizar.
