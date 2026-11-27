+++
title = "Motores de combustión interna"
summary = "Ciclos de cuatro y dos tiempos, motores nafteros y diésel, parámetros de desempeño, sobrealimentación y emisiones."
branch = "Máquinas térmicas e hidráulicas"
order = 6
level = "Intermedio"
tags = ["motores", "Otto", "Diesel", "cilindrada", "turbocompresor"]
+++

## Combustión dentro del cilindro

En un motor de combustión interna, el combustible se quema dentro del cilindro y los gases empujan directamente al pistón. El mecanismo biela-manivela convierte ese vaivén en giro del cigüeñal.

## El ciclo de cuatro tiempos

![Los cuatro tiempos del motor: admisión, compresión, expansión y escape](/media/electromecanica/cuatro-tiempos.svg)

*Un ciclo completo ocupa dos vueltas del cigüeñal y solo una carrera entrega trabajo.*

| Tiempo | Pistón | Válvulas | Qué ocurre |
|---|---|---|---|
| Admisión | baja | admisión abierta | entra aire o mezcla |
| Compresión | sube | cerradas | se comprime la carga |
| Expansión | baja | cerradas | combustión y trabajo |
| Escape | sube | escape abierta | salen los gases |

El **motor de dos tiempos** completa el ciclo en una sola vuelta, usando lumbreras en lugar de válvulas. Es más liviano y potente para su tamaño, pero desperdicia parte de la mezcla. Se usa en motores muy pequeños y, en versión diésel, en los grandes motores marinos.

## Nafta y diésel

| | Encendido por chispa (Otto) | Encendido por compresión (Diesel) |
|---|---|---|
| Qué se comprime | mezcla de aire y combustible | solo aire |
| Encendido | bujía | autoinflamación al inyectar |
| Relación de compresión | 8 a 13 | 15 a 22 |
| Regulación de la carga | mariposa: cantidad de mezcla | cantidad de combustible inyectado |
| Límite | detonación | presiones y humo |
| Rendimiento | 30 a 38 % | 40 a 50 % |
| Propiedad del combustible | número de octano | número de cetano |

El **octanaje** mide la resistencia de la nafta a detonar; el **cetanaje**, la facilidad del gasoil para autoinflamarse.

## Geometría

Con diámetro $D$, carrera $S$ y $z$ cilindros, la **cilindrada** es

$$V_d = z\,\frac{\pi D^2}{4}\,S$$

y la **relación de compresión**, con $V_c$ el volumen de la cámara de combustión,

$$r = \frac{V_d/z + V_c}{V_c}$$

## Parámetros de desempeño

La **presión media efectiva** (pme) es la presión constante que, actuando durante la carrera de expansión, daría el mismo trabajo. Permite comparar motores de distinto tamaño:

$$P = \frac{pme\cdot V_d\cdot n}{60\cdot i}$$

con $n$ en rpm e $i = 2$ para cuatro tiempos o $1$ para dos tiempos. El par resulta proporcional a la pme y a la cilindrada.

> [!NOTE]
> Motor de cuatro tiempos de 2,0 L que entrega 110 kW a 5500 rpm:
>
> $pme = \dfrac{110\,000\cdot 60\cdot 2}{0{,}002\cdot 5500} = 1{,}2\cdot10^6$ Pa, es decir 12 bar.
>
> Par: $T = \dfrac{110\,000}{5500\cdot 2\pi/60}\approx 191$ N·m.

| Parámetro | Definición | Valores típicos |
|---|---|---|
| Rendimiento volumétrico | masa de aire admitida respecto de la que cabría | 0,8 a 0,9 en aspirados |
| Rendimiento mecánico | potencia en el eje respecto de la indicada | 0,8 a 0,9 |
| Consumo específico | combustible por unidad de energía | 250 g/kWh en nafta; 200 g/kWh en diésel |

El rendimiento total y el consumo específico $c_e$ se relacionan por $\eta = \dfrac{1}{c_e\cdot PCI}$.

## Curvas características

A plena carga, el **par** tiene un máximo a velocidad intermedia y la **potencia** sigue creciendo hasta cerca del régimen máximo. El **consumo específico** es mínimo en la zona de par máximo: ahí el motor es más eficiente.

## Mezcla y combustión

La proporción estequiométrica de la nafta es de 14,7 kg de aire por kg de combustible. El factor **lambda** compara la mezcla real con esa:

| $\lambda$ | Mezcla | Efecto |
|---|---|---|
| Menor que 1 | rica | más potencia, más CO e hidrocarburos |
| 1 | estequiométrica | la que necesita el catalizador de tres vías |
| Mayor que 1 | pobre | menor consumo |

La **inyección electrónica** dosifica el combustible según la masa de aire, corrigiendo con la sonda lambda del escape. Los diésel modernos usan inyección directa por conducto común (*common rail*) a más de 2000 bar.

## Sobrealimentación

La potencia depende de la masa de aire que entra al cilindro. Un **turbocompresor** aprovecha la energía de los gases de escape para mover un compresor que aumenta la densidad del aire de admisión. El **intercooler** lo enfría después de comprimirlo para ganar más densidad.

En altura, un motor aspirado pierde potencia en proporción a la densidad del aire: alrededor de 1 % cada 100 m. Los turboalimentados compensan gran parte de esa pérdida.

## Balance térmico

De la energía del combustible, aproximadamente un tercio se convierte en trabajo, un tercio se va por el escape y un tercio por refrigeración y rozamientos. Los grupos electrógenos con **cogeneración** recuperan el calor del escape y del agua de refrigeración.

## Emisiones

| Contaminante | Origen | Control |
|---|---|---|
| CO | combustión incompleta | catalizador |
| Hidrocarburos | combustible sin quemar | catalizador |
| Óxidos de nitrógeno | alta temperatura de combustión | recirculación de gases (EGR), catalizador SCR con urea |
| Material particulado | combustión diésel | filtro de partículas |

## Sistemas auxiliares

- **Lubricación**: bomba de aceite, filtro y cárter; el aceite además refrigera y limpia.
- **Refrigeración**: circuito de agua con bomba, termostato y radiador.
- **Distribución**: el árbol de levas, que gira a la mitad de la velocidad del cigüeñal, abre las válvulas.
- **Arranque y carga**: motor de arranque, alternador y batería.
