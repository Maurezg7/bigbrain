+++
title = "Integrales"
summary = "La integral suma infinitas porciones infinitamente pequeñas: áreas, distancias, trabajo."
branch = "Cálculo"
order = 3
level = "Intermedio"
tags = ["integral", "área", "primitiva"]
+++

## El problema del área

Para medir el área bajo la curva $y = f(x)$ entre $a$ y $b$ la aproximamos con rectángulos de base $\Delta x$. Cuantos más rectángulos, mejor la aproximación. El límite de esa suma es la **integral definida**:

$$\int_a^b f(x)\,dx = \lim_{n \to \infty} \sum_{i=1}^{n} f(x_i)\,\Delta x$$

![Área bajo una curva aproximada con rectángulos](/media/matematicas/integral.svg)

*Suma de Riemann: la suma de las áreas de los rectángulos se acerca al área bajo la curva.*

## Primitivas

> [!IMPORTANT]
> $F$ es una **primitiva** (o antiderivada) de $f$ si $F'(x) = f(x)$. La integral indefinida es la familia de todas ellas: $\int f(x)\,dx = F(x) + C$.

| $f(x)$ | $\int f(x)\,dx$ |
|---|---|
| $x^n$ ($n \neq -1$) | $\dfrac{x^{n+1}}{n+1} + C$ |
| $1/x$ | $\ln \lvert x \rvert + C$ |
| $e^x$ | $e^x + C$ |
| $\sin x$ | $-\cos x + C$ |
| $\cos x$ | $\sin x + C$ |

## Teorema fundamental del cálculo

Derivar e integrar son operaciones inversas. Si $F$ es una primitiva de $f$ (continua en $[a,b]$):

$$\int_a^b f(x)\,dx = F(b) - F(a)$$

> [!NOTE]
> Área bajo $y = x^2$ entre $0$ y $3$:
>
> $$\int_0^3 x^2\,dx = \left[\frac{x^3}{3}\right]_0^3 = \frac{27}{3} - 0 = 9$$

## Métodos de integración

**Sustitución** (la regla de la cadena al revés): si aparece una función y su derivada, se toma $u = g(x)$, $du = g'(x)\,dx$.

$$\int 2x\cos(x^2)\,dx = \int \cos u\,du = \sin(x^2) + C$$

**Integración por partes** (la regla del producto al revés):

$$\int u\,dv = uv - \int v\,du$$

> [!NOTE]
> $\int x e^x\,dx$: con $u = x$ y $dv = e^x dx$ queda $du = dx$, $v = e^x$, y entonces
> $\int x e^x\,dx = x e^x - \int e^x\,dx = (x - 1)e^x + C$.

> [!WARNING]
> No olvides la constante $+C$ en las integrales indefinidas, y recordá que la integral definida da área **con signo**: las partes bajo el eje $x$ restan.
