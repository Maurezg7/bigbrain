+++
title = "Probabilidad y distribución normal"
summary = "Reglas básicas de la probabilidad, esperanza y varianza, y la campana de Gauss."
branch = "Probabilidad y estadística"
order = 1
level = "Intermedio"
tags = ["probabilidad", "Bayes", "normal", "desviación estándar"]
+++

## Reglas básicas

La **probabilidad** de un suceso $A$ es un número entre 0 (imposible) y 1 (seguro). Cuando todos los resultados son igualmente probables:

$$P(A) = \frac{\text{casos favorables}}{\text{casos posibles}}$$

- Complemento: $P(\text{no } A) = 1 - P(A)$
- Unión: $P(A \cup B) = P(A) + P(B) - P(A \cap B)$
- Probabilidad condicional: $P(A \mid B) = \dfrac{P(A \cap B)}{P(B)}$
- Independencia: $A$ y $B$ son independientes si $P(A \cap B) = P(A)\,P(B)$

## Teorema de Bayes

Permite "dar vuelta" una probabilidad condicional:

$$P(A \mid B) = \frac{P(B \mid A)\,P(A)}{P(B)}$$

> [!NOTE]
> Una enfermedad afecta al 1 % de la población. Un test la detecta el 99 % de las veces, pero da falso positivo en el 5 % de los sanos. Si el test da positivo, ¿cuál es la probabilidad de estar enfermo?
>
> $P(+) = 0{,}99 \cdot 0{,}01 + 0{,}05 \cdot 0{,}99 = 0{,}0594$, y entonces
>
> $$P(\text{enfermo} \mid +) = \frac{0{,}99 \cdot 0{,}01}{0{,}0594} \approx 0{,}167$$
>
> Solo un 16,7 %: como la enfermedad es rara, la mayoría de los positivos son falsos.

## Esperanza y varianza

Para una variable aleatoria discreta $X$ que toma valores $x_i$ con probabilidad $p_i$:

$$\mu = E[X] = \sum x_i\,p_i, \qquad \sigma^2 = \operatorname{Var}(X) = \sum (x_i - \mu)^2\,p_i$$

La **desviación estándar** $\sigma$ mide cuánto se alejan típicamente los valores de la media.

## La distribución normal

Muchas magnitudes (errores de medición, alturas, notas) siguen aproximadamente la **campana de Gauss**, determinada por su media $\mu$ y su desviación estándar $\sigma$:

$$f(x) = \frac{1}{\sigma\sqrt{2\pi}}\,e^{-\frac{(x-\mu)^2}{2\sigma^2}}$$

![Campana de Gauss con la regla 68-95-99,7](/media/matematicas/normal.svg)

*Regla empírica: el 68 % de los datos cae a menos de $1\sigma$ de la media, el 95 % a menos de $2\sigma$ y el 99,7 % a menos de $3\sigma$.*

Para comparar valores de distintas normales se **estandariza**: $z = \dfrac{x - \mu}{\sigma}$ indica a cuántas desviaciones estándar de la media está $x$.

> [!TIP]
> El **teorema central del límite** explica por qué la normal aparece en todas partes: el promedio de muchas variables independientes tiende a distribuirse normalmente, sea cual sea la distribución original.
