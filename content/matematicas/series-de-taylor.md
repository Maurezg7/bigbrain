+++
title = "Series de Taylor"
summary = "Cómo aproximar funciones con polinomios, las series de Maclaurin más usadas, el error de la aproximación y sus aplicaciones."
branch = "Cálculo"
order = 8
level = "Avanzado"
tags = ["Taylor", "Maclaurin", "aproximación", "series de potencias"]
+++

## La idea

Un polinomio es fácil de evaluar, derivar e integrar. La serie de Taylor reemplaza una función complicada por un polinomio que **coincide con ella y con todas sus derivadas** en un punto.

> [!IMPORTANT]
> **Serie de Taylor** de $f$ centrada en $a$:
> $$f(x) = \sum_{n=0}^{\infty} \frac{f^{(n)}(a)}{n!}(x - a)^n = f(a) + f'(a)(x - a) + \frac{f''(a)}{2!}(x - a)^2 + \dots$$

Cuando $a = 0$ se llama **serie de Maclaurin**.

Cortando la serie en el grado $n$ se obtiene el **polinomio de Taylor** $P_n(x)$:

- $P_1$: la recta tangente.
- $P_2$: la parábola que mejor aproxima.
- A mayor grado, mejor aproximación cerca de $a$.

## Series fundamentales

| Función | Serie de Maclaurin | Converge para |
|---|---|---|
| $e^x$ | $1 + x + \dfrac{x^2}{2!} + \dfrac{x^3}{3!} + \dots$ | todo $x$ |
| $\sin x$ | $x - \dfrac{x^3}{3!} + \dfrac{x^5}{5!} - \dots$ | todo $x$ |
| $\cos x$ | $1 - \dfrac{x^2}{2!} + \dfrac{x^4}{4!} - \dots$ | todo $x$ |
| $\dfrac{1}{1 - x}$ | $1 + x + x^2 + x^3 + \dots$ | $\lvert x \rvert < 1$ |
| $\ln(1 + x)$ | $x - \dfrac{x^2}{2} + \dfrac{x^3}{3} - \dots$ | $-1 < x \le 1$ |
| $\arctan x$ | $x - \dfrac{x^3}{3} + \dfrac{x^5}{5} - \dots$ | $\lvert x \rvert \le 1$ |
| $(1 + x)^k$ | $1 + kx + \dfrac{k(k-1)}{2!}x^2 + \dots$ | $\lvert x \rvert < 1$ |

> [!NOTE]
> Deducir la serie de $e^x$. Todas las derivadas de $e^x$ son $e^x$, y en $0$ valen $1$:
>
> $$e^x = \sum_{n=0}^{\infty} \frac{x^n}{n!}$$
>
> Con $x = 1$: $e = 1 + 1 + \dfrac{1}{2} + \dfrac{1}{6} + \dfrac{1}{24} + \dots \approx 2{,}718$.

## Obtener series nuevas

No hace falta derivar cada vez: se parte de las series conocidas.

- **Sustitución**: $e^{-x^2} = 1 - x^2 + \dfrac{x^4}{2!} - \dfrac{x^6}{3!} + \dots$
- **Derivación o integración** término a término.
- **Producto**: $x\sin x = x^2 - \dfrac{x^4}{6} + \dots$

> [!NOTE]
> Integrar lo que no tiene primitiva elemental:
>
> $$\int_0^1 e^{-x^2}dx = \int_0^1 \left(1 - x^2 + \frac{x^4}{2} - \frac{x^6}{6} + \dots\right)dx = 1 - \frac{1}{3} + \frac{1}{10} - \frac{1}{42} + \dots \approx 0{,}747$$

## El error

El **resto de Lagrange** acota el error al cortar en grado $n$:

$$R_n(x) = \frac{f^{(n+1)}(c)}{(n+1)!}(x - a)^{n+1}, \quad c \text{ entre } a \text{ y } x$$

Si $|f^{(n+1)}| \le M$ en el intervalo:

$$|R_n(x)| \le \frac{M}{(n+1)!}|x - a|^{n+1}$$

El error crece al alejarse de $a$ y disminuye al aumentar $n$.

> [!NOTE]
> ¿Qué error se comete con $\sin x \approx x$ para $x = 0{,}1$ rad?
>
> El siguiente término es $\dfrac{x^3}{6} = \dfrac{0{,}001}{6} \approx 0{,}00017$. El error relativo es menor al $0{,}2\,\%$.

## Aproximaciones para valores pequeños

Con $|x| \ll 1$ (ángulos en radianes):

| Función | Aproximación |
|---|---|
| $\sin x$ | $x$ |
| $\tan x$ | $x$ |
| $\cos x$ | $1 - \dfrac{x^2}{2}$ |
| $e^x$ | $1 + x$ |
| $\ln(1 + x)$ | $x$ |
| $(1 + x)^k$ | $1 + kx$ |
| $\sqrt{1 + x}$ | $1 + \dfrac{x}{2}$ |
| $\dfrac{1}{1 + x}$ | $1 - x$ |

Son la base de la **linealización** en física e ingeniería: el péndulo simple ($\sin\theta \approx \theta$), la dilatación térmica, los modelos de pequeña señal en electrónica.

## Aplicaciones

- **Límites**: $\displaystyle\lim_{x \to 0} \frac{\sin x - x}{x^3} = \lim_{x \to 0} \frac{-x^3/6 + \dots}{x^3} = -\frac{1}{6}$.
- **Cálculo numérico**: así evalúan funciones las calculadoras y bibliotecas matemáticas.
- **Fórmula de Euler**: sustituyendo $ix$ en la serie de $e^x$ y separando parte real e imaginaria:

$$e^{ix} = \cos x + i\sin x$$

- **Ecuaciones diferenciales**: se busca la solución como serie de potencias.
- **Relatividad**: $E = \dfrac{mc^2}{\sqrt{1 - v^2/c^2}} \approx mc^2 + \dfrac{1}{2}mv^2$ para $v \ll c$, que recupera la energía cinética clásica.

> [!WARNING]
> La serie solo representa a la función **dentro de su intervalo de convergencia**. La serie de $\dfrac{1}{1 - x}$ no sirve para $x = 2$, aunque la función esté definida allí.

> [!TIP]
> Cerca de un punto, toda función suave se comporta como un polinomio. Cuantos más términos, más lejos llega la aproximación.
