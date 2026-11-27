+++
title = "Derivadas parciales y gradiente"
summary = "Funciones de varias variables, derivadas parciales, gradiente, derivada direccional, regla de la cadena, extremos y multiplicadores de Lagrange."
branch = "Cálculo en varias variables"
order = 1
level = "Avanzado"
tags = ["derivadas parciales", "gradiente", "Lagrange", "varias variables"]
+++

## Funciones de varias variables

Muchas magnitudes dependen de más de una variable: la presión de un gas depende de la temperatura y el volumen; la temperatura de una placa, de la posición $(x, y)$.

Una función $z = f(x, y)$ se representa como una **superficie** en el espacio o mediante **curvas de nivel** $f(x, y) = c$, como las de un mapa topográfico.

## Derivadas parciales

> [!IMPORTANT]
> La derivada parcial respecto de $x$ mide cómo cambia $f$ al variar $x$ **manteniendo $y$ constante**:
> $$\frac{\partial f}{\partial x} = \lim_{h \to 0} \frac{f(x + h, y) - f(x, y)}{h}$$

Se calcula derivando normalmente y tratando las otras variables como constantes. Se escribe también $f_x$.

> [!NOTE]
> Para $f(x, y) = x^2 y + \sin(xy)$:
>
> $f_x = 2xy + y\cos(xy)$ y $f_y = x^2 + x\cos(xy)$.

### Derivadas de orden superior

$$f_{xx} = \frac{\partial^2 f}{\partial x^2} \qquad f_{yy} = \frac{\partial^2 f}{\partial y^2} \qquad f_{xy} = \frac{\partial^2 f}{\partial y\,\partial x}$$

**Teorema de Schwarz**: si las derivadas cruzadas son continuas, $f_{xy} = f_{yx}$.

## Plano tangente y diferencial

El plano tangente a $z = f(x, y)$ en $(x_0, y_0)$:

$$z = f(x_0, y_0) + f_x(x_0, y_0)(x - x_0) + f_y(x_0, y_0)(y - y_0)$$

El **diferencial total** estima el cambio de $f$:

$$df = \frac{\partial f}{\partial x}dx + \frac{\partial f}{\partial y}dy$$

Se usa para **propagar errores** de medición.

> [!NOTE]
> El volumen de un cilindro es $V = \pi r^2 h$. Si $r = 5$ cm y $h = 10$ cm se miden con error de $0{,}1$ cm:
>
> $dV = 2\pi r h\,dr + \pi r^2\,dh = 2\pi(50)(0{,}1) + \pi(25)(0{,}1) \approx 39{,}3\ \text{cm}^3$.

## Regla de la cadena

Si $z = f(x, y)$ con $x = x(t)$, $y = y(t)$:

$$\frac{dz}{dt} = \frac{\partial f}{\partial x}\frac{dx}{dt} + \frac{\partial f}{\partial y}\frac{dy}{dt}$$

## Gradiente y derivada direccional

> [!IMPORTANT]
> $$\nabla f = \left(\frac{\partial f}{\partial x}, \frac{\partial f}{\partial y}, \frac{\partial f}{\partial z}\right)$$

La **derivada direccional** en la dirección del versor $\hat{u}$ es

$$D_{\hat{u}} f = \nabla f \cdot \hat{u}$$

Propiedades del gradiente:

- Apunta en la dirección de **máximo crecimiento** de $f$.
- Su módulo es la tasa de crecimiento máxima.
- Es **perpendicular** a las curvas (o superficies) de nivel.

> [!NOTE]
> Temperatura $T(x, y) = 100 - x^2 - 2y^2$. En $(1, 1)$: $\nabla T = (-2x, -4y) = (-2, -4)$.
>
> La temperatura crece más rápido en la dirección $(-2, -4)$, a razón de $\sqrt{20} \approx 4{,}47$ grados por unidad de longitud.

## Extremos

Los **puntos críticos** cumplen $\nabla f = \vec{0}$, es decir $f_x = 0$ y $f_y = 0$.

Para clasificarlos se usa el **hessiano**:

$$D = f_{xx} f_{yy} - (f_{xy})^2$$

| Condición | Tipo |
|---|---|
| $D > 0$ y $f_{xx} > 0$ | mínimo local |
| $D > 0$ y $f_{xx} < 0$ | máximo local |
| $D < 0$ | punto silla |
| $D = 0$ | el criterio no decide |

> [!NOTE]
> $f(x, y) = x^2 + y^2 - 2x - 4y$.
>
> $f_x = 2x - 2 = 0 \Rightarrow x = 1$; $f_y = 2y - 4 = 0 \Rightarrow y = 2$.
>
> $D = 2 \cdot 2 - 0 = 4 > 0$ y $f_{xx} = 2 > 0$: **mínimo** en $(1, 2)$, con $f = -5$.

Un **punto silla** es máximo en una dirección y mínimo en otra, como una silla de montar.

## Multiplicadores de Lagrange

Para optimizar $f(x, y)$ sujeta a una **restricción** $g(x, y) = c$:

> [!IMPORTANT]
> $$\nabla f = \lambda\,\nabla g, \qquad g(x, y) = c$$

En el óptimo, la curva de nivel de $f$ es tangente a la restricción, y por eso los gradientes son paralelos.

> [!NOTE]
> Rectángulo de área máxima con perímetro 20: maximizar $f = xy$ con $2x + 2y = 20$.
>
> $(y, x) = \lambda(2, 2) \Rightarrow y = 2\lambda = x$. Con la restricción: $x = y = 5$. Es un cuadrado de área 25.

## Aplicaciones

- **Ajuste por mínimos cuadrados**: se minimiza la suma de errores al cuadrado respecto de los parámetros.
- **Aprendizaje automático**: el descenso por gradiente avanza en dirección $-\nabla f$.
- **Termodinámica**: las propiedades son derivadas parciales, como $c_p = \left(\dfrac{\partial h}{\partial T}\right)_p$.
- **Física**: una fuerza conservativa es $\vec{F} = -\nabla U$.

> [!WARNING]
> Que existan las derivadas parciales en un punto **no** garantiza que la función sea continua ni diferenciable allí. Hace falta que sean continuas.

> [!TIP]
> El gradiente resume toda la información de primer orden: hacia dónde crece la función y cuánto.
