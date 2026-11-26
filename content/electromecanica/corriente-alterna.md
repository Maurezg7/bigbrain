+++
title = "Corriente alterna y potencia"
summary = "Valores eficaces, impedancia, fasores, potencia activa, reactiva y aparente, y factor de potencia."
branch = "Circuitos eléctricos"
order = 3
level = "Intermedio"
tags = ["impedancia", "fasores", "factor de potencia", "reactiva"]
+++

## La señal senoidal

La red eléctrica entrega una tensión que varía como una senoide:

$$v(t) = V_{m}\sin(\omega t + \varphi), \qquad \omega = 2\pi f$$

con $f = 50$ Hz en Argentina y Europa (60 Hz en gran parte de América). El **valor eficaz** (RMS) es el de la corriente continua que disiparía la misma potencia:

$$V_{ef} = \frac{V_m}{\sqrt{2}}$$

Los 220 V domiciliarios son un valor eficaz: el pico es $220\sqrt2 \approx 311$ V.

## Resistencia, inductor y capacitor

En alterna, cada elemento se describe con su **impedancia** $Z$, un número complejo que generaliza la resistencia: $\mathbf{V} = \mathbf{I}\,Z$.

| Elemento | Impedancia | Desfase |
|---|---|---|
| Resistencia | $Z_R = R$ | tensión y corriente en fase |
| Inductor | $Z_L = j\omega L$ | la corriente atrasa 90° |
| Capacitor | $Z_C = \dfrac{1}{j\omega C}$ | la corriente adelanta 90° |

Las impedancias se combinan en serie y en paralelo igual que las resistencias, pero operando con números complejos. Las tensiones y corrientes se representan con **fasores**: vectores cuyo módulo es el valor eficaz y cuyo ángulo es la fase.

Para un circuito RLC serie:

$$Z = R + j\left(\omega L - \frac{1}{\omega C}\right), \qquad |Z| = \sqrt{R^2 + (X_L - X_C)^2}$$

Cuando $X_L = X_C$ el circuito está en **resonancia**, a la frecuencia $f_0 = \dfrac{1}{2\pi\sqrt{LC}}$.

## Las tres potencias

![Triángulo de potencias con P, Q, S y el ángulo phi](/media/electromecanica/triangulo-potencias.svg)

*Triángulo de potencias: $S$ es la hipotenusa, $P$ y $Q$ los catetos.*

| Potencia | Fórmula | Unidad | Qué representa |
|---|---|---|---|
| Activa | $P = V I \cos\varphi$ | W | la que se convierte en trabajo o calor |
| Reactiva | $Q = V I \sin\varphi$ | VAr | la que va y viene entre la fuente y los campos de L y C |
| Aparente | $S = V I = \sqrt{P^2 + Q^2}$ | VA | la que debe transportar la instalación |

## Factor de potencia

El **factor de potencia** es $\cos\varphi = P/S$. Un valor bajo significa que circula más corriente de la necesaria para la misma potencia útil: más pérdidas en los cables y multas de la distribuidora. Como las cargas industriales suelen ser inductivas (motores), se corrige conectando **capacitores en paralelo** que aporten la reactiva localmente:

$$Q_C = P\,(\tan\varphi_1 - \tan\varphi_2)$$

> [!NOTE]
> Un motor monofásico consume 2 kW de una red de 220 V con $\cos\varphi = 0{,}8$.
>
> $S = \dfrac{2000}{0{,}8} = 2500$ VA, $I = \dfrac{2500}{220} \approx 11{,}4$ A y $Q = \sqrt{2500^2 - 2000^2} = 1500$ VAr.
>
> Para llevarlo a $\cos\varphi_2 = 0{,}95$ ($\tan\varphi_2 = 0{,}329$) hacen falta $Q_C = 2000\,(0{,}75 - 0{,}329) \approx 842$ VAr de capacitores, y la corriente baja a $\dfrac{2000}{220 \cdot 0{,}95} \approx 9{,}6$ A.

## Sistemas trifásicos

La generación y distribución usan tres tensiones desfasadas 120°. En un sistema equilibrado, la tensión de línea es $V_L = \sqrt3\,V_F$ (380 V entre fases si hay 220 V entre fase y neutro) y la potencia total es

$$P = \sqrt3\,V_L I_L \cos\varphi$$
