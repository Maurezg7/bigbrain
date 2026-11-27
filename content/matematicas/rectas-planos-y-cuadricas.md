+++
title = "Rectas, planos y cuádricas"
summary = "Ecuaciones de la recta y del plano en el espacio, posiciones relativas, distancias y ángulos, superficies cuádricas y coordenadas cilíndricas y esféricas."
branch = "Geometría y trigonometría"
order = 5
level = "Intermedio"
tags = ["rectas", "planos", "cuádricas", "geometría del espacio", "coordenadas esféricas"]
+++

## Rectas en el espacio

Una recta queda determinada por un punto $P_0 = (x_0, y_0, z_0)$ y un **vector director** $\mathbf{v} = (a, b, c)$.

| Forma | Ecuación |
|---|---|
| Vectorial | $\mathbf{r}(t) = P_0 + t\,\mathbf{v}$ |
| Paramétrica | $x = x_0 + at,\quad y = y_0 + bt,\quad z = z_0 + ct$ |
| Simétrica | $\dfrac{x - x_0}{a} = \dfrac{y - y_0}{b} = \dfrac{z - z_0}{c}$ |

La forma simétrica exige $a$, $b$ y $c$ no nulos.

> [!NOTE]
> **Recta por dos puntos.** Por $A = (1, 2, 3)$ y $B = (4, 0, 5)$: $\mathbf{v} = B - A = (3, -2, 2)$.
> $$x = 1 + 3t,\quad y = 2 - 2t,\quad z = 3 + 2t$$

En el espacio, una sola ecuación lineal no describe una recta, sino un plano. Una recta es la intersección de **dos** planos.

## Planos

Un plano queda determinado por un punto $P_0$ y un **vector normal** $\mathbf{n} = (A, B, C)$, perpendicular a él. Un punto $P$ pertenece al plano si $\mathbf{n} \cdot (P - P_0) = 0$:

$$A(x - x_0) + B(y - y_0) + C(z - z_0) = 0$$

Desarrollando se obtiene la **ecuación general**:

$$Ax + By + Cz + D = 0$$

Los coeficientes de $x$, $y$, $z$ son las componentes de la normal.

| Forma | Ecuación |
|---|---|
| General | $Ax + By + Cz + D = 0$ |
| Paramétrica | $P = P_0 + s\,\mathbf{u} + t\,\mathbf{w}$ |
| Segmentaria | $\dfrac{x}{p} + \dfrac{y}{q} + \dfrac{z}{r} = 1$ (corta a los ejes en $p$, $q$, $r$) |

### Plano por tres puntos

La normal es el **producto vectorial** de dos vectores del plano.

> [!NOTE]
> **Ejemplo.** Por $A = (1, 0, 0)$, $B = (0, 2, 0)$ y $C = (0, 0, 3)$:
>
> $\vec{AB} = (-1, 2, 0)$, $\vec{AC} = (-1, 0, 3)$
>
> $$\mathbf{n} = \vec{AB} \times \vec{AC} = (2 \cdot 3 - 0 \cdot 0,\ 0 \cdot (-1) - (-1) \cdot 3,\ (-1) \cdot 0 - 2 \cdot (-1)) = (6, 3, 2)$$
>
> Plano: $6(x - 1) + 3y + 2z = 0$, es decir, $6x + 3y + 2z = 6$.

## Posiciones relativas

### Dos planos

| Normales | Posición |
|---|---|
| paralelas, y las ecuaciones son proporcionales | coincidentes |
| paralelas, y no lo son | paralelos |
| no paralelas | se cortan en una recta, de dirección $\mathbf{n}_1 \times \mathbf{n}_2$ |

### Recta y plano

| Condición | Posición |
|---|---|
| $\mathbf{v} \cdot \mathbf{n} \ne 0$ | se cortan en un punto |
| $\mathbf{v} \cdot \mathbf{n} = 0$ y el punto de la recta no está en el plano | paralela |
| $\mathbf{v} \cdot \mathbf{n} = 0$ y el punto está en el plano | contenida |

