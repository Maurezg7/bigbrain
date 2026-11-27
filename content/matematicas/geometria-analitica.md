+++
title = "Geometría analítica"
summary = "Distancia, rectas, circunferencia, parábola, elipse e hipérbola descritas con ecuaciones, más coordenadas polares."
branch = "Geometría y trigonometría"
order = 3
level = "Intermedio"
tags = ["geometría analítica", "cónicas", "recta", "coordenadas polares"]
+++

## La idea

La geometría analítica traduce figuras a ecuaciones: un punto es un par $(x, y)$ y una curva es el conjunto de puntos que cumplen una ecuación.

## Puntos

Para $P_1(x_1, y_1)$ y $P_2(x_2, y_2)$:

$$d = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} \qquad M = \left(\frac{x_1 + x_2}{2}, \frac{y_1 + y_2}{2}\right)$$

## La recta

| Forma | Ecuación | Datos visibles |
|---|---|---|
| Explícita | $y = mx + b$ | pendiente y ordenada al origen |
| Punto-pendiente | $y - y_1 = m(x - x_1)$ | un punto y la pendiente |
| General | $Ax + By + C = 0$ | vector normal $(A, B)$ |
| Segmentaria | $\dfrac{x}{a} + \dfrac{y}{b} = 1$ | cortes con los ejes |

La pendiente es $m = \tan\theta$, con $\theta$ el ángulo respecto del eje $x$.

- **Paralelas**: $m_1 = m_2$. **Perpendiculares**: $m_1 m_2 = -1$.
- **Distancia de un punto a una recta**:

$$d = \frac{|Ax_0 + By_0 + C|}{\sqrt{A^2 + B^2}}$$

> [!NOTE]
> Recta perpendicular a $y = 2x + 1$ que pasa por $(4, 3)$.
>
> Pendiente: $-\tfrac{1}{2}$. Ecuación: $y - 3 = -\tfrac{1}{2}(x - 4) \Rightarrow y = -\tfrac{1}{2}x + 5$.

## Las cónicas

Son las curvas que se obtienen al cortar un cono con un plano. Todas responden a una ecuación de segundo grado.

### Circunferencia

Puntos a distancia $r$ de un centro $(h, k)$:

$$(x - h)^2 + (y - k)^2 = r^2$$

> [!NOTE]
> Centro y radio de $x^2 + y^2 - 4x + 6y - 3 = 0$.
>
> Completando cuadrados: $(x - 2)^2 - 4 + (y + 3)^2 - 9 - 3 = 0 \Rightarrow (x - 2)^2 + (y + 3)^2 = 16$. Centro $(2, -3)$, radio $4$.

### Parábola

Puntos que equidistan de un punto (**foco**) y una recta (**directriz**). Con vértice en el origen y eje vertical:

$$x^2 = 4py$$

El foco está en $(0, p)$ y la directriz es $y = -p$. Con vértice en $(h, k)$: $(x - h)^2 = 4p(y - k)$.

Propiedad: los rayos paralelos al eje se reflejan hacia el foco. Por eso las antenas y los faros son parabólicos.

### Elipse

Puntos cuya **suma** de distancias a dos focos es constante ($2a$).

$$\frac{x^2}{a^2} + \frac{y^2}{b^2} = 1, \qquad c^2 = a^2 - b^2$$

- $a$: semieje mayor; $b$: semieje menor; focos en $(\pm c, 0)$.
- **Excentricidad**: $e = c/a$, entre $0$ (circunferencia) y $1$.
- Área: $\pi a b$.

Las órbitas de los planetas son elipses con el Sol en un foco.

### Hipérbola

Puntos cuya **diferencia** de distancias a dos focos es constante ($2a$).

$$\frac{x^2}{a^2} - \frac{y^2}{b^2} = 1, \qquad c^2 = a^2 + b^2$$

Tiene dos ramas y dos **asíntotas**: $y = \pm \dfrac{b}{a} x$. Su excentricidad es mayor que 1.

### Cómo reconocerlas

En $Ax^2 + Cy^2 + Dx + Ey + F = 0$ (sin término $xy$):

| Condición | Cónica |
|---|---|
| $A = C$ | circunferencia |
| $A$ y $C$ del mismo signo, distintos | elipse |
| $A$ y $C$ de signo opuesto | hipérbola |
| $A = 0$ o $C = 0$ | parábola |

Para llevarla a la forma canónica se **completan cuadrados**.

## Ecuaciones paramétricas

Una curva puede darse con un parámetro $t$:

| Curva | Paramétricas |
|---|---|
| Recta | $x = x_0 + at$, $y = y_0 + bt$ |
| Circunferencia | $x = r\cos t$, $y = r\sin t$ |
| Elipse | $x = a\cos t$, $y = b\sin t$ |

Describen además **cómo** se recorre la curva, como la trayectoria de un móvil en el tiempo.

## Coordenadas polares

Un punto se ubica con su distancia al origen $r$ y su ángulo $\theta$:

$$x = r\cos\theta \qquad y = r\sin\theta \qquad r^2 = x^2 + y^2 \qquad \tan\theta = \frac{y}{x}$$

| Curva | Ecuación polar |
|---|---|
| Circunferencia centrada en el origen | $r = a$ |
| Semirrecta desde el origen | $\theta = \alpha$ |
| Espiral de Arquímedes | $r = a\theta$ |
| Cardioide | $r = a(1 + \cos\theta)$ |

> [!WARNING]
> Al calcular $\theta = \arctan(y/x)$ hay que mirar el cuadrante: $(-1, -1)$ y $(1, 1)$ dan el mismo cociente pero sus ángulos son $225°$ y $45°$.

## En el espacio

- **Distancia**: $d = \sqrt{\Delta x^2 + \Delta y^2 + \Delta z^2}$.
- **Plano**: $Ax + By + Cz + D = 0$, con normal $(A, B, C)$.
- **Esfera**: $(x - h)^2 + (y - k)^2 + (z - l)^2 = r^2$.
- **Recta**: $(x, y, z) = (x_0, y_0, z_0) + t(a, b, c)$.

> [!TIP]
> Completar cuadrados es la herramienta central: convierte cualquier ecuación de segundo grado en su forma canónica, de donde se leen centro, radio, ejes y focos.
