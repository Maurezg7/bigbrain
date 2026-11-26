+++
title = "Métodos de análisis de circuitos"
summary = "Mallas, nodos, superposición y los equivalentes de Thévenin y Norton para resolver cualquier red lineal."
branch = "Circuitos eléctricos"
order = 2
level = "Intermedio"
tags = ["mallas", "nodos", "Thévenin", "Norton", "superposición"]
+++

## Por qué hacen falta métodos

Con la ley de Ohm y las leyes de Kirchhoff se puede resolver cualquier circuito, pero escribir todas las ecuaciones a mano es lento y propenso a errores. Los métodos sistemáticos eligen las incógnitas de forma que el número de ecuaciones sea el mínimo.

| Método | Incógnitas | Conviene cuando |
|---|---|---|
| Corrientes de malla | una corriente por malla | hay pocas mallas y fuentes de tensión |
| Tensiones de nodo | una tensión por nodo (menos el de referencia) | hay pocos nodos y fuentes de corriente |
| Superposición | la respuesta a cada fuente por separado | interesa el aporte de cada fuente |
| Thévenin / Norton | un equivalente de dos terminales | cambia solo la carga |

## Corrientes de malla

Se asigna una corriente ficticia a cada malla, todas en el mismo sentido, y se aplica la ley de tensiones de Kirchhoff en cada una. Para dos mallas que comparten $R_3$:

$$\begin{aligned}(R_1 + R_3)\,I_1 - R_3\,I_2 &= V_1\\ -R_3\,I_1 + (R_2 + R_3)\,I_2 &= -V_2\end{aligned}$$

La matriz queda simétrica: en la diagonal va la suma de resistencias de cada malla y fuera de ella, con signo negativo, las resistencias compartidas.

## Tensiones de nodo

Se elige un nodo de referencia (tierra) y se aplica la ley de corrientes en los demás. Para un nodo con tensión $V_a$ unido por $R_1$ a una fuente $V_1$, por $R_2$ a tierra y por $R_3$ a una fuente $V_2$:

$$\frac{V_a - V_1}{R_1} + \frac{V_a}{R_2} + \frac{V_a - V_2}{R_3} = 0$$

Es el método que usan los simuladores como SPICE, porque se arma de manera automática.

## Superposición

En un circuito **lineal**, la respuesta total es la suma de las respuestas a cada fuente independiente actuando sola. Para anular una fuente:

- una fuente de tensión se reemplaza por un cortocircuito;
- una fuente de corriente se reemplaza por un circuito abierto.

> [!WARNING]
> La superposición vale para tensiones y corrientes, **no para potencias**: la potencia depende del cuadrado de la corriente y no es lineal.

## Thévenin y Norton

![Equivalente de Thévenin: una fuente de tensión en serie con una resistencia alimenta la carga](/media/electromecanica/thevenin.svg)

*Cualquier red lineal vista desde dos terminales se comporta como una fuente con una resistencia.*

Vista desde dos terminales, toda red lineal equivale a:

- **Thévenin**: una fuente $V_{Th}$ en serie con $R_{Th}$;
- **Norton**: una fuente $I_N$ en paralelo con $R_N = R_{Th}$, con $I_N = V_{Th}/R_{Th}$.

$V_{Th}$ es la tensión a circuito abierto entre los terminales y $R_{Th}$ la resistencia vista desde ellos con las fuentes independientes anuladas.

> [!NOTE]
> Un divisor con $V = 12$ V, $R_1 = 4\ \Omega$ arriba y $R_2 = 12\ \Omega$ abajo alimenta una carga conectada sobre $R_2$.
>
> $V_{Th} = 12 \cdot \dfrac{12}{4 + 12} = 9$ V y $R_{Th} = \dfrac{4 \cdot 12}{4 + 12} = 3\ \Omega$.
>
> Con una carga de $6\ \Omega$: $I = \dfrac{9}{3 + 6} = 1$ A y $V = 6$ V, sin volver a resolver todo el circuito.

## Máxima transferencia de potencia

La carga recibe la máxima potencia cuando $R_L = R_{Th}$, y esa potencia vale

$$P_{max} = \frac{V_{Th}^2}{4R_{Th}}$$

En ese punto el rendimiento es apenas del 50 %: sirve en electrónica y comunicaciones, pero en potencia se busca $R_L \gg R_{Th}$ para no desperdiciar energía.

> [!TIP]
> Todos estos métodos valen igual en corriente alterna si se reemplazan resistencias por impedancias y se opera con fasores.
