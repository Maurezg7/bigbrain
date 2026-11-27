+++
title = "Transformada de Laplace"
summary = "Definición, tabla de transformadas, propiedades y resolución de ecuaciones diferenciales con condiciones iniciales."
branch = "Ecuaciones diferenciales"
order = 3
level = "Avanzado"
tags = ["Laplace", "transformada", "función de transferencia", "ecuaciones diferenciales"]
+++

## La idea

La transformada de Laplace convierte una ecuación **diferencial** en una ecuación **algebraica**. Se resuelve con álgebra y se vuelve atrás.

> [!IMPORTANT]
> $$\mathcal{L}\{f(t)\} = F(s) = \int_0^{\infty} e^{-st}f(t)\,dt$$

Pasa una función del tiempo $t$ a una función de la variable compleja $s$.

> [!NOTE]
> $\mathcal{L}\{1\} = \displaystyle\int_0^{\infty} e^{-st}dt = \left[-\frac{e^{-st}}{s}\right]_0^{\infty} = \frac{1}{s}$, para $s > 0$.

## Tabla de transformadas

| $f(t)$ | $F(s)$ |
|---|---|
| $1$ | $\dfrac{1}{s}$ |
| $t$ | $\dfrac{1}{s^2}$ |
| $t^n$ | $\dfrac{n!}{s^{n+1}}$ |
| $e^{at}$ | $\dfrac{1}{s - a}$ |
| $\sin\omega t$ | $\dfrac{\omega}{s^2 + \omega^2}$ |
| $\cos\omega t$ | $\dfrac{s}{s^2 + \omega^2}$ |
| $e^{at}\sin\omega t$ | $\dfrac{\omega}{(s - a)^2 + \omega^2}$ |
| $e^{at}\cos\omega t$ | $\dfrac{s - a}{(s - a)^2 + \omega^2}$ |
| $t\,e^{at}$ | $\dfrac{1}{(s - a)^2}$ |
| $u(t - a)$ (escalón) | $\dfrac{e^{-as}}{s}$ |
| $\delta(t)$ (impulso) | $1$ |

## Propiedades

| Propiedad | En el tiempo | En $s$ |
|---|---|---|
| Linealidad | $af + bg$ | $aF + bG$ |
| Derivada | $f'(t)$ | $sF(s) - f(0)$ |
| Derivada segunda | $f''(t)$ | $s^2F(s) - sf(0) - f'(0)$ |
| Integral | $\int_0^t f$ | $F(s)/s$ |
| Desplazamiento en $s$ | $e^{at}f(t)$ | $F(s - a)$ |
| Desplazamiento en $t$ | $f(t - a)u(t - a)$ | $e^{-as}F(s)$ |
| Multiplicar por $t$ | $t\,f(t)$ | $-F'(s)$ |
| Convolución | $(f * g)(t)$ | $F(s)\,G(s)$ |

La propiedad clave: **derivar se convierte en multiplicar por $s$**, y las condiciones iniciales se incorporan solas.

## Transformada inversa

Para volver al tiempo se descompone $F(s)$ en **fracciones simples** y se lee la tabla al revés.

> [!NOTE]
> $F(s) = \dfrac{1}{s(s + 2)} = \dfrac{1/2}{s} - \dfrac{1/2}{s + 2} \;\Longrightarrow\; f(t) = \dfrac{1}{2}\left(1 - e^{-2t}\right)$

Si el denominador tiene raíces complejas, se **completa el cuadrado**:

$$\frac{1}{s^2 + 2s + 5} = \frac{1}{(s + 1)^2 + 4} \;\Longrightarrow\; \frac{1}{2}e^{-t}\sin 2t$$

## Resolver ecuaciones diferenciales

1. Transformar la ecuación.
2. Despejar $Y(s)$.
3. Descomponer en fracciones simples.
4. Antitransformar.

> [!NOTE]
> $y' + 3y = 6$, con $y(0) = 0$.
>
> $sY + 3Y = \dfrac{6}{s} \Rightarrow Y = \dfrac{6}{s(s + 3)} = \dfrac{2}{s} - \dfrac{2}{s + 3}$.
>
> $y(t) = 2 - 2e^{-3t}$.

> [!NOTE]
> $y'' + y = 0$, con $y(0) = 1$, $y'(0) = 0$.
>
> $s^2Y - s + Y = 0 \Rightarrow Y = \dfrac{s}{s^2 + 1} \Rightarrow y(t) = \cos t$.

## Escalón e impulso

- El **escalón unitario** $u(t - a)$ vale 0 antes de $a$ y 1 después. Modela el encendido de una fuente.
- El **impulso** o delta de Dirac $\delta(t)$ modela un golpe instantáneo de área 1.

Con ellos se resuelven entradas discontinuas que serían engorrosas por otros métodos.

## Teoremas del valor inicial y final

$$f(0^+) = \lim_{s \to \infty} sF(s) \qquad \lim_{t \to \infty} f(t) = \lim_{s \to 0} sF(s)$$

El segundo vale si el límite existe (sistema estable). Permite conocer el régimen permanente sin antitransformar.

En el ejemplo anterior: $\lim_{s \to 0} s\dfrac{6}{s(s + 3)} = 2$. ✓

## Función de transferencia

Para un sistema lineal con entrada $x(t)$ y salida $y(t)$, con condiciones iniciales nulas:

> [!IMPORTANT]
> $$H(s) = \frac{Y(s)}{X(s)}$$

- Los **polos** (raíces del denominador) determinan el comportamiento: si todos tienen parte real negativa, el sistema es **estable**.
- Bloques en serie se multiplican: $H = H_1H_2$.
- Con $s = j\omega$ se obtiene la respuesta en frecuencia.

| Elemento | Impedancia $Z(s)$ |
|---|---|
| Resistencia | $R$ |
| Inductor | $sL$ |
| Capacitor | $\dfrac{1}{sC}$ |

Así un circuito con bobinas y capacitores se analiza como si fuera resistivo.

> [!NOTE]
> Filtro RC pasabajos: $H(s) = \dfrac{1/sC}{R + 1/sC} = \dfrac{1}{1 + sRC}$. Tiene un polo en $s = -\dfrac{1}{RC}$: es estable, con constante de tiempo $\tau = RC$.

> [!WARNING]
> $\mathcal{L}\{f \cdot g\} \neq F \cdot G$. El producto en $s$ corresponde a la **convolución** en el tiempo, no al producto.

> [!TIP]
> Laplace cambia cálculo por álgebra. El costo es aprender una tabla y dominar las fracciones simples.
