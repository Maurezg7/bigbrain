+++
title = "Cinética química"
summary = "Velocidad de reacción, ley de velocidad, órdenes de reacción, vida media, energía de activación, ecuación de Arrhenius, mecanismos y catálisis."
branch = "Termoquímica y cinética"
order = 2
level = "Intermedio"
tags = ["velocidad de reacción", "orden de reacción", "energía de activación", "Arrhenius", "catalizador"]
+++

## Velocidad de reacción

La termodinámica dice si una reacción puede ocurrir; la **cinética**, cuán rápido lo hace.

La velocidad es el cambio de concentración por unidad de tiempo. Para $aA + bB \to cC + dD$:

$$v = -\frac{1}{a}\frac{d[A]}{dt} = -\frac{1}{b}\frac{d[B]}{dt} = \frac{1}{c}\frac{d[C]}{dt} = \frac{1}{d}\frac{d[D]}{dt}$$

Se mide en mol/(L·s).

### Factores que la afectan

| Factor | Efecto | Por qué |
|---|---|---|
| Concentración | en general, más concentración, más velocidad | más choques |
| Temperatura | aumenta mucho | más moléculas con energía suficiente |
| Superficie de contacto | el sólido dividido reacciona más rápido | más área expuesta |
| Catalizador | aumenta | ofrece un camino con menor energía de activación |
| Naturaleza de los reactivos | varía | enlaces más o menos fáciles de romper |

## Teoría de las colisiones

Para reaccionar, las moléculas deben:

1. **Chocar**.
2. Hacerlo con **energía suficiente**: al menos la **energía de activación** $E_a$.
3. Tener la **orientación** adecuada.

En el punto de máxima energía se forma el **complejo activado** o **estado de transición**: una especie inestable con enlaces a medio romper y a medio formar.

La $E_a$ es la barrera que separa reactivos de productos. No tiene relación con $\Delta H$: una reacción muy exotérmica puede ser lentísima (la nafta no arde sin una chispa).

## Ley de velocidad

> [!IMPORTANT]
> $$v = k\,[A]^m[B]^n$$

- $k$ es la **constante de velocidad**: depende de la temperatura, no de las concentraciones.
- $m$ y $n$ son los **órdenes** respecto de cada reactivo. La suma es el **orden global**.

> [!WARNING]
> Los órdenes **no** son los coeficientes de la ecuación balanceada. Se determinan experimentalmente.

### Método de las velocidades iniciales

Se compara cómo cambia la velocidad al variar la concentración de un solo reactivo.

> [!NOTE]
> | Experimento | [A] (M) | [B] (M) | Velocidad inicial (M/s) |
> |---|---|---|---|
> | 1 | 0,10 | 0,10 | $2{,}0 \times 10^{-3}$ |
> | 2 | 0,20 | 0,10 | $8{,}0 \times 10^{-3}$ |
> | 3 | 0,10 | 0,20 | $4{,}0 \times 10^{-3}$ |
>
> De 1 a 2 se duplica [A] y la velocidad se cuadruplica: orden **2** en A.
>
> De 1 a 3 se duplica [B] y la velocidad se duplica: orden **1** en B.
>
> $v = k[A]^2[B]$, con $k = \dfrac{2{,}0 \times 10^{-3}}{(0{,}10)^2(0{,}10)} = 2{,}0\ \text{M}^{-2}\text{s}^{-1}$.

## Leyes integradas

Dan la concentración en función del tiempo, para un solo reactivo.

| Orden | Ley de velocidad | Ley integrada | Gráfico recto | Vida media |
|---|---|---|---|---|
| 0 | $v = k$ | $[A] = [A]_0 - kt$ | $[A]$ contra $t$ | $\dfrac{[A]_0}{2k}$ |
| 1 | $v = k[A]$ | $\ln[A] = \ln[A]_0 - kt$ | $\ln[A]$ contra $t$ | $\dfrac{0{,}693}{k}$ |
| 2 | $v = k[A]^2$ | $\dfrac{1}{[A]} = \dfrac{1}{[A]_0} + kt$ | $\dfrac{1}{[A]}$ contra $t$ | $\dfrac{1}{k[A]_0}$ |

