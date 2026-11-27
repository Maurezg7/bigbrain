+++
title = "Espacios vectoriales"
summary = "Subespacios, combinación lineal, independencia lineal, base, dimensión y cambio de base."
branch = "Álgebra lineal"
order = 3
level = "Avanzado"
tags = ["espacio vectorial", "base", "dimensión", "independencia lineal"]
+++

## Definición

Un **espacio vectorial** es un conjunto $V$ cuyos elementos se pueden **sumar** entre sí y **multiplicar por escalares**, cumpliendo las reglas habituales (conmutativa, asociativa, distributiva, existencia de vector nulo y de opuesto).

| Espacio | Elementos |
|---|---|
| $\mathbb{R}^n$ | $n$-uplas de números reales |
| $\mathbb{R}^{m \times n}$ | matrices de $m \times n$ |
| $P_n$ | polinomios de grado $\le n$ |
| $C[a, b]$ | funciones continuas en $[a, b]$ |

Los «vectores» no tienen por qué ser flechas: un polinomio o una función también lo son.

## Subespacios

> [!IMPORTANT]
> Un subconjunto $W \subseteq V$ es **subespacio** si:
> 1. Contiene al vector nulo.
> 2. Es cerrado para la suma: $\vec{u}, \vec{v} \in W \Rightarrow \vec{u} + \vec{v} \in W$.
> 3. Es cerrado para el producto por escalar: $\vec{u} \in W \Rightarrow k\vec{u} \in W$.

En $\mathbb{R}^3$ los subespacios son: el origen, las rectas por el origen, los planos por el origen y todo $\mathbb{R}^3$.

> [!NOTE]
> ¿Es subespacio $W = \{(x, y, z) : x + y + z = 1\}$? No: no contiene a $(0, 0, 0)$.
>
> En cambio $x + y + z = 0$ sí lo es (un plano por el origen).

## Combinación lineal y generadores

Una **combinación lineal** de $\vec{v}_1, \dots, \vec{v}_k$ es

$$c_1\vec{v}_1 + c_2\vec{v}_2 + \dots + c_k\vec{v}_k$$

El conjunto de todas las combinaciones lineales es el **subespacio generado**: $\text{gen}\{\vec{v}_1, \dots, \vec{v}_k\}$.

## Independencia lineal

> [!IMPORTANT]
> Los vectores $\vec{v}_1, \dots, \vec{v}_k$ son **linealmente independientes** si
> $$c_1\vec{v}_1 + \dots + c_k\vec{v}_k = \vec{0} \;\Longrightarrow\; c_1 = \dots = c_k = 0$$

Si son **dependientes**, alguno es combinación lineal de los demás: es redundante.

Para comprobarlo se arma una matriz con los vectores como columnas y se escalona: son independientes si hay un pivote en cada columna. Para $n$ vectores de $\mathbb{R}^n$, basta que el determinante sea distinto de cero.

> [!NOTE]
> ¿Son independientes $(1, 2, 3)$, $(0, 1, 2)$ y $(1, 3, 5)$?
>
> $\begin{vmatrix} 1 & 0 & 1 \\ 2 & 1 & 3 \\ 3 & 2 & 5 \end{vmatrix} = 1(5 - 6) - 0 + 1(4 - 3) = 0$.
>
> Son **dependientes**: el tercero es la suma de los dos primeros.

## Base y dimensión

> [!IMPORTANT]
> Una **base** de $V$ es un conjunto de vectores **linealmente independientes** que **generan** $V$.

- Todo vector se escribe de **forma única** como combinación lineal de la base. Los coeficientes son sus **coordenadas**.
- Todas las bases de un espacio tienen la misma cantidad de vectores: la **dimensión**.

| Espacio | Base canónica | Dimensión |
|---|---|---|
| $\mathbb{R}^3$ | $(1,0,0)$, $(0,1,0)$, $(0,0,1)$ | 3 |
| $P_2$ | $1$, $x$, $x^2$ | 3 |
| $\mathbb{R}^{2 \times 2}$ | las 4 matrices con un solo 1 | 4 |

En un espacio de dimensión $n$: más de $n$ vectores siempre son dependientes; menos de $n$ nunca generan todo el espacio.

## Los subespacios de una matriz

Para $A$ de $m \times n$:

| Subespacio | Definición | Dimensión |
|---|---|---|
| Espacio columna | generado por las columnas | $\text{rg}(A)$ |
| Espacio fila | generado por las filas | $\text{rg}(A)$ |
| Espacio nulo (núcleo) | soluciones de $A\vec{x} = \vec{0}$ | $n - \text{rg}(A)$ |

> [!IMPORTANT]
> **Teorema de la dimensión**: $\text{rg}(A) + \dim(\text{Nu}\,A) = n$ (número de columnas).

El sistema $A\vec{x} = \vec{b}$ tiene solución si y solo si $\vec{b}$ está en el espacio columna.

> [!NOTE]
> Base del núcleo de $A = \begin{pmatrix} 1 & 2 & 1 \\ 2 & 4 & 3 \end{pmatrix}$.
>
> Escalonando: $\begin{pmatrix} 1 & 2 & 0 \\ 0 & 0 & 1 \end{pmatrix}$. Entonces $z = 0$, $x = -2y$, con $y$ libre.
>
> Núcleo: $\text{gen}\{(-2, 1, 0)\}$, de dimensión $1 = 3 - 2$. ✓

## Cambio de base

Un mismo vector tiene coordenadas distintas en bases distintas. Si $B = \{\vec{b}_1, \dots, \vec{b}_n\}$ y $P$ es la matriz cuyas columnas son los $\vec{b}_i$:

$$\vec{v} = P\,[\vec{v}]_B \qquad [\vec{v}]_B = P^{-1}\vec{v}$$

> [!NOTE]
> Coordenadas de $\vec{v} = (3, 1)$ en la base $B = \{(1, 1), (1, -1)\}$.
>
> Hay que resolver $c_1(1, 1) + c_2(1, -1) = (3, 1)$: $c_1 + c_2 = 3$, $c_1 - c_2 = 1 \Rightarrow c_1 = 2$, $c_2 = 1$.
>
> $[\vec{v}]_B = (2, 1)$.

Elegir bien la base simplifica los problemas: es la idea detrás de la diagonalización y de las series de Fourier.

## Suma e intersección de subespacios

- $U \cap W$ es subespacio; $U \cup W$ en general **no**.
- $U + W = \{\vec{u} + \vec{w}\}$ es subespacio.
- $\dim(U + W) = \dim U + \dim W - \dim(U \cap W)$.
- Si $U \cap W = \{\vec{0}\}$, la suma es **directa**: $U \oplus W$.

> [!TIP]
> Una base es el conjunto mínimo de «direcciones» con que se describe todo el espacio, sin redundancia. La dimensión cuenta los grados de libertad.
