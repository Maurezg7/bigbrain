+++
title = "Estática: equilibrio y reacciones"
summary = "Fuerzas, momentos, diagrama de cuerpo libre, vínculos, reacciones de apoyo, reticulados y centros de gravedad."
branch = "Estabilidad y resistencia de materiales"
order = 1
level = "Básico"
tags = ["equilibrio", "momento", "reacciones", "reticulado", "cuerpo libre"]
+++

## Fuerzas y momentos

Una **fuerza** es un vector: tiene módulo, dirección, sentido y punto de aplicación. Su tendencia a hacer girar un cuerpo alrededor de un punto es el **momento**:

$$M = F\,d$$

donde $d$ es la distancia perpendicular del punto a la recta de acción. Dos fuerzas iguales, opuestas y paralelas forman un **par**, cuyo momento es el mismo respecto de cualquier punto.

## Condiciones de equilibrio

Un cuerpo rígido está en equilibrio si se anulan la resultante de fuerzas y la de momentos. En el plano son tres ecuaciones:

$$\sum F_x = 0,\qquad \sum F_y = 0,\qquad \sum M = 0$$

En el espacio son seis.

## Diagrama de cuerpo libre

Es el paso más importante de cualquier problema: se aísla el cuerpo y se dibujan **todas** las fuerzas que actúan sobre él, reemplazando cada apoyo por las reacciones que puede ejercer.

| Vínculo | Movimientos que impide | Reacciones |
|---|---|---|
| Apoyo móvil (rodillo) | uno | una fuerza perpendicular al plano de apoyo |
| Apoyo fijo (articulación) | dos | dos componentes de fuerza |
| Empotramiento | tres | dos fuerzas y un momento |

Si las incógnitas son tantas como las ecuaciones, la estructura es **isostática**. Si son más, es **hiperestática** y hay que considerar las deformaciones.

## Reacciones en una viga

![Viga simplemente apoyada con una carga puntual y sus dos reacciones de apoyo](/media/electromecanica/viga.svg)

*Viga simplemente apoyada: las reacciones se reparten de forma inversa a las distancias.*

> [!NOTE]
> Viga de 6 m apoyada en $A$ (fijo) y $B$ (móvil), con una carga de 12 kN a 2 m de $A$.
>
> Momentos respecto de $A$: $R_B\cdot 6 - 12\cdot 2 = 0$, de donde $R_B = 4$ kN.
>
> Fuerzas verticales: $R_A = 12 - 4 = 8$ kN. El apoyo más cercano a la carga se lleva la mayor parte.

Una carga **distribuida** uniforme $q$ sobre una longitud $L$ equivale, para calcular reacciones, a una fuerza $qL$ aplicada en su punto medio.

## Reticulados

Un reticulado está formado por barras articuladas en los nudos y cargado solo en ellos, de modo que cada barra trabaja a **tracción o compresión** pura. Es isostático cuando, con $b$ barras y $n$ nudos, $b = 2n - 3$.

- **Método de los nudos**: se plantea el equilibrio de cada nudo, empezando por uno con dos incógnitas.
- **Método de las secciones** (Ritter): se corta la estructura y se equilibra una de las partes; sirve para hallar el esfuerzo de una barra sin resolver todo.

## Centro de gravedad y momentos de inercia

El **centroide** de una sección compuesta se obtiene ponderando por las áreas:

$$\bar x = \frac{\sum A_i x_i}{\sum A_i},\qquad \bar y = \frac{\sum A_i y_i}{\sum A_i}$$

El **momento de inercia** de área mide cómo se distribuye el material respecto de un eje y decide la rigidez a flexión:

| Sección | $I$ respecto del eje baricéntrico |
|---|---|
| Rectángulo $b\times h$ | $\dfrac{b h^3}{12}$ |
| Círculo de diámetro $d$ | $\dfrac{\pi d^4}{64}$ |
| Tubo $D$, $d$ | $\dfrac{\pi (D^4 - d^4)}{64}$ |

Para trasladar a un eje paralelo a distancia $a$ se usa el teorema de Steiner: $I = I_G + A\,a^2$.

## Rozamiento

La fuerza de rozamiento estática alcanza como máximo $F_{max} = \mu_s N$. Un cuerpo sobre un plano inclinado comienza a deslizar cuando $\tan\alpha = \mu_s$. Este límite gobierna frenos, embragues, correas y uniones atornilladas.

> [!TIP]
> Elegí el punto para tomar momentos donde concurran más incógnitas: desaparecen de la ecuación y el sistema se resuelve casi solo.
