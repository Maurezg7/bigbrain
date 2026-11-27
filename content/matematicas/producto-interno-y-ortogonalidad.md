+++
title = "Producto interno y ortogonalidad"
summary = "Norma, ortogonalidad, proyecciones, proceso de Gram-Schmidt y el método de mínimos cuadrados."
branch = "Álgebra lineal"
order = 6
level = "Avanzado"
tags = ["ortogonalidad", "Gram-Schmidt", "mínimos cuadrados", "proyección"]
+++

## Producto interno

En $\mathbb{R}^n$:

$$\langle\vec{u}, \vec{v}\rangle = \vec{u} \cdot \vec{v} = \vec{u}^T\vec{v} = u_1v_1 + \dots + u_nv_n$$

Con él se definen longitudes y ángulos en cualquier dimensión:

$$\|\vec{v}\| = \sqrt{\vec{v} \cdot \vec{v}} \qquad d(\vec{u}, \vec{v}) = \|\vec{u} - \vec{v}\| \qquad \cos\theta = \frac{\vec{u} \cdot \vec{v}}{\|\vec{u}\|\,\|\vec{v}\|}$$

La idea se extiende a funciones: $\langle f, g\rangle = \displaystyle\int_a^b f(x)g(x)\,dx$.

**Desigualdad de Cauchy-Schwarz**: $|\vec{u} \cdot \vec{v}| \le \|\vec{u}\|\,\|\vec{v}\|$.

**Desigualdad triangular**: $\|\vec{u} + \vec{v}\| \le \|\vec{u}\| + \|\vec{v}\|$.

## Ortogonalidad

> [!IMPORTANT]
> $\vec{u}$ y $\vec{v}$ son **ortogonales** si $\vec{u} \cdot \vec{v} = 0$.

- Conjunto **ortogonal**: sus vectores son ortogonales dos a dos.
- Conjunto **ortonormal**: ortogonal y además cada vector tiene norma 1.

Un conjunto ortogonal de vectores no nulos es siempre linealmente independiente.

### Por qué convienen las bases ortonormales

En una base cualquiera, hallar las coordenadas exige resolver un sistema. En una base ortonormal $\{\vec{q}_1, \dots, \vec{q}_n\}$ basta un producto interno:

$$\vec{v} = (\vec{v} \cdot \vec{q}_1)\vec{q}_1 + \dots + (\vec{v} \cdot \vec{q}_n)\vec{q}_n$$

### Matrices ortogonales

Una matriz cuadrada $Q$ con columnas ortonormales cumple $Q^TQ = I$, es decir $Q^{-1} = Q^T$. Representa rotaciones y reflexiones: conserva longitudes y ángulos, y $\det Q = \pm 1$.

## Proyección ortogonal

La proyección de $\vec{b}$ sobre la recta generada por $\vec{a}$:

$$\text{proy}_{\vec{a}}\,\vec{b} = \frac{\vec{a} \cdot \vec{b}}{\vec{a} \cdot \vec{a}}\,\vec{a}$$

Es el punto de la recta **más cercano** a $\vec{b}$. El error $\vec{b} - \text{proy}$ es perpendicular a $\vec{a}$.

Sobre un subespacio $W$ con base ortogonal $\{\vec{w}_1, \dots, \vec{w}_k\}$, se suman las proyecciones sobre cada vector.

El **complemento ortogonal** $W^\perp$ reúne los vectores perpendiculares a todo $W$, y $\dim W + \dim W^\perp = n$.

## Proceso de Gram-Schmidt

Convierte una base cualquiera $\{\vec{v}_1, \dots, \vec{v}_n\}$ en una ortogonal $\{\vec{u}_1, \dots, \vec{u}_n\}$: a cada vector se le resta su proyección sobre los anteriores.

$$\vec{u}_1 = \vec{v}_1$$

$$\vec{u}_2 = \vec{v}_2 - \frac{\vec{v}_2 \cdot \vec{u}_1}{\vec{u}_1 \cdot \vec{u}_1}\vec{u}_1$$

$$\vec{u}_3 = \vec{v}_3 - \frac{\vec{v}_3 \cdot \vec{u}_1}{\vec{u}_1 \cdot \vec{u}_1}\vec{u}_1 - \frac{\vec{v}_3 \cdot \vec{u}_2}{\vec{u}_2 \cdot \vec{u}_2}\vec{u}_2$$

Dividiendo cada $\vec{u}_i$ por su norma se obtiene una base ortonormal.

> [!NOTE]
> Ortogonalizar $\vec{v}_1 = (1, 1, 0)$, $\vec{v}_2 = (1, 0, 1)$.
>
> $\vec{u}_1 = (1, 1, 0)$.
>
> $\vec{u}_2 = (1, 0, 1) - \dfrac{1}{2}(1, 1, 0) = \left(\tfrac{1}{2}, -\tfrac{1}{2}, 1\right)$.
>
> Comprobación: $\vec{u}_1 \cdot \vec{u}_2 = \tfrac{1}{2} - \tfrac{1}{2} + 0 = 0$. ✓

Aplicado a las columnas de una matriz da la **factorización QR**: $A = QR$, con $Q$ de columnas ortonormales y $R$ triangular superior.

## Mínimos cuadrados

Un sistema con más ecuaciones que incógnitas, $A\vec{x} = \vec{b}$, normalmente no tiene solución. Se busca el $\hat{x}$ que **minimiza el error** $\|A\vec{x} - \vec{b}\|$.

La mejor aproximación es la proyección de $\vec{b}$ sobre el espacio columna de $A$. Exigir que el error sea perpendicular a las columnas da las:

> [!IMPORTANT]
> **Ecuaciones normales**: $A^TA\,\hat{x} = A^T\vec{b}$

### Ajuste de una recta

Para ajustar $y = c + mx$ a puntos $(x_i, y_i)$:

$$A = \begin{pmatrix} 1 & x_1 \\ 1 & x_2 \\ \vdots & \vdots \\ 1 & x_n \end{pmatrix}, \quad \vec{b} = \begin{pmatrix} y_1 \\ y_2 \\ \vdots \\ y_n \end{pmatrix}$$

> [!NOTE]
> Recta que mejor ajusta a $(0, 1)$, $(1, 3)$, $(2, 4)$.
>
> $A^TA = \begin{pmatrix} 3 & 3 \\ 3 & 5 \end{pmatrix}$, $A^T\vec{b} = \begin{pmatrix} 8 \\ 11 \end{pmatrix}$.
>
> Resolviendo $3c + 3m = 8$ y $3c + 5m = 11$: $m = 1{,}5$, $c \approx 1{,}17$.
>
> Recta: $y = 1{,}17 + 1{,}5x$.

El mismo método ajusta parábolas, exponenciales (tomando logaritmo) o cualquier modelo lineal en sus parámetros.

## Aplicaciones

- **Regresión** en estadística y ciencia de datos.
- **Series de Fourier**: senos y cosenos forman un conjunto ortogonal de funciones; cada coeficiente es una proyección.
- **Compresión** de imágenes y audio: se proyecta sobre una base adecuada y se descartan los coeficientes pequeños.
- **GPS**: la posición se estima por mínimos cuadrados a partir de más satélites que incógnitas.

> [!TIP]
> Proyectar es encontrar la mejor aproximación. Casi todo ajuste de datos, por complicado que parezca, es una proyección ortogonal.
