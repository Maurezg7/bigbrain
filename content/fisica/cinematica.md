+++
title = "Cinemática: MRU, MRUV y tiro parabólico"
summary = "Describir el movimiento con posición, velocidad y aceleración, en una y dos dimensiones."
branch = "Mecánica"
order = 1
level = "Básico"
tags = ["movimiento", "velocidad", "aceleración", "proyectiles"]
+++

## Las tres magnitudes

La cinemática describe **cómo** se mueve un cuerpo, sin preguntarse por qué.

- **Posición** $x(t)$: dónde está, respecto de un origen elegido.
- **Velocidad** $v = \dfrac{dx}{dt}$: qué tan rápido cambia la posición (m/s).
- **Aceleración** $a = \dfrac{dv}{dt}$: qué tan rápido cambia la velocidad (m/s²).

## Movimiento rectilíneo uniforme (MRU)

Velocidad constante, aceleración nula:

$$x(t) = x_0 + v\,t$$

## Movimiento rectilíneo uniformemente variado (MRUV)

Aceleración constante:

$$v(t) = v_0 + a\,t \qquad x(t) = x_0 + v_0\,t + \tfrac{1}{2}a\,t^2 \qquad v^2 = v_0^2 + 2a\,(x - x_0)$$

La tercera ecuación es útil cuando el tiempo no es dato. La **caída libre** es un MRUV vertical con $a = -g$, donde $g \approx 9{,}8\ \text{m/s}^2$.

> [!NOTE]
> Un auto que va a 20 m/s frena con $a = -4\ \text{m/s}^2$. ¿Qué distancia recorre hasta detenerse?
>
> Con $v = 0$: $0 = 20^2 + 2(-4)\,d \Rightarrow d = \dfrac{400}{8} = 50$ m.

## Tiro parabólico

Un proyectil lanzado con velocidad $v_0$ y ángulo $\theta$ combina dos movimientos **independientes**: un MRU horizontal y un MRUV vertical.

![Trayectoria parabólica de un proyectil con su velocidad inicial descompuesta](/media/fisica/tiro-parabolico.svg)

*La velocidad horizontal no cambia; la vertical disminuye, se anula en la altura máxima y luego crece hacia abajo.*

$$x(t) = v_0\cos\theta\; t \qquad y(t) = v_0\sin\theta\; t - \tfrac{1}{2}g\,t^2$$

De estas ecuaciones salen, para un lanzamiento desde el suelo:

| Magnitud | Fórmula |
|---|---|
| Tiempo de vuelo | $t_v = \dfrac{2 v_0 \sin\theta}{g}$ |
| Altura máxima | $h = \dfrac{v_0^2 \sin^2\theta}{2g}$ |
| Alcance | $R = \dfrac{v_0^2 \sin 2\theta}{g}$ |

El alcance es máximo para $\theta = 45°$ (sin rozamiento con el aire).

> [!NOTE]
> Se patea una pelota a 20 m/s con 30° de elevación. Tomando $g = 9{,}8\ \text{m/s}^2$:
>
> $R = \dfrac{20^2 \sin 60°}{9{,}8} \approx 35{,}3$ m y $h = \dfrac{20^2 \cdot 0{,}5^2}{2 \cdot 9{,}8} \approx 5{,}1$ m.

> [!WARNING]
> Elegí un sistema de referencia y respetalo: si "arriba" es positivo, entonces $g$ entra con signo negativo en todas las ecuaciones.
