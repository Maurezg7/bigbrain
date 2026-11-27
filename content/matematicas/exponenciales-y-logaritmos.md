+++
title = "Exponenciales y logaritmos"
summary = "Dos funciones inversas que modelan crecimiento, decaimiento y escalas como el pH o los decibeles."
branch = "Álgebra"
order = 2
level = "Básico"
tags = ["funciones", "logaritmo", "crecimiento"]
+++

## La función exponencial

Una función exponencial tiene la variable en el exponente: $f(x) = a^x$ con $a > 0$ y $a \neq 1$. Si $a > 1$ crece cada vez más rápido; si $0 < a < 1$ decae hacia cero. La base más usada es $e \approx 2{,}71828$, porque $e^x$ es la única exponencial que es su propia derivada.

Propiedades de las potencias, válidas para cualquier base positiva:

$$a^{x} a^{y} = a^{x+y}, \qquad \frac{a^x}{a^y} = a^{x-y}, \qquad (a^x)^y = a^{xy}, \qquad a^0 = 1$$

## El logaritmo

> [!IMPORTANT]
> El **logaritmo en base $a$** de $x$ es el exponente al que hay que elevar $a$ para obtener $x$:
> $\log_a x = y \iff a^y = x$, con $x > 0$.

Es la función inversa de la exponencial: $\log_a(a^x) = x$ y $a^{\log_a x} = x$. Se escribe $\ln x$ cuando la base es $e$ y $\log x$ cuando es 10.

Cada propiedad de las potencias se traduce en una del logaritmo:

| Propiedad | Fórmula |
|---|---|
| Producto | $\log_a(xy) = \log_a x + \log_a y$ |
| Cociente | $\log_a(x/y) = \log_a x - \log_a y$ |
| Potencia | $\log_a(x^n) = n \log_a x$ |
| Cambio de base | $\log_a x = \dfrac{\ln x}{\ln a}$ |

## Resolver ecuaciones exponenciales

La estrategia es siempre la misma: aislar la exponencial y aplicar logaritmo a ambos lados para "bajar" el exponente.

> [!NOTE]
> Un cultivo de bacterias se duplica cada 3 horas. ¿Cuánto tarda en multiplicarse por 10?
>
> El modelo es $N(t) = N_0 \cdot 2^{t/3}$. Buscamos $t$ tal que $2^{t/3} = 10$:
>
> $$\frac{t}{3}\ln 2 = \ln 10 \;\Rightarrow\; t = \frac{3 \ln 10}{\ln 2} \approx 9{,}97 \text{ horas}$$

## Crecimiento y decaimiento

Muchos fenómenos cambian a una velocidad proporcional a su propio tamaño. Su solución es $N(t) = N_0 e^{kt}$: crecimiento si $k>0$ (interés compuesto, poblaciones) y decaimiento si $k<0$ (radiactividad, descarga de un capacitor). El **tiempo de semivida** es $t_{1/2} = \dfrac{\ln 2}{|k|}$.

> [!WARNING]
> $\log(x + y)$ **no** es $\log x + \log y$. La propiedad vale para el logaritmo de un producto, no de una suma.
