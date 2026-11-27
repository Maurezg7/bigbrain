+++
title = "Transformaciones lineales"
summary = "Qué es una transformación lineal, su matriz asociada, núcleo e imagen, y las transformaciones geométricas del plano."
branch = "Álgebra lineal"
order = 4
level = "Avanzado"
tags = ["transformación lineal", "núcleo", "imagen", "rotación"]
+++

## Definición

> [!IMPORTANT]
> $T: V \to W$ es una **transformación lineal** si para todo $\vec{u}, \vec{v} \in V$ y todo escalar $k$:
> $$T(\vec{u} + \vec{v}) = T(\vec{u}) + T(\vec{v}) \qquad T(k\vec{u}) = k\,T(\vec{u})$$

Consecuencias: $T(\vec{0}) = \vec{0}$, y $T$ preserva combinaciones lineales.

| Transformación | ¿Lineal? |
|---|---|
| $T(x, y) = (2x + y, x - 3y)$ | sí |
| $T(x, y) = (x + 1, y)$ | no: $T(\vec{0}) \neq \vec{0}$ |
| $T(x, y) = (x^2, y)$ | no |
| Derivada $D(p) = p'$ en polinomios | sí |
| Integral $\int_0^x f$ | sí |

## Matriz asociada

Toda transformación lineal $T: \mathbb{R}^n \to \mathbb{R}^m$ es multiplicar por una matriz:

$$T(\vec{x}) = A\vec{x}$$

> [!IMPORTANT]
> Las **columnas** de $A$ son las imágenes de los vectores de la base canónica: $A = [\,T(\vec{e}_1) \;\; T(\vec{e}_2) \;\; \cdots \;\; T(\vec{e}_n)\,]$.

Basta saber qué le pasa a la base para conocer toda la transformación.

> [!NOTE]
> $T(x, y) = (2x + y, x - 3y)$. $T(1, 0) = (2, 1)$ y $T(0, 1) = (1, -3)$:
>
> $$A = \begin{pmatrix} 2 & 1 \\ 1 & -3 \end{pmatrix}$$

## Transformaciones del plano

| Transformación | Matriz |
|---|---|
| Escalado por $a$ en $x$ y $b$ en $y$ | $\begin{pmatrix} a & 0 \\ 0 & b \end{pmatrix}$ |
| Rotación de ángulo $\theta$ | $\begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix}$ |
| Reflexión respecto del eje $x$ | $\begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix}$ |
| Reflexión respecto de $y = x$ | $\begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix}$ |
| Proyección sobre el eje $x$ | $\begin{pmatrix} 1 & 0 \\ 0 & 0 \end{pmatrix}$ |
| Cizalla horizontal | $\begin{pmatrix} 1 & k \\ 0 & 1 \end{pmatrix}$ |

> [!NOTE]
> Rotar $(1, 0)$ un ángulo de $90°$: $\begin{pmatrix} 0 & -1 \\ 1 & 0 \end{pmatrix}\begin{pmatrix} 1 \\ 0 \end{pmatrix} = \begin{pmatrix} 0 \\ 1 \end{pmatrix}$.

La traslación $T(\vec{x}) = \vec{x} + \vec{b}$ **no** es lineal. En gráficos por computadora se la incorpora con **coordenadas homogéneas** (matrices de $3 \times 3$ para el plano).

## Composición

Aplicar $T_1$ y luego $T_2$ equivale a multiplicar las matrices **en orden inverso**:

$$(T_2 \circ T_1)(\vec{x}) = A_2 A_1\vec{x}$$

Por eso el producto de matrices se define como se define, y por eso no es conmutativo: rotar y luego reflejar no es lo mismo que reflejar y luego rotar.

## Núcleo e imagen

> [!IMPORTANT]
> - **Núcleo**: $\text{Nu}(T) = \{\vec{v} : T(\vec{v}) = \vec{0}\}$, los vectores que $T$ anula.
> - **Imagen**: $\text{Im}(T) = \{T(\vec{v})\}$, los vectores que $T$ alcanza.

Ambos son subespacios. La imagen es el espacio columna de $A$ y el núcleo, su espacio nulo.

**Teorema de la dimensión**:

$$\dim \text{Nu}(T) + \dim \text{Im}(T) = \dim V$$

Lo que se «pierde» en el núcleo es lo que le falta a la imagen.

| Propiedad | Condición |
|---|---|
| Inyectiva | $\text{Nu}(T) = \{\vec{0}\}$ |
| Sobreyectiva | $\text{Im}(T) = W$ |
| Biyectiva (isomorfismo) | ambas; la matriz es invertible |

> [!NOTE]
> Proyección sobre el plano $xy$: $T(x, y, z) = (x, y, 0)$.
>
> Núcleo: el eje $z$ (dimensión 1). Imagen: el plano $xy$ (dimensión 2). $1 + 2 = 3$. ✓
>
> No es inyectiva: todos los puntos de una recta vertical tienen la misma imagen.

## Cambio de base y semejanza

La matriz de $T$ depende de la base elegida. Si $A$ es la matriz en la base canónica y $P$ tiene por columnas a los vectores de una base $B$:

$$[T]_B = P^{-1}AP$$

Dos matrices relacionadas así son **semejantes**: representan la **misma transformación** vista desde bases distintas. Comparten determinante, traza, rango y autovalores.

La pregunta natural es: ¿en qué base la matriz es lo más simple posible? La respuesta lleva a los autovectores.

## Aplicaciones

- **Gráficos 3D**: rotar, escalar y proyectar objetos son productos de matrices.
- **Robótica**: la posición de cada eslabón se obtiene componiendo transformaciones.
- **Sistemas lineales**: resolver $A\vec{x} = \vec{b}$ es preguntar qué vector tiene imagen $\vec{b}$.
- **Ecuaciones diferenciales**: derivar es un operador lineal, y por eso vale el principio de superposición.
- **Circuitos**: en una red lineal, la respuesta a una suma de fuentes es la suma de las respuestas.

> [!WARNING]
> «Lineal» aquí no significa «recta»: $f(x) = 2x + 3$ es una recta, pero no es transformación lineal porque $f(0) \neq 0$.

> [!TIP]
> Una matriz no es solo una tabla de números: es una función que transforma el espacio. Sus columnas dicen adónde van los ejes.
