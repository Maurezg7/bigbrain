+++
title = "Sistemas trifásicos"
summary = "Tensiones de fase y de línea, conexiones estrella y triángulo, potencia trifásica y cargas desequilibradas."
branch = "Circuitos eléctricos"
order = 4
level = "Intermedio"
tags = ["trifásica", "estrella", "triángulo", "neutro", "potencia"]
+++

## Por qué tres fases

Casi toda la energía eléctrica se genera, transporta y consume en forma trifásica porque:

- para la misma potencia necesita menos cobre que un sistema monofásico;
- la potencia total entregada es **constante** en el tiempo, sin pulsaciones;
- produce de forma natural un campo magnético giratorio, que es lo que hace funcionar a los motores.

## Tensiones de fase y de línea

![Tres fasores de tensión desfasados 120 grados y la tensión de línea entre dos de ellos](/media/electromecanica/trifasica.svg)

*Las tres tensiones de fase tienen el mismo módulo y están desfasadas 120°.*

Un generador trifásico equilibrado produce

$$v_R = V_m\sin(\omega t),\quad v_S = V_m\sin(\omega t - 120^\circ),\quad v_T = V_m\sin(\omega t + 120^\circ)$$

y su suma es cero en todo instante. Se distinguen dos tensiones:

- **de fase** $V_F$: entre un conductor de línea y el neutro;
- **de línea** $V_L$: entre dos conductores de línea.

$$V_L = \sqrt{3}\,V_F$$

En la red argentina de baja tensión, $V_F = 220$ V y $V_L = 380$ V.

## Estrella y triángulo

| Conexión | Tensión en cada impedancia | Corriente en cada impedancia | Neutro |
|---|---|---|---|
| Estrella (Y) | $V_F = V_L/\sqrt3$ | $I_F = I_L$ | disponible |
| Triángulo (Δ) | $V_L$ | $I_F = I_L/\sqrt3$ | no hay |

Tres impedancias iguales $Z$ en triángulo equivalen a tres impedancias $Z/3$ en estrella. Por eso un motor conectado en triángulo toma el triple de corriente de línea y de potencia que el mismo motor en estrella: es la base del **arranque estrella-triángulo**.

## Potencia trifásica

Para una carga equilibrada, sin importar la conexión:

$$P = \sqrt3\,V_L I_L \cos\varphi,\qquad Q = \sqrt3\,V_L I_L \sin\varphi,\qquad S = \sqrt3\,V_L I_L$$

donde $\varphi$ es el ángulo de la impedancia de cada fase.

> [!NOTE]
> Un motor trifásico de 380 V entrega 15 kW en el eje con rendimiento 0,90 y $\cos\varphi = 0{,}85$.
>
> Potencia eléctrica absorbida: $P = \dfrac{15}{0{,}90} = 16{,}7$ kW.
>
> Corriente de línea: $I_L = \dfrac{16\,700}{\sqrt3 \cdot 380 \cdot 0{,}85} \approx 29{,}8$ A.

## Cargas desequilibradas

Cuando las tres impedancias no son iguales (lo habitual en una instalación con cargas monofásicas), las corrientes dejan de sumar cero y la diferencia circula por el **neutro**:

$$\mathbf{I}_N = \mathbf{I}_R + \mathbf{I}_S + \mathbf{I}_T$$

> [!WARNING]
> Si el neutro se corta en una instalación en estrella desequilibrada, el punto central se desplaza: las fases poco cargadas reciben sobretensiones de hasta 380 V y los equipos se queman. Por eso el neutro nunca lleva fusible propio.

Para estudiar sistemas desequilibrados y fallas se usan las **componentes simétricas** (Fortescue): cualquier terna desequilibrada se descompone en una secuencia directa, una inversa y una homopolar, cada una equilibrada.

## Secuencia de fases

El orden en que las tensiones alcanzan su máximo (RST o RTS) determina el sentido de giro de los motores. Intercambiar dos conductores cualesquiera invierte la secuencia y, con ella, el giro.
