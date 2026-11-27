+++
title = "Instrumentos de medición y errores"
summary = "Error absoluto y relativo, clase de exactitud, incertidumbre, y cómo se miden tensión, corriente y resistencia."
branch = "Mediciones eléctricas"
order = 1
level = "Básico"
tags = ["error", "incertidumbre", "multímetro", "clase", "osciloscopio"]
+++

## Medir es comparar con incertidumbre

Ninguna medición es exacta. Un resultado completo indica el valor y cuánto se puede confiar en él: $R = (100{,}0 \pm 0{,}5)\ \Omega$.

- **Error absoluto**: $\Delta x = x_{medido} - x_{verdadero}$.
- **Error relativo**: $e = \dfrac{\Delta x}{x_{verdadero}}$, normalmente en porcentaje.

| Tipo de error | Origen | Cómo se reduce |
|---|---|---|
| Sistemático | calibración, método, efecto de carga | corrigiendo o calibrando |
| Aleatorio | ruido, lectura, variaciones del entorno | repitiendo y promediando |
| Grosero | equivocación del operador | repitiendo la medición |

**Exactitud** es cercanía al valor verdadero; **precisión** es repetibilidad. Un instrumento puede ser muy preciso y poco exacto si está descalibrado.

## Clase de un instrumento

En los analógicos, la **clase** es el error máximo expresado como porcentaje del fondo de escala:

$$\Delta x_{max} = \frac{\text{clase}}{100}\cdot \text{alcance}$$

> [!NOTE]
> Un voltímetro clase 1,5 con alcance de 300 V tiene un error máximo de $\pm 4{,}5$ V en toda la escala.
>
> Midiendo 220 V el error relativo es 2 %, pero midiendo 30 V sube al 15 %. Conviene elegir el alcance para leer en el tercio superior de la escala.

En los digitales el fabricante da $\pm(\%\ \text{de la lectura} + \text{dígitos})$. Un multímetro de $\pm(0{,}5\ \% + 2\ \text{dígitos})$ que muestra 12,00 V garantiza $\pm(0{,}06 + 0{,}02) = \pm 0{,}08$ V.

## Propagación de errores

Cuando una magnitud se calcula a partir de otras medidas, en el peor caso:

- en sumas y restas se suman los errores **absolutos**;
- en productos y cocientes se suman los errores **relativos**.

Para $P = V I$, si $V$ tiene 1 % e $I$ tiene 2 %, $P$ tiene hasta 3 %. Para errores independientes es más realista sumar en cuadratura: $\sqrt{1^2 + 2^2} \approx 2{,}2\ \%$.

## Voltímetro y amperímetro

![Conexión de un amperímetro en serie y un voltímetro en paralelo con la carga](/media/electromecanica/voltimetro-amperimetro.svg)

*El amperímetro va en serie y el voltímetro en paralelo con el elemento a medir.*

| Instrumento | Conexión | Resistencia interna ideal |
|---|---|---|
| Voltímetro | en paralelo | infinita |
| Amperímetro | en serie | cero |

Como no son ideales, el instrumento altera el circuito: es el **efecto de carga**. Un voltímetro de $1\ \text{M}\Omega$ sobre una resistencia de $1\ \text{M}\Omega$ la convierte en $500\ \text{k}\Omega$ y la lectura sale mucho menor que la real.

> [!WARNING]
> Conectar un amperímetro en paralelo con una fuente es un cortocircuito. Verificá siempre la función y los bornes antes de tocar el circuito, y usá instrumentos con la categoría de seguridad (CAT III o IV) adecuada a la instalación.

Para corrientes y tensiones grandes se usan **transformadores de medida** (de corriente, con secundario de 5 A, y de tensión, con 110 V) y la **pinza amperométrica**, que mide sin abrir el circuito.

## Valor eficaz verdadero

Los instrumentos económicos miden el valor medio rectificado y lo multiplican por 1,11, lo que solo es correcto para senoides puras. Con ondas deformadas por armónicos hay que usar un instrumento de **verdadero valor eficaz** (True RMS).

## Medición de resistencias

| Rango | Método |
|---|---|
| Muy bajas (contactos, bobinados) | cuatro hilos (Kelvin), puente doble |
| Medias | multímetro, puente de Wheatstone |
| Muy altas (aislación) | megóhmetro, con 500 a 5000 V |
| Puesta a tierra | telurímetro |

El **puente de Wheatstone** está en equilibrio cuando $R_1 R_4 = R_2 R_3$; es también el circuito con el que se leen galgas extensométricas y sensores resistivos.

## Osciloscopio

Muestra la tensión en función del tiempo y permite medir amplitud, período, frecuencia, desfase y tiempos de subida. Hay que respetar su ancho de banda y recordar que la masa de la punta suele estar unida a la tierra de la red.
