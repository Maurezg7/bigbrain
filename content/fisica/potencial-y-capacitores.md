+++
title = "Potencial eléctrico y capacitores"
summary = "Energía potencial y potencial eléctrico, superficies equipotenciales, capacidad, asociación de capacitores, dieléctricos y energía almacenada."
branch = "Electromagnetismo"
order = 2
level = "Intermedio"
tags = ["potencial eléctrico", "capacitores", "dieléctricos", "energía eléctrica"]
+++

## Energía potencial eléctrica

La fuerza eléctrica es conservativa, como la gravitatoria. Para dos cargas puntuales:

$$U = k\,\frac{q_1q_2}{r}, \qquad k = \frac{1}{4\pi\varepsilon_0} = 8{,}99 \times 10^9\ \text{N·m}^2/\text{C}^2$$

Es positiva si las cargas se repelen (mismo signo) y negativa si se atraen.

## Potencial eléctrico

> [!IMPORTANT]
> $$V = \frac{U}{q} \qquad \Delta V = -\int\vec{E} \cdot d\vec{l}$$

El **potencial** es la energía potencial por unidad de carga. Se mide en **volts** (V = J/C). Es un escalar, lo que lo hace más cómodo que el campo.

| Configuración | Potencial |
|---|---|
| Carga puntual | $V = k\dfrac{q}{r}$ |
| Varias cargas | $V = k\displaystyle\sum\dfrac{q_i}{r_i}$ (suma escalar) |
| Campo uniforme | $\Delta V = -E\,d$ |
| Esfera conductora (exterior) | $V = k\dfrac{Q}{r}$ |

Solo importan las **diferencias** de potencial (tensión o voltaje). El trabajo para mover una carga $q$ es $W = q\,\Delta V$.

- Las cargas positivas se mueven espontáneamente hacia potenciales **menores**; las negativas, hacia potenciales mayores.
- El campo apunta en el sentido en que el potencial **disminuye**: $\vec{E} = -\nabla V$. Por eso el campo también se mide en V/m.

> [!NOTE]
> Un electrón se acelera desde el reposo a través de $1000$ V.
>
> $\tfrac{1}{2}mv^2 = e\,\Delta V \Rightarrow v = \sqrt{\dfrac{2 \cdot 1{,}6 \times 10^{-19} \cdot 1000}{9{,}11 \times 10^{-31}}} = 1{,}87 \times 10^7$ m/s.

El **electronvolt** es la energía que gana un electrón al atravesar 1 V: $1\ \text{eV} = 1{,}6 \times 10^{-19}$ J.

### Superficies equipotenciales

Son las superficies donde $V$ es constante. Son siempre **perpendiculares** a las líneas de campo, y mover una carga sobre ellas no requiere trabajo.

### Conductores en equilibrio

- El campo en el interior es **cero**.
- Todo el conductor está al **mismo potencial**.
- La carga se ubica en la superficie y se concentra en las puntas (efecto de puntas: pararrayos).
- Un recinto conductor blinda su interior (**jaula de Faraday**).

## Capacitores

Un **capacitor** son dos conductores separados por un aislante. Almacena carga y energía.

> [!IMPORTANT]
> $$C = \frac{Q}{V}$$

La **capacidad** se mide en **farads** (F = C/V). Los valores usuales van de picofarads a milifarads.

Para placas paralelas de área $A$ separadas una distancia $d$:

$$C = \varepsilon_0\frac{A}{d}, \qquad \varepsilon_0 = 8{,}85 \times 10^{-12}\ \text{F/m}$$

La capacidad depende solo de la geometría y del material, no de la carga ni de la tensión.

### Asociación

| | Serie | Paralelo |
|---|---|---|
| Capacidad equivalente | $\dfrac{1}{C_{eq}} = \dfrac{1}{C_1} + \dfrac{1}{C_2} + \dots$ | $C_{eq} = C_1 + C_2 + \dots$ |
| Igual en todos | la carga | la tensión |

> [!WARNING]
> Es al revés que con las resistencias: en paralelo las capacidades se **suman**.

> [!NOTE]
> $C_1 = 6\ \mu\text{F}$ y $C_2 = 3\ \mu\text{F}$ en serie con $12$ V.
>
> $C_{eq} = \dfrac{6 \cdot 3}{9} = 2\ \mu\text{F}$; $Q = 2 \cdot 12 = 24\ \mu\text{C}$ en cada uno.
>
> $V_1 = \dfrac{24}{6} = 4$ V y $V_2 = \dfrac{24}{3} = 8$ V. El más chico se lleva más tensión.

### Energía almacenada

$$U = \tfrac{1}{2}CV^2 = \frac{Q^2}{2C} = \tfrac{1}{2}QV$$

La energía reside en el campo eléctrico, con densidad $u = \tfrac{1}{2}\varepsilon_0E^2$.

> [!NOTE]
> Un capacitor de $100\ \mu\text{F}$ cargado a $300$ V (el flash de una cámara): $U = \tfrac{1}{2}(10^{-4})(300^2) = 4{,}5$ J, liberados en milisegundos.

### Dieléctricos

Al llenar el espacio entre placas con un aislante, sus moléculas se polarizan y reducen el campo. La capacidad se multiplica por la **constante dieléctrica** $\kappa$:

$$C = \kappa C_0$$

| Material | $\kappa$ | Rigidez dieléctrica (MV/m) |
|---|---|---|
| Vacío | 1 | — |
| Aire | 1,0006 | 3 |
| Papel | 3,7 | 16 |
| Vidrio | 5–10 | 14 |
| Agua | 80 | — |

La **rigidez dieléctrica** es el campo máximo antes de que el aislante se perfore y conduzca. Fija la tensión máxima del capacitor.

## Carga y descarga

A través de una resistencia $R$, un capacitor no se carga al instante:

$$\text{carga: } V_C = V_0\left(1 - e^{-t/RC}\right) \qquad \text{descarga: } V_C = V_0e^{-t/RC}$$

La **constante de tiempo** $\tau = RC$ es el tiempo para alcanzar el 63 % del cambio. Tras $5\tau$ el proceso está prácticamente completo.

## Usos

| Uso | Función |
|---|---|
| Fuentes de alimentación | filtrar y suavizar la tensión |
| Temporizadores | fijar tiempos con $RC$ |
| Desfibriladores, flashes | entregar energía de golpe |
| Corrección del factor de potencia | compensar cargas inductivas |
| Pantallas táctiles | detectar el cambio de capacidad del dedo |

> [!TIP]
> El potencial es a la carga lo que la altura es a la masa: las cargas positivas «caen» hacia potenciales bajos.
