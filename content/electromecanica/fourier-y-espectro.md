+++
title = "Series de Fourier y espectro de una señal"
summary = "Cualquier señal periódica es una suma de senoides: armónicos, espectro, distorsión y muestreo."
branch = "Sistemas y señales"
order = 3
level = "Avanzado"
tags = ["Fourier", "armónicos", "espectro", "THD", "muestreo"]
+++

## Señales como suma de senoides

![Una onda cuadrada aproximada por la suma de sus primeros armónicos impares](/media/electromecanica/armonicos.svg)

*Sumando la fundamental y los armónicos 3 y 5 ya se reconoce la onda cuadrada.*

Toda señal periódica de período $T$ (con $\omega_0 = 2\pi/T$) puede escribirse como una componente continua más senoides de frecuencias múltiplos de la fundamental, los **armónicos**:

$$f(t) = \frac{a_0}{2} + \sum_{n=1}^{\infty}\left[a_n\cos(n\omega_0 t) + b_n\sin(n\omega_0 t)\right]$$

$$a_n = \frac{2}{T}\int_0^T f(t)\cos(n\omega_0 t)\,dt,\qquad b_n = \frac{2}{T}\int_0^T f(t)\sin(n\omega_0 t)\,dt$$

Como los sistemas lineales responden a cada senoide por separado, basta conocer la respuesta en frecuencia para saber qué le hacen a cualquier señal.

## Simetrías que ahorran trabajo

| Simetría | Consecuencia |
|---|---|
| Par, $f(-t) = f(t)$ | solo cosenos: $b_n = 0$ |
| Impar, $f(-t) = -f(t)$ | solo senos: $a_n = 0$ |
| Media onda, $f(t + T/2) = -f(t)$ | solo armónicos impares |

Una onda cuadrada de amplitud $A$ tiene simetría impar y de media onda:

$$f(t) = \frac{4A}{\pi}\left[\sin(\omega_0 t) + \frac{1}{3}\sin(3\omega_0 t) + \frac{1}{5}\sin(5\omega_0 t) + \dots\right]$$

## Valor eficaz y distorsión

Por el teorema de Parseval, el valor eficaz total se obtiene sumando en cuadratura los de cada componente:

$$F_{ef} = \sqrt{F_0^2 + F_1^2 + F_2^2 + \dots}$$

La **distorsión armónica total** mide cuánto se aparta una onda de la senoide pura:

$$THD = \frac{\sqrt{F_2^2 + F_3^2 + F_4^2 + \dots}}{F_1}$$

> [!NOTE]
> Una corriente tiene fundamental de 10 A, tercer armónico de 4 A y quinto de 3 A (valores eficaces).
>
> $THD = \dfrac{\sqrt{4^2 + 3^2}}{10} = 50\ \%$ y el valor eficaz total es $\sqrt{10^2 + 4^2 + 3^2} \approx 11{,}2$ A.

> [!WARNING]
> Las cargas no lineales (rectificadores, variadores, fuentes conmutadas, LED) inyectan armónicos en la red. Los de orden 3, 9, 15… de las tres fases se **suman** en el neutro, que puede sobrecalentarse aunque las fases estén equilibradas.

## Transformada de Fourier

Para señales no periódicas, la suma se vuelve una integral y el espectro es continuo:

$$F(\omega) = \int_{-\infty}^{\infty} f(t)\,e^{-j\omega t}\,dt$$

Una regla general: lo que es **corto en el tiempo es ancho en frecuencia**. Un pulso estrecho o un flanco abrupto contienen frecuencias muy altas.

## Muestreo

Para procesar una señal en una computadora o un PLC se la muestrea cada $T_s$ segundos. El teorema de Nyquist-Shannon exige

$$f_s > 2 f_{max}$$

Si no se cumple aparece el **aliasing**: las frecuencias altas se disfrazan de bajas y la información se pierde. Por eso antes del conversor se coloca un filtro pasabajos (antialiasing). El espectro de una señal muestreada se calcula con la transformada rápida de Fourier (FFT), herramienta básica del análisis de vibraciones y de la calidad de energía.
