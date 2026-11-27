+++
title = "Variable compleja"
summary = "Funciones de variable compleja, ecuaciones de Cauchy-Riemann, integración compleja, fórmula de Cauchy y teorema de los residuos."
branch = "Análisis numérico y variable compleja"
order = 2
level = "Avanzado"
tags = ["variable compleja", "Cauchy-Riemann", "residuos", "funciones analíticas"]
+++

## Funciones complejas

Una función de variable compleja toma $z = x + iy$ y devuelve $w = f(z) = u(x, y) + iv(x, y)$.

> [!NOTE]
> $f(z) = z^2 = (x + iy)^2 = (x^2 - y^2) + i(2xy)$. Entonces $u = x^2 - y^2$ y $v = 2xy$.

Como entrada y salida tienen dos dimensiones, no se puede dibujar un gráfico habitual. Se piensa a $f$ como una **transformación del plano**.

### Funciones elementales

| Función | Definición | Propiedad |
|---|---|---|
| Exponencial | $e^z = e^x(\cos y + i\sin y)$ | periódica, de período $2\pi i$ |
| Seno | $\sin z = \dfrac{e^{iz} - e^{-iz}}{2i}$ | no está acotado |
| Coseno | $\cos z = \dfrac{e^{iz} + e^{-iz}}{2}$ | no está acotado |
| Logaritmo | $\ln z = \ln\lvert z \rvert + i(\theta + 2k\pi)$ | tiene infinitos valores |

El valor con $-\pi < \theta \le \pi$ es la **rama principal** del logaritmo. Por ejemplo, $\ln(-1) = i\pi$.

## Derivada compleja

$$f'(z) = \lim_{\Delta z \to 0}\frac{f(z + \Delta z) - f(z)}{\Delta z}$$

La definición es igual a la real, pero $\Delta z$ puede acercarse a cero **desde cualquier dirección** del plano y el límite debe ser el mismo. Es una condición muy exigente.

> [!IMPORTANT]
> **Ecuaciones de Cauchy-Riemann**: $f = u + iv$ es derivable solo si
> $$\frac{\partial u}{\partial x} = \frac{\partial v}{\partial y} \qquad \frac{\partial u}{\partial y} = -\frac{\partial v}{\partial x}$$

Una función derivable en todo un entorno se llama **analítica** u **holomorfa**.

> [!NOTE]
> Para $f(z) = z^2$: $u_x = 2x = v_y$ y $u_y = -2y = -v_x$. ✓ Es analítica, con $f'(z) = 2z$.
>
> Para $f(z) = \bar{z} = x - iy$: $u_x = 1$ pero $v_y = -1$. ✗ No es analítica en ningún punto.

Las reglas de derivación (suma, producto, cadena) son las mismas que en variable real.

### Funciones armónicas

Si $f$ es analítica, $u$ y $v$ cumplen la ecuación de Laplace:

$$u_{xx} + u_{yy} = 0 \qquad v_{xx} + v_{yy} = 0$$

Por eso las funciones analíticas resuelven problemas de potencial eléctrico, flujo de fluidos y calor en dos dimensiones. Además son **transformaciones conformes**: conservan los ángulos, y permiten convertir una geometría complicada en una simple.

## Integración compleja

La integral se calcula sobre una **curva** $C$ del plano, parametrizada por $z(t)$:

$$\int_C f(z)\,dz = \int_a^b f(z(t))\,z'(t)\,dt$$

> [!IMPORTANT]
> **Teorema de Cauchy**: si $f$ es analítica dentro y sobre una curva cerrada $C$,
> $$\oint_C f(z)\,dz = 0$$

Consecuencia: la integral entre dos puntos no depende del camino, y se puede usar una primitiva como en el caso real.

> [!NOTE]
> El ejemplo fundamental, sobre una circunferencia centrada en el origen:
>
> $$\oint_C \frac{dz}{z} = 2\pi i$$
>
> No da cero porque $\dfrac{1}{z}$ no es analítica en $z = 0$, que queda dentro de la curva.

### Fórmula integral de Cauchy

Si $f$ es analítica dentro y sobre $C$, y $z_0$ está en el interior:

$$f(z_0) = \frac{1}{2\pi i}\oint_C \frac{f(z)}{z - z_0}\,dz$$

Los valores en el borde determinan todos los valores en el interior. Además, una función analítica tiene **derivadas de todos los órdenes**:

$$f^{(n)}(z_0) = \frac{n!}{2\pi i}\oint_C \frac{f(z)}{(z - z_0)^{n+1}}\,dz$$

## Series de Laurent y singularidades

Cerca de un punto donde $f$ no es analítica (una **singularidad**), se usa una serie con potencias negativas:

$$f(z) = \sum_{n=-\infty}^{\infty} a_n(z - z_0)^n$$

| Tipo de singularidad | Potencias negativas | Ejemplo |
|---|---|---|
| Evitable | ninguna | $\dfrac{\sin z}{z}$ en $0$ |
| Polo de orden $m$ | hasta $(z - z_0)^{-m}$ | $\dfrac{1}{z^2}$ en $0$ |
| Esencial | infinitas | $e^{1/z}$ en $0$ |

## Residuos

El **residuo** de $f$ en $z_0$ es el coeficiente $a_{-1}$ de su serie de Laurent.

Para un **polo simple**:

$$\text{Res}(f, z_0) = \lim_{z \to z_0}(z - z_0)f(z)$$

Para un polo de orden $m$:

$$\text{Res}(f, z_0) = \frac{1}{(m - 1)!}\lim_{z \to z_0}\frac{d^{m-1}}{dz^{m-1}}\left[(z - z_0)^mf(z)\right]$$

> [!IMPORTANT]
> **Teorema de los residuos**:
> $$\oint_C f(z)\,dz = 2\pi i\sum\text{Res}(f, z_k)$$
> sumando sobre las singularidades encerradas por $C$.

> [!NOTE]
> $\displaystyle\oint_C \frac{dz}{z^2 + 1}$ sobre la circunferencia $|z| = 2$.
>
> Polos en $z = \pm i$, ambos dentro. $\text{Res}(i) = \dfrac{1}{2i}$ y $\text{Res}(-i) = -\dfrac{1}{2i}$. La suma es $0$: la integral vale $0$.

### Integrales reales

Los residuos permiten calcular integrales reales difíciles. Cerrando el camino con un semicírculo en el semiplano superior:

$$\int_{-\infty}^{\infty}\frac{dx}{x^2 + 1} = 2\pi i\,\text{Res}(i) = 2\pi i \cdot \frac{1}{2i} = \pi$$

## Aplicaciones

| Área | Uso |
|---|---|
| Circuitos de alterna | fasores e impedancias complejas |
| Control | polos y ceros de la función de transferencia en el plano $s$ |
| Transformadas | la inversa de Laplace es una integral compleja |
| Fluidos | potencial complejo y perfiles alares (transformación de Joukowski) |
| Señales | transformada Z y filtros digitales |

> [!TIP]
> Ser derivable en sentido complejo es mucho más fuerte que en sentido real: implica infinitas derivadas, serie de Taylor convergente y que el borde determine el interior.
