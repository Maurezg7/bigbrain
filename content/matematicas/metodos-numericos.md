+++
title = "Métodos numéricos"
summary = "Errores de redondeo, búsqueda de raíces, interpolación, integración numérica y resolución aproximada de sistemas y ecuaciones diferenciales."
branch = "Análisis numérico y variable compleja"
order = 1
level = "Avanzado"
tags = ["métodos numéricos", "Newton-Raphson", "interpolación", "Simpson", "Runge-Kutta"]
+++

## Por qué métodos numéricos

La mayoría de los problemas reales no tienen solución exacta con fórmula: integrales sin primitiva, ecuaciones no lineales, sistemas de miles de incógnitas. Los métodos numéricos dan **aproximaciones** con un error controlado.

## Errores

| Error | Definición |
|---|---|
| Absoluto | $E_a = \lvert x - \tilde{x} \rvert$ |
| Relativo | $E_r = \dfrac{\lvert x - \tilde{x} \rvert}{\lvert x \rvert}$ |

Fuentes de error:

- **Redondeo**: la computadora guarda una cantidad finita de cifras (unas 16 en doble precisión). Por eso $0{,}1 + 0{,}2 \neq 0{,}3$ exactamente.
- **Truncamiento**: se corta un proceso infinito (una serie, un límite).

> [!WARNING]
> **Cancelación catastrófica**: restar dos números casi iguales destruye cifras significativas. Conviene reformular: en vez de $\sqrt{x + 1} - \sqrt{x}$, usar $\dfrac{1}{\sqrt{x + 1} + \sqrt{x}}$.

Un problema está **mal condicionado** si pequeños cambios en los datos producen grandes cambios en el resultado.

## Raíces de ecuaciones

Resolver $f(x) = 0$.

### Bisección

Si $f(a)$ y $f(b)$ tienen signos opuestos, hay una raíz entre ellos. Se toma el punto medio y se conserva la mitad donde cambia el signo.

- Siempre converge, pero lento: el error se reduce a la mitad en cada paso.
- Tras $n$ pasos: error $\le \dfrac{b - a}{2^n}$.

### Newton-Raphson

> [!IMPORTANT]
> $$x_{n+1} = x_n - \frac{f(x_n)}{f'(x_n)}$$

Sigue la tangente hasta el eje. Su convergencia es **cuadrática**: las cifras correctas se duplican en cada paso. Requiere la derivada y un buen valor inicial.

> [!NOTE]
> Calcular $\sqrt{2}$: raíz de $f(x) = x^2 - 2$. La iteración es $x_{n+1} = \dfrac{1}{2}\left(x_n + \dfrac{2}{x_n}\right)$.
>
> Con $x_0 = 1$: $1{,}5$; $1{,}41667$; $1{,}414216$; $1{,}41421356\dots$ Cuatro pasos dan ocho cifras.

| Método | Convergencia | Requiere | Robustez |
|---|---|---|---|
| Bisección | lineal | cambio de signo | siempre converge |
| Secante | superlineal (orden 1,62) | dos puntos iniciales | puede fallar |
| Newton-Raphson | cuadrática | derivada | puede fallar |
| Punto fijo $x = g(x)$ | lineal | $\lvert g' \rvert < 1$ | depende de $g$ |

## Sistemas lineales

Para $A\vec{x} = \vec{b}$:

- **Métodos directos**: eliminación de Gauss con **pivoteo** (se elige el mayor pivote para reducir errores) y factorización $LU$. Costo proporcional a $n^3$.
- **Métodos iterativos**: Jacobi y Gauss-Seidel. Parten de una estimación y la mejoran. Convienen para sistemas enormes con muchos ceros; convergen si la matriz es diagonalmente dominante.

El **número de condición** de $A$ indica cuánto se amplifican los errores.

## Interpolación

Dados $n + 1$ puntos, existe un único polinomio de grado $\le n$ que pasa por todos.

**Polinomio de Lagrange**:

$$P(x) = \sum_{i=0}^{n} y_iL_i(x), \qquad L_i(x) = \prod_{j \neq i}\frac{x - x_j}{x_i - x_j}$$

> [!NOTE]
> Interpolación lineal entre $(1, 2)$ y $(3, 8)$ en $x = 2$: $y = 2 + \dfrac{8 - 2}{3 - 1}(2 - 1) = 5$.

> [!WARNING]
> Usar un polinomio de grado alto con puntos equiespaciados produce oscilaciones violentas en los extremos (**fenómeno de Runge**). Es mejor usar **splines**: polinomios cúbicos por tramos, unidos suavemente.

Interpolar pasa por los puntos; **ajustar** (mínimos cuadrados) busca la tendencia cuando los datos tienen ruido.

## Derivación numérica

$$f'(x) \approx \frac{f(x + h) - f(x)}{h} \qquad \text{error} \sim h$$

$$f'(x) \approx \frac{f(x + h) - f(x - h)}{2h} \qquad \text{error} \sim h^2$$

$$f''(x) \approx \frac{f(x + h) - 2f(x) + f(x - h)}{h^2}$$

No conviene tomar $h$ demasiado pequeño: el error de redondeo crece al dividir por $h$.

## Integración numérica

Para $\displaystyle\int_a^b f(x)\,dx$ con $n$ subintervalos de ancho $h = \dfrac{b - a}{n}$:

**Regla del trapecio**:

$$\int_a^b f \approx \frac{h}{2}\left[f(x_0) + 2f(x_1) + \dots + 2f(x_{n-1}) + f(x_n)\right]$$

**Regla de Simpson** ($n$ par): aproxima con parábolas.

$$\int_a^b f \approx \frac{h}{3}\left[f(x_0) + 4f(x_1) + 2f(x_2) + 4f(x_3) + \dots + 4f(x_{n-1}) + f(x_n)\right]$$

| Regla | Error | Exacta para |
|---|---|---|
| Trapecio | $\sim h^2$ | rectas |
| Simpson | $\sim h^4$ | polinomios hasta grado 3 |

> [!NOTE]
> $\displaystyle\int_0^1 x^2dx$ con $n = 2$ ($h = 0{,}5$).
>
> Trapecio: $\dfrac{0{,}5}{2}[0 + 2(0{,}25) + 1] = 0{,}375$.
>
> Simpson: $\dfrac{0{,}5}{3}[0 + 4(0{,}25) + 1] = 0{,}3333$, que es el valor exacto.

## Ecuaciones diferenciales

Para $y' = f(t, y)$, $y(t_0) = y_0$, con paso $h$:

**Euler**: $y_{n+1} = y_n + h\,f(t_n, y_n)$. Error global $\sim h$.

**Runge-Kutta de orden 4**:

$$k_1 = f(t_n, y_n) \qquad k_2 = f\left(t_n + \tfrac{h}{2}, y_n + \tfrac{h}{2}k_1\right)$$

$$k_3 = f\left(t_n + \tfrac{h}{2}, y_n + \tfrac{h}{2}k_2\right) \qquad k_4 = f(t_n + h, y_n + hk_3)$$

$$y_{n+1} = y_n + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4)$$

Error global $\sim h^4$: reducir el paso a la mitad divide el error por 16.

Las ecuaciones en derivadas parciales se resuelven con **diferencias finitas** o **elementos finitos**, la base del software de simulación en ingeniería.

> [!TIP]
> Un resultado numérico sin estimación del error no sirve. Una verificación simple: repetir el cálculo con la mitad del paso y ver cuánto cambia.
