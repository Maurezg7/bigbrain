+++
title = "Determinantes y matriz inversa"
summary = "Cálculo de determinantes, sus propiedades, la matriz inversa, la regla de Cramer y el rango."
branch = "Álgebra lineal"
order = 2
level = "Intermedio"
tags = ["determinante", "matriz inversa", "Cramer", "rango"]
+++

## Qué es el determinante

A cada matriz **cuadrada** $A$ se le asocia un número, $\det A$ o $|A|$, que indica si la matriz es invertible y cuánto escala áreas o volúmenes.

### Orden 2

> [!IMPORTANT]
> $$\begin{vmatrix} a & b \\ c & d \end{vmatrix} = ad - bc$$

### Orden 3: regla de Sarrus

$$\begin{vmatrix} a & b & c \\ d & e & f \\ g & h & i \end{vmatrix} = aei + bfg + cdh - ceg - afh - bdi$$

### Orden $n$: desarrollo por cofactores

Se elige una fila (o columna) y se suma cada elemento por su **cofactor**:

$$\det A = \sum_{j=1}^{n} a_{ij}\,C_{ij}, \qquad C_{ij} = (-1)^{i+j} M_{ij}$$

donde $M_{ij}$ es el determinante que queda al tachar la fila $i$ y la columna $j$. Conviene elegir la línea con más ceros.

> [!NOTE]
> $$\begin{vmatrix} 2 & 0 & 1 \\ 1 & 3 & 2 \\ 0 & 1 & 4 \end{vmatrix} = 2\begin{vmatrix} 3 & 2 \\ 1 & 4 \end{vmatrix} - 0 + 1\begin{vmatrix} 1 & 3 \\ 0 & 1 \end{vmatrix} = 2(10) + 1(1) = 21$$

## Propiedades

| Operación | Efecto en el determinante |
|---|---|
| Intercambiar dos filas | cambia de signo |
| Multiplicar una fila por $k$ | se multiplica por $k$ |
| Sumar a una fila un múltiplo de otra | no cambia |
| Trasponer | no cambia: $\det A^T = \det A$ |
| Dos filas iguales o proporcionales | vale $0$ |
| Una fila de ceros | vale $0$ |
| Matriz triangular | producto de la diagonal |

Además:

$$\det(AB) = \det A \cdot \det B \qquad \det(A^{-1}) = \frac{1}{\det A} \qquad \det(kA) = k^n\det A$$

> [!WARNING]
> $\det(A + B) \neq \det A + \det B$ en general.

Método práctico para matrices grandes: **triangular** con operaciones de fila (eliminación de Gauss) y multiplicar la diagonal, cuidando los cambios de signo.

## Interpretación geométrica

- En $2 \times 2$: $|\det A|$ es el **área del paralelogramo** formado por las columnas.
- En $3 \times 3$: es el **volumen del paralelepípedo**.
- Como transformación, $A$ multiplica las áreas (o volúmenes) por $|\det A|$. Si $\det A < 0$, invierte la orientación.
- Si $\det A = 0$, la transformación «aplasta» el espacio a una dimensión menor.

## Matriz inversa

$A^{-1}$ es la matriz que cumple $A A^{-1} = A^{-1} A = I$.

> [!IMPORTANT]
> $A$ es invertible si y solo si $\det A \neq 0$.

### Orden 2

$$A = \begin{pmatrix} a & b \\ c & d \end{pmatrix} \;\Longrightarrow\; A^{-1} = \frac{1}{ad - bc}\begin{pmatrix} d & -b \\ -c & a \end{pmatrix}$$

### Método de Gauss-Jordan

Se escribe $[A \mid I]$ y se aplican operaciones de fila hasta obtener $[I \mid A^{-1}]$.

### Por la adjunta

$$A^{-1} = \frac{1}{\det A}\,\text{adj}(A)$$

donde $\text{adj}(A)$ es la traspuesta de la matriz de cofactores.

> [!NOTE]
> $A = \begin{pmatrix} 2 & 1 \\ 5 & 3 \end{pmatrix}$: $\det A = 1$, así que $A^{-1} = \begin{pmatrix} 3 & -1 \\ -5 & 2 \end{pmatrix}$.
>
> El sistema $A\vec{x} = \vec{b}$ con $\vec{b} = (4, 11)$ se resuelve como $\vec{x} = A^{-1}\vec{b} = (1, 2)$.

Propiedades: $(AB)^{-1} = B^{-1}A^{-1}$ y $(A^T)^{-1} = (A^{-1})^T$.

## Regla de Cramer

Para un sistema $A\vec{x} = \vec{b}$ de $n$ ecuaciones y $n$ incógnitas con $\det A \neq 0$:

$$x_i = \frac{\det A_i}{\det A}$$

donde $A_i$ es $A$ con la columna $i$ reemplazada por $\vec{b}$.

> [!NOTE]
> $\begin{cases} 2x + y = 5 \\ x - 3y = -1 \end{cases}$
>
> $\det A = -7$; $\det A_x = \begin{vmatrix} 5 & 1 \\ -1 & -3 \end{vmatrix} = -14$; $\det A_y = \begin{vmatrix} 2 & 5 \\ 1 & -1 \end{vmatrix} = -7$.
>
> $x = 2$, $y = 1$.

Es práctica para $2 \times 2$ y $3 \times 3$; para sistemas grandes, Gauss es mucho más eficiente.

## Rango

El **rango** de una matriz es el número de filas (o columnas) linealmente independientes. Equivale al número de pivotes tras escalonar, o al orden del mayor menor no nulo.

**Teorema de Rouché-Frobenius** para $A\vec{x} = \vec{b}$ con $n$ incógnitas:

| Condición | Sistema |
|---|---|
| $\text{rg}(A) = \text{rg}(A \mid b) = n$ | compatible determinado (solución única) |
| $\text{rg}(A) = \text{rg}(A \mid b) < n$ | compatible indeterminado (infinitas) |
| $\text{rg}(A) < \text{rg}(A \mid b)$ | incompatible (sin solución) |

## Equivalencias

Para una matriz cuadrada $A$ de orden $n$, son equivalentes:

- $\det A \neq 0$.
- $A$ es invertible.
- $\text{rg}(A) = n$.
- Las columnas son linealmente independientes.
- $A\vec{x} = \vec{b}$ tiene solución única para todo $\vec{b}$.
- $A\vec{x} = \vec{0}$ solo tiene la solución trivial.
- $0$ no es autovalor de $A$.

> [!TIP]
> El determinante responde una sola pregunta, pero es la más importante: ¿esta matriz pierde información o no?
