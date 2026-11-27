+++
title = "Turbinas de gas"
summary = "Ciclo Brayton real, compresor, cámara de combustión y turbina, regeneración, ciclo combinado y aplicaciones."
branch = "Máquinas térmicas e hidráulicas"
order = 5
level = "Avanzado"
tags = ["turbina de gas", "Brayton", "ciclo combinado", "compresor", "cogeneración"]
+++

## Cómo funciona

Una turbina de gas tiene tres partes sobre un mismo eje:

| Componente | Función |
|---|---|
| Compresor axial | comprime el aire entre 10 y 30 veces |
| Cámara de combustión | quema combustible a presión constante |
| Turbina | expande los gases calientes y entrega trabajo |

La turbina mueve al compresor y lo que sobra es la potencia útil. En un **turborreactor** de aviación, ese sobrante se usa para acelerar los gases en una tobera y producir empuje.

## Ciclo Brayton ideal

![Diagrama temperatura-entropía del ciclo Brayton con compresión, combustión, expansión y escape](/media/electromecanica/brayton.svg)

*Ciclo Brayton en el diagrama T-s: dos procesos a presión constante y dos adiabáticos.*

Con relación de presiones $r_p$ y $k = 1{,}4$:

$$\eta = 1 - \frac{1}{r_p^{\,(k-1)/k}}$$

El trabajo neto por kilogramo de aire es

$$w = c_p\,(T_3 - T_4) - c_p\,(T_2 - T_1)$$

## Ciclo real

El compresor absorbe entre el 50 y el 65 % del trabajo de la turbina, así que el rendimiento de ambos pesa mucho. Con rendimientos isoentrópicos $\eta_c$ y $\eta_t$:

$$T_2 = T_1 + \frac{T_{2s} - T_1}{\eta_c},\qquad T_4 = T_3 - \eta_t\,(T_3 - T_{4s})$$

> [!NOTE]
> Aire a 288 K, $r_p = 12$, temperatura de entrada a la turbina 1400 K, $\eta_c = 0{,}86$, $\eta_t = 0{,}89$, $c_p = 1{,}005$ kJ/(kg·K).
>
> $12^{0{,}286} = 2{,}034$. $T_{2s} = 586$ K y $T_2 = 288 + \dfrac{298}{0{,}86}\approx 634$ K.
>
> $T_{4s} = \dfrac{1400}{2{,}034} = 688$ K y $T_4 = 1400 - 0{,}89\cdot 712\approx 766$ K.
>
> Compresor: $1{,}005\cdot 346\approx 348$ kJ/kg. Turbina: $1{,}005\cdot 634\approx 637$ kJ/kg. Neto: 289 kJ/kg.
>
> Calor aportado: $1{,}005\cdot(1400 - 634)\approx 770$ kJ/kg. Rendimiento: $\dfrac{289}{770}\approx 37{,}5\ \%$, frente al 50,8 % del ciclo ideal.

Los gases salen a 766 K, casi 500 °C: hay mucha energía por recuperar.

## Temperatura de entrada

El rendimiento y la potencia crecen con la temperatura de entrada a la turbina, limitada por los materiales. Las máquinas actuales superan los 1500 °C gracias a:

- superaleaciones de níquel, con álabes monocristalinos;
- **refrigeración** de los álabes con aire extraído del compresor, que circula por canales internos y forma una película sobre la superficie;
- recubrimientos cerámicos de barrera térmica.

## Mejoras del ciclo

| Mejora | En qué consiste |
|---|---|
| Regeneración | los gases de escape precalientan el aire antes de la cámara; útil con relaciones de presión bajas |
| Enfriamiento intermedio | reduce el trabajo del compresor |
| Recalentamiento | segunda combustión entre etapas de turbina |
| Ciclo combinado | el escape genera vapor para una turbina de vapor |

## Ciclo combinado

Una **caldera de recuperación** aprovecha los gases de escape para producir vapor que mueve una turbina de vapor. Si la turbina de gas rinde $\eta_g$ y el ciclo de vapor $\eta_v$ sobre el calor recuperado:

$$\eta_{cc}\approx\eta_g + (1 - \eta_g)\,\eta_v$$

Con $\eta_g = 0{,}38$ y $\eta_v = 0{,}35$ resulta cerca del 60 %. Es la tecnología térmica más eficiente y la base de la generación con gas natural en Argentina.

## Influencia del ambiente

La potencia depende de la masa de aire que entra. Disminuye con:

- la **temperatura** ambiente: alrededor de 0,7 % por cada grado por encima de 15 °C;
- la **altitud**: cerca de 1 % por cada 100 m.

Por eso algunas instalaciones enfrían el aire de admisión en verano.

## Emisiones

La alta temperatura de llama produce óxidos de nitrógeno. Se reducen con combustores de **premezcla pobre**, inyección de agua o vapor y catalizadores en el escape.

## Características

| Ventajas | Desventajas |
|---|---|
| Mucha potencia en poco peso y volumen | rendimiento bajo en ciclo simple |
| Arranque en minutos | rinde mal con cargas parciales |
| Poca agua de refrigeración | requiere combustibles limpios |
| Pocas vibraciones | costo y complejidad del mantenimiento |

| Aplicación | Tipo |
|---|---|
| Aviación | turborreactor, turbofán, turbohélice |
| Generación de punta | ciclo simple |
| Generación de base | ciclo combinado |
| Gasoductos | accionamiento de compresores |
| Industria | cogeneración de electricidad y calor |
