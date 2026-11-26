+++
title = "Ecuaciones diferenciales de primer orden"
summary = "Ecuaciones cuya incógnita es una función: variables separables y lineales, con aplicaciones."
branch = "Ecuaciones diferenciales"
order = 1
level = "Avanzado"
tags = ["EDO", "variables separables", "factor integrante"]
+++

## Qué es una ecuación diferencial

Una **ecuación diferencial ordinaria** (EDO) relaciona una función desconocida $y(x)$ con sus derivadas. Su **orden** es el de la derivada más alta. Resolverla es hallar todas las funciones que la cumplen: la **solución general** contiene constantes arbitrarias, que se fijan con **condiciones iniciales** como $y(0) = y_0$.

## Variables separables

Si la ecuación puede escribirse como $\dfrac{dy}{dx} = g(x)\,h(y)$, se separan las variables y se integra cada lado:

$$\int \frac{dy}{h(y)} = \int g(x)\,dx$$

> [!NOTE]
> **Ley de enfriamiento de Newton.** Un café a 90 °C se deja en una habitación a 20 °C. Su temperatura cumple $\dfrac{dT}{dt} = -k\,(T - 20)$.
>
> Separando: $\dfrac{dT}{T - 20} = -k\,dt \;\Rightarrow\; \ln|T - 20| = -kt + C$, es decir
>
> $$T(t) = 20 + 70\,e^{-kt}$$
>
> donde la constante $70$ sale de $T(0) = 90$. La temperatura decae exponencialmente hacia la del ambiente.

## Lineales de primer orden

Tienen la forma

$$y' + p(x)\,y = q(x)$$

Se resuelven multiplicando por el **factor integrante** $\mu(x) = e^{\int p(x)\,dx}$, que convierte el lado izquierdo en la derivada de un producto:

$$\big(\mu\,y\big)' = \mu\,q \;\Longrightarrow\; y = \frac{1}{\mu(x)}\left(\int \mu(x)\,q(x)\,dx + C\right)$$

> [!NOTE]
> Resolver $y' + 2y = 4$ con $y(0) = 0$.
>
> $\mu = e^{2x}$, entonces $(e^{2x}y)' = 4e^{2x} \Rightarrow e^{2x}y = 2e^{2x} + C \Rightarrow y = 2 + Ce^{-2x}$.
> Con $y(0)=0$: $C = -2$, y la solución es $y = 2\,(1 - e^{-2x})$.

Esta misma ecuación describe la carga de un capacitor en un circuito RC o la velocidad de un cuerpo que cae con rozamiento: un transitorio exponencial que tiende a un valor de equilibrio.

## Dónde aparecen

| Fenómeno | Ecuación |
|---|---|
| Crecimiento poblacional | $P' = kP$ |
| Desintegración radiactiva | $N' = -\lambda N$ |
| Circuito RC | $RC\,v' + v = V$ |
| Caída con rozamiento | $m v' = mg - bv$ |

> [!TIP]
> Siempre podés verificar una solución derivándola y reemplazando en la ecuación original. Es más rápido que rehacer la cuenta.
