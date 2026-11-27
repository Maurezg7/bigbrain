+++
title = "Series de Fourier y ecuaciones en derivadas parciales"
summary = "Descomposición de funciones periódicas en senos y cosenos, y las ecuaciones del calor, de ondas y de Laplace."
branch = "Ecuaciones diferenciales"
order = 5
level = "Avanzado"
tags = ["Fourier", "ecuación del calor", "ecuación de ondas", "separación de variables"]
+++

## Series de Fourier

Cualquier función periódica razonable puede escribirse como suma de senos y cosenos. Un sonido complejo es una suma de tonos puros.

> [!IMPORTANT]
> Para $f$ de período $2L$:
> $$f(x) = \frac{a_0}{2} + \sum_{n=1}^{\infty}\left[a_n\cos\frac{n\pi x}{L} + b_n\sin\frac{n\pi x}{L}\right]$$
> $$a_n = \frac{1}{L}\int_{-L}^{L} f(x)\cos\frac{n\pi x}{L}\,dx \qquad b_n = \frac{1}{L}\int_{-L}^{L} f(x)\sin\frac{n\pi x}{L}\,dx$$

- $\dfrac{a_0}{2}$ es el **valor medio** de $f$.
- El término $n = 1$ es la **fundamental**; los demás son **armónicos**.

Las fórmulas salen de la **ortogonalidad**: la integral del producto de dos senos o cosenos de distinta frecuencia sobre un período es cero. Cada coeficiente es una proyección.

### Simetrías

| Si $f$ es | Entonces | Serie |
|---|---|---|
| par: $f(-x) = f(x)$ | $b_n = 0$ | solo cosenos |
| impar: $f(-x) = -f(x)$ | $a_n = 0$ | solo senos |

> [!NOTE]
> Onda cuadrada: $f(x) = 1$ en $(0, \pi)$ y $-1$ en $(-\pi, 0)$. Es impar, así que $a_n = 0$.
>
> $b_n = \dfrac{2}{\pi}\displaystyle\int_0^{\pi}\sin nx\,dx = \dfrac{2}{n\pi}(1 - \cos n\pi)$, que vale $\dfrac{4}{n\pi}$ para $n$ impar y $0$ para $n$ par.
>
> $$f(x) = \frac{4}{\pi}\left(\sin x + \frac{\sin 3x}{3} + \frac{\sin 5x}{5} + \dots\right)$$

### Convergencia

Si $f$ es suave a trozos, la serie converge a $f(x)$ donde es continua, y al **promedio de los límites laterales** en cada salto. Cerca de los saltos aparece un sobrepico de cerca del 9 % que no desaparece al sumar más términos (**fenómeno de Gibbs**).

**Identidad de Parseval**: la energía de la señal es la suma de las energías de sus armónicos.

$$\frac{1}{L}\int_{-L}^{L} f^2\,dx = \frac{a_0^2}{2} + \sum_{n=1}^{\infty}(a_n^2 + b_n^2)$$

### Forma compleja y transformada

$$f(x) = \sum_{n=-\infty}^{\infty} c_ne^{in\pi x/L}$$

Para funciones no periódicas, la suma se convierte en integral: la **transformada de Fourier** $F(\omega) = \displaystyle\int_{-\infty}^{\infty} f(t)e^{-i\omega t}dt$, que da el espectro de frecuencias de una señal.

## Ecuaciones en derivadas parciales

Una **EDP** involucra derivadas respecto de varias variables, típicamente posición y tiempo.

| Ecuación | Forma | Describe |
|---|---|---|
| Calor (difusión) | $u_t = \alpha\,u_{xx}$ | conducción de calor, difusión |
| Ondas | $u_{tt} = c^2u_{xx}$ | cuerdas, sonido, luz |
| Laplace | $u_{xx} + u_{yy} = 0$ | estados estacionarios, potencial |

Para determinar la solución hacen falta **condiciones de contorno** (qué pasa en los bordes) y **condiciones iniciales**.

## Separación de variables

Se busca una solución producto: $u(x, t) = X(x)\,T(t)$.

### Ecuación del calor

Una barra de longitud $L$ con extremos a temperatura cero: $u(0, t) = u(L, t) = 0$, y temperatura inicial $u(x, 0) = f(x)$.

Sustituyendo $u = XT$ en $u_t = \alpha u_{xx}$:

$$\frac{T'}{\alpha T} = \frac{X''}{X} = -\lambda$$

Un lado depende solo de $t$ y el otro solo de $x$, así que ambos son constantes. Quedan dos EDO:

- $X'' + \lambda X = 0$ con $X(0) = X(L) = 0$, cuyas soluciones son $X_n = \sin\dfrac{n\pi x}{L}$.
- $T' = -\alpha\lambda T$, con solución $T_n = e^{-\alpha(n\pi/L)^2t}$.

Por superposición:

> [!IMPORTANT]
> $$u(x, t) = \sum_{n=1}^{\infty} b_n\sin\frac{n\pi x}{L}\,e^{-\alpha(n\pi/L)^2t}$$
> donde los $b_n$ son los coeficientes de Fourier de la temperatura inicial $f(x)$.

Cada modo decae exponencialmente, y los de alta frecuencia mucho más rápido: el calor **suaviza** las irregularidades.

### Ecuación de ondas

Para una cuerda fija en sus extremos:

$$u(x, t) = \sum_{n=1}^{\infty}\sin\frac{n\pi x}{L}\left[A_n\cos\frac{n\pi ct}{L} + B_n\sin\frac{n\pi ct}{L}\right]$$

Cada término es un **modo normal** con frecuencia $f_n = \dfrac{nc}{2L}$: la fundamental y sus armónicos. Por eso los instrumentos de cuerda producen notas definidas.

A diferencia del calor, las ondas **no se amortiguan**: la energía se conserva.

Solución de D'Alembert: $u(x, t) = F(x - ct) + G(x + ct)$, dos ondas que viajan en sentidos opuestos.

### Ecuación de Laplace

$\nabla^2u = 0$ describe situaciones de equilibrio: temperatura estacionaria, potencial eléctrico sin cargas, flujo ideal. Sus soluciones (funciones **armónicas**) no tienen máximos ni mínimos en el interior: los extremos están siempre en el borde.

## Aplicaciones

- **Procesamiento de señales**: filtros, ecualizadores, compresión MP3 y JPEG.
- **Electrotecnia**: análisis de armónicos en redes eléctricas.
- **Vibraciones**: modos propios de estructuras.
- **Transferencia de calor**: enfriamiento de piezas.
- **Telecomunicaciones**: modulación y ancho de banda.

> [!TIP]
> Fourier convierte un problema difícil con una función cualquiera en infinitos problemas fáciles con senoides. Funciona porque las ecuaciones son lineales.
