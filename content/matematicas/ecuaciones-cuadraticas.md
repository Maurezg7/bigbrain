+++
title = "Ecuaciones cuadráticas"
summary = "Cómo resolver ax² + bx + c = 0, qué dice el discriminante y cómo se ve en la parábola."
branch = "Álgebra"
order = 1
level = "Básico"
tags = ["ecuaciones", "parábola", "discriminante"]
+++

## Qué es una ecuación cuadrática

> [!IMPORTANT]
> Una **ecuación cuadrática** (o de segundo grado) es toda ecuación que puede escribirse como
> $ax^2 + bx + c = 0$ con $a \neq 0$.

Resolverla es encontrar los valores de $x$ que la cumplen: sus **raíces**. Geométricamente, son los puntos donde la parábola $y = ax^2 + bx + c$ corta al eje $x$.

![Parábola con sus raíces, su vértice y su eje de simetría](/media/matematicas/parabola.svg)

*La parábola $y = x^2 - 2x - 3$: raíces en $x=-1$ y $x=3$, vértice en $(1,-4)$.*

## La fórmula resolvente

Completando cuadrados en la ecuación general se obtiene

$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

La cantidad bajo la raíz, $\Delta = b^2 - 4ac$, se llama **discriminante** y decide cuántas soluciones reales hay:

| Discriminante | Raíces | Gráfico |
|---|---|---|
| $\Delta > 0$ | dos reales distintas | corta al eje $x$ en dos puntos |
| $\Delta = 0$ | una real doble | toca al eje $x$ en el vértice |
| $\Delta < 0$ | dos complejas conjugadas | no toca al eje $x$ |

## De dónde sale: completar cuadrados

Dividimos por $a$ y sumamos a ambos lados lo necesario para formar un cuadrado perfecto:

$$x^2 + \frac{b}{a}x = -\frac{c}{a} \;\Longrightarrow\; \left(x + \frac{b}{2a}\right)^2 = \frac{b^2 - 4ac}{4a^2}$$

Sacando raíz cuadrada y despejando $x$ aparece la fórmula. Del mismo cálculo sale el **vértice** de la parábola:

$$x_v = -\frac{b}{2a}, \qquad y_v = c - \frac{b^2}{4a}$$

## Otros métodos

- **Factorización**: si $x^2 + bx + c = (x - r_1)(x - r_2)$, las raíces son $r_1$ y $r_2$. Sirve cuando encontrás dos números que sumados dan $-b$ y multiplicados dan $c$.
- **Relaciones de Vieta**: $r_1 + r_2 = -\dfrac{b}{a}$ y $r_1 r_2 = \dfrac{c}{a}$. Útiles para verificar.

> [!NOTE]
> Resolver $x^2 - 2x - 3 = 0$.
>
> Aquí $a=1$, $b=-2$, $c=-3$, así que $\Delta = (-2)^2 - 4(1)(-3) = 16$.
>
> $$x = \frac{2 \pm \sqrt{16}}{2} = \frac{2 \pm 4}{2} \;\Rightarrow\; x_1 = 3,\; x_2 = -1$$
>
> Verificación con Vieta: $3 + (-1) = 2 = -b/a$ y $3 \cdot (-1) = -3 = c/a$.

> [!WARNING]
> El signo de $b$ entra en la fórmula. Si $b = -2$, entonces $-b = +2$. Y el denominador $2a$ divide a **todo** el numerador, no solo a la raíz.
