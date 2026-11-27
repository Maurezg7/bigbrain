+++
title = "Energía eólica y fotovoltaica"
summary = "Potencia del viento, límite de Betz y aerogeneradores; celda solar, curva I-V y dimensionamiento de sistemas fotovoltaicos."
branch = "Instalaciones y sistemas eléctricos"
order = 7
level = "Intermedio"
tags = ["eólica", "fotovoltaica", "Betz", "panel solar", "renovables"]
+++

## Energía eólica

### Potencia del viento

El aire que atraviesa el área $A$ barrida por el rotor con velocidad $v$ lleva una potencia

$$P_{viento} = \tfrac12\,\rho\,A\,v^3$$

con $\rho\approx 1{,}225\ \text{kg/m}^3$ a nivel del mar. Que dependa del **cubo** de la velocidad explica por qué el emplazamiento lo es todo: con el doble de viento hay ocho veces más potencia.

### Límite de Betz

Ninguna turbina puede extraer toda la energía, porque el aire tiene que seguir saliendo. El máximo teórico es

$$C_{p,max} = \frac{16}{27}\approx 59{,}3\ \%$$

Las máquinas reales alcanzan coeficientes de potencia $C_p$ de 0,40 a 0,50:

$$P = \tfrac12\,\rho\,A\,v^3\,C_p$$

> [!NOTE]
> Aerogenerador de 100 m de diámetro con viento de 10 m/s y $C_p = 0{,}45$.
>
> $A = \pi\cdot 50^2\approx 7854\ \text{m}^2$ y $P = 0{,}5\cdot 1{,}225\cdot 7854\cdot 1000\cdot 0{,}45\approx 2{,}16$ MW.

### Curva de potencia

| Velocidad | Comportamiento |
|---|---|
| Menos de 3 a 4 m/s | no arranca |
| Entre arranque y nominal (unos 12 m/s) | la potencia crece con el cubo |
| Entre nominal y 25 m/s | potencia constante, regulada con el ángulo de las palas (*pitch*) |
| Más de 25 m/s | se detiene por seguridad |

El viento de un sitio se describe con la distribución de **Weibull** y su energía anual, con el factor de planta. Los mejores sitios de la Patagonia superan el 50 %.

Los aerogeneradores actuales son de eje horizontal y tres palas, de velocidad variable. Usan un generador de inducción doblemente alimentado o uno síncrono de imanes permanentes con un convertidor electrónico entre la máquina y la red.

## Energía solar fotovoltaica

### La celda

Una celda es una unión PN de gran superficie: la luz libera electrones y la unión los separa, generando corriente continua. Las de silicio cristalino tienen rendimientos del 18 al 23 % y entregan unos 0,6 V cada una; un panel conecta en serie 60 a 144 celdas.

### Curva I-V

![Curva corriente-tensión de un panel solar con el punto de máxima potencia](/media/electromecanica/curva-iv.svg)

*El panel se comporta casi como una fuente de corriente hasta el codo, donde está el punto de máxima potencia.*

| Parámetro | Significado |
|---|---|
| $I_{sc}$ | corriente de cortocircuito; proporcional a la irradiancia |
| $V_{oc}$ | tensión a circuito abierto; baja con la temperatura |
| $P_{mp} = V_{mp} I_{mp}$ | potencia máxima, en vatios pico (Wp) |

La potencia nominal se mide en condiciones estándar: $1000\ \text{W/m}^2$ y 25 °C de celda. En servicio, cada grado por encima de 25 °C resta alrededor de un 0,4 % de potencia. El inversor lleva un **seguidor del punto de máxima potencia** (MPPT) que ajusta la tensión de trabajo continuamente.

### Dimensionamiento

La energía diaria de un generador de potencia pico $P_p$ es

$$E = P_p\cdot HSP\cdot PR$$

donde $HSP$ son las **horas solares pico** del lugar (la irradiación diaria en kWh/m²) y $PR$ el rendimiento global del sistema, entre 0,75 y 0,85.

> [!NOTE]
> En el noroeste argentino hay del orden de 5,5 a 6,5 HSP de promedio anual, de los mejores valores del mundo. Con 6 HSP y $PR = 0{,}8$, un sistema de 5 kWp genera $5\cdot 6\cdot 0{,}8 = 24$ kWh por día, unos 8800 kWh por año.

### Tipos de sistemas

| Sistema | Componentes | Uso |
|---|---|---|
| Conectado a red | paneles, inversor de red, medidor bidireccional | generación distribuida, parques |
| Aislado | paneles, regulador de carga, baterías, inversor | sitios sin red |
| Híbrido | ambos, con respaldo | continuidad de servicio |

> [!WARNING]
> Los paneles generan tensión siempre que reciben luz: no se pueden apagar. Las cadenas en serie alcanzan de 600 a 1500 V de continua, cuyo arco es difícil de extinguir. Se usan seccionadores y fusibles específicos para corriente continua.

## Integración a la red

Ambas fuentes son **variables** y no aportan inercia mecánica al sistema. Su integración requiere pronósticos, reserva de centrales flexibles, redes de transporte suficientes y, cada vez más, almacenamiento en baterías o en centrales de bombeo.
