+++
title = "Movimiento en dos dimensiones"
summary = "Tiro oblicuo, movimiento circular uniforme, aceleración centrípeta y movimiento relativo."
branch = "Mecánica"
order = 2
level = "Básico"
tags = ["tiro oblicuo", "movimiento circular", "aceleración centrípeta", "movimiento relativo"]
+++

## Vectores de posición, velocidad y aceleración

En el plano, el movimiento se describe con vectores:

$$\vec{r} = (x, y) \qquad \vec{v} = \frac{d\vec{r}}{dt} \qquad \vec{a} = \frac{d\vec{v}}{dt}$$

> [!IMPORTANT]
> **Principio de independencia**: el movimiento en $x$ y el movimiento en $y$ se estudian por separado. Solo comparten el tiempo.

La velocidad es siempre **tangente** a la trayectoria.

## Tiro oblicuo

Un proyectil lanzado con velocidad $v_0$ y ángulo $\theta$, despreciando el aire, solo tiene la aceleración de la gravedad hacia abajo.

| | Eje $x$ (MRU) | Eje $y$ (MRUV) |
|---|---|---|
| Aceleración | $0$ | $-g$ |
| Velocidad | $v_x = v_0\cos\theta$ | $v_y = v_0\sin\theta - gt$ |
| Posición | $x = v_0\cos\theta\,t$ | $y = y_0 + v_0\sin\theta\,t - \tfrac{1}{2}gt^2$ |

Eliminando el tiempo se obtiene la trayectoria, que es una **parábola**:

$$y = x\tan\theta - \frac{g}{2v_0^2\cos^2\theta}\,x^2$$

### Resultados para lanzamiento desde el suelo

$$t_{vuelo} = \frac{2v_0\sin\theta}{g} \qquad h_{máx} = \frac{v_0^2\sin^2\theta}{2g} \qquad R = \frac{v_0^2\sin 2\theta}{g}$$

- El **alcance** $R$ es máximo para $\theta = 45°$.
- Ángulos complementarios ($30°$ y $60°$) dan el mismo alcance.
- En el punto más alto, $v_y = 0$ pero $v_x$ no: la velocidad **no** es cero.

> [!NOTE]
> Se patea una pelota a $20$ m/s con un ángulo de $30°$ ($g = 9{,}8\ \text{m/s}^2$).
>
> $v_x = 20\cos 30° = 17{,}3$ m/s; $v_{0y} = 20\sin 30° = 10$ m/s.
>
> $t_{vuelo} = \dfrac{2 \cdot 10}{9{,}8} = 2{,}04$ s; $h_{máx} = \dfrac{10^2}{2 \cdot 9{,}8} = 5{,}1$ m; $R = 17{,}3 \cdot 2{,}04 = 35{,}3$ m.

> [!NOTE]
> **Tiro horizontal**: se lanza una piedra horizontalmente a $15$ m/s desde $45$ m de altura.
>
> Tiempo de caída: $45 = \tfrac{1}{2}(9{,}8)t^2 \Rightarrow t = 3{,}03$ s. Es el mismo que si se la dejara caer.
>
> Alcance: $15 \cdot 3{,}03 = 45{,}5$ m.

> [!WARNING]
> Las fórmulas de alcance y tiempo de vuelo solo valen si el proyectil **vuelve a la misma altura**. Si no, hay que usar las ecuaciones generales.

## Movimiento circular uniforme

Un objeto recorre una circunferencia de radio $r$ con rapidez constante.

| Magnitud | Símbolo | Relación |
|---|---|---|
| Período | $T$ | tiempo de una vuelta |
| Frecuencia | $f$ | $f = 1/T$ |
| Velocidad angular | $\omega$ | $\omega = \dfrac{2\pi}{T} = 2\pi f$ |
| Velocidad tangencial | $v$ | $v = \omega r$ |

Aunque la rapidez es constante, la **dirección** de la velocidad cambia: hay aceleración.

> [!IMPORTANT]
> **Aceleración centrípeta**, dirigida hacia el centro:
> $$a_c = \frac{v^2}{r} = \omega^2r$$

Por la segunda ley de Newton, hace falta una **fuerza centrípeta** $F_c = \dfrac{mv^2}{r}$. No es una fuerza nueva: es la resultante hacia el centro (tensión, rozamiento, gravedad, normal).

> [!NOTE]
> Un auto toma una curva de $50$ m de radio a $72$ km/h ($20$ m/s).
>
> $a_c = \dfrac{20^2}{50} = 8\ \text{m/s}^2$. El rozamiento debe proveerla: hace falta $\mu \ge \dfrac{a_c}{g} = 0{,}82$.

> [!WARNING]
> La «fuerza centrífuga» no existe en un sistema inercial. Lo que se siente al doblar es la inercia: el cuerpo tiende a seguir en línea recta.

### Movimiento circular no uniforme

Si la rapidez cambia, hay además **aceleración tangencial** $a_t = \dfrac{dv}{dt} = \alpha r$. La aceleración total es $a = \sqrt{a_c^2 + a_t^2}$.

Las ecuaciones angulares son análogas a las lineales:

| Lineal | Angular |
|---|---|
| $v = v_0 + at$ | $\omega = \omega_0 + \alpha t$ |
| $x = x_0 + v_0t + \tfrac{1}{2}at^2$ | $\theta = \theta_0 + \omega_0t + \tfrac{1}{2}\alpha t^2$ |
| $v^2 = v_0^2 + 2a\Delta x$ | $\omega^2 = \omega_0^2 + 2\alpha\Delta\theta$ |

## Movimiento relativo

La velocidad depende del observador. Si $A$ se mueve respecto de $B$, y $B$ respecto de $C$:

$$\vec{v}_{A/C} = \vec{v}_{A/B} + \vec{v}_{B/C}$$

> [!NOTE]
> Un bote cruza un río de $100$ m de ancho apuntando perpendicular a la orilla a $4$ m/s. La corriente es de $3$ m/s.
>
> Velocidad respecto de la orilla: $\sqrt{4^2 + 3^2} = 5$ m/s. Tarda $\dfrac{100}{4} = 25$ s en cruzar y es arrastrado $3 \cdot 25 = 75$ m río abajo.
>
> Para llegar justo enfrente debe apuntar aguas arriba con $\sin\theta = \tfrac{3}{4}$, y entonces cruza a $\sqrt{16 - 9} = 2{,}65$ m/s.

La suma simple de velocidades vale para rapideces mucho menores que la de la luz.

> [!TIP]
> Descomponer siempre en ejes. Un problema difícil en dos dimensiones son dos problemas fáciles en una.
