+++
title = "Gases"
summary = "Presión, leyes de los gases, ecuación del gas ideal, ley de Dalton de las presiones parciales, estequiometría con gases y gases reales."
branch = "Estados de la materia y soluciones"
order = 1
level = "Básico"
tags = ["gases", "gas ideal", "presiones parciales", "volumen molar"]
+++

## El estado gaseoso

Los gases no tienen forma ni volumen propios, son muy compresibles, tienen baja densidad y se mezclan en cualquier proporción.

El estado de un gas se describe con cuatro variables: **presión** ($P$), **volumen** ($V$), **temperatura** ($T$) y **cantidad de sustancia** ($n$).

### Presión

| Unidad | Equivalencia con 1 atm |
|---|---|
| atmósfera | 1 atm |
| milímetro de mercurio (torr) | 760 mmHg |
| pascal | 101 325 Pa |
| bar | 1,013 bar |

## Leyes de los gases

| Ley | Constantes | Relación | Enunciado |
|---|---|---|---|
| Boyle | $n$, $T$ | $P_1V_1 = P_2V_2$ | a más presión, menos volumen |
| Charles | $n$, $P$ | $\dfrac{V_1}{T_1} = \dfrac{V_2}{T_2}$ | el volumen es proporcional a $T$ |
| Gay-Lussac | $n$, $V$ | $\dfrac{P_1}{T_1} = \dfrac{P_2}{T_2}$ | la presión es proporcional a $T$ |
| Avogadro | $P$, $T$ | $\dfrac{V_1}{n_1} = \dfrac{V_2}{n_2}$ | volúmenes iguales contienen igual número de moléculas |

Combinadas, para una cantidad fija de gas:

$$\frac{P_1V_1}{T_1} = \frac{P_2V_2}{T_2}$$

> [!WARNING]
> La temperatura debe estar siempre en **kelvin**: $T = T_C + 273{,}15$.

> [!NOTE]
> Un globo de $2{,}0$ L a $27$ °C y $1{,}0$ atm sube hasta donde hay $-23$ °C y $0{,}50$ atm.
>
> $V_2 = 2{,}0 \cdot \dfrac{1{,}0}{0{,}50} \cdot \dfrac{250}{300} = 3{,}3$ L.

## Ecuación del gas ideal

> [!IMPORTANT]
> $$PV = nRT$$
> $R = 0{,}08206\ \text{L·atm/(mol·K)} = 8{,}314\ \text{J/(mol·K)}$

**Volumen molar**: en condiciones normales de presión y temperatura (0 °C y 1 atm), un mol de gas ideal ocupa **22,4 L**.

> [!NOTE]
> ¿Qué volumen ocupan $8{,}0$ g de O₂ a $25$ °C y $1{,}5$ atm?
>
> $n = \dfrac{8{,}0}{32{,}0} = 0{,}25$ mol. $V = \dfrac{0{,}25 \cdot 0{,}08206 \cdot 298}{1{,}5} = 4{,}1$ L.

### Masa molar y densidad

Como $n = \dfrac{m}{M}$:

$$M = \frac{mRT}{PV} \qquad d = \frac{PM}{RT}$$

La densidad de un gas es proporcional a su masa molar. Un gas con $M$ menor que la del aire (~29 g/mol), como el helio o el metano, asciende.

> [!NOTE]
> Un gas tiene densidad $1{,}96$ g/L a $0$ °C y $1$ atm. $M = \dfrac{1{,}96 \cdot 0{,}08206 \cdot 273}{1} = 43{,}9$ g/mol. Es CO₂.

## Mezclas de gases

> [!IMPORTANT]
> **Ley de Dalton**: la presión total de una mezcla es la suma de las **presiones parciales** de sus componentes.
> $$P_{total} = P_A + P_B + \dots \qquad P_A = x_A\,P_{total}$$

$x_A = \dfrac{n_A}{n_{total}}$ es la **fracción molar**.

> [!NOTE]
> El aire tiene 78 % de N₂, 21 % de O₂ y 1 % de Ar en moles. A $1$ atm, las presiones parciales son $0{,}78$, $0{,}21$ y $0{,}01$ atm.

Cuando un gas se **recoge sobre agua**, viene mezclado con vapor: $P_{gas} = P_{total} - P_{vapor\ de\ agua}$.

## Estequiometría con gases

Los coeficientes de una ecuación dan la relación en moles y, a igual presión y temperatura, también en **volúmenes** de gases.

> [!NOTE]
> ¿Qué volumen de CO₂ a $25$ °C y $1$ atm produce la descomposición de $50{,}0$ g de CaCO₃?
>
> CaCO₃ → CaO + CO₂
>
> $n = \dfrac{50{,}0}{100{,}1} = 0{,}500$ mol de CO₂. $V = \dfrac{0{,}500 \cdot 0{,}08206 \cdot 298}{1} = 12{,}2$ L.

## Teoría cinética

Explica el comportamiento de los gases a nivel molecular:

1. Las moléculas son puntuales: su volumen es despreciable.
2. Se mueven al azar y en línea recta.
3. No se atraen ni se repelen.
4. Los choques son elásticos.
5. La energía cinética media es proporcional a la temperatura absoluta.

La presión resulta de los choques contra las paredes.

$$v_{rms} = \sqrt{\frac{3RT}{M}}$$

Las moléculas livianas son más rápidas.

**Ley de Graham**: la velocidad de **efusión** (escape por un orificio pequeño) es inversamente proporcional a la raíz de la masa molar:

$$\frac{v_1}{v_2} = \sqrt{\frac{M_2}{M_1}}$$

> [!NOTE]
> El hidrógeno ($M = 2$) efunde $\sqrt{\dfrac{32}{2}} = 4$ veces más rápido que el oxígeno ($M = 32$).

## Gases reales

Los gases se apartan del comportamiento ideal a **presión alta** y **temperatura baja**, cuando las moléculas están cerca: su volumen ya no es despreciable y se atraen.

**Ecuación de van der Waals**:

$$\left(P + \frac{an^2}{V^2}\right)(V - nb) = nRT$$

- $a$ corrige las **atracciones** entre moléculas.
- $b$ corrige el **volumen** propio de las moléculas.

| Gas | $a$ (L²·atm/mol²) | $b$ (L/mol) |
|---|---|---|
| He | 0,034 | 0,0237 |
| N₂ | 1,39 | 0,0391 |
| CO₂ | 3,59 | 0,0427 |
| H₂O | 5,46 | 0,0305 |

Cuanto más intensas las fuerzas intermoleculares, mayor es $a$ y más fácil es licuar el gas.

> [!TIP]
> Antes de usar $PV = nRT$, verificar las unidades contra la constante: con $R = 0{,}08206$ van litros, atmósferas, moles y kelvin.
