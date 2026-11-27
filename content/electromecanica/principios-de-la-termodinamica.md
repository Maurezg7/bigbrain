+++
title = "Principios de la termodinámica"
summary = "Sistemas y propiedades, primer y segundo principio, entalpía, entropía, gases ideales y procesos, vapor de agua."
branch = "Termodinámica y transmisión de calor"
order = 1
level = "Intermedio"
tags = ["primer principio", "entropía", "entalpía", "gas ideal", "vapor"]
+++

## Sistemas y propiedades

La termodinámica estudia las transformaciones de la energía. Se empieza por definir el **sistema**:

| Sistema | Intercambia | Ejemplo |
|---|---|---|
| Cerrado | energía, no masa | gas en un cilindro con pistón |
| Abierto (volumen de control) | energía y masa | turbina, bomba, caldera |
| Aislado | nada | termo ideal |

El **estado** queda definido por propiedades como presión $p$, temperatura $T$, volumen específico $v$, energía interna $u$, entalpía $h$ y entropía $s$. Calor y trabajo no son propiedades: son energía **en tránsito** y dependen del camino.

## Primer principio

La energía se conserva. Para un sistema cerrado:

$$Q - W = \Delta U$$

con $Q$ el calor recibido y $W$ el trabajo entregado. En un proceso cuasiestático, el trabajo de expansión es $W = \int p\,dV$, el área bajo la curva en el diagrama $p$-$V$.

Para un sistema **abierto** en régimen permanente conviene la **entalpía** $h = u + pv$, que incluye el trabajo de flujo:

$$\dot Q - \dot W = \dot m\left[(h_2 - h_1) + \frac{c_2^2 - c_1^2}{2} + g\,(z_2 - z_1)\right]$$

Despreciando energías cinética y potencial:

| Equipo | Balance |
|---|---|
| Turbina | $\dot W = \dot m\,(h_1 - h_2)$ |
| Compresor o bomba | $\dot W_{abs} = \dot m\,(h_2 - h_1)$ |
| Caldera o intercambiador | $\dot Q = \dot m\,(h_2 - h_1)$ |
| Válvula de estrangulación | $h_2 = h_1$ |

## Gases ideales

$$p\,v = R\,T,\qquad p\,V = m\,R\,T$$

con $R = 287$ J/(kg·K) para el aire. La energía interna y la entalpía dependen solo de la temperatura:

$$\Delta u = c_v\,\Delta T,\qquad \Delta h = c_p\,\Delta T,\qquad c_p - c_v = R,\qquad k = \frac{c_p}{c_v}$$

Para el aire, $c_p\approx 1{,}005$ kJ/(kg·K), $c_v\approx 0{,}718$ kJ/(kg·K) y $k = 1{,}4$.

| Proceso | Relación | Trabajo por unidad de masa |
|---|---|---|
| Isobárico ($p$ constante) | $v/T$ constante | $p\,(v_2 - v_1)$ |
| Isocórico ($v$ constante) | $p/T$ constante | 0 |
| Isotérmico ($T$ constante) | $p\,v$ constante | $R\,T\ln(v_2/v_1)$ |
| Adiabático reversible | $p\,v^k$ constante | $\dfrac{p_1v_1 - p_2v_2}{k - 1}$ |
| Politrópico | $p\,v^n$ constante | $\dfrac{p_1v_1 - p_2v_2}{n - 1}$ |

En el adiabático reversible, además, $\dfrac{T_2}{T_1} = \left(\dfrac{p_2}{p_1}\right)^{(k-1)/k}$.

> [!NOTE]
> Aire a 300 K y 1 bar se comprime de forma adiabática reversible hasta 8 bar.
>
> $T_2 = 300\cdot 8^{0{,}286}\approx 543$ K (270 °C). El trabajo es $c_p\,(T_2 - T_1) = 1{,}005\cdot 243\approx 244$ kJ/kg.

## Segundo principio

El primer principio no dice en qué sentido ocurren los procesos. El segundo sí:

- **Clausius**: el calor no pasa por sí solo de un cuerpo frío a uno caliente.
- **Kelvin-Planck**: ninguna máquina cíclica convierte en trabajo todo el calor que recibe; siempre debe ceder una parte a una fuente fría.

El rendimiento de una máquina térmica es

$$\eta = \frac{W}{Q_c} = 1 - \frac{Q_f}{Q_c}$$

y el máximo posible entre dos temperaturas absolutas es el de **Carnot**:

$$\eta_{Carnot} = 1 - \frac{T_f}{T_c}$$

Entre 800 K y 300 K ninguna máquina supera el 62,5 %. Para mejorar el rendimiento hay que subir la temperatura caliente o bajar la fría.

## Entropía

La entropía mide la degradación de la energía. Se define con $dS = \dfrac{\delta Q_{rev}}{T}$ y cumple

$$\Delta S_{universo}\ge 0$$

Vale cero solo en procesos reversibles, que son ideales. Rozamiento, mezcla, estrangulación y transferencia de calor con salto finito de temperatura son **irreversibilidades**: generan entropía y destruyen capacidad de producir trabajo (exergía).

Las máquinas reales se comparan con el proceso ideal isoentrópico mediante el **rendimiento isoentrópico**:

$$\eta_{turbina} = \frac{h_1 - h_2}{h_1 - h_{2s}},\qquad \eta_{compresor} = \frac{h_{2s} - h_1}{h_2 - h_1}$$

## Vapor de agua

El agua no es un gas ideal cerca del cambio de fase: sus propiedades se leen de **tablas de vapor** o del diagrama de Mollier ($h$-$s$).

| Estado | Descripción |
|---|---|
| Líquido subenfriado | por debajo de la temperatura de saturación |
| Líquido saturado | a punto de hervir |
| Vapor húmedo | mezcla de líquido y vapor; título $x$ entre 0 y 1 |
| Vapor saturado seco | todo vapor, a temperatura de saturación |
| Vapor sobrecalentado | por encima de la temperatura de saturación |

En la zona húmeda, cualquier propiedad se calcula con el **título** $x$ (fracción en masa de vapor):

$$h = h_f + x\,h_{fg}$$

A 1 bar el agua hierve a 99,6 °C con un calor latente $h_{fg}\approx 2258$ kJ/kg; a 10 bar, a 179,9 °C con 2015 kJ/kg.
