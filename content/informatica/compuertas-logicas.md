+++
title = "Compuertas lógicas y álgebra de Boole"
summary = "AND, OR, NOT y compañía: los circuitos elementales con los que se construye un procesador."
branch = "Sistemas numéricos y lógica digital"
order = 2
level = "Básico"
tags = ["compuertas", "Boole", "tablas de verdad", "sumador"]
+++

## Álgebra de Boole

El **álgebra de Boole** trabaja con variables que solo valen 0 (falso) o 1 (verdadero) y tres operaciones básicas:

| Operación | Notación | Vale 1 cuando… |
|---|---|---|
| AND (y) | $A \cdot B$ | ambas entradas valen 1 |
| OR (o) | $A + B$ | al menos una entrada vale 1 |
| NOT (no) | $\overline{A}$ | la entrada vale 0 |

Una **compuerta lógica** es el circuito electrónico, hecho con transistores, que realiza una de estas operaciones.

![Símbolos de las compuertas AND, OR, NOT, NAND, NOR y XOR](/media/informatica/compuertas.svg)

*Símbolos normalizados de las compuertas. El círculo en la salida indica negación.*

## Tablas de verdad

| $A$ | $B$ | AND | OR | NAND | NOR | XOR |
|---|---|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 1 | 1 | 0 |
| 0 | 1 | 0 | 1 | 1 | 0 | 1 |
| 1 | 0 | 0 | 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 | 0 | 0 | 0 |

La **XOR** (o exclusiva) vale 1 cuando las entradas son distintas. **NAND** y **NOR** son compuertas **universales**: cualquier circuito lógico puede construirse usando solo una de ellas.

## Leyes del álgebra de Boole

| Ley | Forma AND | Forma OR |
|---|---|---|
| Identidad | $A \cdot 1 = A$ | $A + 0 = A$ |
| Elemento nulo | $A \cdot 0 = 0$ | $A + 1 = 1$ |
| Idempotencia | $A \cdot A = A$ | $A + A = A$ |
| Complemento | $A \cdot \overline{A} = 0$ | $A + \overline{A} = 1$ |
| Distributiva | $A(B + C) = AB + AC$ | $A + BC = (A + B)(A + C)$ |
| Absorción | $A(A + B) = A$ | $A + AB = A$ |
| De Morgan | $\overline{A \cdot B} = \overline{A} + \overline{B}$ | $\overline{A + B} = \overline{A} \cdot \overline{B}$ |

Sirven para **simplificar** funciones: menos compuertas significan un circuito más barato, más rápido y de menor consumo.

> [!NOTE]
> Simplificar $F = AB + A\overline{B} + \overline{A}B$.
>
> $AB + A\overline{B} = A(B + \overline{B}) = A$, así que $F = A + \overline{A}B$. Por la ley distributiva, $A + \overline{A}B = (A + \overline{A})(A + B) = A + B$.
>
> Tres compuertas AND, dos NOT y una OR se redujeron a una sola OR.

## De la lógica a la aritmética: el sumador

Sumar dos bits $A$ y $B$ produce una suma $S$ y un acarreo $C$:

| $A$ | $B$ | $C$ (acarreo) | $S$ (suma) |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 1 | 0 | 1 |
| 1 | 0 | 0 | 1 |
| 1 | 1 | 1 | 0 |

Comparando con las tablas de arriba: $S = A \oplus B$ (una XOR) y $C = A \cdot B$ (una AND). Este circuito es el **semisumador**. Agregándole una entrada de acarreo se obtiene el **sumador completo**, y encadenando 64 de ellos, el sumador de un procesador de 64 bits.

## Circuitos combinacionales y secuenciales

- En un circuito **combinacional**, la salida depende solo de las entradas actuales: sumadores, multiplexores, decodificadores.
- En un circuito **secuencial**, depende también del estado anterior, porque tiene **memoria**. Su elemento básico es el **flip-flop**, que almacena un bit y cambia al ritmo de una señal de **reloj**. Con ellos se construyen registros, contadores y la memoria RAM.

> [!TIP]
> Todo lo que hace una computadora, desde sumar hasta reproducir un video, se reduce a miles de millones de estas compuertas conmutando miles de millones de veces por segundo.