> [!NOTE]
> **Intersección.** La recta $x = 1 + t$, $y = 2t$, $z = 3 - t$ con el plano $2x + y - z = 5$:
> $$2(1 + t) + 2t - (3 - t) = 5 \;\Rightarrow\; 5t - 1 = 5 \;\Rightarrow\; t = 1{,}2$$
> El punto es $(2{,}2;\ 2{,}4;\ 1{,}8)$.

### Dos rectas

| Posición | Condición |
|---|---|
| Paralelas | directores proporcionales |
| Secantes | se cortan en un punto |
| **Alabeadas** | ni se cortan ni son paralelas: no están en un mismo plano |

Dos rectas con directores $\mathbf{v}_1$, $\mathbf{v}_2$ y puntos $P_1$, $P_2$ son coplanares si y solo si el producto mixto se anula:

$$(P_2 - P_1) \cdot (\mathbf{v}_1 \times \mathbf{v}_2) = 0$$

## Ángulos

| Entre | Fórmula |
|---|---|
| Dos rectas | $\cos\theta = \dfrac{\lvert \mathbf{v}_1 \cdot \mathbf{v}_2 \rvert}{\lVert \mathbf{v}_1 \rVert \lVert \mathbf{v}_2 \rVert}$ |
| Dos planos | $\cos\theta = \dfrac{\lvert \mathbf{n}_1 \cdot \mathbf{n}_2 \rvert}{\lVert \mathbf{n}_1 \rVert \lVert \mathbf{n}_2 \rVert}$ |
| Recta y plano | $\sin\theta = \dfrac{\lvert \mathbf{v} \cdot \mathbf{n} \rvert}{\lVert \mathbf{v} \rVert \lVert \mathbf{n} \rVert}$ |

En el último caso aparece el seno porque la normal forma con la recta el ángulo complementario.

## Distancias

| Entre | Fórmula |
|---|---|
| Dos puntos | $\sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2 + (z_2 - z_1)^2}$ |
| Punto $P_1$ y plano | $\dfrac{\lvert A x_1 + B y_1 + C z_1 + D \rvert}{\sqrt{A^2 + B^2 + C^2}}$ |
| Punto $Q$ y recta | $\dfrac{\lVert (Q - P_0) \times \mathbf{v} \rVert}{\lVert \mathbf{v} \rVert}$ |
| Planos paralelos | $\dfrac{\lvert D_2 - D_1 \rvert}{\sqrt{A^2 + B^2 + C^2}}$ |
| Rectas alabeadas | $\dfrac{\lvert (P_2 - P_1) \cdot (\mathbf{v}_1 \times \mathbf{v}_2) \rvert}{\lVert \mathbf{v}_1 \times \mathbf{v}_2 \rVert}$ |

> [!NOTE]
> **Ejemplo.** Distancia de $(1, 2, 3)$ al plano $2x - y + 2z - 4 = 0$:
> $$d = \frac{\lvert 2 - 2 + 6 - 4 \rvert}{\sqrt{4 + 1 + 4}} = \frac{2}{3}$$

## Superficies cuádricas

Son las superficies de **segundo grado**: el análogo en el espacio de las cónicas. Con los ejes adecuados se reducen a formas canónicas.

| Superficie | Ecuación canónica | Rasgo |
|---|---|---|
| **Elipsoide** | $\dfrac{x^2}{a^2} + \dfrac{y^2}{b^2} + \dfrac{z^2}{c^2} = 1$ | cerrada; si $a = b = c$ es una esfera |
| **Hiperboloide de una hoja** | $\dfrac{x^2}{a^2} + \dfrac{y^2}{b^2} - \dfrac{z^2}{c^2} = 1$ | conexa; es una superficie reglada |
| **Hiperboloide de dos hojas** | $-\dfrac{x^2}{a^2} - \dfrac{y^2}{b^2} + \dfrac{z^2}{c^2} = 1$ | dos partes separadas |
| **Cono elíptico** | $\dfrac{x^2}{a^2} + \dfrac{y^2}{b^2} = \dfrac{z^2}{c^2}$ | pasa por el origen |
| **Paraboloide elíptico** | $z = \dfrac{x^2}{a^2} + \dfrac{y^2}{b^2}$ | forma de cuenco |
| **Paraboloide hiperbólico** | $z = \dfrac{x^2}{a^2} - \dfrac{y^2}{b^2}$ | silla de montar; reglada |
| Cilindro elíptico | $\dfrac{x^2}{a^2} + \dfrac{y^2}{b^2} = 1$ | falta una variable |
| Cilindro parabólico | $y = a x^2$ | falta una variable |

