+++
title = "Vectores"
summary = "Vectores en el plano y el espacio: componentes, módulo, producto escalar, producto vectorial, proyecciones, rectas y planos."
branch = "Geometría y trigonometría"
order = 4
level = "Intermedio"
tags = ["vectores", "producto escalar", "producto vectorial", "planos"]
+++

## Qué es un vector

Una magnitud **escalar** queda definida por un número (masa, temperatura). Una magnitud **vectorial** necesita además dirección y sentido (fuerza, velocidad).

> [!IMPORTANT]
> Un vector se representa por sus componentes: $\vec{v} = (v_x, v_y, v_z) = v_x \hat{\imath} + v_y \hat{\jmath} + v_z \hat{k}$

El vector que va de $A$ a $B$ es $\overrightarrow{AB} = B - A$.

**Módulo** (longitud):

$$|\vec{v}| = \sqrt{v_x^2 + v_y^2 + v_z^2}$$

Un **versor** o vector unitario tiene módulo 1: $\hat{v} = \dfrac{\vec{v}}{|\vec{v}|}$.

En el plano, si el vector forma un ángulo $\theta$ con el eje $x$: $v_x = |\vec{v}|\cos\theta$, $v_y = |\vec{v}|\sin\theta$.

## Operaciones lineales

- **Suma**: componente a componente. Gráficamente, regla del paralelogramo.
- **Producto por un escalar**: $k\vec{v} = (kv_x, kv_y, kv_z)$. Cambia el módulo y, si $k < 0$, el sentido.

Dos vectores son **paralelos** si uno es múltiplo del otro.

## Producto escalar

> [!IMPORTANT]
> $$\vec{u} \cdot \vec{v} = u_x v_x + u_y v_y + u_z v_z = |\vec{u}|\,|\vec{v}|\cos\theta$$

El resultado es un **número**. Sirve para:

- **Ángulo entre vectores**: $\cos\theta = \dfrac{\vec{u} \cdot \vec{v}}{|\vec{u}|\,|\vec{v}|}$
- **Perpendicularidad**: $\vec{u} \perp \vec{v} \iff \vec{u} \cdot \vec{v} = 0$
- **Módulo**: $\vec{v} \cdot \vec{v} = |\vec{v}|^2$
- **Trabajo** de una fuerza: $W = \vec{F} \cdot \vec{d}$

### Proyección

La proyección de $\vec{u}$ sobre $\vec{v}$ es la «sombra» de $\vec{u}$ en la dirección de $\vec{v}$:

$$\text{proy}_{\vec{v}}\,\vec{u} = \frac{\vec{u} \cdot \vec{v}}{|\vec{v}|^2}\,\vec{v}$$

> [!NOTE]
> Ángulo entre $\vec{u} = (1, 2, 2)$ y $\vec{v} = (3, 0, 4)$.
>
> $\vec{u} \cdot \vec{v} = 3 + 0 + 8 = 11$; $|\vec{u}| = 3$; $|\vec{v}| = 5$.
>
> $\cos\theta = \dfrac{11}{15} \Rightarrow \theta \approx 42{,}8°$.

## Producto vectorial

Solo existe en tres dimensiones. El resultado es un **vector** perpendicular a ambos.

$$\vec{u} \times \vec{v} = \begin{vmatrix} \hat{\imath} & \hat{\jmath} & \hat{k} \\ u_x & u_y & u_z \\ v_x & v_y & v_z \end{vmatrix} = (u_y v_z - u_z v_y,\; u_z v_x - u_x v_z,\; u_x v_y - u_y v_x)$$

- **Módulo**: $|\vec{u} \times \vec{v}| = |\vec{u}|\,|\vec{v}|\sin\theta$, que es el **área del paralelogramo** que forman.
- **Sentido**: regla de la mano derecha.
- **Anticonmutativo**: $\vec{u} \times \vec{v} = -\vec{v} \times \vec{u}$.
- $\vec{u} \times \vec{v} = \vec{0}$ si y solo si son paralelos.

Aplicaciones: momento de una fuerza $\vec{M} = \vec{r} \times \vec{F}$, fuerza magnética $\vec{F} = q\vec{v} \times \vec{B}$.

> [!NOTE]
> $\vec{u} = (1, 0, 2)$, $\vec{v} = (0, 3, 1)$.
>
> $\vec{u} \times \vec{v} = (0 \cdot 1 - 2 \cdot 3,\; 2 \cdot 0 - 1 \cdot 1,\; 1 \cdot 3 - 0 \cdot 0) = (-6, -1, 3)$.
>
> Comprobación: $(-6, -1, 3) \cdot (1, 0, 2) = -6 + 6 = 0$. ✓

### Producto mixto

$$\vec{u} \cdot (\vec{v} \times \vec{w}) = \det[\vec{u}, \vec{v}, \vec{w}]$$

Su valor absoluto es el **volumen del paralelepípedo**. Si vale cero, los tres vectores son coplanares.

| | Producto escalar | Producto vectorial |
|---|---|---|
| Resultado | número | vector |
| Depende de | $\cos\theta$ | $\sin\theta$ |
| Vale cero si | son perpendiculares | son paralelos |
| Conmutativo | sí | no (cambia el signo) |

## Rectas y planos en el espacio

**Recta** por $P_0$ con dirección $\vec{d}$:

$$\vec{r}(t) = P_0 + t\,\vec{d}$$

**Plano** por $P_0$ con normal $\vec{n} = (A, B, C)$:

$$\vec{n} \cdot (P - P_0) = 0 \quad\Longrightarrow\quad Ax + By + Cz + D = 0$$

> [!NOTE]
> Plano por $A(1, 0, 0)$, $B(0, 2, 0)$ y $C(0, 0, 3)$.
>
> $\overrightarrow{AB} = (-1, 2, 0)$, $\overrightarrow{AC} = (-1, 0, 3)$. Normal: $\overrightarrow{AB} \times \overrightarrow{AC} = (6, 3, 2)$.
>
> Plano: $6(x - 1) + 3y + 2z = 0 \Rightarrow 6x + 3y + 2z = 6$.

**Distancia de un punto a un plano**:

$$d = \frac{|Ax_0 + By_0 + Cz_0 + D|}{\sqrt{A^2 + B^2 + C^2}}$$

| Posición relativa | Condición |
|---|---|
| Planos paralelos | normales paralelas |
| Planos perpendiculares | $\vec{n}_1 \cdot \vec{n}_2 = 0$ |
| Recta paralela a un plano | $\vec{d} \cdot \vec{n} = 0$ |
| Recta perpendicular a un plano | $\vec{d}$ paralelo a $\vec{n}$ |

> [!WARNING]
> No existe la división de vectores, y $\vec{u} \cdot \vec{v} = \vec{u} \cdot \vec{w}$ no implica $\vec{v} = \vec{w}$.

> [!TIP]
> El producto escalar mide cuánto **coinciden** dos direcciones; el vectorial, cuánto **difieren**. Con los dos se resuelve casi toda la geometría del espacio.
