+++
title = "Resonancia, filtros y diagramas de Bode"
summary = "Respuesta en frecuencia de circuitos: resonancia serie y paralelo, factor de calidad, filtros y decibeles."
branch = "Sistemas y señales"
order = 4
level = "Intermedio"
tags = ["resonancia", "filtros", "Bode", "decibel", "frecuencia de corte"]
+++

## Respuesta en frecuencia

La reactancia de un inductor crece con la frecuencia ($X_L = \omega L$) y la de un capacitor disminuye ($X_C = 1/\omega C$). Por eso un mismo circuito trata distinto a cada frecuencia: es un **filtro**.

## Resonancia

![Curva de corriente en función de la frecuencia con un pico en la frecuencia de resonancia y el ancho de banda marcado](/media/electromecanica/resonancia.svg)

*En resonancia la corriente es máxima; el ancho de banda se mide donde cae a 0,707 del máximo.*

En un RLC serie, cuando $X_L = X_C$ las reactancias se cancelan, la impedancia se reduce a $R$ y la corriente es máxima. Eso ocurre a

$$f_0 = \frac{1}{2\pi\sqrt{LC}}$$

El **factor de calidad** indica cuán selectivo es el circuito:

$$Q = \frac{\omega_0 L}{R} = \frac{1}{R}\sqrt{\frac{L}{C}},\qquad BW = \frac{f_0}{Q}$$

En resonancia, la tensión sobre el inductor y sobre el capacitor vale $Q$ veces la de la fuente.

> [!NOTE]
> $R = 10\ \Omega$, $L = 10$ mH, $C = 1\ \mu$F en serie.
>
> $f_0 = \dfrac{1}{2\pi\sqrt{10^{-2}\cdot 10^{-6}}} \approx 1592$ Hz, $Q = \dfrac{1}{10}\sqrt{\dfrac{10^{-2}}{10^{-6}}} = 10$ y $BW \approx 159$ Hz.
>
> Con 1 V de fuente hay 10 V sobre el capacitor.

> [!WARNING]
> En instalaciones con bancos de capacitores, la inductancia de la red puede entrar en resonancia con ellos a la frecuencia de algún armónico y amplificarlo. Se evita con reactores de desintonía en serie.

En el circuito **paralelo** ocurre lo contrario: en resonancia la impedancia es máxima y la corriente tomada de la fuente, mínima.

## Filtros

| Tipo | Deja pasar | Ejemplo simple |
|---|---|---|
| Pasabajos | frecuencias menores que $f_c$ | RC con salida en el capacitor |
| Pasaaltos | frecuencias mayores que $f_c$ | RC con salida en la resistencia |
| Pasabanda | una banda alrededor de $f_0$ | RLC serie con salida en la resistencia |
| Eliminabanda | todo menos una banda | RLC con salida en L y C |

Para el pasabajos RC:

$$G(j\omega) = \frac{1}{1 + j\omega RC},\qquad |G| = \frac{1}{\sqrt{1 + (\omega RC)^2}},\qquad f_c = \frac{1}{2\pi RC}$$

En la **frecuencia de corte** la ganancia vale $1/\sqrt2 \approx 0{,}707$ y la potencia se reduce a la mitad.

## Decibeles

Las ganancias se expresan en escala logarítmica:

$$G_{dB} = 20\log_{10}\left|\frac{V_{sal}}{V_{ent}}\right| = 10\log_{10}\frac{P_{sal}}{P_{ent}}$$

| Relación de tensiones | 0,1 | 0,707 | 1 | 2 | 10 | 100 |
|---|---|---|---|---|---|---|
| dB | −20 | −3 | 0 | +6 | +20 | +40 |

## Diagrama de Bode

Son dos gráficos con la frecuencia en escala logarítmica: ganancia en dB y fase en grados. Su ventaja es que se trazan con rectas (asíntotas):

- cada **polo** hace caer la ganancia a −20 dB por década a partir de su frecuencia y resta hasta 90° de fase;
- cada **cero** la hace subir +20 dB por década y suma hasta 90°;
- las ganancias de etapas en cascada, al estar en dB, simplemente se suman.

Un filtro de primer orden atenúa 20 dB por década; uno de orden $n$, $20n$ dB por década.
