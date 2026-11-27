+++
title = "Oscilaciones"
summary = "Movimiento armónico simple, sistema masa-resorte, péndulo, energía, amortiguamiento y resonancia."
branch = "Mecánica"
order = 8
level = "Intermedio"
tags = ["movimiento armónico simple", "péndulo", "resorte", "resonancia"]
+++

## Movimiento armónico simple

Un cuerpo oscila con **movimiento armónico simple** (MAS) cuando la fuerza que lo devuelve al equilibrio es proporcional al apartamiento:

> [!IMPORTANT]
> $$F = -kx \quad\Longrightarrow\quad \frac{d^2x}{dt^2} = -\omega^2x$$
> $$x(t) = A\cos(\omega t + \varphi)$$

| Magnitud | Símbolo | Significado |
|---|---|---|
| Amplitud | $A$ | apartamiento máximo |
| Frecuencia angular | $\omega$ | rad/s |
| Período | $T = 2\pi/\omega$ | tiempo de una oscilación |
| Frecuencia | $f = 1/T$ | oscilaciones por segundo (Hz) |
| Fase inicial | $\varphi$ | fija la posición en $t = 0$ |

Derivando:

$$v(t) = -A\omega\sin(\omega t + \varphi) \qquad a(t) = -A\omega^2\cos(\omega t + \varphi)$$

| Posición | Velocidad | Aceleración |
|---|---|---|
| Extremos ($x = \pm A$) | $0$ | máxima: $A\omega^2$ |
| Equilibrio ($x = 0$) | máxima: $A\omega$ | $0$ |

## Masa-resorte

$$\omega = \sqrt{\frac{k}{m}} \qquad T = 2\pi\sqrt{\frac{m}{k}}$$

El período **no depende de la amplitud**. Un resorte vertical oscila igual, alrededor de su nueva posición de equilibrio.

> [!NOTE]
> Una masa de $0{,}5$ kg cuelga de un resorte de $k = 200$ N/m.
>
> $\omega = \sqrt{400} = 20$ rad/s; $T = \dfrac{2\pi}{20} = 0{,}314$ s; $f = 3{,}18$ Hz.
>
> Si la amplitud es $5$ cm: $v_{máx} = 0{,}05 \cdot 20 = 1$ m/s.

## Péndulo simple

Para ángulos pequeños ($\sin\theta \approx \theta$, hasta unos $15°$):

$$T = 2\pi\sqrt{\frac{L}{g}}$$

No depende de la masa ni de la amplitud. Un péndulo de $1$ m tiene un período de cerca de $2$ s. Midiendo $T$ se obtiene $g$.

**Péndulo físico** (un cuerpo rígido que oscila): $T = 2\pi\sqrt{\dfrac{I}{mgd}}$, con $d$ la distancia del eje al centro de masa.

## Energía

Sin rozamiento, la energía mecánica se conserva y pasa continuamente de cinética a potencial:

$$E = \tfrac{1}{2}mv^2 + \tfrac{1}{2}kx^2 = \tfrac{1}{2}kA^2$$

La energía es proporcional al **cuadrado de la amplitud**. De aquí sale la velocidad en cualquier posición:

$$v = \pm\omega\sqrt{A^2 - x^2}$$

## Oscilaciones amortiguadas

Con una fuerza de rozamiento $-bv$, la amplitud decae exponencialmente:

$$x(t) = Ae^{-\frac{b}{2m}t}\cos(\omega't + \varphi)$$

| Régimen | Comportamiento |
|---|---|
| Subamortiguado | oscila con amplitud decreciente |
| Crítico | vuelve al equilibrio lo más rápido posible sin oscilar |
| Sobreamortiguado | vuelve lentamente sin oscilar |

Los amortiguadores de un auto y el cierre de una puerta se diseñan cerca del régimen crítico.

## Oscilaciones forzadas y resonancia

Si una fuerza externa periódica actúa sobre el oscilador, este termina oscilando a la frecuencia de la fuerza.

> [!IMPORTANT]
> **Resonancia**: con amortiguamiento débil, la amplitud es máxima cuando la frecuencia de la fuerza es (prácticamente) igual a la **frecuencia natural** del sistema.

Cuanto menor es el amortiguamiento, más alto y angosto es el pico, y más cerca queda de la frecuencia natural; con amortiguamiento fuerte el pico desaparece.

| Resonancia útil | Resonancia peligrosa |
|---|---|
| sintonía de radio | vibración de puentes y edificios |
| instrumentos musicales | máquinas rotativas desbalanceadas |
| resonancia magnética | copas que se rompen con el sonido |
| hamacarse | |

> [!WARNING]
> El período del péndulo solo es independiente de la amplitud para ángulos pequeños. Para oscilaciones grandes, el período aumenta.

> [!TIP]
> Todo sistema cerca de un equilibrio estable se comporta como un resorte. Por eso el MAS aparece en átomos, circuitos, estructuras y ondas.
