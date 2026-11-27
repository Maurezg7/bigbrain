+++
title = "Transitorios en circuitos RC, RL y RLC"
summary = "Qué pasa entre un estado y otro: constante de tiempo, carga de un capacitor y respuestas de segundo orden."
branch = "Sistemas y señales"
order = 1
level = "Intermedio"
tags = ["constante de tiempo", "RC", "RL", "amortiguamiento", "ecuaciones diferenciales"]
+++

## Régimen transitorio y permanente

Cuando se cierra una llave, el circuito no pasa al instante a su nuevo estado: capacitores e inductores almacenan energía y esa energía no puede cambiar de golpe.

- La **tensión de un capacitor** no puede saltar: $i_C = C\,\dfrac{dv_C}{dt}$.
- La **corriente de un inductor** no puede saltar: $v_L = L\,\dfrac{di_L}{dt}$.

La respuesta completa es la suma de una parte **transitoria**, que se extingue, y una **permanente**, que queda.

## Circuitos de primer orden

![Curva exponencial de carga de un capacitor que alcanza el 63 por ciento en una constante de tiempo](/media/electromecanica/carga-rc.svg)

*Carga de un capacitor: en un tiempo τ llega al 63 % del valor final y en 5τ se considera cargado.*

Un capacitor descargado que se conecta a una fuente $V$ a través de $R$ cumple $RC\,\dfrac{dv_C}{dt} + v_C = V$, cuya solución es

$$v_C(t) = V\left(1 - e^{-t/\tau}\right),\qquad \tau = RC$$

Para un inductor en serie con $R$, la corriente crece igual con $\tau = L/R$:

$$i_L(t) = \frac{V}{R}\left(1 - e^{-t/\tau}\right)$$

Cualquier variable de un circuito de primer orden sigue la misma fórmula general:

$$x(t) = x(\infty) + \left[x(0^+) - x(\infty)\right]e^{-t/\tau}$$

| Tiempo | $\tau$ | $2\tau$ | $3\tau$ | $5\tau$ |
|---|---|---|---|---|
| Avance hacia el valor final | 63,2 % | 86,5 % | 95,0 % | 99,3 % |

> [!NOTE]
> Un capacitor de $100\ \mu$F se carga a 12 V a través de $10\ \text{k}\Omega$.
>
> $\tau = 10^4 \cdot 10^{-4} = 1$ s. A los 2 s la tensión es $12\,(1 - e^{-2}) \approx 10{,}4$ V, y se considera cargado a los 5 s.

> [!WARNING]
> Al abrir un circuito inductivo la corriente intenta seguir circulando y aparece una sobretensión $v = L\,di/dt$ muy alta, capaz de destruir un transistor o producir un arco en un contacto. Por eso las bobinas de relés llevan un diodo en antiparalelo.

## Circuitos de segundo orden

Con un inductor y un capacitor, la energía puede ir y venir entre ambos. Para el RLC serie:

$$\frac{d^2 i}{dt^2} + \frac{R}{L}\frac{di}{dt} + \frac{1}{LC}\,i = 0$$

Se definen la frecuencia natural y el factor de amortiguamiento:

$$\omega_0 = \frac{1}{\sqrt{LC}},\qquad \zeta = \frac{R}{2}\sqrt{\frac{C}{L}}$$

| Caso | Condición | Respuesta |
|---|---|---|
| Sobreamortiguado | $\zeta > 1$ | dos exponenciales, sin oscilar, lenta |
| Críticamente amortiguado | $\zeta = 1$ | la más rápida sin sobrepasar |
| Subamortiguado | $\zeta < 1$ | oscila a $\omega_d = \omega_0\sqrt{1 - \zeta^2}$ mientras se extingue |

La misma ecuación describe una masa con resorte y amortiguador: la inductancia hace de masa, la resistencia de rozamiento y la inversa de la capacidad de rigidez. Esta analogía es la base del estudio unificado de sistemas dinámicos.

> [!TIP]
> Para resolver un transitorio: 1) hallar las condiciones iniciales justo antes de conmutar, 2) hallar el valor final con capacitores abiertos e inductores en cortocircuito, 3) calcular $\tau$ con la resistencia vista desde el elemento que almacena energía.
