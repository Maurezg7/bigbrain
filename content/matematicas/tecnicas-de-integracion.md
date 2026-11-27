+++
title = "Técnicas de integración"
summary = "Sustitución, integración por partes, fracciones simples, integrales trigonométricas, sustitución trigonométrica e integrales impropias."
branch = "Cálculo"
order = 5
level = "Intermedio"
tags = ["integrales", "sustitución", "integración por partes", "fracciones simples"]
+++

## Por qué hacen falta técnicas

Derivar es mecánico; integrar no. No hay una regla del producto ni de la cadena para integrales, sino métodos que transforman una integral en otra más simple.

### Integrales inmediatas

| $f(x)$ | $\int f(x)\,dx$ |
|---|---|
| $x^n$, $n \neq -1$ | $\dfrac{x^{n+1}}{n+1} + C$ |
| $\dfrac{1}{x}$ | $\ln\lvert x \rvert + C$ |
| $e^x$ | $e^x + C$ |
| $a^x$ | $\dfrac{a^x}{\ln a} + C$ |
| $\sin x$ | $-\cos x + C$ |
| $\cos x$ | $\sin x + C$ |
| $\sec^2 x$ | $\tan x + C$ |
| $\dfrac{1}{1 + x^2}$ | $\arctan x + C$ |
| $\dfrac{1}{\sqrt{1 - x^2}}$ | $\arcsin x + C$ |

## Sustitución

Es la regla de la cadena al revés. Si aparece una función y su derivada:

> [!IMPORTANT]
> $$\int f(g(x))\,g'(x)\,dx = \int f(u)\,du, \qquad u = g(x)$$

> [!NOTE]
> $\displaystyle\int 2x\cos(x^2)\,dx$. Con $u = x^2$, $du = 2x\,dx$:
>
> $\displaystyle\int \cos u\,du = \sin u + C = \sin(x^2) + C$.

> [!NOTE]
> $\displaystyle\int \frac{x}{x^2 + 1}\,dx$. Con $u = x^2 + 1$, $du = 2x\,dx$:
>
> $\displaystyle\frac{1}{2}\int \frac{du}{u} = \frac{1}{2}\ln(x^2 + 1) + C$.

En integrales definidas hay que **cambiar los límites** a valores de $u$, o volver a $x$ antes de evaluar.

## Integración por partes

Es la regla del producto al revés:

> [!IMPORTANT]
> $$\int u\,dv = uv - \int v\,du$$

Para elegir $u$ sirve la regla **ILATE**: se toma como $u$ la primera que aparezca entre **I**nversas trigonométricas, **L**ogaritmos, **A**lgebraicas, **T**rigonométricas, **E**xponenciales.

> [!NOTE]
> $\displaystyle\int x e^x\,dx$. Con $u = x$, $dv = e^x dx$: $du = dx$, $v = e^x$.
>
> $\displaystyle x e^x - \int e^x\,dx = e^x(x - 1) + C$.

> [!NOTE]
> $\displaystyle\int \ln x\,dx$. Con $u = \ln x$, $dv = dx$: $du = \dfrac{dx}{x}$, $v = x$.
>
> $\displaystyle x\ln x - \int dx = x\ln x - x + C$.

A veces hay que aplicarla dos veces, o la integral original reaparece y se despeja, como en $\int e^x \sin x\,dx$.

## Fracciones simples

Para integrar un cociente de polinomios $\dfrac{P(x)}{Q(x)}$:

1. Si $\operatorname{gr}(P) \ge \operatorname{gr}(Q)$, dividir primero.
2. Factorizar $Q$.
3. Descomponer según los factores:

| Factor de $Q$ | Términos |
|---|---|
| $(x - a)$ | $\dfrac{A}{x - a}$ |
| $(x - a)^2$ | $\dfrac{A}{x - a} + \dfrac{B}{(x - a)^2}$ |
| $x^2 + bx + c$ irreducible | $\dfrac{Ax + B}{x^2 + bx + c}$ |

> [!NOTE]
> $\displaystyle\int \frac{dx}{x^2 - 1}$. Se descompone $\dfrac{1}{(x - 1)(x + 1)} = \dfrac{1/2}{x - 1} - \dfrac{1/2}{x + 1}$.
>
> $\displaystyle\frac{1}{2}\ln\left|\frac{x - 1}{x + 1}\right| + C$.

## Integrales trigonométricas

- **Potencia impar** de seno o coseno: separar un factor y usar $\sin^2 x + \cos^2 x = 1$.
- **Potencias pares**: usar reducción de potencia.

$$\int \sin^2 x\,dx = \int \frac{1 - \cos 2x}{2}\,dx = \frac{x}{2} - \frac{\sin 2x}{4} + C$$

> [!NOTE]
> $\displaystyle\int \sin^3 x\,dx = \int (1 - \cos^2 x)\sin x\,dx$. Con $u = \cos x$:
>
> $\displaystyle -\int (1 - u^2)\,du = -\cos x + \frac{\cos^3 x}{3} + C$.

## Sustitución trigonométrica

| Aparece | Sustitución | Identidad |
|---|---|---|
| $\sqrt{a^2 - x^2}$ | $x = a\sin\theta$ | $1 - \sin^2\theta = \cos^2\theta$ |
| $\sqrt{a^2 + x^2}$ | $x = a\tan\theta$ | $1 + \tan^2\theta = \sec^2\theta$ |
| $\sqrt{x^2 - a^2}$ | $x = a\sec\theta$ | $\sec^2\theta - 1 = \tan^2\theta$ |

Con ella se obtiene, por ejemplo, el área del círculo: $\displaystyle\int_{-r}^{r} 2\sqrt{r^2 - x^2}\,dx = \pi r^2$.

## Integrales impropias

Tienen un límite infinito o un integrando no acotado. Se definen como límite:

$$\int_1^{\infty} \frac{dx}{x^2} = \lim_{b \to \infty} \left[-\frac{1}{x}\right]_1^b = 1$$

Si el límite existe, la integral **converge**; si no, **diverge**.

> [!IMPORTANT]
> $\displaystyle\int_1^{\infty} \frac{dx}{x^p}$ converge si $p > 1$ y diverge si $p \le 1$.

## Qué método usar

| Si el integrando tiene | Probar |
|---|---|
| Una función y su derivada | sustitución |
| Producto de tipos distintos ($x e^x$, $x\ln x$) | partes |
| Cociente de polinomios | fracciones simples |
| Potencias de seno y coseno | identidades trigonométricas |
| Raíces de $a^2 \pm x^2$ | sustitución trigonométrica |

> [!WARNING]
> No toda función elemental tiene primitiva elemental: $\int e^{-x^2}dx$ y $\int \dfrac{\sin x}{x}dx$ no se expresan con funciones conocidas. Se calculan con series o métodos numéricos.

> [!TIP]
> Siempre se puede verificar una integral derivando el resultado.
