+++
title = "Espontaneidad y energía libre"
summary = "Procesos espontáneos, entropía, segunda ley, energía libre de Gibbs y su relación con el equilibrio."
branch = "Termoquímica y cinética"
order = 3
level = "Avanzado"
tags = ["entropía", "energía libre", "Gibbs", "espontaneidad", "segunda ley"]
+++

## Procesos espontáneos

Un proceso **espontáneo** ocurre por sí solo, sin intervención externa continua. El inverso no lo es.

- El hielo se funde a 25 °C; el agua no se congela sola a esa temperatura.
- El hierro se oxida al aire; el óxido no vuelve solo a hierro.

> [!WARNING]
> Espontáneo no significa rápido. El diamante se convierte espontáneamente en grafito, pero tarda millones de años. La velocidad es tema de la cinética.

Muchas reacciones espontáneas son exotérmicas, pero no todas: el hielo se funde absorbiendo calor. Hace falta otro criterio.

## Entropía

La **entropía** $S$ mide la dispersión de la energía: el número de formas microscópicas en que puede organizarse un sistema.

$$S = k_B\ln W$$

con $W$ la cantidad de microestados. Se mide en J/(mol·K).

La entropía **aumenta** cuando:

| Cambio | Ejemplo |
|---|---|
| sólido → líquido → gas | fusión, vaporización |
| aumenta la cantidad de moles de gas | CaCO₃(s) → CaO(s) + CO₂(g) |
| aumenta la temperatura | |
| un sólido se disuelve | en general |
| un gas se expande | |

> [!NOTE]
> Predecir el signo de $\Delta S$:
>
> - 2 H₂(g) + O₂(g) → 2 H₂O(l): de 3 moles de gas a 0. $\Delta S < 0$.
> - N₂O₄(g) → 2 NO₂(g): de 1 mol de gas a 2. $\Delta S > 0$.

**Tercera ley**: la entropía de un cristal perfecto a 0 K es cero. Por eso hay entropías absolutas $S^0$, y la de los elementos **no** es cero.

$$\Delta S^0_{reacción} = \sum n\,S^0(\text{productos}) - \sum n\,S^0(\text{reactivos})$$

## Segunda ley

> [!IMPORTANT]
> En todo proceso espontáneo, la entropía del **universo** aumenta:
> $$\Delta S_{universo} = \Delta S_{sistema} + \Delta S_{entorno} > 0$$

La entropía del sistema puede disminuir si la del entorno aumenta más. A presión y temperatura constantes:

$$\Delta S_{entorno} = -\frac{\Delta H_{sistema}}{T}$$

Una reacción exotérmica calienta el entorno y aumenta su entropía.

## Energía libre de Gibbs

Para no tener que calcular el entorno, se define una función solo del sistema:

> [!IMPORTANT]
> $$\Delta G = \Delta H - T\Delta S$$
>
> A presión y temperatura constantes:
> - $\Delta G < 0$: espontáneo.
> - $\Delta G > 0$: no espontáneo (lo es el inverso).
> - $\Delta G = 0$: equilibrio.

$\Delta G$ es además el **trabajo útil máximo** que puede obtenerse del proceso.

### Efecto de la temperatura

| $\Delta H$ | $\Delta S$ | $\Delta G$ | Espontaneidad |
|---|---|---|---|
| − | + | siempre − | a toda temperatura |
| + | − | siempre + | a ninguna |
| − | − | − a baja $T$ | a temperaturas **bajas** |
| + | + | − a alta $T$ | a temperaturas **altas** |

Cuando ambos signos coinciden, hay una temperatura de cambio, estimada suponiendo que $\Delta H$ y $\Delta S$ no varían con $T$:

$$T = \frac{\Delta H}{\Delta S}$$

> [!NOTE]
> Descomposición de la caliza: CaCO₃(s) → CaO(s) + CO₂(g), con $\Delta H^0 = +178$ kJ y $\Delta S^0 = +161$ J/K.
>
> A $25$ °C: $\Delta G^0 = 178 - 298(0{,}161) = +130$ kJ. No es espontánea.
>
> Se vuelve espontánea por encima de $T = \dfrac{178\,000}{161} = 1106$ K (unos $830$ °C): la temperatura de los hornos de cal.

> [!WARNING]
> Cuidar las unidades: $\Delta H$ suele estar en **kJ** y $\Delta S$ en **J/K**. Convertir antes de operar.

### Energía libre estándar

$$\Delta G^0_{reacción} = \sum n\,\Delta G_f^0(\text{productos}) - \sum n\,\Delta G_f^0(\text{reactivos})$$

Como con la entalpía, $\Delta G_f^0$ de un elemento en su forma estable es cero.

## Energía libre y equilibrio

$\Delta G^0$ corresponde a condiciones estándar (1 bar, 1 M). En otras condiciones:

$$\Delta G = \Delta G^0 + RT\ln Q$$

con $Q$ el cociente de reacción. En el equilibrio, $\Delta G = 0$ y $Q = K$:

> [!IMPORTANT]
> $$\Delta G^0 = -RT\ln K$$

| $\Delta G^0$ | $K$ | En el equilibrio predominan |
|---|---|---|
| muy negativo | $\gg 1$ | productos |
| cero | $1$ | ninguno |
| muy positivo | $\ll 1$ | reactivos |

> [!NOTE]
> Para N₂(g) + 3 H₂(g) ⇌ 2 NH₃(g), $\Delta G^0 = -32{,}8$ kJ a $298$ K.
>
> $\ln K = \dfrac{32\,800}{8{,}314 \cdot 298} = 13{,}24 \Rightarrow K = 5{,}6 \times 10^5$.

Una reacción con $\Delta G^0 > 0$ no es imposible: solo que en el equilibrio habrá pocos productos.

## Reacciones acopladas

Una reacción no espontánea puede ocurrir si se la **acopla** con otra muy espontánea, de modo que el $\Delta G$ total sea negativo.

En las células, la hidrólisis del **ATP** ($\Delta G^0 \approx -30{,}5$ kJ/mol) impulsa la síntesis de proteínas, el transporte y la contracción muscular.

En la metalurgia, la reducción de un óxido metálico se acopla con la oxidación del carbono.

## Resumen

| Función | Símbolo | Indica |
|---|---|---|
| Entalpía | $\Delta H$ | calor a presión constante |
| Entropía | $\Delta S$ | dispersión de la energía |
| Energía libre | $\Delta G$ | espontaneidad y trabajo útil máximo |

> [!TIP]
> La termodinámica dice **hacia dónde** va una reacción y **hasta dónde** llega; la cinética dice **cuánto tarda**. Hacen falta las dos.
