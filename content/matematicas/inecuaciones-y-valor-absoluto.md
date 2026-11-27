+++
title = "Inecuaciones y valor absoluto"
summary = "Cómo resolver desigualdades lineales, cuadráticas y racionales, y ecuaciones e inecuaciones con valor absoluto."
branch = "Álgebra"
order = 4
level = "Básico"
tags = ["inecuaciones", "valor absoluto", "desigualdades", "intervalos"]
+++

## Desigualdades

Una **inecuación** es una desigualdad con una incógnita. Su solución no suele ser un número, sino un **intervalo** o unión de intervalos.

### Reglas

| Operación | Efecto |
|---|---|
| Sumar o restar un número a ambos lados | el sentido se mantiene |
| Multiplicar o dividir por un **positivo** | el sentido se mantiene |
| Multiplicar o dividir por un **negativo** | el sentido **se invierte** |

> [!WARNING]
> El error más común: olvidar invertir la desigualdad al dividir por un negativo. $-2x < 6 \Rightarrow x > -3$.

Tampoco se puede multiplicar ambos lados por una expresión con $x$ sin conocer su signo.

## Inecuaciones lineales

> [!NOTE]
> Resolver $3 - 2x \le 7$.
>
> $-2x \le 4 \Rightarrow x \ge -2$. Solución: $[-2, +\infty)$.

Una inecuación doble se resuelve operando en los tres miembros: $-1 < 2x + 3 \le 9 \Rightarrow -4 < 2x \le 6 \Rightarrow -2 < x \le 3$.

## Inecuaciones cuadráticas y polinómicas

Método del **cuadro de signos**:

1. Pasar todo a un miembro, dejando $0$ del otro.
2. Factorizar.
3. Marcar las raíces en la recta: dividen la recta en intervalos.
4. Estudiar el signo de cada factor en cada intervalo.
5. Elegir los intervalos que cumplen la desigualdad.

> [!NOTE]
> Resolver $x^2 - x - 6 > 0$.
>
> Factorizando: $(x - 3)(x + 2) > 0$. Raíces: $-2$ y $3$.
>
> | | $x < -2$ | $-2 < x < 3$ | $x > 3$ |
> |---|---|---|---|
> | $x + 2$ | − | + | + |
> | $x - 3$ | − | − | + |
> | producto | **+** | − | **+** |
>
> Solución: $(-\infty, -2) \cup (3, +\infty)$.

Atajo para una parábola $ax^2 + bx + c$ con $a > 0$: es negativa **entre** las raíces y positiva **fuera** de ellas.

## Inecuaciones racionales

Se hace el mismo cuadro de signos con numerador y denominador. Las raíces del denominador **nunca** se incluyen.

> [!NOTE]
> Resolver $\dfrac{x + 1}{x - 2} \le 0$.
>
> Puntos críticos: $-1$ (numerador) y $2$ (denominador). El cociente es negativo entre ambos. Se incluye $-1$ (da cero) y se excluye $2$.
>
> Solución: $[-1, 2)$.

> [!WARNING]
> No «pasar multiplicando» el denominador: $\dfrac{1}{x} < 2$ no equivale a $1 < 2x$. Lo correcto es $\dfrac{1}{x} - 2 < 0 \Rightarrow \dfrac{1 - 2x}{x} < 0$, cuya solución es $(-\infty, 0) \cup (1/2, +\infty)$.

## Valor absoluto

> [!IMPORTANT]
> $$|x| = \begin{cases} x & \text{si } x \ge 0 \\ -x & \text{si } x < 0 \end{cases}$$

Geométricamente, $|x|$ es la **distancia** de $x$ al origen, y $|x - a|$ es la distancia entre $x$ y $a$.

### Propiedades

$$|x| \ge 0 \qquad |xy| = |x|\,|y| \qquad \left|\frac{x}{y}\right| = \frac{|x|}{|y|} \qquad \sqrt{x^2} = |x|$$

**Desigualdad triangular**: $|x + y| \le |x| + |y|$.

### Ecuaciones

Para $k > 0$:

$$|A| = k \iff A = k \ \text{ o } \ A = -k$$

Si $k < 0$ no hay solución.

> [!NOTE]
> $|2x - 1| = 5$. Entonces $2x - 1 = 5 \Rightarrow x = 3$, o $2x - 1 = -5 \Rightarrow x = -2$.

### Inecuaciones

Para $k > 0$:

| Inecuación | Equivale a | Solución |
|---|---|---|
| $\lvert A \rvert < k$ | $-k < A < k$ | un intervalo |
| $\lvert A \rvert > k$ | $A < -k$ o $A > k$ | dos semirrectas |

> [!NOTE]
> Resolver $|x - 3| < 2$.
>
> $-2 < x - 3 < 2 \Rightarrow 1 < x < 5$. Son los puntos a distancia menor que 2 del 3: el intervalo $(1, 5)$.

> [!NOTE]
> Resolver $|2x + 1| \ge 3$.
>
> $2x + 1 \le -3 \Rightarrow x \le -2$, o $2x + 1 \ge 3 \Rightarrow x \ge 1$. Solución: $(-\infty, -2] \cup [1, +\infty)$.

Si aparecen varios valores absolutos, se divide la recta en los puntos donde cada uno cambia de signo y se resuelve por tramos.

## Para qué sirve

- **Dominios**: $\sqrt{x - 2}$ exige $x - 2 \ge 0$; $\ln(4 - x^2)$ exige $4 - x^2 > 0$.
- **Tolerancias**: una pieza de $50 \pm 0{,}1$ mm cumple $|x - 50| \le 0{,}1$.
- **Límites**: la definición formal usa $|x - a| < \delta$ y $|f(x) - L| < \varepsilon$.

> [!TIP]
> Toda inecuación se reduce a estudiar el signo de una expresión: llevar todo a un lado, factorizar y armar el cuadro de signos.
