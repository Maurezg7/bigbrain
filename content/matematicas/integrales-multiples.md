+++
title = "Integrales múltiples"
summary = "Integrales dobles y triples, cambio de orden, coordenadas polares, cilíndricas y esféricas, y sus aplicaciones."
branch = "Cálculo en varias variables"
order = 2
level = "Avanzado"
tags = ["integral doble", "integral triple", "jacobiano", "coordenadas polares"]
+++

## Integral doble

La integral simple suma áreas bajo una curva. La **integral doble** suma volúmenes bajo una superficie $z = f(x, y)$ sobre una región $R$ del plano:

$$\iint_R f(x, y)\,dA$$

Si $f = 1$, el resultado es el **área** de $R$.

### Integrales iteradas

**Teorema de Fubini**: se calcula integrando primero respecto de una variable y luego de la otra. Sobre un rectángulo $[a, b] \times [c, d]$:

$$\iint_R f\,dA = \int_a^b \left[\int_c^d f(x, y)\,dy\right]dx$$

> [!NOTE]
> $\displaystyle\int_0^2 \int_0^1 xy^2\,dy\,dx = \int_0^2 x\left[\frac{y^3}{3}\right]_0^1 dx = \int_0^2 \frac{x}{3}\,dx = \frac{2}{3}$.

### Regiones generales

Si $R$ está entre $y = g_1(x)$ e $y = g_2(x)$, para $a \le x \le b$:

$$\iint_R f\,dA = \int_a^b \int_{g_1(x)}^{g_2(x)} f(x, y)\,dy\,dx$$

> [!IMPORTANT]
> Los límites de la integral **interior** pueden depender de la variable exterior. Los de la **exterior** son siempre constantes.

> [!NOTE]
> Integrar $f = x + y$ sobre el triángulo de vértices $(0,0)$, $(1,0)$, $(1,1)$: $0 \le y \le x$, $0 \le x \le 1$.
>
> $$\int_0^1 \int_0^x (x + y)\,dy\,dx = \int_0^1 \left[xy + \frac{y^2}{2}\right]_0^x dx = \int_0^1 \frac{3x^2}{2}\,dx = \frac{1}{2}$$

### Cambio del orden de integración

A veces una integral imposible en un orden es fácil en el otro. Hay que **redibujar la región** y describirla al revés.

$$\int_0^1 \int_x^1 e^{y^2}dy\,dx = \int_0^1 \int_0^y e^{y^2}dx\,dy = \int_0^1 y e^{y^2}dy = \frac{e - 1}{2}$$

## Coordenadas polares

Para regiones circulares: $x = r\cos\theta$, $y = r\sin\theta$.

> [!IMPORTANT]
> $$dA = r\,dr\,d\theta$$

> [!WARNING]
> No olvidar el factor $r$. Es el **jacobiano** del cambio: un sector polar es más grande cuanto más lejos está del origen.

> [!NOTE]
> Volumen bajo el paraboloide $z = 4 - x^2 - y^2$ sobre el plano $z = 0$. La base es el disco $r \le 2$.
>
> $$V = \int_0^{2\pi}\int_0^2 (4 - r^2)\,r\,dr\,d\theta = 2\pi\left[2r^2 - \frac{r^4}{4}\right]_0^2 = 8\pi$$

La integral gaussiana se resuelve así: $\displaystyle\int_{-\infty}^{\infty} e^{-x^2}dx = \sqrt{\pi}$.

## Integral triple

$$\iiint_E f(x, y, z)\,dV$$

Con $f = 1$ da el **volumen** de $E$; con $f = \rho$ (densidad), la **masa**.

### Coordenadas cilíndricas

Polares en el plano más la altura $z$. Para cilindros, conos y paraboloides.

$$x = r\cos\theta, \quad y = r\sin\theta, \quad z = z \qquad dV = r\,dz\,dr\,d\theta$$

### Coordenadas esféricas

Para esferas. Con $\rho$ la distancia al origen, $\phi$ el ángulo desde el eje $z$ y $\theta$ el ángulo en el plano $xy$:

$$x = \rho\sin\phi\cos\theta, \quad y = \rho\sin\phi\sin\theta, \quad z = \rho\cos\phi$$

$$dV = \rho^2\sin\phi\,d\rho\,d\phi\,d\theta$$

> [!NOTE]
> Volumen de una esfera de radio $R$:
>
> $$V = \int_0^{2\pi}\int_0^{\pi}\int_0^R \rho^2\sin\phi\,d\rho\,d\phi\,d\theta = 2\pi \cdot 2 \cdot \frac{R^3}{3} = \frac{4}{3}\pi R^3$$

| Sistema | Elemento de volumen | Conviene para |
|---|---|---|
| Cartesianas | $dx\,dy\,dz$ | cajas, regiones planas |
| Cilíndricas | $r\,dz\,dr\,d\theta$ | simetría alrededor de un eje |
| Esféricas | $\rho^2\sin\phi\,d\rho\,d\phi\,d\theta$ | simetría respecto de un punto |

## Cambio de variables general

Para un cambio $x = x(u, v)$, $y = y(u, v)$:

$$dA = \left|\frac{\partial(x, y)}{\partial(u, v)}\right| du\,dv, \qquad \frac{\partial(x, y)}{\partial(u, v)} = \begin{vmatrix} x_u & x_v \\ y_u & y_v \end{vmatrix}$$

El **jacobiano** mide cuánto se estira o encoge el área con el cambio.

## Aplicaciones

| Magnitud | Fórmula |
|---|---|
| Masa | $M = \displaystyle\iint \rho\,dA$ |
| Centro de masa | $\bar{x} = \dfrac{1}{M}\displaystyle\iint x\rho\,dA$ |
| Momento de inercia respecto del eje $z$ | $I_z = \displaystyle\iint (x^2 + y^2)\rho\,dA$ |
| Valor medio | $\bar{f} = \dfrac{1}{\text{Área}}\displaystyle\iint f\,dA$ |
| Probabilidad conjunta | $P = \displaystyle\iint f(x, y)\,dA$ |

> [!NOTE]
> Momento de inercia de un disco homogéneo de masa $M$ y radio $R$ respecto de su eje. Con $\sigma = \dfrac{M}{\pi R^2}$:
>
> $$I = \int_0^{2\pi}\int_0^R r^2\,\sigma\,r\,dr\,d\theta = 2\pi\sigma\frac{R^4}{4} = \frac{1}{2}MR^2$$

> [!TIP]
> Lo difícil no es integrar sino **plantear los límites**. Siempre dibujar la región y elegir las coordenadas que respeten su simetría.
