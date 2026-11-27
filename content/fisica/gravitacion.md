+++
title = "Gravitación"
summary = "Ley de gravitación universal, campo y energía potencial gravitatoria, órbitas, velocidad de escape y leyes de Kepler."
branch = "Mecánica"
order = 7
level = "Intermedio"
tags = ["gravitación", "Kepler", "órbitas", "velocidad de escape"]
+++

## Ley de gravitación universal

Newton unificó la caída de una manzana con el movimiento de la Luna: es la misma fuerza.

> [!IMPORTANT]
> $$F = G\,\frac{m_1m_2}{r^2}, \qquad G = 6{,}674 \times 10^{-11}\ \text{N·m}^2/\text{kg}^2$$

- Siempre es **atractiva** y actúa sobre la línea que une los centros.
- Disminuye con el **cuadrado** de la distancia: al doble de distancia, un cuarto de fuerza.
- Una esfera homogénea atrae como si toda su masa estuviera en el centro.

$G$ es tan pequeña que la gravedad solo se nota cuando al menos una masa es enorme.

## Campo gravitatorio

La aceleración de la gravedad a distancia $r$ del centro de un cuerpo de masa $M$:

$$g = \frac{GM}{r^2}$$

> [!NOTE]
> En la superficie terrestre ($M = 5{,}97 \times 10^{24}$ kg, $R = 6{,}37 \times 10^6$ m):
>
> $g = \dfrac{6{,}674 \times 10^{-11} \cdot 5{,}97 \times 10^{24}}{(6{,}37 \times 10^6)^2} = 9{,}82\ \text{m/s}^2$.

A una altura $h$: $g(h) = g_0\left(\dfrac{R}{R + h}\right)^2$. A $400$ km (la estación espacial) todavía vale cerca del 89 % del valor en superficie.

| Cuerpo | $g$ en superficie (m/s²) |
|---|---|
| Luna | 1,62 |
| Marte | 3,71 |
| Tierra | 9,81 |
| Júpiter | 24,8 |

> [!WARNING]
> **Masa** y **peso** no son lo mismo. La masa (kg) es propia del cuerpo; el peso (N) es la fuerza gravitatoria y depende de dónde esté. Un astronauta de 80 kg pesa 785 N en la Tierra y 130 N en la Luna.

## Energía potencial gravitatoria

La fórmula $mgh$ solo vale cerca de la superficie. En general, tomando cero en el infinito:

> [!IMPORTANT]
> $$U = -\frac{GMm}{r}$$

Es negativa porque el cuerpo está **ligado**: hay que entregarle energía para alejarlo.

### Velocidad de escape

Es la mínima velocidad para alejarse indefinidamente. Se obtiene con energía total cero:

$$\tfrac{1}{2}mv_e^2 - \frac{GMm}{R} = 0 \quad\Longrightarrow\quad v_e = \sqrt{\frac{2GM}{R}}$$

Para la Tierra, $v_e \approx 11{,}2$ km/s. No depende de la masa del objeto.

Si un cuerpo es tan compacto que $v_e$ supera la velocidad de la luz, es un **agujero negro**.

## Órbitas circulares

La gravedad provee la fuerza centrípeta:

$$\frac{GMm}{r^2} = \frac{mv^2}{r} \quad\Longrightarrow\quad v = \sqrt{\frac{GM}{r}}$$

- Cuanto más alta la órbita, **más lento** va el satélite.
- La velocidad no depende de la masa del satélite.

**Período**:

$$T = \frac{2\pi r}{v} = 2\pi\sqrt{\frac{r^3}{GM}}$$

**Energía total** en órbita: $E = -\dfrac{GMm}{2r}$.

> [!NOTE]
> Estación espacial a $400$ km: $r = 6{,}77 \times 10^6$ m.
>
> $v = \sqrt{\dfrac{3{,}986 \times 10^{14}}{6{,}77 \times 10^6}} = 7{,}67$ km/s. $T = \dfrac{2\pi r}{v} \approx 92$ minutos.

| Órbita | Altura | Período | Uso |
|---|---|---|---|
| Baja (LEO) | 200–2000 km | ~90 min | observación, estación espacial |
| Media (MEO) | ~20 000 km | ~12 h | GPS |
| Geoestacionaria | 35 786 km | 24 h | comunicaciones, meteorología |

### Ingravidez

Los astronautas en órbita **no** están fuera de la gravedad: están en **caída libre** permanente junto con su nave. No sienten peso porque nada los sostiene.

## Leyes de Kepler

Kepler las dedujo de observaciones; Newton las explicó con su ley.

> [!IMPORTANT]
> 1. **Órbitas**: los planetas describen elipses con el Sol en un foco.
> 2. **Áreas**: el radio que une el planeta con el Sol barre áreas iguales en tiempos iguales.
> 3. **Períodos**: $\dfrac{T^2}{a^3} = \dfrac{4\pi^2}{GM}$, constante para todos los cuerpos que orbitan al mismo astro.

- La segunda es la conservación del momento angular: el planeta va más rápido en el **perihelio** (punto más cercano) que en el **afelio**.
- En la tercera, $a$ es el semieje mayor de la elipse.

> [!NOTE]
> Marte orbita a $1{,}52$ unidades astronómicas del Sol. Con $T$ en años y $a$ en UA, $T^2 = a^3$:
>
> $T = \sqrt{1{,}52^3} = 1{,}87$ años.

La tercera ley permite **pesar** astros: conociendo el período y el radio de la órbita de un satélite se obtiene la masa del cuerpo central.

## Tipos de trayectoria

Según la energía total $E$:

| Energía | Trayectoria |
|---|---|
| $E < 0$ | elipse o circunferencia (órbita ligada) |
| $E = 0$ | parábola (escape justo) |
| $E > 0$ | hipérbola (pasa una sola vez) |

## Mareas

La Luna atrae más al lado cercano de la Tierra que al lejano. Esa **diferencia** estira los océanos y produce dos pleamares por día. El Sol contribuye con cerca de la mitad del efecto lunar.

## Más allá de Newton

La **relatividad general** de Einstein describe la gravedad como curvatura del espacio-tiempo. Explica la precesión de la órbita de Mercurio, la desviación de la luz y las ondas gravitacionales. El GPS debe corregir sus relojes por efectos relativistas para no acumular kilómetros de error por día.

> [!TIP]
> Dos ideas resuelven casi todo: en una órbita circular la gravedad es la fuerza centrípeta, y entre dos puntos cualesquiera se conserva la energía con $U = -GMm/r$.
