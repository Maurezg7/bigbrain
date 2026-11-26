+++
title = "Estabilidad y respuesta de sistemas de control"
summary = "Diagramas de bloques, sistemas de primer y segundo orden, polos, criterio de Routh, márgenes de ganancia y fase, y error estacionario."
branch = "Instrumentación y control automático"
order = 3
level = "Avanzado"
tags = ["estabilidad", "polos", "Routh", "Bode", "margen de fase"]
+++

## Diagramas de bloques

Cada componente del lazo se representa con su función de transferencia. Las reglas de reducción son:

| Conexión | Equivalente |
|---|---|
| Serie | $G_1\,G_2$ |
| Paralelo | $G_1 + G_2$ |
| Realimentación negativa | $\dfrac{G}{1 + G\,H}$ |

Con $G$ en el camino directo y $H$ en la realimentación, la función de transferencia de lazo cerrado es

$$T(s) = \frac{G(s)}{1 + G(s)\,H(s)}$$

El denominador igualado a cero es la **ecuación característica**: sus raíces son los polos de lazo cerrado y deciden el comportamiento.

## Sistema de primer orden

$$G(s) = \frac{K}{\tau s + 1}$$

Ante un escalón, la salida se acerca al valor final de forma exponencial, sin oscilar:

$$y(t) = K\left(1 - e^{-t/\tau}\right)$$

Alcanza el 63 % en $\tau$ y el 98 % en $4\tau$. Así responden un tanque que se llena, un horno o un circuito RC. Muchos procesos reales agregan un **tiempo muerto** $L$ (transporte), que se modela con $e^{-Ls}$ y es lo que más dificulta el control.

## Sistema de segundo orden

![Respuesta al escalón de un sistema de segundo orden para distintos factores de amortiguamiento](/media/electromecanica/segundo-orden.svg)

*Con poco amortiguamiento la respuesta es rápida pero oscila; con mucho, es lenta.*

$$G(s) = \frac{\omega_n^2}{s^2 + 2\zeta\omega_n s + \omega_n^2}$$

$\omega_n$ es la frecuencia natural y $\zeta$ el factor de amortiguamiento.

| $\zeta$ | Respuesta |
|---|---|
| 0 | oscila indefinidamente |
| Entre 0 y 1 | subamortiguada: sobrepasa y oscila |
| 1 | críticamente amortiguada: la más rápida sin sobrepasar |
| Mayor que 1 | sobreamortiguada: lenta |

Para el caso subamortiguado:

$$M_p = e^{-\pi\zeta/\sqrt{1 - \zeta^2}},\qquad t_s\approx\frac{4}{\zeta\,\omega_n},\qquad t_p = \frac{\pi}{\omega_n\sqrt{1 - \zeta^2}}$$

con $M_p$ el sobrepaso máximo, $t_s$ el tiempo de establecimiento al 2 % y $t_p$ el tiempo de pico.

> [!NOTE]
> Con $\zeta = 0{,}5$ y $\omega_n = 4$ rad/s: $M_p = e^{-\pi\cdot 0{,}5/0{,}866}\approx 16{,}3\ \%$, $t_s\approx 2$ s y $t_p\approx 0{,}91$ s.

Un buen compromiso habitual es $\zeta$ entre 0,5 y 0,7.

## Estabilidad

Un sistema es **estable** si ante una entrada acotada la salida permanece acotada. Equivale a que **todos los polos de lazo cerrado tengan parte real negativa**, es decir, estén en el semiplano izquierdo.

| Ubicación del polo | Respuesta asociada |
|---|---|
| Real negativo | exponencial que se extingue |
| Complejo con parte real negativa | oscilación amortiguada |
| Sobre el eje imaginario | oscilación sostenida |
| Con parte real positiva | crece sin límite: inestable |

Cuanto más a la izquierda, más rápido se extingue; cuanto mayor la parte imaginaria, más rápido oscila.

## Criterio de Routh-Hurwitz

Permite saber si hay polos en el semiplano derecho sin calcularlos. Para un polinomio característico de tercer orden

$$a_3 s^3 + a_2 s^2 + a_1 s + a_0 = 0$$

con todos los coeficientes positivos, el sistema es estable si

$$a_2\,a_1 > a_3\,a_0$$

> [!NOTE]
> Lazo con $G(s) = \dfrac{K}{s\,(s + 1)(s + 2)}$ y realimentación unitaria. La ecuación característica es $s^3 + 3s^2 + 2s + K = 0$.
>
> Estable si $3\cdot 2 > K$: la ganancia debe cumplir $0 < K < 6$. Con $K = 6$ el lazo oscila de forma sostenida a $\omega = \sqrt2$ rad/s.

Aumentar la ganancia hace al lazo más rápido y exacto, pero lo acerca a la inestabilidad: ese es el compromiso central del control.

## Lugar de las raíces

Muestra cómo se mueven los polos de lazo cerrado al variar la ganancia $K$ desde cero. Parten de los polos de lazo abierto y terminan en los ceros o en el infinito. Permite elegir la ganancia que da el amortiguamiento deseado.

## Respuesta en frecuencia

El diagrama de **Bode** del lazo abierto $G(j\omega)H(j\omega)$ grafica la ganancia en decibeles y la fase en función de la frecuencia. De él se leen dos márgenes:

| Margen | Definición | Valor recomendado |
|---|---|---|
| De fase | cuánto falta para −180° a la frecuencia donde la ganancia es 0 dB | 45° a 60° |
| De ganancia | cuánto puede aumentar la ganancia cuando la fase vale −180° | más de 6 dB |

Miden cuán lejos está el lazo de la inestabilidad y, por lo tanto, cuánto tolera errores de modelado. El **criterio de Nyquist** generaliza esta idea contando las vueltas del diagrama polar alrededor del punto $-1$.

## Error en estado estacionario

El error permanente depende del **tipo** del sistema, que es el número de integradores en el lazo abierto:

| Tipo | Error ante un escalón | Error ante una rampa |
|---|---|---|
| 0 | $\dfrac{1}{1 + K_p}$ | infinito |
| 1 | 0 | $\dfrac{1}{K_v}$ |
| 2 | 0 | 0 |

Por eso la acción **integral** de un PID elimina el error permanente ante cambios en escalón: agrega un integrador al lazo.

## Compensadores

| Compensador | Efecto |
|---|---|
| De adelanto | agrega fase: mejora la estabilidad y la velocidad |
| De atraso | aumenta la ganancia a baja frecuencia: reduce el error |
| PID | combina ambos efectos |

## Control digital

Hoy los controladores son digitales: muestrean cada $T$ segundos. El muestreo agrega un retardo de aproximadamente $T/2$, que resta margen de fase. Como regla, se muestrea al menos diez veces más rápido que el ancho de banda del lazo. El análisis se hace con la transformada $z$, donde la región estable es el interior del círculo unitario.
