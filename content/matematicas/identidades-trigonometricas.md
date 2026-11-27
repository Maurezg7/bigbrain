+++
title = "Identidades y ecuaciones trigonométricas"
summary = "Las identidades fundamentales, suma de ángulos, ángulo doble, funciones inversas, ecuaciones trigonométricas y teoremas del seno y del coseno."
branch = "Geometría y trigonometría"
order = 2
level = "Intermedio"
tags = ["trigonometría", "identidades", "teorema del coseno", "ecuaciones trigonométricas"]
+++

## Identidades fundamentales

Una **identidad** es una igualdad que vale para todo ángulo donde ambos miembros estén definidos.

> [!IMPORTANT]
> $$\sin^2 x + \cos^2 x = 1$$

Dividiendo por $\cos^2 x$ o por $\sin^2 x$:

$$1 + \tan^2 x = \sec^2 x \qquad 1 + \cot^2 x = \csc^2 x$$

donde $\sec x = \dfrac{1}{\cos x}$, $\csc x = \dfrac{1}{\sin x}$, $\cot x = \dfrac{\cos x}{\sin x}$.

### Paridad y desfases

$$\sin(-x) = -\sin x \qquad \cos(-x) = \cos x \qquad \tan(-x) = -\tan x$$

$$\sin\left(\tfrac{\pi}{2} - x\right) = \cos x \qquad \cos\left(\tfrac{\pi}{2} - x\right) = \sin x$$

$$\sin(\pi - x) = \sin x \qquad \cos(\pi - x) = -\cos x$$

## Suma y resta de ángulos

$$\sin(a \pm b) = \sin a \cos b \pm \cos a \sin b$$

$$\cos(a \pm b) = \cos a \cos b \mp \sin a \sin b$$

$$\tan(a \pm b) = \frac{\tan a \pm \tan b}{1 \mp \tan a \tan b}$$

> [!NOTE]
> Valor exacto de $\sin 75°$.
>
> $\sin(45° + 30°) = \dfrac{\sqrt{2}}{2} \cdot \dfrac{\sqrt{3}}{2} + \dfrac{\sqrt{2}}{2} \cdot \dfrac{1}{2} = \dfrac{\sqrt{6} + \sqrt{2}}{4} \approx 0{,}966$

## Ángulo doble y ángulo mitad

Haciendo $a = b$ en las fórmulas anteriores:

$$\sin 2x = 2 \sin x \cos x$$

$$\cos 2x = \cos^2 x - \sin^2 x = 2\cos^2 x - 1 = 1 - 2\sin^2 x$$

Despejando salen las fórmulas de **reducción de potencia**, indispensables para integrar:

$$\sin^2 x = \frac{1 - \cos 2x}{2} \qquad \cos^2 x = \frac{1 + \cos 2x}{2}$$

## Producto a suma

$$\sin a \cos b = \tfrac{1}{2}[\sin(a + b) + \sin(a - b)]$$

$$\cos a \cos b = \tfrac{1}{2}[\cos(a - b) + \cos(a + b)]$$

$$\sin a \sin b = \tfrac{1}{2}[\cos(a - b) - \cos(a + b)]$$

Explican los batidos del sonido y la modulación de señales.

## Suma de seno y coseno de igual frecuencia

$$A\sin x + B\cos x = R \sin(x + \varphi), \qquad R = \sqrt{A^2 + B^2}, \quad \tan\varphi = \frac{B}{A}$$

La suma de dos sinusoides de la misma frecuencia es otra sinusoide: es la base de los fasores.

## Funciones inversas

| Función | Dominio | Imagen |
|---|---|---|
| $\arcsin x$ | $[-1, 1]$ | $[-\pi/2, \pi/2]$ |
| $\arccos x$ | $[-1, 1]$ | $[0, \pi]$ |
| $\arctan x$ | $\mathbb{R}$ | $(-\pi/2, \pi/2)$ |

> [!WARNING]
> La calculadora devuelve **un solo** ángulo. $\sin x = 0{,}5$ tiene dos soluciones por vuelta: $30°$ y $150°$. Y $\sin^{-1} x$ significa $\arcsin x$, no $1/\sin x$.

## Ecuaciones trigonométricas

Soluciones generales, con $k \in \mathbb{Z}$:

| Ecuación | Soluciones |
|---|---|
| $\sin x = \sin\alpha$ | $x = \alpha + 2k\pi$ o $x = \pi - \alpha + 2k\pi$ |
| $\cos x = \cos\alpha$ | $x = \pm\alpha + 2k\pi$ |
| $\tan x = \tan\alpha$ | $x = \alpha + k\pi$ |

> [!NOTE]
> Resolver $2\cos^2 x - \cos x - 1 = 0$ en $[0, 2\pi)$.
>
> Con $u = \cos x$: $2u^2 - u - 1 = 0 \Rightarrow u = 1$ o $u = -\tfrac{1}{2}$.
>
> $\cos x = 1 \Rightarrow x = 0$. $\cos x = -\tfrac{1}{2} \Rightarrow x = \tfrac{2\pi}{3}$ o $x = \tfrac{4\pi}{3}$.

> [!WARNING]
> No dividir por $\sin x$ o $\cos x$: se pierden soluciones. En $\sin x \cos x = \sin x$ hay que factorizar: $\sin x(\cos x - 1) = 0$.

## Triángulos cualesquiera

En un triángulo de lados $a, b, c$ opuestos a los ángulos $A, B, C$:

> [!IMPORTANT]
> **Teorema del seno**: $\dfrac{a}{\sin A} = \dfrac{b}{\sin B} = \dfrac{c}{\sin C}$
>
> **Teorema del coseno**: $c^2 = a^2 + b^2 - 2ab\cos C$

| Datos | Usar |
|---|---|
| Dos ángulos y un lado | seno |
| Dos lados y el ángulo comprendido | coseno |
| Tres lados | coseno |
| Dos lados y un ángulo opuesto | seno (puede haber dos soluciones) |

**Área**: $\text{Área} = \tfrac{1}{2} ab \sin C$.

> [!NOTE]
> Dos lados miden 5 y 8, y forman un ángulo de $60°$. El tercer lado:
>
> $c^2 = 25 + 64 - 2 \cdot 5 \cdot 8 \cdot 0{,}5 = 49 \Rightarrow c = 7$.

> [!TIP]
> No hace falta memorizar todo: de $\sin^2 + \cos^2 = 1$ y de las fórmulas de suma se deducen casi todas las demás.