### Cómo reconocerlas

Se estudian las **trazas**: las curvas que resultan al cortar con planos paralelos a los coordenados ($z = k$, etc.).

| Signos de los términos cuadráticos | Superficie |
|---|---|
| tres positivos, igual a 1 | elipsoide |
| dos positivos y uno negativo, igual a 1 | hiperboloide de una hoja |
| uno positivo y dos negativos, igual a 1 | hiperboloide de dos hojas |
| dos positivos y uno negativo, igual a 0 | cono |
| una variable aparece solo en primer grado | paraboloide |
| falta una variable | cilindro |

> [!NOTE]
> **Ejemplo.** $4x^2 + y^2 - z^2 = 4$. Dividiendo por 4:
> $$x^2 + \frac{y^2}{4} - \frac{z^2}{4} = 1$$
> Hiperboloide de una hoja con eje $z$. Las trazas $z = k$ son elipses; las trazas $x = 0$ e $y = 0$, hipérbolas.

Si la ecuación tiene términos lineales, se **completan cuadrados** para hallar el centro o el vértice.

> [!NOTE]
> $x^2 + y^2 + z^2 - 2x + 4y - 4 = 0$ equivale a $(x - 1)^2 + (y + 2)^2 + z^2 = 9$: esfera de centro $(1, -2, 0)$ y radio 3.

### En ingeniería

| Superficie | Aplicación |
|---|---|
| Paraboloide de revolución | antenas y reflectores: concentra en el foco los rayos paralelos al eje |
| Hiperboloide de una hoja | torres de enfriamiento: se construye con barras rectas |
| Paraboloide hiperbólico | cubiertas de hormigón delgadas |
| Elipsoide | tanques, cúpulas, modelo de la forma de la Tierra |

## Coordenadas cilíndricas y esféricas

### Cilíndricas $(r, \theta, z)$

Coordenadas polares en el plano $xy$, más la altura.

$$x = r\cos\theta \qquad y = r\sin\theta \qquad z = z$$

$$r = \sqrt{x^2 + y^2} \qquad \tan\theta = \frac{y}{x}$$

### Esféricas $(\rho, \theta, \varphi)$

$\rho$ es la distancia al origen, $\theta$ el ángulo en el plano $xy$ (azimut) y $\varphi$ el ángulo medido desde el eje $z$ positivo.

$$x = \rho \sin\varphi \cos\theta \qquad y = \rho \sin\varphi \sin\theta \qquad z = \rho \cos\varphi$$

$$\rho = \sqrt{x^2 + y^2 + z^2} \qquad \cos\varphi = \frac{z}{\rho}$$

> [!WARNING]
> En física suele usarse la convención inversa: $\theta$ para el ángulo polar y $\varphi$ para el azimut. Verificar siempre cuál usa cada texto.

| Superficie | Cartesianas | Cilíndricas | Esféricas |
|---|---|---|---|
| Esfera | $x^2 + y^2 + z^2 = a^2$ | $r^2 + z^2 = a^2$ | $\rho = a$ |
| Cilindro | $x^2 + y^2 = a^2$ | $r = a$ | $\rho \sin\varphi = a$ |
| Cono | $z = \sqrt{x^2 + y^2}$ | $z = r$ | $\varphi = \pi/4$ |
| Paraboloide | $z = x^2 + y^2$ | $z = r^2$ | |

> [!NOTE]
> **Ejemplo.** El punto $(1, 1, \sqrt{2})$:
>
> - cilíndricas: $r = \sqrt{2}$, $\theta = \pi/4$, $z = \sqrt{2}$;
> - esféricas: $\rho = \sqrt{1 + 1 + 2} = 2$, $\theta = \pi/4$, $\cos\varphi = \sqrt{2}/2$, luego $\varphi = \pi/4$.

> [!TIP]
> Elegir las coordenadas según la simetría: cilíndricas si hay un eje de simetría, esféricas si hay un centro. Una buena elección convierte superficies complicadas en ecuaciones del tipo «coordenada = constante».
