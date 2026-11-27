+++
title = "Ecuaciones lineales de segundo orden"
summary = "Ecuaciones homogéneas con coeficientes constantes, solución particular, oscilador amortiguado y resonancia."
branch = "Ecuaciones diferenciales"
order = 2
level = "Avanzado"
tags = ["segundo orden", "ecuación característica", "oscilador", "resonancia"]
+++

## Por qué importan

La segunda ley de Newton ($F = m\ddot{x}$) y los circuitos con bobina y capacitor llevan a ecuaciones de segundo orden. La más común:

$$a\,y'' + b\,y' + c\,y = g(x)$$

con $a, b, c$ constantes. Si $g = 0$ es **homogénea**.

> [!IMPORTANT]
> La solución general es $y = y_h + y_p$: la solución de la homogénea (con dos constantes arbitrarias) más una solución particular cualquiera.

## Ecuación homogénea

Se prueba $y = e^{rx}$. Sustituyendo en $ay'' + by' + cy = 0$ queda la **ecuación característica**:

$$ar^2 + br + c = 0$$

| Raíces | Solución general |
|---|---|
| Reales distintas $r_1 \neq r_2$ | $y = C_1e^{r_1x} + C_2e^{r_2x}$ |
| Real doble $r$ | $y = (C_1 + C_2x)e^{rx}$ |
| Complejas $\alpha \pm \beta i$ | $y = e^{\alpha x}(C_1\cos\beta x + C_2\sin\beta x)$ |

> [!NOTE]
> $y'' - 5y' + 6y = 0$: $r^2 - 5r + 6 = 0 \Rightarrow r = 2, 3$. Solución: $y = C_1e^{2x} + C_2e^{3x}$.

> [!NOTE]
> $y'' + 4y = 0$: $r = \pm 2i$. Solución: $y = C_1\cos 2x + C_2\sin 2x$. Es una oscilación.

Las constantes se determinan con dos **condiciones iniciales**: $y(0)$ e $y'(0)$.

## Solución particular: coeficientes indeterminados

Se propone una $y_p$ con la misma forma que $g(x)$ y coeficientes a determinar.

| $g(x)$ | Proponer $y_p$ |
|---|---|
| Polinomio de grado $n$ | polinomio de grado $n$ completo |
| $ke^{mx}$ | $Ae^{mx}$ |
| $k\cos\omega x$ o $k\sin\omega x$ | $A\cos\omega x + B\sin\omega x$ |
| Producto de los anteriores | producto de las formas |

> [!WARNING]
> Si la propuesta ya es solución de la homogénea, hay que **multiplicarla por $x$** (o por $x^2$ si la raíz es doble). Es lo que ocurre en la resonancia.

> [!NOTE]
> $y'' - 5y' + 6y = 12$. Proponiendo $y_p = A$: $6A = 12 \Rightarrow A = 2$.
>
> $y = C_1e^{2x} + C_2e^{3x} + 2$.

> [!NOTE]
> $y'' + y = e^{2x}$. Proponiendo $y_p = Ae^{2x}$: $4A + A = 1 \Rightarrow A = \tfrac{1}{5}$.
>
> $y = C_1\cos x + C_2\sin x + \tfrac{1}{5}e^{2x}$.

Para funciones $g$ más generales se usa **variación de parámetros**.

## El oscilador amortiguado

Una masa $m$ con resorte $k$ y amortiguador $c$:

$$m\ddot{x} + c\dot{x} + kx = 0$$

Con $\omega_0 = \sqrt{k/m}$ (frecuencia natural) y $\zeta = \dfrac{c}{2\sqrt{km}}$ (factor de amortiguamiento):

$$\ddot{x} + 2\zeta\omega_0\dot{x} + \omega_0^2x = 0$$

| Caso | Condición | Comportamiento |
|---|---|---|
| Sin amortiguamiento | $\zeta = 0$ | oscila para siempre |
| Subamortiguado | $0 < \zeta < 1$ | oscila con amplitud decreciente |
| Crítico | $\zeta = 1$ | vuelve al equilibrio lo más rápido posible, sin oscilar |
| Sobreamortiguado | $\zeta > 1$ | vuelve lentamente, sin oscilar |

En el caso subamortiguado:

$$x(t) = Ae^{-\zeta\omega_0t}\cos(\omega_dt - \varphi), \qquad \omega_d = \omega_0\sqrt{1 - \zeta^2}$$

La suspensión de un auto se diseña cerca del amortiguamiento crítico.

### Analogía con el circuito RLC

$$L\ddot{q} + R\dot{q} + \frac{1}{C}q = V(t)$$

| Mecánico | Eléctrico |
|---|---|
| masa $m$ | inductancia $L$ |
| amortiguamiento $c$ | resistencia $R$ |
| rigidez $k$ | $1/C$ |
| posición $x$ | carga $q$ |
| fuerza $F$ | tensión $V$ |

## Oscilaciones forzadas y resonancia

Con una fuerza externa periódica:

$$m\ddot{x} + c\dot{x} + kx = F_0\cos\omega t$$

La solución tiene dos partes: un **transitorio** (la homogénea, que se extingue) y un **régimen permanente** que oscila a la frecuencia de la fuerza, con amplitud

$$X = \frac{F_0/k}{\sqrt{(1 - r^2)^2 + (2\zeta r)^2}}, \qquad r = \frac{\omega}{\omega_0}$$

> [!IMPORTANT]
> **Resonancia**: cuando $\omega \approx \omega_0$, la amplitud se hace máxima. Sin amortiguamiento crecería sin límite.

Sin amortiguamiento y con $\omega = \omega_0$, la solución particular es $x_p = \dfrac{F_0}{2m\omega_0}\,t\sin\omega_0t$: la amplitud crece linealmente con el tiempo.

La resonancia es deseable al sintonizar una radio y peligrosa en puentes, edificios y máquinas rotativas.

## Ecuación de Euler-Cauchy

$$x^2y'' + axy' + by = 0$$

Se resuelve probando $y = x^r$, lo que da $r(r - 1) + ar + b = 0$.

## Principio de superposición

Para ecuaciones **lineales**:

- La suma de soluciones de la homogénea es solución.
- Si $g = g_1 + g_2$, una solución particular es $y_{p1} + y_{p2}$.

Por eso se puede descomponer una entrada complicada en senoides (Fourier) y sumar las respuestas.

> [!TIP]
> Las raíces de la ecuación característica lo dicen todo: la parte real indica si la solución crece o decae; la parte imaginaria, si oscila y con qué frecuencia.