Para saber el orden, se grafican los datos de las tres formas y se ve cuál da una recta.

La **vida media** $t_{1/2}$ es el tiempo en que la concentración cae a la mitad. En las reacciones de **primer orden** es constante: no depende de la concentración inicial. Es el caso de la desintegración radiactiva y de la eliminación de muchos fármacos.

> [!NOTE]
> Una reacción de primer orden tiene $k = 0{,}0231\ \text{min}^{-1}$.
>
> $t_{1/2} = \dfrac{0{,}693}{0{,}0231} = 30$ min. A los $90$ min (tres vidas medias) queda $\tfrac{1}{8}$ del reactivo.

## Temperatura: ecuación de Arrhenius

> [!IMPORTANT]
> $$k = A\,e^{-E_a/RT}$$

$A$ es el **factor de frecuencia** (choques con la orientación correcta) y $R = 8{,}314$ J/(mol·K).

En forma lineal: $\ln k = \ln A - \dfrac{E_a}{R}\cdot\dfrac{1}{T}$. Graficando $\ln k$ contra $\dfrac{1}{T}$, la pendiente es $-\dfrac{E_a}{R}$.

Con dos temperaturas:

$$\ln\frac{k_2}{k_1} = \frac{E_a}{R}\left(\frac{1}{T_1} - \frac{1}{T_2}\right)$$

> [!NOTE]
> Una reacción duplica su constante al pasar de $25$ a $35$ °C.
>
> $\ln 2 = \dfrac{E_a}{8{,}314}\left(\dfrac{1}{298} - \dfrac{1}{308}\right) \Rightarrow E_a = 52{,}9$ kJ/mol.

Regla práctica aproximada: muchas reacciones duplican su velocidad por cada 10 °C. Por eso la heladera conserva los alimentos.

## Mecanismos de reacción

La mayoría de las reacciones ocurre en varias **etapas elementales**. El conjunto es el **mecanismo**.

- En una etapa elemental, los órdenes **sí** coinciden con los coeficientes (su **molecularidad**).
- Un **intermediario** se forma en una etapa y se consume en otra; no aparece en la ecuación global.
- La **etapa determinante** es la más lenta: fija la velocidad de toda la reacción.

> [!NOTE]
> Para NO₂ + CO → NO + CO₂ se observa $v = k[\text{NO}_2]^2$ (no interviene el CO). Mecanismo propuesto:
>
> 1. NO₂ + NO₂ → NO₃ + NO (lenta)
> 2. NO₃ + CO → NO₂ + CO₂ (rápida)
>
> La etapa lenta da $v = k[\text{NO}_2]^2$, que coincide. NO₃ es un intermediario.

Un mecanismo es válido si sus etapas suman la reacción global y predicen la ley de velocidad observada. Nunca se «demuestra»: solo es compatible con los datos.

## Catálisis

Un **catalizador** aumenta la velocidad sin consumirse. Ofrece un mecanismo alternativo con **menor energía de activación**.

> [!IMPORTANT]
> Un catalizador **no** cambia $\Delta H$ ni la posición del equilibrio: acelera por igual la reacción directa y la inversa.

| Tipo | Característica | Ejemplo |
|---|---|---|
| Homogénea | catalizador en la misma fase | ácidos en la hidrólisis de ésteres |
| Heterogénea | catalizador en otra fase (sólido) | platino en el convertidor catalítico; hierro en la síntesis de amoníaco |
| Enzimática | proteínas muy específicas | amilasa, catalasa |

En la catálisis heterogénea los reactivos se **adsorben** en la superficie, donde sus enlaces se debilitan.

Las **enzimas** aceleran las reacciones biológicas millones de veces, en condiciones suaves. Cada una actúa sobre un **sustrato** que encaja en su **sitio activo**.

Un **inhibidor** disminuye la velocidad: así actúan muchos medicamentos y venenos.

> [!TIP]
> Si al duplicar una concentración la velocidad no cambia, el orden es 0; si se duplica, 1; si se cuadruplica, 2.
