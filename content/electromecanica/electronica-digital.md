+++
title = "Electrónica digital"
summary = "Compuertas, álgebra de Boole, circuitos combinacionales y secuenciales, conversores y microcontroladores."
branch = "Electrónica"
order = 4
level = "Intermedio"
tags = ["compuertas", "Boole", "flip-flop", "ADC", "microcontrolador"]
+++

## Señales digitales

Una señal digital solo toma dos niveles, 0 y 1. Eso la hace inmune al ruido moderado y fácil de almacenar y procesar. Los fundamentos (sistema binario y compuertas) están en el área de Informática; aquí interesa cómo se diseñan circuitos con ellos.

## Álgebra de Boole

| Ley | Expresión |
|---|---|
| Identidad | $A + 0 = A$, $A\cdot 1 = A$ |
| Complemento | $A + \bar A = 1$, $A\cdot\bar A = 0$ |
| Absorción | $A + AB = A$ |
| De Morgan | $\overline{A + B} = \bar A\,\bar B$, $\overline{AB} = \bar A + \bar B$ |

Por De Morgan, cualquier función puede construirse solo con compuertas NAND o solo con NOR.

## Circuitos combinacionales

La salida depende únicamente de las entradas actuales. El diseño sigue cuatro pasos: tabla de verdad, expresión en suma de productos, simplificación (álgebra o **mapa de Karnaugh**) e implementación.

> [!NOTE]
> Una alarma $F$ debe activarse si hay sobretemperatura ($T$) y además falla la bomba ($\bar B$) o falta caudal ($\bar C$).
>
> $F = T\,\bar B + T\,\bar C = T\,(\bar B + \bar C) = T\,\overline{BC}$: una NAND y una AND.

Bloques combinacionales habituales: sumadores, comparadores, codificadores, decodificadores y multiplexores.

## Circuitos secuenciales

La salida depende también del pasado: tienen **memoria**. El elemento básico es el *flip-flop*, que guarda un bit y cambia en el flanco del reloj.

| Flip-flop | Comportamiento |
|---|---|
| SR | set y reset |
| D | copia la entrada D en cada flanco |
| JK | como el SR, pero con J = K = 1 conmuta |
| T | conmuta en cada flanco si T = 1 |

Con flip-flops se arman **registros** (guardan una palabra), **contadores** y **máquinas de estados**, que son la forma ordenada de diseñar secuencias: se dibuja el diagrama de estados, se asigna un código a cada uno y se obtiene la lógica del próximo estado.

## Familias lógicas

| Familia | Alimentación | Característica |
|---|---|---|
| TTL (74LS) | 5 V | robusta, consumo alto |
| CMOS (74HC, 4000) | 2 a 15 V | consumo casi nulo en reposo |
| Lógica de 3,3 V y menos | 1,2 a 3,3 V | microcontroladores y procesadores actuales |

> [!WARNING]
> Las entradas CMOS sin conectar quedan flotando y toman valores erráticos. Se fijan con resistencias de *pull-up* o *pull-down*.

## Conversión analógica-digital

Un conversor (ADC) de $n$ bits divide su rango en $2^n$ escalones. La **resolución** es

$$\Delta = \frac{V_{ref}}{2^n}$$

Con 10 bits y 5 V de referencia, cada escalón vale 4,9 mV; con 12 bits, 1,2 mV. Para sensores industriales de 4 a 20 mA se pasa la corriente por una resistencia de $250\ \Omega$, que la convierte en 1 a 5 V.

El conversor inverso (DAC) genera una tensión a partir de un número, por ejemplo con una red R-2R o por modulación de ancho de pulso (PWM) filtrada.

## Microcontroladores

Un microcontrolador reúne en un solo chip procesador, memoria y periféricos: entradas y salidas digitales, ADC, temporizadores, PWM y comunicaciones (UART, SPI, I²C, CAN). Es el corazón de variadores, instrumentos y controladores, y la base sobre la que están construidos los PLC.
