+++
title = "Ciclos termodinámicos"
summary = "Carnot, Rankine, Otto, Diesel, Brayton y el ciclo de refrigeración: cómo se convierten calor y trabajo."
branch = "Termodinámica y transmisión de calor"
order = 2
level = "Intermedio"
tags = ["Rankine", "Otto", "Diesel", "Brayton", "rendimiento"]
+++

## Qué es un ciclo

Una máquina térmica hace recorrer a un fluido una serie de procesos que lo devuelven al estado inicial. En cada vuelta recibe calor $Q_c$ de una fuente caliente, entrega trabajo $W$ y cede calor $Q_f$ a una fuente fría:

$$W = Q_c - Q_f,\qquad \eta = \frac{W}{Q_c}$$

En un diagrama $p$-$V$, el área encerrada por el ciclo es el trabajo neto.

## Ciclo de Carnot

Dos isotermas y dos adiabáticas reversibles. Es el ciclo ideal, con el máximo rendimiento posible entre dos temperaturas:

$$\eta = 1 - \frac{T_f}{T_c}$$

No se construye, pero sirve de referencia para todos los demás.

## Ciclo Rankine

![Esquema del ciclo Rankine con caldera, turbina, condensador y bomba](/media/electromecanica/rankine.svg)

*Ciclo Rankine: el agua se evapora en la caldera, se expande en la turbina, se condensa y vuelve bombeada.*

Es el ciclo de las centrales de vapor.

| Etapa | Equipo | Proceso |
|---|---|---|
| 1 → 2 | bomba | compresión del líquido |
| 2 → 3 | caldera | calentamiento, evaporación y sobrecalentamiento a presión constante |
| 3 → 4 | turbina | expansión con entrega de trabajo |
| 4 → 1 | condensador | condensación a presión constante |

$$\eta = \frac{(h_3 - h_4) - (h_2 - h_1)}{h_3 - h_2}$$

El trabajo de la bomba es muy pequeño frente al de la turbina, porque comprimir un líquido cuesta poco. El rendimiento mejora con:

- mayor presión y temperatura del vapor;
- menor presión en el condensador (vacío);
- **recalentamiento** intermedio del vapor;
- **regeneración**: precalentar el agua de alimentación con vapor extraído de la turbina.

Las centrales modernas alcanzan del 38 al 45 %.

## Ciclo Otto

Modelo ideal del motor de encendido por chispa: compresión adiabática, aporte de calor a **volumen constante**, expansión adiabática y cesión de calor a volumen constante. Depende solo de la **relación de compresión** $r$:

$$\eta = 1 - \frac{1}{r^{\,k-1}}$$

> [!NOTE]
> Con $r = 10$ y $k = 1{,}4$: $\eta = 1 - \dfrac{1}{10^{0{,}4}}\approx 60\ \%$. Un motor real ronda el 30 a 38 %, por las pérdidas de calor, de rozamiento y de bombeo.

La relación de compresión está limitada por la **detonación** de la mezcla.

## Ciclo Diesel

El aire se comprime solo, hasta una temperatura que enciende el combustible al inyectarlo. El calor se aporta a **presión constante**. Con $r_c$ la relación de corte (cuánto crece el volumen durante la combustión):

$$\eta = 1 - \frac{1}{r^{\,k-1}}\cdot\frac{r_c^{\,k} - 1}{k\,(r_c - 1)}$$

A igual relación de compresión rinde menos que el Otto, pero como comprime solo aire puede usar relaciones de 15 a 22, y en la práctica rinde más: del 40 al 50 % en motores grandes.

## Ciclo Brayton

Es el de las turbinas de gas: compresión adiabática, combustión a presión constante y expansión en la turbina. Depende de la relación de presiones $r_p$:

$$\eta = 1 - \frac{1}{r_p^{\,(k-1)/k}}$$

El compresor consume entre la mitad y dos tercios del trabajo de la turbina. Los gases de escape salen a más de 500 °C y se aprovechan en una caldera de recuperación para alimentar un ciclo Rankine: es el **ciclo combinado**, que supera el 60 %.

## Ciclo de refrigeración

Recorre el ciclo en sentido inverso: consume trabajo para llevar calor de un lugar frío a uno caliente.

| Etapa | Equipo | Proceso |
|---|---|---|
| 1 → 2 | compresor | compresión del vapor |
| 2 → 3 | condensador | cede calor al ambiente y se condensa |
| 3 → 4 | válvula de expansión | cae la presión a entalpía constante |
| 4 → 1 | evaporador | absorbe calor del recinto y se evapora |

Su eficacia se mide con el **coeficiente de desempeño**:

$$COP_{frío} = \frac{Q_f}{W} = \frac{h_1 - h_4}{h_2 - h_1},\qquad COP_{Carnot} = \frac{T_f}{T_c - T_f}$$

Usada para calefaccionar se llama **bomba de calor**, con $COP_{calor} = COP_{frío} + 1$: entrega varias veces más calor que la energía eléctrica que consume.

## Comparación

| Ciclo | Fluido | Aplicación | Rendimiento real |
|---|---|---|---|
| Rankine | agua y vapor | centrales térmicas y nucleares | 33 a 45 % |
| Otto | aire y nafta o gas | automóviles | 30 a 38 % |
| Diesel | aire y gasoil | camiones, barcos, grupos electrógenos | 40 a 50 % |
| Brayton | aire y gases | aviación, centrales de punta | 30 a 40 % |
| Combinado | gases y vapor | centrales de base | 55 a 63 % |

> [!TIP]
> **Cogeneración** es aprovechar el calor residual de un ciclo para un proceso o para calefacción. El aprovechamiento total del combustible puede superar el 80 %.
