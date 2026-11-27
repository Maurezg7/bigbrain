+++
title = "Regresión y correlación"
summary = "Covarianza, coeficiente de correlación, recta de mínimos cuadrados, coeficiente de determinación y análisis de residuos."
branch = "Probabilidad y estadística"
order = 5
level = "Intermedio"
tags = ["regresión lineal", "correlación", "mínimos cuadrados", "R cuadrado"]
+++

## Relación entre dos variables

Con pares de datos $(x_i, y_i)$ se quiere saber si las variables están relacionadas, cuán fuerte es la relación y cómo predecir una a partir de la otra.

El primer paso es siempre un **diagrama de dispersión**.

## Covarianza y correlación

$$s_{xy} = \frac{1}{n - 1}\sum(x_i - \bar{x})(y_i - \bar{y})$$

La **covarianza** es positiva si las variables crecen juntas y negativa si una crece cuando la otra decrece. Depende de las unidades, así que se la normaliza:

> [!IMPORTANT]
> **Coeficiente de correlación de Pearson**:
> $$r = \frac{s_{xy}}{s_xs_y}, \qquad -1 \le r \le 1$$

| Valor de $r$ | Interpretación |
|---|---|
| $r = 1$ | relación lineal positiva perfecta |
| $0{,}7 < r < 1$ | positiva fuerte |
| $0{,}3 < r < 0{,}7$ | positiva moderada |
| $r \approx 0$ | sin relación **lineal** |
| $r < 0$ | relación negativa |
| $r = -1$ | lineal negativa perfecta |

> [!WARNING]
> **Correlación no implica causalidad.** Las ventas de helado y los ahogamientos están correlacionados porque ambos aumentan en verano: hay una tercera variable (la temperatura) que influye en las dos.

Además, $r$ solo mide relación **lineal**: puntos sobre una parábola pueden dar $r = 0$ pese a estar perfectamente relacionados. Y un único valor atípico puede cambiar $r$ por completo.

## Recta de regresión

Se busca la recta $\hat{y} = a + bx$ que mejor ajusta a los datos. El criterio de **mínimos cuadrados** minimiza la suma de los cuadrados de los **residuos** $e_i = y_i - \hat{y}_i$.

> [!IMPORTANT]
> $$b = \frac{s_{xy}}{s_x^2} = r\,\frac{s_y}{s_x} \qquad a = \bar{y} - b\bar{x}$$

- $b$ (**pendiente**): cuánto cambia $y$ en promedio al aumentar $x$ en una unidad.
- $a$ (**ordenada**): valor predicho para $x = 0$ (no siempre tiene sentido).
- La recta siempre pasa por $(\bar{x}, \bar{y})$.

> [!NOTE]
> Horas de estudio ($x$) y nota ($y$) de cinco estudiantes:
>
> | $x$ | 1 | 2 | 3 | 4 | 5 |
> |---|---|---|---|---|---|
> | $y$ | 3 | 5 | 4 | 7 | 8 |
>
> $\bar{x} = 3$, $\bar{y} = 5{,}4$. $\sum(x - \bar{x})(y - \bar{y}) = 12$; $\sum(x - \bar{x})^2 = 10$.
>
> $b = 1{,}2$ y $a = 5{,}4 - 1{,}2(3) = 1{,}8$. Recta: $\hat{y} = 1{,}8 + 1{,}2x$.
>
> Cada hora adicional de estudio se asocia con $1{,}2$ puntos más. Para 6 horas se predice $9$.

## Coeficiente de determinación

> [!IMPORTANT]
> $$R^2 = 1 - \frac{\sum(y_i - \hat{y}_i)^2}{\sum(y_i - \bar{y})^2}$$

Es la proporción de la variabilidad de $y$ **explicada** por el modelo. En regresión lineal simple, $R^2 = r^2$.

En el ejemplo: $\sum(y - \bar{y})^2 = 17{,}2$, y $r^2 = \dfrac{12^2}{10 \cdot 17{,}2} \approx 0{,}84$. Las horas de estudio explican el 84 % de la variación de las notas.

## Análisis de residuos

Un $R^2$ alto no garantiza que el modelo sea adecuado. Hay que graficar los residuos contra $x$:

| Patrón de los residuos | Diagnóstico |
|---|---|
| Nube sin forma alrededor de cero | el modelo lineal es adecuado |
| Forma de curva | la relación no es lineal |
| Forma de embudo | la variabilidad no es constante |
| Un punto muy alejado | valor atípico o influyente |

Supuestos del modelo: relación lineal, residuos independientes, de varianza constante y aproximadamente normales.

## Interpolar y extrapolar

- **Interpolar**: predecir dentro del rango de los datos. Es confiable.
- **Extrapolar**: predecir fuera del rango. Es riesgoso, porque nada garantiza que la relación se mantenga.

> [!WARNING]
> Una recta ajustada a la altura de niños de 2 a 10 años predice adultos de más de 3 metros.

## Modelos no lineales

Muchas relaciones se **linealizan** con una transformación:

| Modelo | Transformación | Recta |
|---|---|---|
| Exponencial $y = ae^{bx}$ | $\ln y$ | $\ln y = \ln a + bx$ |
| Potencial $y = ax^b$ | $\ln y$, $\ln x$ | $\ln y = \ln a + b\ln x$ |
| Logarítmico $y = a + b\ln x$ | $\ln x$ | lineal en $\ln x$ |

También se puede ajustar un polinomio: $y = a + bx + cx^2$.

## Regresión múltiple

Con varias variables explicativas:

$$\hat{y} = b_0 + b_1x_1 + b_2x_2 + \dots + b_kx_k$$

Cada coeficiente indica el efecto de una variable **manteniendo constantes las demás**. En forma matricial, $\vec{b} = (X^TX)^{-1}X^T\vec{y}$.

Agregar variables siempre aumenta $R^2$, por eso se usa el **$R^2$ ajustado**, que penaliza la complejidad.

## Inferencia sobre la pendiente

Para saber si la relación es real o producto del azar, se prueba $H_0: \beta = 0$ con

$$t = \frac{b}{SE_b}, \quad n - 2 \text{ grados de libertad}$$

Si el valor p es pequeño, hay evidencia de relación lineal.

> [!TIP]
> Siempre graficar antes de calcular. Conjuntos de datos muy distintos pueden tener exactamente la misma recta y el mismo $r$ (cuarteto de Anscombe).
