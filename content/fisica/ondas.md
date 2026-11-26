+++
title = "Ondas"
summary = "Amplitud, longitud de onda, frecuencia y velocidad; superposición, ondas estacionarias y efecto Doppler."
branch = "Ondas y óptica"
order = 1
level = "Intermedio"
tags = ["frecuencia", "longitud de onda", "interferencia", "sonido"]
+++

## Qué es una onda

Una **onda** es una perturbación que se propaga transportando energía, pero no materia. Las ondas **mecánicas** (sonido, olas, cuerdas) necesitan un medio; las **electromagnéticas** (luz, radio) viajan también en el vacío. Si la oscilación es perpendicular a la propagación la onda es **transversal**; si es paralela, **longitudinal** (como el sonido).

![Onda sinusoidal con su amplitud y su longitud de onda](/media/fisica/onda.svg)

*La longitud de onda $\lambda$ es la distancia entre dos crestas; la amplitud $A$, la separación máxima del equilibrio.*

## Magnitudes

| Magnitud | Símbolo | Unidad | Significado |
|---|---|---|---|
| Amplitud | $A$ | m | desplazamiento máximo |
| Longitud de onda | $\lambda$ | m | distancia que ocupa un ciclo |
| Período | $T$ | s | tiempo de un ciclo |
| Frecuencia | $f = 1/T$ | Hz | ciclos por segundo |

La relación fundamental entre ellas es

$$v = \lambda f$$

La velocidad $v$ depende del medio, no de la fuente: el sonido viaja a unos 343 m/s en el aire y la luz a $c \approx 3 \times 10^8$ m/s en el vacío.

Una onda armónica que avanza hacia $+x$ se describe con

$$y(x, t) = A\sin(kx - \omega t), \qquad k = \frac{2\pi}{\lambda},\quad \omega = 2\pi f$$

> [!NOTE]
> La nota *la* tiene $f = 440$ Hz. Su longitud de onda en el aire es $\lambda = \dfrac{343}{440} \approx 0{,}78$ m.

## Superposición e interferencia

Cuando dos ondas coinciden en un punto, sus desplazamientos se suman. Si llegan en fase se refuerzan (**interferencia constructiva**); si llegan en oposición de fase se cancelan (**destructiva**).

## Ondas estacionarias

En una cuerda de longitud $L$ fija en ambos extremos, la onda y su reflexión forman un patrón que no avanza, con nodos fijos. Solo caben ciertas longitudes de onda:

$$\lambda_n = \frac{2L}{n}, \qquad f_n = n\,\frac{v}{2L}, \qquad n = 1, 2, 3, \dots$$

$f_1$ es la frecuencia **fundamental** y las demás sus **armónicos**. Así funcionan las cuerdas de una guitarra.

## Efecto Doppler

Si la fuente y el observador se acercan, la frecuencia percibida aumenta; si se alejan, disminuye. Para el sonido:

$$f' = f\,\frac{v \pm v_o}{v \mp v_f}$$

con los signos superiores cuando se acercan. Es el cambio de tono de una sirena al pasar.
