+++
title = "Cálculo vectorial"
summary = "Campos vectoriales, divergencia y rotor, integrales de línea y de superficie, y los teoremas de Green, Gauss y Stokes."
branch = "Cálculo en varias variables"
order = 3
level = "Avanzado"
tags = ["campos vectoriales", "divergencia", "rotor", "Stokes", "Gauss"]
+++

## Campos

- **Campo escalar**: asigna un número a cada punto. Ejemplos: temperatura, presión, potencial.
- **Campo vectorial**: asigna un vector a cada punto. Ejemplos: velocidad de un fluido, campo eléctrico, gravedad.

$$\vec{F}(x, y, z) = (P, Q, R)$$

## El operador nabla

$$\nabla = \left(\frac{\partial}{\partial x}, \frac{\partial}{\partial y}, \frac{\partial}{\partial z}\right)$$

| Operación | Fórmula | Actúa sobre | Da | Significado |
|---|---|---|---|---|
| Gradiente | $\nabla f$ | escalar | vector | dirección de máximo crecimiento |
| Divergencia | $\nabla \cdot \vec{F}$ | vector | escalar | cuánto «brota» el campo de un punto |
| Rotor | $\nabla \times \vec{F}$ | vector | vector | cuánto «gira» el campo alrededor de un punto |
| Laplaciano | $\nabla^2 f = \nabla \cdot \nabla f$ | escalar | escalar | diferencia con el promedio vecino |

$$\nabla \cdot \vec{F} = \frac{\partial P}{\partial x} + \frac{\partial Q}{\partial y} + \frac{\partial R}{\partial z}$$

$$\nabla \times \vec{F} = \begin{vmatrix} \hat{\imath} & \hat{\jmath} & \hat{k} \\ \partial_x & \partial_y & \partial_z \\ P & Q & R \end{vmatrix}$$

Dos identidades siempre válidas: $\nabla \times (\nabla f) = \vec{0}$ y $\nabla \cdot (\nabla \times \vec{F}) = 0$.

> [!NOTE]
> Para $\vec{F} = (x, y, z)$: $\nabla \cdot \vec{F} = 3$ (el campo se expande) y $\nabla \times \vec{F} = \vec{0}$.
>
> Para $\vec{F} = (-y, x, 0)$: $\nabla \cdot \vec{F} = 0$ y $\nabla \times \vec{F} = (0, 0, 2)$ (el campo gira alrededor del eje $z$).

## Integral de línea

Suma un campo a lo largo de una curva $C$ parametrizada por $\vec{r}(t)$, $a \le t \le b$:

> [!IMPORTANT]
> $$\int_C \vec{F} \cdot d\vec{r} = \int_a^b \vec{F}(\vec{r}(t)) \cdot \vec{r}\,'(t)\,dt$$

Físicamente es el **trabajo** de la fuerza $\vec{F}$ a lo largo del camino. Sobre una curva cerrada se llama **circulación** y se escribe $\oint$.

> [!NOTE]
> Trabajo de $\vec{F} = (y, x)$ sobre $\vec{r}(t) = (t, t^2)$, $0 \le t \le 1$.
>
> $\vec{F} = (t^2, t)$, $\vec{r}\,' = (1, 2t)$. $\displaystyle\int_0^1 (t^2 + 2t^2)\,dt = 1$.

### Campos conservativos

Un campo es **conservativo** si $\vec{F} = \nabla f$ para alguna función potencial $f$. Entonces:

$$\int_C \vec{F} \cdot d\vec{r} = f(B) - f(A)$$

El trabajo **no depende del camino**, solo de los extremos, y la circulación en cualquier curva cerrada es cero.

Criterio (en una región sin agujeros): $\vec{F}$ es conservativo si y solo si $\nabla \times \vec{F} = \vec{0}$.

En el ejemplo anterior, $\vec{F} = \nabla(xy)$, así que el trabajo es $f(1,1) - f(0,0) = 1$. ✓

## Integral de superficie y flujo

El **flujo** de $\vec{F}$ a través de una superficie $S$ mide cuánto campo la atraviesa:

$$\Phi = \iint_S \vec{F} \cdot \hat{n}\,dS$$

Para una superficie parametrizada $\vec{r}(u, v)$: $\hat{n}\,dS = (\vec{r}_u \times \vec{r}_v)\,du\,dv$.

## Los grandes teoremas

Todos dicen lo mismo: **la integral de una derivada sobre una región es igual a la integral de la función sobre su borde**.

### Teorema de Green (plano)

$$\oint_C P\,dx + Q\,dy = \iint_D \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA$$

con $C$ recorrida en sentido antihorario. Permite calcular áreas: $A = \tfrac{1}{2}\oint_C x\,dy - y\,dx$.

### Teorema de la divergencia (Gauss)

> [!IMPORTANT]
> $$\oiint_S \vec{F} \cdot \hat{n}\,dS = \iiint_V \nabla \cdot \vec{F}\,dV$$

El flujo que sale por una superficie cerrada es igual a la suma de las «fuentes» en su interior.

> [!NOTE]
> Flujo de $\vec{F} = (x, y, z)$ a través de una esfera de radio $R$: como $\nabla \cdot \vec{F} = 3$,
>
> $\Phi = 3 \cdot \dfrac{4}{3}\pi R^3 = 4\pi R^3$. Sin calcular ninguna integral de superficie.

### Teorema de Stokes

> [!IMPORTANT]
> $$\oint_C \vec{F} \cdot d\vec{r} = \iint_S (\nabla \times \vec{F}) \cdot \hat{n}\,dS$$

La circulación sobre el borde es igual al flujo del rotor a través de la superficie.

| Teorema | Relaciona | Dimensión |
|---|---|---|
| Fundamental del cálculo | integral de $f'$ con valores en los extremos | 1 |
| Campos conservativos | integral de línea de $\nabla f$ con valores en los extremos | curva |
| Green | integral doble con circulación en el borde | plano |
| Stokes | flujo del rotor con circulación en el borde | superficie |
| Gauss | integral triple de la divergencia con flujo en el borde | volumen |

## Aplicación: ecuaciones de Maxwell

| Ley | Forma diferencial |
|---|---|
| Gauss eléctrica | $\nabla \cdot \vec{E} = \rho/\varepsilon_0$ |
| Gauss magnética | $\nabla \cdot \vec{B} = 0$ |
| Faraday | $\nabla \times \vec{E} = -\partial\vec{B}/\partial t$ |
| Ampère-Maxwell | $\nabla \times \vec{B} = \mu_0\vec{J} + \mu_0\varepsilon_0\,\partial\vec{E}/\partial t$ |

Los teoremas de Gauss y Stokes convierten estas formas diferenciales en las integrales. También son la base de la mecánica de fluidos (ecuación de continuidad) y la transferencia de calor.

> [!WARNING]
> La orientación importa: invertir el sentido de la curva o de la normal cambia el signo. En Stokes, curva y normal se relacionan por la regla de la mano derecha.

> [!TIP]
> Divergencia: ¿hay fuentes o sumideros? Rotor: ¿hay remolinos? Con esas dos preguntas se entiende cualquier campo.
