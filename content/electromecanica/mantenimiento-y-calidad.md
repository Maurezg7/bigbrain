+++
title = "Mantenimiento y gestión de la calidad"
summary = "Mantenimiento correctivo, preventivo y predictivo, confiabilidad, disponibilidad, OEE, control estadístico de procesos e ISO 9001."
branch = "Gestión, ambiente y seguridad"
order = 4
level = "Intermedio"
tags = ["mantenimiento", "confiabilidad", "disponibilidad", "calidad", "control estadístico"]
+++

## Tipos de mantenimiento

| Tipo | Cuándo se interviene | Ventaja | Desventaja |
|---|---|---|---|
| Correctivo | cuando el equipo falla | no gasta antes de tiempo | paradas imprevistas y daños mayores |
| Preventivo | a intervalos fijos de tiempo o de uso | reduce las fallas | puede cambiar piezas en buen estado |
| Predictivo | cuando una medición indica deterioro | interviene justo a tiempo | requiere instrumentos y conocimiento |
| Proactivo | se elimina la causa de la falla | mejora permanente | exige análisis |

No existe un tipo "mejor": para una lámpara conviene el correctivo; para una turbina, el predictivo.

## Técnicas predictivas

| Técnica | Detecta |
|---|---|
| Análisis de vibraciones | desbalance, desalineación, rodamientos, engranajes |
| Termografía infrarroja | conexiones flojas, sobrecargas, fallas de aislación térmica |
| Análisis de aceite | desgaste, contaminación, degradación del lubricante |
| Ultrasonido | fugas de aire y vapor, descargas eléctricas |
| Análisis de corriente del motor | barras rotas, excentricidades |
| Medición de aislación | deterioro de bobinados y cables |

## Confiabilidad

La **confiabilidad** $R(t)$ es la probabilidad de que un equipo funcione sin fallar durante un tiempo $t$. Con tasa de fallas $\lambda$ constante:

$$R(t) = e^{-\lambda t},\qquad MTBF = \frac{1}{\lambda}$$

$MTBF$ es el tiempo medio entre fallas.

![Curva de la bañera: tasa de fallas alta al inicio, baja y constante en la vida útil y creciente por desgaste](/media/electromecanica/banera.svg)

*Curva de la bañera: fallas infantiles, vida útil y desgaste.*

| Etapa | Tasa de fallas | Causa |
|---|---|---|
| Fallas tempranas | decreciente | defectos de fabricación y de montaje |
| Vida útil | constante | fallas aleatorias |
| Desgaste | creciente | envejecimiento |

> [!NOTE]
> Una bomba con $MTBF$ de 8000 horas: la probabilidad de que funcione 1000 horas sin fallar es $R = e^{-1000/8000}\approx 88\ \%$.

Para varios componentes:

| Configuración | Confiabilidad del conjunto |
|---|---|
| Serie (todos deben funcionar) | $R = R_1\,R_2\cdots R_n$ |
| Paralelo (alcanza con uno) | $R = 1 - (1 - R_1)(1 - R_2)\cdots(1 - R_n)$ |

Dos bombas de 90 % en serie dan 81 %; en paralelo, 99 %. Por eso los servicios críticos llevan equipos de **reserva**.

## Disponibilidad

Es la fracción del tiempo en que el equipo está en condiciones de funcionar:

$$A = \frac{MTBF}{MTBF + MTTR}$$

con $MTTR$ el tiempo medio de reparación. Se mejora fallando menos o reparando más rápido (repuestos a mano, diseño accesible, personal capacitado).

> [!NOTE]
> Con $MTBF = 500$ h y $MTTR = 10$ h: $A = \dfrac{500}{510}\approx 98\ \%$.

## Eficiencia global de los equipos

El **OEE** resume en un número cuánto se aprovecha una máquina:

$$OEE = \text{Disponibilidad}\times\text{Rendimiento}\times\text{Calidad}$$

| Factor | Pérdidas que mide |
|---|---|
| Disponibilidad | averías, cambios de formato |
| Rendimiento | microparadas, velocidad reducida |
| Calidad | rechazos y reprocesos |

Con 90 %, 95 % y 98 % resulta un OEE del 84 %. Un valor del 85 % se considera de clase mundial.

## Estrategias

- **Mantenimiento centrado en la confiabilidad** (RCM): para cada equipo se analizan sus funciones, sus modos de falla y sus consecuencias, y se elige la tarea más adecuada.
- **Mantenimiento productivo total** (TPM): los operadores hacen el cuidado básico de sus máquinas (limpieza, lubricación, inspección).
- **Análisis de modos de falla y efectos** (AMFE): prioriza las fallas multiplicando gravedad, frecuencia y dificultad de detección.
- **Análisis de causa raíz**: los cinco porqués y el diagrama de espina de pescado (Ishikawa).

La gestión se apoya en **órdenes de trabajo**, historial de cada equipo, plan de lubricación y control de repuestos, normalmente en un sistema informático (GMAO).

## Calidad

Calidad es el grado en que un producto cumple los requisitos del cliente. El enfoque evolucionó de **inspeccionar** al final, a **controlar** el proceso, a **asegurar** con un sistema y a la **gestión total**, que involucra a toda la organización.

| Costos de la calidad | Ejemplos |
|---|---|
| De prevención | capacitación, diseño del proceso |
| De evaluación | inspecciones, ensayos, calibraciones |
| De fallas internas | desperdicios, reprocesos |
| De fallas externas | garantías, devoluciones, pérdida de clientes |

Invertir en prevención reduce mucho más los otros tres.

## Control estadístico de procesos

Todo proceso varía. La variación por **causas comunes** es propia del proceso; la de **causas especiales** indica que algo cambió y hay que actuar.

El **gráfico de control** representa una característica a lo largo del tiempo, con límites a tres desviaciones estándar de la media:

$$LCS = \bar x + 3\,\frac{\sigma}{\sqrt n},\qquad LCI = \bar x - 3\,\frac{\sigma}{\sqrt n}$$

Un punto fuera de los límites, o una racha de siete puntos del mismo lado de la media, señala una causa especial.

La **capacidad del proceso** compara su variación natural con la tolerancia especificada:

$$C_p = \frac{LSE - LIE}{6\sigma},\qquad C_{pk} = \min\left(\frac{LSE - \bar x}{3\sigma},\ \frac{\bar x - LIE}{3\sigma}\right)$$

$C_{pk}$ considera además si el proceso está centrado. Se pide al menos 1,33.

> [!NOTE]
> Un eje con tolerancia de 19,95 a 20,05 mm se fabrica con media 20,01 y $\sigma = 0{,}01$ mm.
>
> $C_p = \dfrac{0{,}10}{0{,}06} = 1{,}67$, pero $C_{pk} = \min\left(\dfrac{0{,}04}{0{,}03},\ \dfrac{0{,}06}{0{,}03}\right) = 1{,}33$: el proceso es capaz, aunque está descentrado.

## Herramientas básicas

Hoja de verificación, histograma, diagrama de Pareto (los pocos problemas que causan la mayoría de los defectos), diagrama de causa y efecto, diagrama de dispersión, estratificación y gráfico de control.

## ISO 9001

Establece los requisitos de un sistema de gestión de la calidad, con enfoque en el cliente, en los procesos y en los riesgos. Se apoya en el ciclo de **mejora continua**: planificar, hacer, verificar y actuar. La certificación demuestra que la organización trabaja de forma controlada y consistente, no que su producto sea el mejor.

**Seis Sigma** es una metodología que reduce la variación mediante proyectos guiados por datos, en cinco fases: definir, medir, analizar, mejorar y controlar.
