+++
title = "Amplificadores operacionales"
summary = "El modelo ideal, la realimentación negativa y los circuitos básicos: inversor, no inversor, sumador, integrador y comparador."
branch = "Electrónica"
order = 3
level = "Intermedio"
tags = ["operacional", "realimentación", "inversor", "comparador", "filtro activo"]
+++

## El operacional ideal

Un amplificador operacional tiene dos entradas, la no inversora (+) y la inversora (−), y una salida:

$$V_o = A\,(V_+ - V_-)$$

con una ganancia $A$ enorme, del orden de $10^5$. El modelo ideal supone:

- ganancia infinita;
- impedancia de entrada infinita: **no entra corriente** por las entradas;
- impedancia de salida nula.

## Las dos reglas de oro

Cuando hay **realimentación negativa** (una conexión de la salida a la entrada inversora), la salida se acomoda para que:

1. $V_+ = V_-$ (cortocircuito virtual);
2. $I_+ = I_- = 0$.

Con estas dos reglas y las leyes de Kirchhoff se resuelve casi cualquier circuito.

## Circuitos básicos

![Amplificador inversor con resistencia de entrada R1 y resistencia de realimentación R2](/media/electromecanica/operacional.svg)

*Amplificador inversor: la ganancia depende solo del cociente de resistencias.*

| Circuito | Salida |
|---|---|
| Inversor | $V_o = -\dfrac{R_2}{R_1}\,V_i$ |
| No inversor | $V_o = \left(1 + \dfrac{R_2}{R_1}\right)V_i$ |
| Seguidor | $V_o = V_i$ |
| Sumador inversor | $V_o = -R_f\left(\dfrac{V_1}{R_1} + \dfrac{V_2}{R_2}\right)$ |
| Diferencial (resistencias apareadas) | $V_o = \dfrac{R_2}{R_1}\,(V_2 - V_1)$ |
| Integrador | $V_o = -\dfrac{1}{RC}\displaystyle\int V_i\,dt$ |
| Derivador | $V_o = -RC\,\dfrac{dV_i}{dt}$ |

> [!NOTE]
> Un sensor entrega de 0 a 100 mV y el conversor analógico-digital espera de 0 a 5 V.
>
> Hace falta ganancia 50. Con un no inversor: $1 + \dfrac{R_2}{R_1} = 50$, por ejemplo $R_1 = 1\ \text{k}\Omega$ y $R_2 = 49\ \text{k}\Omega$.
>
> Se elige el no inversor porque su impedancia de entrada es altísima y no carga al sensor.

## Comparador

Sin realimentación, la salida se va a uno de los dos extremos de alimentación según cuál entrada sea mayor. Para evitar que oscile con el ruido se añade realimentación **positiva**: es el disparador de Schmitt, con dos umbrales distintos (histéresis), como el de un termostato.

## Amplificador de instrumentación

Tres operacionales forman un amplificador diferencial de altísima impedancia de entrada y gran rechazo de modo común (CMRR). Es el circuito estándar para leer puentes de galgas, termocuplas y otras señales débiles en ambientes ruidosos.

## Límites del operacional real

| Parámetro | Efecto |
|---|---|
| Saturación | la salida no supera la alimentación |
| Producto ganancia-ancho de banda | a más ganancia, menos ancho de banda: $A\cdot BW = \text{cte}$ |
| Slew rate | velocidad máxima de cambio de la salida, en V/µs |
| Tensión de offset | salida no nula con entrada cero |
| Corrientes de polarización | pequeños errores con resistencias grandes |

> [!TIP]
> Un operacional con 1 MHz de producto ganancia-ancho de banda usado con ganancia 100 solo amplifica bien hasta 10 kHz.

## Filtros activos

Combinando operacionales con resistencias y capacitores se construyen filtros sin inductores y con ganancia. Las respuestas más usadas son Butterworth (plana en la banda de paso), Chebyshev (corte más abrupto con ondulación) y Bessel (fase lineal).
