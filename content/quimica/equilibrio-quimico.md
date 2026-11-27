+++
title = "Equilibrio químico"
summary = "Reacciones reversibles, constante de equilibrio, cociente de reacción, cálculo de concentraciones y principio de Le Châtelier."
branch = "Equilibrio químico"
order = 0
level = "Intermedio"
tags = ["equilibrio", "constante de equilibrio", "Le Châtelier", "cociente de reacción"]
+++

## Reacciones reversibles

Muchas reacciones no se completan: los productos reaccionan entre sí y regeneran los reactivos. Se indican con doble flecha (⇌).

> [!IMPORTANT]
> En el **equilibrio químico**, las velocidades de la reacción directa y de la inversa son iguales, y las concentraciones dejan de cambiar.

Es un equilibrio **dinámico**: las dos reacciones siguen ocurriendo. Se llega al mismo estado partiendo de los reactivos o de los productos.

## Constante de equilibrio

Para $aA + bB \rightleftharpoons cC + dD$:

> [!IMPORTANT]
> $$K_c = \frac{[C]^c[D]^d}{[A]^a[B]^b}$$

con las concentraciones **en el equilibrio**. $K$ depende solo de la **temperatura**.

Para gases también se usa $K_p$, con presiones parciales:

$$K_p = K_c(RT)^{\Delta n}$$

con $\Delta n$ = moles de gas de productos − moles de gas de reactivos, y $R = 0{,}08206$ L·atm/(mol·K).

| Valor de $K$ | Significado |
|---|---|
| $K \gg 1$ | predominan los productos |
| $K \approx 1$ | cantidades comparables |
| $K \ll 1$ | predominan los reactivos |

### Reglas

- Los **sólidos y líquidos puros** no aparecen en la expresión. Para CaCO₃(s) ⇌ CaO(s) + CO₂(g): $K_c = [\text{CO}_2]$.
- Si se **invierte** la reacción: $K' = \dfrac{1}{K}$.
- Si se **multiplican** los coeficientes por $n$: $K' = K^n$.
- Si se **suman** reacciones: $K = K_1 \cdot K_2$.

> [!NOTE]
> Para N₂(g) + 3 H₂(g) ⇌ 2 NH₃(g), en el equilibrio a cierta temperatura: $[\text{N}_2] = 0{,}50$ M, $[\text{H}_2] = 0{,}20$ M, $[\text{NH}_3] = 0{,}10$ M.
>
> $K_c = \dfrac{(0{,}10)^2}{(0{,}50)(0{,}20)^3} = 2{,}5$.

## Cociente de reacción

$Q$ tiene la misma forma que $K$, pero con las concentraciones de **cualquier momento**.

| Comparación | La reacción avanza |
|---|---|
| $Q < K$ | hacia los **productos** |
| $Q = K$ | está en equilibrio |
| $Q > K$ | hacia los **reactivos** |

## Cálculo de concentraciones

Se usa una tabla **ICE**: Inicial, Cambio, Equilibrio.

> [!NOTE]
> H₂(g) + I₂(g) ⇌ 2 HI(g), con $K_c = 49$. Se parte de $1{,}00$ M de H₂ y $1{,}00$ M de I₂.
>
> | | H₂ | I₂ | HI |
> |---|---|---|---|
> | Inicial | 1,00 | 1,00 | 0 |
> | Cambio | $-x$ | $-x$ | $+2x$ |
> | Equilibrio | $1{,}00 - x$ | $1{,}00 - x$ | $2x$ |
>
> $\dfrac{(2x)^2}{(1{,}00 - x)^2} = 49$. Sacando raíz: $\dfrac{2x}{1{,}00 - x} = 7 \Rightarrow x = 0{,}778$.
>
> En el equilibrio: $[\text{H}_2] = [\text{I}_2] = 0{,}222$ M y $[\text{HI}] = 1{,}56$ M.

Cuando $K$ es muy pequeña, $x$ es despreciable frente a la concentración inicial y se simplifica: $C_0 - x \approx C_0$. La aproximación es aceptable si $x$ resulta menor que el 5 % de $C_0$.

## Principio de Le Châtelier

> [!IMPORTANT]
> Si se perturba un sistema en equilibrio, este se desplaza en el sentido que **contrarresta** parcialmente la perturbación.

| Perturbación | Desplazamiento | ¿Cambia $K$? |
|---|---|---|
| Agregar un reactivo | hacia productos | no |
| Quitar un producto | hacia productos | no |
| Aumentar la presión (reducir el volumen) | hacia donde hay **menos moles de gas** | no |
| Aumentar la temperatura | en el sentido **endotérmico** | **sí** |
| Agregar un catalizador | ninguno (se llega antes) | no |
| Agregar un gas inerte a volumen constante | ninguno | no |

Solo la **temperatura** cambia el valor de $K$: para una reacción exotérmica, $K$ disminuye al calentar.

Si hay igual cantidad de moles de gas a ambos lados, la presión no desplaza el equilibrio.

> [!NOTE]
> N₂(g) + 3 H₂(g) ⇌ 2 NH₃(g), $\Delta H = -92$ kJ.
>
> - Más presión: hacia el amoníaco (4 moles de gas → 2).
> - Más temperatura: hacia los reactivos (es exotérmica).
> - Retirar el NH₃: hacia el amoníaco.

### Proceso Haber-Bosch

La síntesis industrial del amoníaco muestra el compromiso entre equilibrio y velocidad:

| Condición | Valor | Motivo |
|---|---|---|
| Presión | 150–300 atm | desplaza hacia el producto |
| Temperatura | 400–500 °C | más baja favorecería el equilibrio, pero sería demasiado lenta |
| Catalizador | hierro | acelera la reacción |
| Extracción | se licúa el NH₃ | desplaza hacia el producto |

Produce los fertilizantes nitrogenados que sostienen a buena parte de la población mundial.

## Equilibrio y energía libre

$$\Delta G^0 = -RT\ln K$$

La dependencia de $K$ con la temperatura está dada por la **ecuación de van't Hoff**:

$$\ln\frac{K_2}{K_1} = -\frac{\Delta H^0}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)$$

## Equilibrios heterogéneos

Cuando intervienen varias fases. La cantidad de sólido presente no afecta al equilibrio, siempre que haya algo.

> [!NOTE]
> CaCO₃(s) ⇌ CaO(s) + CO₂(g): $K_p = P_{\text{CO}_2}$. A cada temperatura hay una única presión de CO₂ de equilibrio, sin importar cuánto sólido haya.

> [!WARNING]
> Un catalizador no desplaza el equilibrio ni cambia el rendimiento: solo acorta el tiempo para alcanzarlo.

> [!TIP]
> Para cualquier problema: escribir la expresión de $K$, armar la tabla ICE, y comprobar al final que los valores hallados reproducen $K$.
