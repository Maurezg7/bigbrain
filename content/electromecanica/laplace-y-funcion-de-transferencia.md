+++
title = "Transformada de Laplace y función de transferencia"
summary = "Cómo convertir ecuaciones diferenciales en álgebra y describir un sistema lineal con polos, ceros y diagramas de bloques."
branch = "Sistemas y señales"
order = 2
level = "Avanzado"
tags = ["Laplace", "función de transferencia", "polos", "ceros", "sistemas lineales"]
+++

## La idea

Un sistema lineal e invariante en el tiempo (un circuito, un motor, un tanque) se describe con una ecuación diferencial. La **transformada de Laplace** convierte derivadas en multiplicaciones por $s$, de modo que resolver la ecuación pasa a ser un problema de álgebra:

$$F(s) = \mathcal{L}\{f(t)\} = \int_0^\infty f(t)\,e^{-st}\,dt$$

| $f(t)$, para $t \ge 0$ | $F(s)$ |
|---|---|
| Impulso $\delta(t)$ | $1$ |
| Escalón unitario | $\dfrac{1}{s}$ |
| Rampa $t$ | $\dfrac{1}{s^2}$ |
| $e^{-at}$ | $\dfrac{1}{s + a}$ |
| $\sin(\omega t)$ | $\dfrac{\omega}{s^2 + \omega^2}$ |
| $\cos(\omega t)$ | $\dfrac{s}{s^2 + \omega^2}$ |

Propiedades clave:

$$\mathcal{L}\left\{\frac{df}{dt}\right\} = sF(s) - f(0),\qquad \mathcal{L}\left\{\int_0^t f\,d\tau\right\} = \frac{F(s)}{s}$$

## Impedancias en el dominio s

Con condiciones iniciales nulas, cada elemento tiene una impedancia operacional y el circuito se resuelve como uno resistivo:

| Elemento | $Z(s)$ |
|---|---|
| Resistencia | $R$ |
| Inductor | $sL$ |
| Capacitor | $\dfrac{1}{sC}$ |

## Función de transferencia

Es el cociente entre la salida y la entrada, ambas transformadas y con condiciones iniciales nulas:

$$G(s) = \frac{Y(s)}{X(s)}$$

Para el filtro RC con salida en el capacitor:

$$G(s) = \frac{1/sC}{R + 1/sC} = \frac{1}{1 + sRC}$$

Las raíces del denominador son los **polos** y las del numerador los **ceros**. Los polos deciden la forma de la respuesta:

- polo real negativo $-a$: exponencial que decae con $\tau = 1/a$;
- par complejo con parte real negativa: oscilación amortiguada;
- cualquier polo con parte real positiva: la salida crece sin límite, el sistema es **inestable**.

## Sistemas típicos

**Primer orden**, con ganancia $K$ y constante de tiempo $\tau$:

$$G(s) = \frac{K}{\tau s + 1}$$

**Segundo orden**, con frecuencia natural $\omega_n$ y amortiguamiento $\zeta$:

$$G(s) = \frac{K\,\omega_n^2}{s^2 + 2\zeta\omega_n s + \omega_n^2}$$

Para $\zeta < 1$ la respuesta al escalón sobrepasa el valor final en

$$M_p = e^{-\pi\zeta/\sqrt{1 - \zeta^2}}$$

y se establece (criterio del 2 %) en $t_s \approx \dfrac{4}{\zeta\omega_n}$.

> [!NOTE]
> Respuesta al escalón de $G(s) = \dfrac{5}{2s + 1}$.
>
> $Y(s) = \dfrac{5}{s\,(2s + 1)} = \dfrac{5}{s} - \dfrac{5}{s + 0{,}5}$, de donde $y(t) = 5\left(1 - e^{-t/2}\right)$.
>
> El valor final es 5 (la ganancia) y la constante de tiempo es 2 s.

## Teoremas del valor inicial y final

$$f(0^+) = \lim_{s\to\infty} sF(s),\qquad f(\infty) = \lim_{s\to 0} sF(s)$$

El segundo solo vale si el sistema es estable. Permite conocer el valor final de una respuesta sin antitransformar.

## Diagramas de bloques

Los bloques en serie multiplican sus funciones de transferencia y los que están en paralelo las suman. Un lazo con realimentación negativa, con $G$ en la rama directa y $H$ en la de realimentación, se reduce a

$$\frac{Y(s)}{R(s)} = \frac{G(s)}{1 + G(s)H(s)}$$

> [!TIP]
> Reemplazar $s$ por $j\omega$ en $G(s)$ da la respuesta en frecuencia: el módulo es la ganancia y el argumento el desfase para una senoide de pulsación $\omega$.
