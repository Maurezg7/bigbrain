+++
title = "Líneas eléctricas"
summary = "Por qué se transmite en alta tensión, parámetros de una línea, modelos, caída de tensión, cálculo mecánico y efecto corona."
branch = "Instalaciones y sistemas eléctricos"
order = 5
level = "Avanzado"
tags = ["transmisión", "alta tensión", "línea", "flecha", "efecto corona"]
+++

## Por qué alta tensión

Para transportar una potencia $P$, la corriente es $I = P/(\sqrt3\,V\cos\varphi)$ y las pérdidas son $3I^2R$. Entonces:

$$P_{perd}\propto\frac{1}{V^2}$$

Duplicar la tensión reduce las pérdidas a la cuarta parte. Por eso la tensión crece con la distancia y la potencia.

| Nivel | Tensiones usuales en Argentina | Función |
|---|---|---|
| Baja tensión | 380/220 V | distribución a usuarios |
| Media tensión | 13,2 y 33 kV | distribución primaria |
| Alta tensión | 132 y 220 kV | subtransmisión |
| Extra alta tensión | 500 kV | transporte troncal entre regiones |

## Parámetros de una línea

Una línea tiene cuatro parámetros repartidos a lo largo de su longitud:

| Parámetro | Origen |
|---|---|
| Resistencia $R$ | material y sección del conductor, temperatura, efecto pelicular |
| Inductancia $L$ | campo magnético alrededor de los conductores; crece con su separación |
| Capacidad $C$ | campo eléctrico entre conductores y con tierra |
| Conductancia $G$ | fugas por aisladores y efecto corona; suele despreciarse |

En líneas aéreas la reactancia inductiva ronda $0{,}4\ \Omega$/km y en alta tensión domina sobre la resistencia. Los cables subterráneos tienen menos inductancia y mucha más capacidad.

## Modelos

| Línea | Longitud a 50 Hz | Modelo |
|---|---|---|
| Corta | hasta 80 km | impedancia serie $Z = R + jX$ |
| Media | 80 a 250 km | circuito en π con la capacidad repartida en los extremos |
| Larga | más de 250 km | parámetros distribuidos |

Para la línea corta, la caída de tensión por fase es aproximadamente

$$\Delta V\approx I\,(R\cos\varphi + X\sin\varphi)$$

y la **regulación** es la variación de tensión en la carga entre vacío y plena carga, en porcentaje.

> [!NOTE]
> Línea de 13,2 kV y 10 km con $r = 0{,}3$ y $x = 0{,}4\ \Omega$/km, que entrega 3 MW con $\cos\varphi = 0{,}9$.
>
> $I = \dfrac{3\cdot10^6}{\sqrt3\cdot 13\,200\cdot 0{,}9}\approx 146$ A.
>
> $\Delta V = 146\,(3\cdot 0{,}9 + 4\cdot 0{,}436)\approx 649$ V por fase, un 8,5 % de la tensión de fase (7621 V). Pérdidas: $3\cdot 146^2\cdot 3\approx 192$ kW, el 6,4 %.

## Impedancia característica y potencia natural

Una línea sin pérdidas tiene una impedancia característica $Z_c = \sqrt{L/C}$, de unos $400\ \Omega$ en líneas aéreas. Cuando transmite su **potencia natural**

$$P_n = \frac{V_L^2}{Z_c}$$

la reactiva que genera su capacidad iguala a la que consume su inductancia. Con menos carga la línea genera reactiva y la tensión en el extremo receptor **sube** (efecto Ferranti); se compensa con reactores en derivación.

## Capacidad de transporte

La potencia que puede transmitir una línea entre dos barras con tensiones $V_1$ y $V_2$ desfasadas un ángulo $\delta$ es

$$P = \frac{V_1 V_2}{X}\,\sin\delta$$

Las líneas cortas están limitadas por el calentamiento, las medias por la caída de tensión y las largas por la estabilidad.

## Conductores, aisladores y efecto corona

Los conductores aéreos suelen ser de aluminio con alma de acero (ACSR). Se sostienen con cadenas de aisladores de vidrio, porcelana o poliméricos, cuyo largo crece con la tensión.

Cuando el campo eléctrico en la superficie del conductor supera la rigidez del aire (unos 30 kV/cm) el aire se ioniza: es el **efecto corona**, que produce pérdidas, zumbido e interferencias. Se reduce aumentando el diámetro o usando haces de varios conductores por fase.

## Cálculo mecánico

Un conductor tendido entre dos apoyos toma la forma de una catenaria, que se aproxima por una parábola. Con vano $a$, peso por metro $w$ y tiro horizontal $T$, la **flecha** es

$$f = \frac{w\,a^2}{8\,T}$$

Tender más tenso reduce la flecha pero carga más a conductores y estructuras. Se verifican las condiciones extremas: máxima temperatura (flecha máxima y distancia al suelo), mínima temperatura y viento máximo.
