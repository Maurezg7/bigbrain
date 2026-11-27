+++
title = "Mecánica cuántica"
summary = "Dualidad onda-partícula, principio de incertidumbre, función de onda, ecuación de Schrödinger, pozo de potencial, efecto túnel y el átomo."
branch = "Física moderna"
order = 3
level = "Avanzado"
tags = ["mecánica cuántica", "Schrödinger", "incertidumbre", "función de onda", "efecto túnel"]
+++

## Los orígenes

A comienzos del siglo XX, varios experimentos no podían explicarse con la física clásica:

| Fenómeno | Explicación | Autor |
|---|---|---|
| Radiación de cuerpo negro | la energía se emite en cuantos $E = hf$ | Planck (1900) |
| Efecto fotoeléctrico | la luz está formada por fotones | Einstein (1905) |
| Espectros atómicos | los electrones ocupan niveles discretos | Bohr (1913) |
| Efecto Compton | los fotones tienen cantidad de movimiento | Compton (1923) |

La **constante de Planck** es $h = 6{,}626 \times 10^{-34}$ J·s, y $\hbar = \dfrac{h}{2\pi}$.

## Dualidad onda-partícula

La luz se comporta como onda (interferencia) y como partícula (fotones):

$$E = hf \qquad p = \frac{h}{\lambda}$$

De Broglie propuso en 1924 que la materia también tiene propiedades ondulatorias:

> [!IMPORTANT]
> $$\lambda = \frac{h}{p} = \frac{h}{mv}$$

Se confirmó al observar difracción de electrones en cristales.

> [!NOTE]
> Un electrón a $10^6$ m/s: $\lambda = \dfrac{6{,}626 \times 10^{-34}}{9{,}11 \times 10^{-31} \cdot 10^6} = 0{,}73$ nm, del orden del tamaño de un átomo.
>
> Una pelota de $0{,}15$ kg a $40$ m/s: $\lambda \approx 10^{-34}$ m. Imposible de detectar: por eso los objetos cotidianos no muestran efectos cuánticos.

En el experimento de la doble rendija, los electrones enviados **de a uno** forman igualmente un patrón de interferencia. Pero si se detecta por qué rendija pasa cada uno, el patrón desaparece.

## Principio de incertidumbre

> [!IMPORTANT]
> **Heisenberg**:
> $$\Delta x\,\Delta p \ge \frac{\hbar}{2} \qquad \Delta E\,\Delta t \ge \frac{\hbar}{2}$$

No se pueden conocer con precisión arbitraria la posición y la cantidad de movimiento a la vez. No es una limitación de los instrumentos: es una propiedad de la naturaleza. Una partícula **no tiene** trayectoria definida.

> [!NOTE]
> Un electrón confinado en un átomo ($\Delta x \approx 10^{-10}$ m): $\Delta p \ge \dfrac{1{,}05 \times 10^{-34}}{2 \times 10^{-10}} = 5{,}3 \times 10^{-25}$ kg·m/s, lo que da $\Delta v \approx 6 \times 10^5$ m/s.

## La función de onda

El estado de una partícula se describe con una **función de onda** $\Psi(x, t)$, en general compleja.

> [!IMPORTANT]
> **Interpretación de Born**: $\lvert\Psi\rvert^2$ es la **densidad de probabilidad** de encontrar la partícula.
> $$P(a \le x \le b) = \int_a^b \lvert\Psi\rvert^2\,dx \qquad \int_{-\infty}^{\infty}\lvert\Psi\rvert^2\,dx = 1$$

La mecánica cuántica no predice el resultado de una medición individual, sino las **probabilidades** de cada resultado.

- **Superposición**: si dos estados son posibles, también lo es cualquier combinación de ellos.
- **Medición**: al medir, se obtiene uno solo de los valores posibles y el estado cambia.

## Ecuación de Schrödinger

Es la ecuación fundamental, el equivalente de la segunda ley de Newton:

$$i\hbar\,\frac{\partial\Psi}{\partial t} = -\frac{\hbar^2}{2m}\frac{\partial^2\Psi}{\partial x^2} + U(x)\,\Psi$$

Para estados de energía definida (**estacionarios**), se reduce a la ecuación independiente del tiempo:

