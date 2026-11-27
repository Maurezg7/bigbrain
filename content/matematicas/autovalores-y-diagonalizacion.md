+++
title = "Autovalores y diagonalización"
summary = "Autovalores y autovectores, polinomio característico, diagonalización de matrices y sus aplicaciones."
branch = "Álgebra lineal"
order = 5
level = "Avanzado"
tags = ["autovalores", "autovectores", "diagonalización", "polinomio característico"]
+++

## La idea

En general, una matriz cambia la dirección de los vectores. Pero hay direcciones especiales que **no cambian**: el vector solo se estira o se encoge.

> [!IMPORTANT]
> $\vec{v} \neq \vec{0}$ es **autovector** de $A$ con **autovalor** $\lambda$ si
> $$A\vec{v} = \lambda\vec{v}$$

- $\lambda > 1$: estira. $0 < \lambda < 1$: encoge. $\lambda < 0$: invierte el sentido. $\lambda = 0$: aplasta.
- Si $\vec{v}$ es autovector, cualquier múltiplo suyo también.

## Cómo calcularlos

$A\vec{v} = \lambda\vec{v}$ equivale a $(A - \lambda I)\vec{v} = \vec{0}$. Para que exista solución no nula:

> [!IMPORTANT]
> **Ecuación característica**: $\det(A - \lambda I) = 0$

1. Resolver la ecuación característica para hallar los autovalores.
2. Para cada $\lambda$, resolver $(A - \lambda I)\vec{v} = \vec{0}$ para hallar los autovectores.

> [!NOTE]
> $A = \begin{pmatrix} 4 & 1 \\ 2 & 3 \end{pmatrix}$.
>
> $\det\begin{pmatrix} 4 - \lambda & 1 \\ 2 & 3 - \lambda \end{pmatrix} = \lambda^2 - 7\lambda + 10 = 0 \Rightarrow \lambda_1 = 5,\ \lambda_2 = 2$.
>
> Para $\lambda = 5$: $-x + y = 0 \Rightarrow \vec{v}_1 = (1, 1)$.
>
> Para $\lambda = 2$: $2x + y = 0 \Rightarrow \vec{v}_2 = (1, -2)$.
>
> Comprobación: $A(1, 1) = (5, 5) = 5(1, 1)$. ✓

### Propiedades

$$\lambda_1 + \lambda_2 + \dots + \lambda_n = \text{tr}(A) \qquad \lambda_1\lambda_2\cdots\lambda_n = \det A$$

- Para una matriz $2 \times 2$: $\lambda^2 - \text{tr}(A)\lambda + \det A = 0$.
- Matriz triangular: los autovalores son los elementos de la diagonal.
- $A$ es invertible si y solo si ningún autovalor es $0$.
- $A^k$ tiene autovalores $\lambda^k$; $A^{-1}$ tiene $1/\lambda$. Los autovectores son los mismos.
- Autovectores de autovalores **distintos** son linealmente independientes.

Los autovalores pueden ser complejos: una rotación de $90°$ no deja fija ninguna dirección real, y sus autovalores son $\pm i$.

## Diagonalización

Si $A$ de orden $n$ tiene $n$ autovectores linealmente independientes, se arma $P$ con ellos como columnas y $D$ con los autovalores en la diagonal:

> [!IMPORTANT]
> $$A = PDP^{-1}$$

En la base de autovectores, la transformación es simplemente un escalado en cada eje.

| Caso | ¿Diagonalizable? |
|---|---|
| $n$ autovalores distintos | sí |
| Matriz simétrica real | sí, siempre |
| Autovalores repetidos | depende: hace falta suficientes autovectores |

> [!NOTE]
> La matriz $\begin{pmatrix} 1 & 1 \\ 0 & 1 \end{pmatrix}$ tiene $\lambda = 1$ doble pero un solo autovector, $(1, 0)$. **No** es diagonalizable.

### Potencias de matrices

$$A^k = PD^kP^{-1}, \qquad D^k = \begin{pmatrix} \lambda_1^k & & \\ & \ddots & \\ & & \lambda_n^k \end{pmatrix}$$

Elevar una diagonal es trivial. Con el ejemplo anterior, $A^{10}$ solo requiere calcular $5^{10}$ y $2^{10}$.

## Matrices simétricas

**Teorema espectral**: si $A = A^T$ (real):

- Todos sus autovalores son **reales**.
- Autovectores de autovalores distintos son **ortogonales**.
- Se diagonaliza con una matriz ortogonal: $A = QDQ^T$.

De ahí su importancia: las matrices de inercia, de tensiones, de covarianza y de rigidez son simétricas.

## Aplicaciones

### Sistemas dinámicos

En $\vec{x}_{k+1} = A\vec{x}_k$, la solución es $\vec{x}_k = c_1\lambda_1^k\vec{v}_1 + \dots + c_n\lambda_n^k\vec{v}_n$. El autovalor de mayor módulo domina a largo plazo:

- $|\lambda| < 1$ para todos: el sistema tiende a cero (**estable**).
- Algún $|\lambda| > 1$: crece sin límite (**inestable**).

### Ecuaciones diferenciales

En $\vec{x}\,' = A\vec{x}$, la solución es $\vec{x}(t) = c_1 e^{\lambda_1 t}\vec{v}_1 + \dots$ Es estable si todos los autovalores tienen **parte real negativa**.

### Otras

| Área | Autovalores | Autovectores |
|---|---|---|
| Vibraciones | frecuencias naturales al cuadrado | modos de vibración |
| Resistencia de materiales | tensiones principales | direcciones principales |
| Mecánica | momentos principales de inercia | ejes principales |
| Estadística (PCA) | varianza explicada | componentes principales |
| Mecánica cuántica | valores medibles de la energía | estados estacionarios |
| Buscadores web | — | el autovector dominante ordena las páginas |
| Cadenas de Markov | $\lambda = 1$ | distribución estacionaria |

> [!WARNING]
> El vector nulo nunca es autovector, aunque $A\vec{0} = \lambda\vec{0}$ para todo $\lambda$. En cambio $\lambda = 0$ sí puede ser autovalor.

> [!TIP]
> Los autovectores son los ejes naturales de una transformación. Mirado desde ellos, un problema acoplado se separa en problemas independientes de una variable.
