+++
title = "Polinomios y factorización"
summary = "Operaciones con polinomios, división y regla de Ruffini, teorema del resto, raíces y los casos de factoreo."
branch = "Álgebra"
order = 3
level = "Básico"
tags = ["polinomios", "factorización", "Ruffini", "raíces"]
+++

## Qué es un polinomio

> [!IMPORTANT]
> $$P(x) = a_n x^n + a_{n-1} x^{n-1} + \dots + a_1 x + a_0, \qquad a_n \neq 0$$

- $n$ es el **grado**; $a_n$, el **coeficiente principal**; $a_0$, el **término independiente**.
- Si $a_n = 1$ el polinomio es **mónico**.
- Los exponentes son enteros no negativos: $\sqrt{x}$ o $1/x$ no son polinomios.

## Operaciones

**Suma y resta**: se operan los términos del mismo grado.

**Producto**: se aplica la distributiva y se agrupan términos semejantes. El grado del producto es la suma de los grados.

### Productos notables

$$(a \pm b)^2 = a^2 \pm 2ab + b^2$$

$$(a + b)(a - b) = a^2 - b^2$$

$$(a \pm b)^3 = a^3 \pm 3a^2 b + 3ab^2 \pm b^3$$

Para potencias mayores, el **binomio de Newton**:

$$(a + b)^n = \sum_{k=0}^{n} \binom{n}{k} a^{n-k} b^k$$

## División

Dados $P(x)$ y $D(x)$, existen únicos cociente $C(x)$ y resto $R(x)$ con

$$P(x) = D(x)\, C(x) + R(x), \qquad \operatorname{gr}(R) < \operatorname{gr}(D)$$

### Regla de Ruffini

Sirve para dividir por $(x - a)$. Ejemplo: $(2x^3 - 3x^2 + 0x + 4) \div (x - 2)$.

| | 2 | −3 | 0 | 4 |
|---|---|---|---|---|
| **2** | | 4 | 2 | 4 |
| | **2** | **1** | **2** | **8** |

Se baja el primer coeficiente; se multiplica por $a$ y se suma al siguiente. Resultado: $C(x) = 2x^2 + x + 2$, resto $8$.

> [!WARNING]
> Hay que completar con ceros los términos que faltan, y si se divide por $(x + 2)$ el número que se usa es $a = -2$.

## Teorema del resto y raíces

> [!IMPORTANT]
> El resto de dividir $P(x)$ por $(x - a)$ es $P(a)$.

Consecuencia (**teorema del factor**): $a$ es **raíz** de $P$, es decir $P(a) = 0$, si y solo si $(x - a)$ divide a $P$.

**Teorema fundamental del álgebra**: un polinomio de grado $n$ tiene exactamente $n$ raíces complejas, contadas con su multiplicidad. Entonces se factoriza por completo:

$$P(x) = a_n (x - x_1)(x - x_2) \cdots (x - x_n)$$

Si los coeficientes son reales, las raíces complejas vienen en pares conjugados.

### Teorema de Gauss

Si $P$ tiene coeficientes enteros, sus raíces racionales son de la forma $p/q$, donde $p$ divide al término independiente y $q$ al coeficiente principal.

> [!NOTE]
> Factorizar $P(x) = x^3 - 6x^2 + 11x - 6$.
>
> Candidatas: $\pm 1, \pm 2, \pm 3, \pm 6$. Como $P(1) = 1 - 6 + 11 - 6 = 0$, $x = 1$ es raíz. Ruffini con $a = 1$ da $x^2 - 5x + 6$, que se factoriza como $(x - 2)(x - 3)$.
>
> $$P(x) = (x - 1)(x - 2)(x - 3)$$

## Casos de factoreo

| Caso | Forma | Factorización |
|---|---|---|
| Factor común | $ab + ac$ | $a(b + c)$ |
| Factor común por grupos | $ax + ay + bx + by$ | $(a + b)(x + y)$ |
| Trinomio cuadrado perfecto | $a^2 \pm 2ab + b^2$ | $(a \pm b)^2$ |
| Diferencia de cuadrados | $a^2 - b^2$ | $(a + b)(a - b)$ |
| Suma de cubos | $a^3 + b^3$ | $(a + b)(a^2 - ab + b^2)$ |
| Diferencia de cubos | $a^3 - b^3$ | $(a - b)(a^2 + ab + b^2)$ |
| Trinomio de segundo grado | $ax^2 + bx + c$ | $a(x - x_1)(x - x_2)$ |

La suma de cuadrados $a^2 + b^2$ **no** se factoriza con números reales.

> [!NOTE]
> Factorizar $2x^3 - 8x$.
>
> Factor común: $2x(x^2 - 4)$. Diferencia de cuadrados: $2x(x + 2)(x - 2)$.

## Fracciones algebraicas

Se operan como fracciones numéricas, factorizando primero:

$$\frac{x^2 - 1}{x^2 + 2x + 1} = \frac{(x + 1)(x - 1)}{(x + 1)^2} = \frac{x - 1}{x + 1}, \qquad x \neq -1$$

> [!WARNING]
> Solo se simplifican **factores**, nunca sumandos: en $\dfrac{x + 3}{x}$ no se puede tachar la $x$.

### Fracciones simples

Una fracción con denominador factorizado se descompone en suma de fracciones más sencillas, lo que es clave para integrar y para la transformada de Laplace:

$$\frac{1}{(x - 1)(x + 2)} = \frac{A}{x - 1} + \frac{B}{x + 2}$$

Multiplicando por el denominador: $1 = A(x + 2) + B(x - 1)$. Con $x = 1$: $A = 1/3$. Con $x = -2$: $B = -1/3$.

> [!TIP]
> Factorizar es convertir sumas en productos. Un producto vale cero cuando algún factor vale cero: por eso factorizar resuelve ecuaciones.
