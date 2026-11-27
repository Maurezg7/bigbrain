+++
title = "Derivadas"
summary = "La derivada mide qué tan rápido cambia una función: es la pendiente de la recta tangente."
branch = "Cálculo"
order = 2
level = "Intermedio"
tags = ["derivada", "tangente", "optimización"]
+++

## Qué mide la derivada

La pendiente de la recta que une dos puntos del gráfico de $f$ (recta secante) es el cambio promedio $\frac{f(x+h) - f(x)}{h}$. Si acercamos los puntos haciendo $h \to 0$, la secante se convierte en la **recta tangente** y su pendiente es la derivada.

> [!IMPORTANT]
> $$f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

![Una recta secante que se convierte en tangente](/media/matematicas/derivada.svg)

*Al achicar $h$, la secante (punteada) tiende a la tangente en $P$ (naranja).*

Si $f$ es la posición de un objeto en función del tiempo, $f'$ es su velocidad y $f''$ su aceleración.

## Tabla de derivadas

| $f(x)$ | $f'(x)$ |
|---|---|
| $c$ (constante) | $0$ |
| $x^n$ | $n x^{n-1}$ |
| $e^x$ | $e^x$ |
| $\ln x$ | $1/x$ |
| $\sin x$ | $\cos x$ |
| $\cos x$ | $-\sin x$ |
| $\tan x$ | $1/\cos^2 x$ |

## Reglas de derivación

$$(f + g)' = f' + g' \qquad (c f)' = c f'$$

$$(f g)' = f' g + f g' \qquad \left(\frac{f}{g}\right)' = \frac{f' g - f g'}{g^2}$$

**Regla de la cadena**, para funciones compuestas:

$$\big(f(g(x))\big)' = f'(g(x)) \cdot g'(x)$$

> [!NOTE]
> Derivar $h(x) = \sin(3x^2)$.
>
> La función de afuera es $\sin u$ y la de adentro $u = 3x^2$. Entonces
> $h'(x) = \cos(3x^2) \cdot 6x = 6x\cos(3x^2)$.

## Máximos y mínimos

En un máximo o mínimo local de una función derivable la tangente es horizontal: $f'(x) = 0$. Esos son los **puntos críticos**. Para clasificarlos:

- Si $f''(x_0) > 0$, es un **mínimo** (la curva es cóncava hacia arriba).
- Si $f''(x_0) < 0$, es un **máximo**.
- Si $f''(x_0) = 0$, el criterio no decide: hay que mirar el signo de $f'$ a cada lado.

> [!NOTE]
> Con 40 m de alambre, ¿qué rectángulo encierra el área máxima?
>
> Si un lado mide $x$, el otro mide $20 - x$ y el área es $A(x) = 20x - x^2$. Derivando: $A'(x) = 20 - 2x = 0 \Rightarrow x = 10$. Como $A'' = -2 < 0$ es un máximo: un cuadrado de 10 m de lado, con área $100\ \text{m}^2$.
