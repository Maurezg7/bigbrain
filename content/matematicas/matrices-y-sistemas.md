+++
title = "Matrices y sistemas lineales"
summary = "Operar con matrices, calcular determinantes y resolver sistemas por eliminación de Gauss."
branch = "Álgebra lineal"
order = 1
level = "Intermedio"
tags = ["matrices", "determinante", "Gauss"]
+++

## Matrices

Una **matriz** $m \times n$ es una tabla de números con $m$ filas y $n$ columnas. Se suman elemento a elemento (si tienen el mismo tamaño) y se multiplican por un escalar multiplicando cada elemento.

### Producto de matrices

$AB$ existe solo si $A$ tiene tantas columnas como filas tiene $B$. El elemento $(i, j)$ del resultado es el producto escalar de la fila $i$ de $A$ por la columna $j$ de $B$:

$$\begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}\begin{pmatrix} 5 & 6 \\ 7 & 8 \end{pmatrix} = \begin{pmatrix} 1\cdot5 + 2\cdot7 & 1\cdot6 + 2\cdot8 \\ 3\cdot5 + 4\cdot7 & 3\cdot6 + 4\cdot8 \end{pmatrix} = \begin{pmatrix} 19 & 22 \\ 43 & 50 \end{pmatrix}$$

> [!WARNING]
> El producto de matrices **no es conmutativo**: en general $AB \neq BA$.

## Determinante e inversa

Para una matriz $2 \times 2$:

$$\det\begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc$$

Una matriz cuadrada tiene **inversa** $A^{-1}$ (con $AA^{-1} = I$) si y solo si $\det A \neq 0$. En el caso $2\times 2$:

$$A^{-1} = \frac{1}{ad - bc}\begin{pmatrix} d & -b \\ -c & a \end{pmatrix}$$

Geométricamente, $|\det A|$ es el factor por el que la transformación $A$ escala las áreas (o volúmenes).

## Sistemas de ecuaciones lineales

Todo sistema lineal se escribe como $A\mathbf{x} = \mathbf{b}$. La **eliminación de Gauss** lo lleva a forma triangular usando tres operaciones que no cambian las soluciones: intercambiar filas, multiplicar una fila por un número no nulo y sumarle a una fila un múltiplo de otra.

> [!NOTE]
> Resolver $\;x + y = 5,\quad 2x - y = 1$.
>
> $$\left(\begin{array}{cc|c} 1 & 1 & 5 \\ 2 & -1 & 1 \end{array}\right) \xrightarrow{F_2 - 2F_1} \left(\begin{array}{cc|c} 1 & 1 & 5 \\ 0 & -3 & -9 \end{array}\right)$$
>
> De la segunda fila $y = 3$; sustituyendo hacia atrás, $x = 5 - 3 = 2$.

## Cuántas soluciones hay

Según el teorema de Rouché-Frobenius, comparando el rango de $A$ con el de la matriz ampliada $(A|\mathbf{b})$ y el número de incógnitas $n$:

| Caso | Tipo de sistema |
|---|---|
| rango $A$ = rango $(A\vert\mathbf{b})$ = $n$ | compatible determinado (solución única) |
| rango $A$ = rango $(A\vert\mathbf{b})$ < $n$ | compatible indeterminado (infinitas soluciones) |
| rango $A$ < rango $(A\vert\mathbf{b})$ | incompatible (sin solución) |
