+++
title = "Transformadores"
summary = "Cómo cambia la tensión un transformador, por qué permite transportar energía y cuáles son sus pérdidas."
branch = "Máquinas eléctricas"
order = 1
level = "Intermedio"
tags = ["inducción", "relación de transformación", "pérdidas", "rendimiento"]
+++

## Principio de funcionamiento

Un transformador tiene dos bobinados arrollados sobre un mismo núcleo de hierro. La corriente alterna del **primario** crea un flujo magnético variable en el núcleo; ese flujo atraviesa el **secundario** e induce en él una tensión, según la **ley de Faraday**:

$$e = -N\,\frac{d\Phi}{dt}$$

Como hace falta un flujo que varíe en el tiempo, un transformador **no funciona con corriente continua**.

![Transformador con núcleo de hierro, bobinado primario y secundario](/media/electromecanica/transformador.svg)

*Los dos bobinados comparten el mismo flujo $\Phi$ a través del núcleo.*

## Transformador ideal

Como ambos bobinados abrazan el mismo flujo, las tensiones son proporcionales al número de espiras:

$$\frac{V_1}{V_2} = \frac{N_1}{N_2} = a$$

donde $a$ es la **relación de transformación**. Sin pérdidas, la potencia que entra es la que sale, $V_1 I_1 = V_2 I_2$, así que las corrientes guardan la relación inversa:

$$\frac{I_1}{I_2} = \frac{N_2}{N_1} = \frac{1}{a}$$

Si $a > 1$ el transformador es **reductor**; si $a < 1$, **elevador**.

> [!NOTE]
> Un transformador de 220 V / 12 V tiene 1100 espiras en el primario. ¿Cuántas tiene el secundario y qué corriente toma de la red al alimentar una carga de 5 A?
>
> $a = \dfrac{220}{12} \approx 18{,}3$, entonces $N_2 = \dfrac{1100}{18{,}3} = 60$ espiras e $I_1 = \dfrac{5}{18{,}3} \approx 0{,}27$ A.

## Por qué son indispensables

Las pérdidas en una línea son $P = I^2 R$. Para transmitir la misma potencia $P = VI$ con menos corriente, se **eleva la tensión**: multiplicarla por 10 reduce las pérdidas 100 veces. Por eso se genera en media tensión, se transmite a cientos de kilovolts y se reduce en etapas hasta los 220/380 V de uso domiciliario.

## Transformador real: pérdidas

| Pérdida | Origen | Depende de | Cómo se reduce |
|---|---|---|---|
| En el cobre | resistencia de los bobinados ($I^2R$) | la carga | conductores de mayor sección |
| Por histéresis | magnetización cíclica del núcleo | tensión y frecuencia | acero al silicio |
| Por corrientes parásitas | corrientes inducidas en el núcleo | tensión y frecuencia | núcleo de chapas laminadas y aisladas |

Las pérdidas en el hierro se miden con el **ensayo de vacío** (secundario abierto, tensión nominal) y las del cobre con el **ensayo de cortocircuito** (secundario en corto, corriente nominal).

## Rendimiento

$$\eta = \frac{P_{\text{salida}}}{P_{\text{salida}} + P_{Fe} + P_{Cu}}$$

Los transformadores de potencia superan el 98 %. El rendimiento es máximo cuando las pérdidas en el cobre igualan a las del hierro.

> [!WARNING]
> La potencia nominal de un transformador se expresa en **VA** (o kVA), no en watts: el calentamiento depende de la tensión y la corriente, sin importar el factor de potencia de la carga.
