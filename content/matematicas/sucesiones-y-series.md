+++
title = "Sucesiones y series"
summary = "Límite de sucesiones, series geométricas y armónicas, criterios de convergencia y series de potencias."
branch = "Cálculo"
order = 7
level = "Avanzado"
tags = ["series", "sucesiones", "convergencia", "serie geométrica"]
+++

## Sucesiones

Una **sucesión** es una lista ordenada e infinita de números $a_1, a_2, a_3, \dots$

Converge a $L$ si sus términos se acercan a $L$ tanto como se quiera: $\lim_{n \to \infty} a_n = L$.

| Sucesión | Límite |
|---|---|
| $\dfrac{1}{n}$ | $0$ |
| $r^n$ con $\lvert r \rvert < 1$ | $0$ |
| $\sqrt[n]{n}$ | $1$ |
| $\left(1 + \dfrac{1}{n}\right)^n$ | $e$ |
| $(-1)^n$ | no existe |

Toda sucesión **monótona y acotada** converge.

### Progresiones

| | Aritmética | Geométrica |
|---|---|---|
| Regla | se suma $d$ | se multiplica por $r$ |
| Término general | $a_n = a_1 + (n - 1)d$ | $a_n = a_1 r^{n-1}$ |
| Suma de $n$ términos | $\dfrac{n(a_1 + a_n)}{2}$ | $a_1\dfrac{1 - r^n}{1 - r}$ |

## Series

Una **serie** es la suma de los términos de una sucesión. Se define a través de sus **sumas parciales** $S_n = a_1 + \dots + a_n$:

> [!IMPORTANT]
> $$\sum_{n=1}^{\infty} a_n = \lim_{n \to \infty} S_n$$
> Si el límite existe y es finito, la serie **converge**; si no, **diverge**.

### Serie geométrica

$$\sum_{n=0}^{\infty} a r^n = \frac{a}{1 - r} \quad \text{si } |r| < 1$$

Si $|r| \ge 1$, diverge.

> [!NOTE]
> $\dfrac{1}{2} + \dfrac{1}{4} + \dfrac{1}{8} + \dots = \dfrac{1/2}{1 - 1/2} = 1$.
>
> $0{,}333\dots = \dfrac{3}{10} + \dfrac{3}{100} + \dots = \dfrac{3/10}{1 - 1/10} = \dfrac{1}{3}$.

### Serie armónica y series $p$

$$\sum_{n=1}^{\infty} \frac{1}{n^p} \quad \text{converge si } p > 1 \text{ y diverge si } p \le 1$$

La serie armónica $\sum \dfrac{1}{n}$ **diverge**, aunque sus términos tiendan a cero.

## Criterios de convergencia

> [!WARNING]
> **Condición necesaria**: si $\sum a_n$ converge, entonces $a_n \to 0$. El recíproco es **falso**: que $a_n \to 0$ no garantiza convergencia (serie armónica). Sí sirve al revés: si $a_n \not\to 0$, la serie diverge.

| Criterio | Cuándo usarlo | Regla |
|---|---|---|
| Comparación | términos parecidos a una serie conocida | si $0 \le a_n \le b_n$ y $\sum b_n$ converge, $\sum a_n$ converge |
| Comparación en el límite | cocientes de polinomios | si $\lim \dfrac{a_n}{b_n} = c > 0$, ambas se comportan igual |
| Cociente (D'Alembert) | factoriales y potencias | $L = \lim \left\lvert\dfrac{a_{n+1}}{a_n}\right\rvert$ |
| Raíz (Cauchy) | potencias enésimas | $L = \lim \sqrt[n]{\lvert a_n \rvert}$ |
| Integral | $a_n = f(n)$ con $f$ decreciente | $\sum a_n$ y $\int_1^{\infty} f$ se comportan igual |
| Leibniz | series alternadas | converge si $\lvert a_n \rvert$ decrece a $0$ |

En los criterios del cociente y la raíz: converge si $L < 1$, diverge si $L > 1$, y no decide si $L = 1$.

> [!NOTE]
> ¿Converge $\displaystyle\sum \frac{2^n}{n!}$?
>
> $\dfrac{a_{n+1}}{a_n} = \dfrac{2^{n+1}}{(n+1)!} \cdot \dfrac{n!}{2^n} = \dfrac{2}{n + 1} \to 0 < 1$. **Converge**.

> [!NOTE]
> ¿Converge $\displaystyle\sum \frac{n}{n^3 + 1}$?
>
> Se parece a $\dfrac{1}{n^2}$. El cociente tiende a 1 y $\sum \dfrac{1}{n^2}$ converge: **converge**.

### Convergencia absoluta y condicional

- **Absoluta**: $\sum |a_n|$ converge. Implica que $\sum a_n$ converge.
- **Condicional**: $\sum a_n$ converge pero $\sum |a_n|$ no.

La serie armónica alternada $1 - \dfrac{1}{2} + \dfrac{1}{3} - \dots = \ln 2$ converge condicionalmente.

## Series de potencias

$$\sum_{n=0}^{\infty} c_n (x - a)^n$$

Es un «polinomio infinito» centrado en $a$. Converge en un intervalo simétrico $|x - a| < R$, donde $R$ es el **radio de convergencia**:

$$R = \lim_{n \to \infty} \left|\frac{c_n}{c_{n+1}}\right|$$

En los extremos $x = a \pm R$ hay que estudiar cada caso por separado.

> [!NOTE]
> $\displaystyle\sum \frac{x^n}{n}$: $R = \lim \dfrac{n + 1}{n} = 1$. En $x = 1$ es la armónica (diverge); en $x = -1$ es la armónica alternada (converge). Intervalo: $[-1, 1)$.

Dentro del intervalo, una serie de potencias se puede **derivar e integrar término a término**. Partiendo de la geométrica:

$$\frac{1}{1 - x} = \sum_{n=0}^{\infty} x^n \;\Longrightarrow\; \ln(1 + x) = \sum_{n=1}^{\infty} (-1)^{n+1}\frac{x^n}{n}, \quad |x| < 1$$

> [!TIP]
> Dos series sirven de patrón para casi todas las comparaciones: la geométrica y la serie $p$.
