+++
title = "Ley de Ohm y leyes de Kirchhoff"
summary = "Tensión, corriente y resistencia; cómo resolver cualquier circuito de corriente continua."
branch = "Circuitos eléctricos"
order = 1
level = "Básico"
tags = ["tensión", "corriente", "resistencia", "mallas", "nodos"]
+++

## Las tres magnitudes

| Magnitud | Símbolo | Unidad | Analogía hidráulica |
|---|---|---|---|
| Tensión (voltaje) | $V$ | volt (V) | diferencia de presión |
| Corriente | $I$ | ampere (A) | caudal |
| Resistencia | $R$ | ohm (Ω) | estrechez de la cañería |

## Ley de Ohm

En un conductor óhmico la corriente es proporcional a la tensión aplicada:

$$V = I\,R$$

La **potencia** que disipa una resistencia en forma de calor (efecto Joule) es

$$P = V\,I = I^2 R = \frac{V^2}{R}$$

## Resistencias en serie y en paralelo

- **Serie** (misma corriente): $R_{eq} = R_1 + R_2 + \dots$
- **Paralelo** (misma tensión): $\dfrac{1}{R_{eq}} = \dfrac{1}{R_1} + \dfrac{1}{R_2} + \dots$ Para dos resistencias, $R_{eq} = \dfrac{R_1 R_2}{R_1 + R_2}$.

## Leyes de Kirchhoff

> [!IMPORTANT]
> **Ley de nodos (LCK):** la suma de las corrientes que entran a un nodo es igual a la suma de las que salen. Expresa la conservación de la carga.
>
> **Ley de mallas (LVK):** la suma algebraica de las tensiones a lo largo de cualquier lazo cerrado es cero. Expresa la conservación de la energía.

![Circuito con una fuente de 12 V, una resistencia en serie y dos en paralelo](/media/electromecanica/circuito.svg)

*Circuito del ejemplo: $R_1$ en serie con el paralelo de $R_2$ y $R_3$.*

> [!NOTE]
> En el circuito de la figura, $V = 12$ V, $R_1 = 2\ \Omega$, $R_2 = 6\ \Omega$ y $R_3 = 3\ \Omega$.
>
> 1. Paralelo: $R_{23} = \dfrac{6 \cdot 3}{6 + 3} = 2\ \Omega$.
> 2. Total: $R_{eq} = 2 + 2 = 4\ \Omega$, así que $I_1 = \dfrac{12}{4} = 3$ A.
> 3. Tensión en el paralelo: $V_{23} = 12 - 3 \cdot 2 = 6$ V (ley de mallas).
> 4. Corrientes: $I_2 = \dfrac{6}{6} = 1$ A e $I_3 = \dfrac{6}{3} = 2$ A.
>
> Verificación con la ley de nodos: $I_1 = I_2 + I_3 = 3$ A. La fuente entrega $P = 12 \cdot 3 = 36$ W.

## Método de mallas

Para circuitos que no se reducen con serie y paralelo:

1. Asigná una corriente a cada malla independiente, todas en el mismo sentido.
2. Escribí la ley de mallas en cada una, usando $V = IR$ para las resistencias.
3. Resolvé el sistema de ecuaciones lineales.

Si una corriente da negativa, simplemente circula en sentido contrario al supuesto.

> [!WARNING]
> Convención de signos al recorrer una malla: en una resistencia recorrida **a favor** de la corriente la tensión cae ($-IR$); en una fuente recorrida de $-$ a $+$ la tensión sube ($+V$). Mantené la misma convención en todo el circuito.