> [!IMPORTANT]
> $$-\frac{\hbar^2}{2m}\frac{d^2\psi}{dx^2} + U(x)\,\psi = E\,\psi$$

Es un problema de autovalores: solo ciertas energías $E$ tienen solución aceptable. Así aparece la **cuantización**.

## Partícula en una caja

Una partícula confinada entre paredes impenetrables en $x = 0$ y $x = L$. La función de onda debe anularse en las paredes, como una cuerda fija en los extremos:

$$\psi_n(x) = \sqrt{\frac{2}{L}}\sin\left(\frac{n\pi x}{L}\right) \qquad E_n = \frac{n^2h^2}{8mL^2}, \quad n = 1, 2, 3, \dots$$

Conclusiones:

- La energía está **cuantizada**: solo valores discretos.
- La energía mínima **no es cero** (energía de punto cero): una partícula confinada no puede estar quieta.
- Cuanto más chica la caja, mayores las energías y su separación.

> [!NOTE]
> Un electrón en una caja de $1$ nm: $E_1 = \dfrac{(6{,}626 \times 10^{-34})^2}{8 \cdot 9{,}11 \times 10^{-31} \cdot (10^{-9})^2} = 6{,}0 \times 10^{-20}$ J $= 0{,}38$ eV.
>
> Los niveles son $E_n = 0{,}38\,n^2$ eV. Al pasar de $n = 2$ a $n = 1$ emite un fotón de $1{,}13$ eV (infrarrojo).

Es el principio de los **puntos cuánticos**: su color depende del tamaño.

## Efecto túnel

Una partícula puede atravesar una barrera de energía mayor que la suya, algo imposible clásicamente. La función de onda decae exponencialmente dentro de la barrera, pero no llega a cero. Para una barrera ancha de altura $U_0$ y ancho $L$, la probabilidad de transmisión es aproximadamente

$$T \approx e^{-2\kappa L}, \qquad \kappa = \frac{\sqrt{2m(U_0 - E)}}{\hbar}$$

Depende muy fuertemente del ancho y de la masa.

| Dónde aparece | Qué hace |
|---|---|
| Desintegración alfa | la partícula escapa del núcleo |
| Fusión en el Sol | los protones vencen su repulsión |
| Microscopio de efecto túnel | ve átomos individuales |
| Memorias flash, diodo túnel | los electrones cruzan capas aislantes |

## Oscilador armónico cuántico

$$E_n = \left(n + \tfrac{1}{2}\right)\hbar\omega, \qquad n = 0, 1, 2, \dots$$

Niveles igualmente espaciados. Describe las vibraciones de moléculas y redes cristalinas.

## El átomo de hidrógeno

Resolviendo la ecuación de Schrödinger en tres dimensiones:

$$E_n = -\frac{13{,}6\ \text{eV}}{n^2}$$

El estado del electrón queda definido por cuatro **números cuánticos**:

| Número | Símbolo | Valores | Determina |
|---|---|---|---|
| Principal | $n$ | $1, 2, 3, \dots$ | energía y tamaño |
| Orbital | $l$ | $0$ a $n - 1$ | forma del orbital (s, p, d, f) |
| Magnético | $m_l$ | $-l$ a $+l$ | orientación |
| De espín | $m_s$ | $\pm\tfrac{1}{2}$ | espín |

Los electrones no describen órbitas: ocupan **orbitales**, regiones de probabilidad.

**Principio de exclusión de Pauli**: dos electrones no pueden tener los cuatro números cuánticos iguales. Explica la estructura de la tabla periódica y la estabilidad de la materia.

## Aplicaciones

| Tecnología | Fundamento cuántico |
|---|---|
| Láser | emisión estimulada entre niveles |
| Transistores y chips | bandas de energía en semiconductores |
| LED y celdas solares | transiciones en junturas |
| Resonancia magnética | espín nuclear |
| Relojes atómicos | frecuencia de una transición |
| Computación cuántica | superposición y entrelazamiento |

> [!WARNING]
> La mecánica cuántica no dice que «todo es incierto». Sus predicciones estadísticas son las más precisas de toda la ciencia.

> [!TIP]
> Confinar una partícula cuantiza su energía, igual que fijar los extremos de una cuerda selecciona sus armónicos.
