+++
title = "Leyes de la termodinámica"
summary = "Calor, trabajo y energía interna; por qué ninguna máquina térmica tiene rendimiento del 100 %."
branch = "Termodinámica"
order = 1
level = "Intermedio"
tags = ["calor", "entropía", "gas ideal", "Carnot"]
+++

## Temperatura y calor

La **temperatura** mide la energía cinética media de las partículas de un cuerpo. El **calor** $Q$ es energía que pasa de un cuerpo a otro por diferencia de temperatura. Para calentar una masa $m$ sin cambio de estado:

$$Q = m\,c\,\Delta T$$

donde $c$ es el calor específico (para el agua, $4186\ \text{J/(kg·K)}$). Durante un cambio de estado la temperatura no cambia y $Q = mL$, con $L$ el calor latente.

## Ley cero

Si dos sistemas están cada uno en equilibrio térmico con un tercero, lo están entre sí. Es lo que permite definir la temperatura y usar termómetros.

## Primera ley: conservación de la energía

$$\Delta U = Q - W$$

La energía interna $U$ de un sistema aumenta con el calor que recibe y disminuye con el trabajo que hace. Para un gas, $W = \int p\,dV$.

| Proceso | Condición | Consecuencia |
|---|---|---|
| Isocórico | $V$ constante | $W = 0$, $\Delta U = Q$ |
| Isobárico | $p$ constante | $W = p\,\Delta V$ |
| Isotérmico | $T$ constante | $\Delta U = 0$, $Q = W$ (gas ideal) |
| Adiabático | $Q = 0$ | $\Delta U = -W$ |

Para un **gas ideal**: $pV = nRT$, con $R = 8{,}314\ \text{J/(mol·K)}$ y $T$ en kelvin.

## Segunda ley: el sentido de los procesos

El calor fluye espontáneamente del cuerpo caliente al frío, nunca al revés. De forma equivalente: es imposible convertir todo el calor en trabajo en un proceso cíclico. Esto se cuantifica con la **entropía** $S$, que en un sistema aislado nunca disminuye:

$$\Delta S \ge 0$$

## Máquinas térmicas

Una máquina térmica toma calor $Q_c$ de una fuente caliente, hace trabajo $W$ y cede $Q_f$ a una fuente fría. Su rendimiento es

$$\eta = \frac{W}{Q_c} = 1 - \frac{Q_f}{Q_c} \;\le\; 1 - \frac{T_f}{T_c}$$

El límite de la derecha es el rendimiento de **Carnot**, inalcanzable en la práctica, con las temperaturas en kelvin.

> [!NOTE]
> Una turbina trabaja entre vapor a 500 °C y un condensador a 30 °C. Su rendimiento máximo teórico es
> $\eta = 1 - \dfrac{303}{773} \approx 0{,}61$. Ninguna mejora de diseño puede superar ese 61 %.

## Tercera ley

Es imposible alcanzar el cero absoluto (0 K) en un número finito de pasos; allí la entropía de un cristal perfecto es cero.
