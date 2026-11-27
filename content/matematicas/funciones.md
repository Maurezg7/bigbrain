+++
title = "Funciones"
summary = "Dominio e imagen, gráficos, funciones lineales, cuadráticas y racionales, composición, inversa y transformaciones del gráfico."
branch = "Álgebra"
order = 5
level = "Básico"
tags = ["funciones", "dominio", "composición", "función inversa"]
+++

## Qué es una función

> [!IMPORTANT]
> Una función $f: A \to B$ asigna a **cada** elemento $x$ de $A$ **un único** elemento $f(x)$ de $B$.

- **Dominio**: los valores de $x$ para los que $f$ está definida.
- **Imagen** (o rango): los valores que $f$ efectivamente toma.

**Prueba de la recta vertical**: una curva es gráfico de una función si ninguna recta vertical la corta más de una vez.

### Cómo hallar el dominio

| Expresión | Condición |
|---|---|
| Cociente $\dfrac{P}{Q}$ | $Q \neq 0$ |
| Raíz par $\sqrt{A}$ | $A \ge 0$ |
| Logaritmo $\ln A$ | $A > 0$ |
| Tangente $\tan A$ | $A \neq \frac{\pi}{2} + k\pi$ |

> [!NOTE]
> Dominio de $f(x) = \dfrac{\sqrt{x + 1}}{x - 2}$: hace falta $x + 1 \ge 0$ y $x \neq 2$. Dominio: $[-1, 2) \cup (2, +\infty)$.

## Propiedades

| Propiedad | Definición |
|---|---|
| Creciente | $x_1 < x_2 \Rightarrow f(x_1) < f(x_2)$ |
| Decreciente | $x_1 < x_2 \Rightarrow f(x_1) > f(x_2)$ |
| Par | $f(-x) = f(x)$: simétrica respecto del eje $y$ |
| Impar | $f(-x) = -f(x)$: simétrica respecto del origen |
| Periódica | $f(x + T) = f(x)$ |
| Inyectiva | valores distintos de $x$ dan imágenes distintas |
| Sobreyectiva | la imagen es todo el conjunto de llegada |
| Biyectiva | inyectiva y sobreyectiva |

## Función lineal

$$f(x) = mx + b$$

$m$ es la **pendiente** y $b$ la **ordenada al origen**. Dados dos puntos:

$$m = \frac{y_2 - y_1}{x_2 - x_1} \qquad y - y_1 = m(x - x_1)$$

Dos rectas son **paralelas** si $m_1 = m_2$ y **perpendiculares** si $m_1 m_2 = -1$.

## Función cuadrática

$$f(x) = ax^2 + bx + c = a(x - h)^2 + k$$

Su gráfico es una parábola con **vértice** en $(h, k)$:

$$h = -\frac{b}{2a} \qquad k = f(h)$$

Abre hacia arriba si $a > 0$ (el vértice es mínimo) y hacia abajo si $a < 0$ (máximo).

## Otras funciones básicas

| Función | Fórmula | Dominio | Rasgo |
|---|---|---|---|
| Potencia | $x^n$ | $\mathbb{R}$ | par si $n$ par, impar si $n$ impar |
| Raíz | $\sqrt{x}$ | $[0, +\infty)$ | crece cada vez más lento |
| Homográfica | $\dfrac{1}{x}$ | $x \neq 0$ | asíntotas en los ejes |
| Valor absoluto | $\lvert x \rvert$ | $\mathbb{R}$ | forma de V |
| Exponencial | $a^x$ | $\mathbb{R}$ | siempre positiva |
| Logaritmo | $\log_a x$ | $(0, +\infty)$ | inversa de la exponencial |

### Funciones racionales y asíntotas

Para $f(x) = \dfrac{P(x)}{Q(x)}$:

- **Asíntota vertical** en cada raíz de $Q$ que no anule a $P$.
- **Asíntota horizontal**: $y = 0$ si $\operatorname{gr}(P) < \operatorname{gr}(Q)$; el cociente de coeficientes principales si los grados son iguales; no hay si $\operatorname{gr}(P) > \operatorname{gr}(Q)$.

### Funciones a trozos

Usan fórmulas distintas en distintos tramos del dominio:

$$f(x) = \begin{cases} x^2 & \text{si } x < 1 \\ 2x - 1 & \text{si } x \ge 1 \end{cases}$$

## Operaciones y composición

$(f \pm g)(x) = f(x) \pm g(x)$, $(fg)(x) = f(x)g(x)$, y $(f/g)(x) = f(x)/g(x)$ donde $g(x) \neq 0$.

> [!IMPORTANT]
> **Composición**: $(f \circ g)(x) = f(g(x))$. Primero se aplica $g$, después $f$.

> [!NOTE]
> Si $f(x) = x^2 + 1$ y $g(x) = 3x$:
>
> $(f \circ g)(x) = f(3x) = 9x^2 + 1$ y $(g \circ f)(x) = 3(x^2 + 1) = 3x^2 + 3$.
>
> La composición **no es conmutativa**.

## Función inversa

Si $f$ es biyectiva, su inversa $f^{-1}$ deshace lo que hace $f$:

$$f^{-1}(f(x)) = x \qquad f(f^{-1}(y)) = y$$

Para hallarla: se escribe $y = f(x)$, se despeja $x$ y se intercambian los nombres. El gráfico de $f^{-1}$ es el de $f$ reflejado en la recta $y = x$.

> [!NOTE]
> Inversa de $f(x) = \dfrac{2x + 1}{x - 3}$.
>
> $y(x - 3) = 2x + 1 \Rightarrow x(y - 2) = 3y + 1 \Rightarrow x = \dfrac{3y + 1}{y - 2}$. Entonces $f^{-1}(x) = \dfrac{3x + 1}{x - 2}$.

> [!WARNING]
> $f^{-1}(x)$ **no** es $\dfrac{1}{f(x)}$. Y si $f$ no es inyectiva hay que restringir el dominio: $x^2$ solo tiene inversa $\sqrt{x}$ para $x \ge 0$.

## Transformaciones del gráfico

| Transformación | Efecto sobre el gráfico de $f(x)$ |
|---|---|
| $f(x) + k$ | sube $k$ unidades |
| $f(x - h)$ | se corre $h$ unidades a la **derecha** |
| $a \cdot f(x)$ | se estira verticalmente si $\lvert a \rvert > 1$ |
| $-f(x)$ | se refleja en el eje $x$ |
| $f(-x)$ | se refleja en el eje $y$ |
| $f(bx)$ | se comprime horizontalmente si $\lvert b \rvert > 1$ |

> [!TIP]
> Lo que se opera **fuera** de $f$ mueve el gráfico en vertical y hace lo que parece. Lo que se opera **dentro**, sobre $x$, lo mueve en horizontal y hace lo contrario.
