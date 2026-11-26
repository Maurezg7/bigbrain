+++
title = "Pandeo de columnas"
summary = "Inestabilidad de barras comprimidas: carga crítica de Euler, esbeltez, condiciones de apoyo y límites de la fórmula."
branch = "Estabilidad y resistencia de materiales"
order = 4
level = "Intermedio"
tags = ["pandeo", "Euler", "esbeltez", "columna", "estabilidad"]
+++

## Una falla que no avisa

Una barra corta comprimida falla cuando el material se aplasta. Una barra **esbelta** falla mucho antes, de otro modo: de pronto se curva hacia un costado y colapsa. Es el **pandeo**, una falla de estabilidad y no de resistencia, que ocurre con tensiones muy inferiores a la de fluencia.

## Carga crítica de Euler

Para una columna ideal, recta y articulada en ambos extremos, la carga a la que deja de ser estable es

$$P_{cr} = \frac{\pi^2 E I}{L^2}$$

Depende de la rigidez $EI$ y del cuadrado de la longitud, pero **no de la resistencia del material**. Usar un acero más resistente no mejora el pandeo, porque todos los aceros tienen casi el mismo $E$.

## Condiciones de apoyo

![Cuatro columnas con distintas condiciones de apoyo y su longitud de pandeo](/media/electromecanica/pandeo.svg)

*La longitud de pandeo es la distancia entre puntos de inflexión de la deformada.*

Los vínculos cambian la forma en que se curva la barra. Se usa una **longitud de pandeo** $L_k = k\,L$:

$$P_{cr} = \frac{\pi^2 E I}{(kL)^2}$$

| Extremos | $k$ | Carga crítica relativa |
|---|---|---|
| Empotrado y libre | 2 | 0,25 |
| Articulado y articulado | 1 | 1 |
| Empotrado y articulado | 0,7 | 2 |
| Empotrado y empotrado | 0,5 | 4 |

## Esbeltez

Dividiendo por el área se obtiene la tensión crítica en función de la **esbeltez** $\lambda$:

$$\sigma_{cr} = \frac{\pi^2 E}{\lambda^2},\qquad \lambda = \frac{L_k}{i},\qquad i = \sqrt{\frac{I}{A}}$$

$i$ es el radio de giro. La barra pandea respecto del eje de **menor** inercia, por eso los perfiles tubulares, con igual inercia en todas las direcciones, son los más eficientes a compresión.

## Hasta dónde vale Euler

La fórmula supone material elástico, así que solo vale si $\sigma_{cr}$ no supera el límite de proporcionalidad. Para acero común eso ocurre con $\lambda$ mayor que unos 100.

| Esbeltez | Comportamiento | Cálculo |
|---|---|---|
| Baja (columna corta) | aplastamiento | $\sigma = P/A$ |
| Intermedia | pandeo inelástico | fórmulas empíricas (Tetmajer, Johnson) o curvas de norma |
| Alta (columna esbelta) | pandeo elástico | Euler |

> [!NOTE]
> Barra de acero macizo de 20 mm de diámetro y 1 m de largo, articulada en ambos extremos, $E = 210$ GPa.
>
> $I = \dfrac{\pi\cdot 20^4}{64} = 7854\ \text{mm}^4$, $A = 314\ \text{mm}^2$, $i = 5$ mm y $\lambda = 200$: vale Euler.
>
> $P_{cr} = \dfrac{\pi^2\cdot 210\,000\cdot 7854}{1000^2}\approx 16{,}3$ kN, es decir $\sigma_{cr}\approx 52$ MPa, cuando el material fluye recién a 250 MPa.

## En la práctica

Las columnas reales no son perfectamente rectas ni la carga está perfectamente centrada, por lo que se curvan desde el principio. Las normas usan **curvas de pandeo** que incluyen esas imperfecciones, y coeficientes de seguridad mayores que en tracción, de 3 a 5 con Euler.

> [!WARNING]
> El pandeo es súbito y catastrófico. Hay que verificarlo en toda pieza comprimida y esbelta: columnas, barras de reticulados, vástagos de cilindros hidráulicos, bielas y tornillos de potencia.

Fenómenos emparentados son el **pandeo lateral** de vigas altas y delgadas y la **abolladura** de chapas y tubos de pared fina.
