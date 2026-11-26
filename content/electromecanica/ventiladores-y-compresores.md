+++
title = "Ventiladores y compresores"
summary = "Ventiladores centrífugos y axiales, compresores alternativos y rotativos, trabajo de compresión, etapas y redes de aire comprimido."
branch = "Máquinas térmicas e hidráulicas"
order = 3
level = "Intermedio"
tags = ["compresores", "ventiladores", "aire comprimido", "compresión", "tornillo"]
+++

## Mover y comprimir gases

Según cuánto elevan la presión:

| Máquina | Relación de presiones | Uso |
|---|---|---|
| Ventilador | hasta 1,1 | ventilación, tiro de calderas |
| Soplador | 1,1 a 3 | transporte neumático, aireación |
| Compresor | más de 3 | aire comprimido, gases de proceso, refrigeración |

En un ventilador el gas casi no cambia de densidad y se lo trata como un fluido incompresible, igual que en una bomba.

## Ventiladores

| Tipo | Características |
|---|---|
| Axial (de hélice) | grandes caudales, poca presión |
| Centrífugo de palas hacia adelante | compacto y silencioso; puede sobrecargar al motor |
| Centrífugo de palas hacia atrás | el de mejor rendimiento; no sobrecarga |
| Centrífugo de palas radiales | robusto; gases con polvo |

La potencia es

$$P = \frac{Q\,\Delta p}{\eta}$$

y valen las mismas leyes de semejanza que en las bombas: caudal proporcional a $n$, presión a $n^2$ y potencia a $n^3$.

> [!NOTE]
> Ventilador que mueve $5\ \text{m}^3$/s contra 800 Pa con rendimiento 0,7: $P = \dfrac{5\cdot 800}{0{,}7}\approx 5{,}7$ kW.

## Tipos de compresores

| Tipo | Principio | Uso |
|---|---|---|
| Alternativo (de pistón) | desplazamiento positivo | altas presiones, caudales chicos y medios |
| De tornillo | dos rotores helicoidales | el estándar del aire comprimido industrial |
| De paletas, scroll | desplazamiento positivo rotativo | caudales pequeños, refrigeración |
| Centrífugo | dinámico | grandes caudales |
| Axial | dinámico | caudales muy grandes; turbinas de gas |

## Trabajo de compresión

Comprimir un gas lo calienta. El trabajo por unidad de masa depende de cómo sea el proceso entre $p_1$ y $p_2$:

| Proceso | Trabajo |
|---|---|
| Isotérmico (ideal, con enfriamiento perfecto) | $R\,T_1\ln\dfrac{p_2}{p_1}$ |
| Adiabático reversible | $\dfrac{k}{k-1}\,R\,T_1\left[\left(\dfrac{p_2}{p_1}\right)^{(k-1)/k} - 1\right]$ |

El isotérmico es el que menos trabajo requiere: conviene enfriar el gas mientras se lo comprime.

> [!NOTE]
> Aire de 1 a 8 bar desde 293 K.
>
> Isotérmico: $287\cdot 293\cdot\ln 8\approx 175$ kJ/kg.
>
> Adiabático: $3{,}5\cdot 287\cdot 293\cdot(8^{0{,}286} - 1)\approx 239$ kJ/kg, con una temperatura final de unos 531 K (258 °C).

## Compresión en etapas

Dividir la compresión en varias etapas con **enfriamiento intermedio** acerca el proceso al isotérmico, ahorra trabajo y limita la temperatura de descarga. El trabajo es mínimo cuando todas las etapas tienen la misma relación de presiones:

$$r_{etapa} = \left(\frac{p_{final}}{p_{inicial}}\right)^{1/N}$$

En el ejemplo anterior, con dos etapas ($r = 2{,}83$) y enfriando hasta 293 K entre ellas, el trabajo baja a unos 205 kJ/kg, un 14 % menos.

## Compresor alternativo

Al final de la carrera queda gas en el **espacio nocivo**, que se reexpande y retrasa la admisión. El **rendimiento volumétrico** mide cuánto del cilindro se aprovecha:

$$\eta_v = 1 - c\left[\left(\frac{p_2}{p_1}\right)^{1/n} - 1\right]$$

con $c$ la fracción de espacio nocivo. Empeora al aumentar la relación de presiones, otro motivo para comprimir en etapas.

## Compresores dinámicos

En los centrífugos y axiales, si el caudal baja demasiado el flujo se invierte de forma periódica: es el **bombeo** (*surge*), que produce vibraciones violentas. Un sistema antibombeo recircula o ventea gas para mantener el caudal por encima del límite.

## Instalaciones de aire comprimido

El aire comprimido es una forma **cara** de energía: solo del 10 al 15 % de la electricidad consumida llega como trabajo útil; el resto es calor.

| Componente | Función |
|---|---|
| Filtro de aspiración | protege al compresor |
| Posenfriador y separador | enfría y condensa la humedad |
| Tanque pulmón | amortigua las pulsaciones y almacena |
| Secador (frigorífico o de adsorción) | baja el punto de rocío |
| Filtros | retienen partículas y aceite |
| Red de distribución | en anillo y con pendiente para drenar |
| Unidad de mantenimiento | filtro, regulador y lubricador en cada punto de uso |

> [!TIP]
> En una red sin mantenimiento, las **fugas** se llevan del 20 al 30 % del aire producido. Un orificio de 3 mm a 7 bar pierde unos 10 L/s y cuesta alrededor de 3 kW de forma permanente. Bajar la presión de trabajo 1 bar ahorra cerca del 7 % de la energía.

El compresor aspira aire a presión atmosférica, de modo que su caudal se indica en condiciones de aspiración (aire libre). A 1200 m de altitud, la menor densidad del aire reduce la masa entregada en alrededor del 13 %.
