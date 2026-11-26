+++
title = "Sistemas de numeración: binario y hexadecimal"
summary = "Cómo representa números una computadora, cómo convertir entre bases y qué es el complemento a dos."
branch = "Sistemas numéricos y lógica digital"
order = 1
level = "Básico"
tags = ["binario", "hexadecimal", "bits", "complemento a dos"]
+++

## Sistemas posicionales

En el sistema decimal cada dígito vale según su posición: $472 = 4 \cdot 10^2 + 7 \cdot 10^1 + 2 \cdot 10^0$. Lo mismo vale para cualquier **base** $b$, usando $b$ símbolos distintos.

Las computadoras usan **base 2** porque sus circuitos distinguen fácilmente dos estados: con tensión o sin ella. Cada dígito binario es un **bit**; ocho bits forman un **byte**.

## De binario a decimal

Se suman las potencias de 2 de las posiciones que tienen un 1:

![El número binario 1011 0110 con el peso de cada posición](/media/informatica/binario.svg)

*Cada bit tiene un peso que es una potencia de 2. $10110110_2 = 128 + 32 + 16 + 4 + 2 = 182$.*

## De decimal a binario

Se divide sucesivamente por 2 y se leen los restos **de abajo hacia arriba**.

> [!NOTE]
> Convertir 25 a binario:
>
> | División | Cociente | Resto |
> |---|---|---|
> | 25 ÷ 2 | 12 | 1 |
> | 12 ÷ 2 | 6 | 0 |
> | 6 ÷ 2 | 3 | 0 |
> | 3 ÷ 2 | 1 | 1 |
> | 1 ÷ 2 | 0 | 1 |
>
> Leyendo los restos de abajo hacia arriba: $25 = 11001_2$. Verificación: $16 + 8 + 1 = 25$.

## Hexadecimal

Los números binarios son largos e incómodos, así que se abrevian en **base 16**, con los dígitos 0–9 y A–F (A = 10, …, F = 15). Como $16 = 2^4$, cada dígito hexadecimal equivale exactamente a **4 bits**:

| Hex | Binario | Hex | Binario | Hex | Binario | Hex | Binario |
|---|---|---|---|---|---|---|---|
| 0 | 0000 | 4 | 0100 | 8 | 1000 | C | 1100 |
| 1 | 0001 | 5 | 0101 | 9 | 1001 | D | 1101 |
| 2 | 0010 | 6 | 0110 | A | 1010 | E | 1110 |
| 3 | 0011 | 7 | 0111 | B | 1011 | F | 1111 |

Para convertir, se agrupan los bits de a cuatro desde la derecha: $1011\,0110_2 = \text{B6}_{16}$. Por eso los colores web (`#8B7BFF`), las direcciones de memoria y las direcciones MAC se escriben en hexadecimal.

## Cuánto se puede representar

Con $n$ bits hay $2^n$ combinaciones: un byte representa 256 valores (de 0 a 255) y 32 bits, unos 4300 millones.

| Unidad | Equivale a |
|---|---|
| 1 byte | 8 bits |
| 1 kilobyte (kB) | 1000 bytes |
| 1 megabyte (MB) | $10^6$ bytes |
| 1 gigabyte (GB) | $10^9$ bytes |
| 1 kibibyte (KiB) | 1024 bytes |

## Números negativos: complemento a dos

Para representar enteros con signo se usa el **complemento a dos**. Para obtener $-x$: se invierten todos los bits de $x$ y se suma 1. El bit más significativo indica el signo (1 = negativo), y con $n$ bits el rango es de $-2^{n-1}$ a $2^{n-1} - 1$ (de $-128$ a $127$ con 8 bits).

> [!NOTE]
> Representar $-5$ en 8 bits: $5 = 0000\,0101$. Invirtiendo: $1111\,1010$. Sumando 1: $1111\,1011$.
>
> La ventaja es que la resta se hace con el mismo circuito que la suma: $7 + (-5) = 0000\,0111 + 1111\,1011 = 1\,0000\,0010$, y descartando el acarreo queda $0000\,0010 = 2$.

> [!WARNING]
> Si el resultado no entra en los bits disponibles hay **desbordamiento** (*overflow*): con 8 bits con signo, $127 + 1$ da $-128$. Es una fuente clásica de errores.

## Texto y decimales

Los caracteres se guardan como números según una tabla: **ASCII** (7 bits, 128 caracteres) y **Unicode**, que cubre todos los sistemas de escritura y se codifica habitualmente en **UTF-8**. Los números con decimales usan el formato de **punto flotante** IEEE 754, que tiene precisión limitada: por eso `0.1 + 0.2` no da exactamente `0.3`.
