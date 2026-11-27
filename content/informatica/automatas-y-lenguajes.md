+++
title = "Autómatas y lenguajes formales"
summary = "Alfabetos y lenguajes, autómatas finitos deterministas y no deterministas, expresiones regulares, gramáticas libres de contexto, autómatas de pila y jerarquía de Chomsky."
branch = "Teoría de la computación"
order = 1
level = "Intermedio"
tags = ["autómatas", "expresiones regulares", "gramáticas", "lenguajes formales", "Chomsky"]
+++

## Lenguajes formales

| Concepto | Definición | Ejemplo |
|---|---|---|
| **Alfabeto** $\Sigma$ | conjunto finito de símbolos | $\{0, 1\}$ |
| **Cadena** | secuencia finita de símbolos | $0110$ |
| Cadena vacía $\varepsilon$ | la de longitud cero | |
| $\Sigma^*$ | todas las cadenas sobre $\Sigma$ | $\varepsilon, 0, 1, 00, 01, \dots$ |
| **Lenguaje** | un subconjunto de $\Sigma^*$ | las cadenas con cantidad par de ceros |

La pregunta central: dado un lenguaje $L$ y una cadena $w$, ¿pertenece $w$ a $L$? Distintas clases de máquinas reconocen distintas clases de lenguajes.

## Autómata finito determinista (AFD)

Una máquina con una cantidad **finita de estados** y sin más memoria. Se define con cinco elementos:

| Elemento | Significado |
|---|---|
| $Q$ | conjunto finito de estados |
| $\Sigma$ | alfabeto |
| $\delta : Q \times \Sigma \to Q$ | función de transición |
| $q_0$ | estado inicial |
| $F \subseteq Q$ | estados de aceptación |

Lee la cadena símbolo a símbolo. Si al terminar está en un estado de aceptación, la **acepta**.

> [!NOTE]
> **Ejemplo.** Cadenas binarias con cantidad **par** de ceros.
>
> ```text
>            1                    1
>          ┌───┐                ┌───┐
>          ▼   │       0        ▼   │
>   ──▶ (( par ))  ─────────▶  ( impar )
>                  ◀─────────
>                       0
> ```
>
> | Estado | con 0 | con 1 |
> |---|---|---|
> | par (inicial, acepta) | impar | par |
> | impar | par | impar |
>
> Con $1001$: par → par → impar → par → par. **Acepta**.

## Autómata finito no determinista (AFN)

Desde un estado puede haber **varias** transiciones con el mismo símbolo, o transiciones $\varepsilon$ (sin consumir entrada). Acepta si **algún** camino termina en un estado de aceptación.

> [!IMPORTANT]
> Los AFN no son más potentes: todo AFN tiene un AFD equivalente (**construcción de subconjuntos**). Cada estado del AFD es un conjunto de estados del AFN, por lo que puede llegar a tener $2^n$ estados.

Los AFN suelen ser más fáciles de diseñar; los AFD, más rápidos de ejecutar.

## Expresiones regulares

Describen lenguajes con tres operaciones:

| Operación | Notación | Significado |
|---|---|---|
| Unión | $r \mid s$ | $r$ o $s$ |
| Concatenación | $rs$ | $r$ seguido de $s$ |
| Clausura de Kleene | $r^*$ | cero o más repeticiones de $r$ |

| Expresión | Lenguaje |
|---|---|
| $(0 \mid 1)^*$ | todas las cadenas binarias |
| $1^* (0 1^* 0 1^*)^*$ | cantidad par de ceros |
| $(0 \mid 1)^* 01$ | terminan en 01 |

> [!IMPORTANT]
> **Teorema de Kleene.** Expresiones regulares, AFD y AFN describen exactamente la misma clase: los **lenguajes regulares**.

### En la práctica

| Sintaxis | Significado |
|---|---|
| `.` | cualquier carácter |
| `[a-z]`, `[^0-9]` | un carácter del conjunto; fuera del conjunto |
| `\d`, `\w`, `\s` | dígito, carácter de palabra, espacio |
| `*`, `+`, `?` | cero o más, uno o más, opcional |
| `{2,4}` | entre 2 y 4 repeticiones |
| `^`, `$` | inicio y fin |
| `( )` | grupo |

```python
import re

patron = re.compile(r"^(\d{4})-(\d{2})-(\d{2})$")
m = patron.match("2025-03-14")
print(m.groups())                      # ('2025', '03', '14')
```

## Límites de los lenguajes regulares

Un autómata finito no puede **contar sin límite**. El lenguaje

$$L = \{ a^n b^n : n \ge 0 \} = \{\varepsilon, ab, aabb, aaabbb, \dots\}$$

no es regular: habría que recordar cuántas $a$ se leyeron, y los estados son finitos.

### Lema del bombeo

Si $L$ es regular, existe $p$ tal que toda cadena $w \in L$ con $\lvert w \rvert \ge p$ se puede partir en $w = xyz$ con

1. $\lvert xy \rvert \le p$,
2. $\lvert y \rvert \ge 1$,
3. $x y^i z \in L$ para todo $i \ge 0$.

Sirve para probar que un lenguaje **no** es regular.

> [!NOTE]
> **$a^n b^n$ no es regular.** Tomar $w = a^p b^p$. Como $\lvert xy \rvert \le p$, el tramo $y$ contiene solo letras $a$. Entonces $xy^2z$ tiene más $a$ que $b$ y no pertenece a $L$: contradicción.

Por eso las expresiones regulares no pueden verificar paréntesis balanceados ni analizar HTML anidado.

### Propiedades de clausura

Los lenguajes regulares son cerrados bajo unión, intersección, complemento, concatenación y clausura. Además, se puede decidir si un AFD acepta alguna cadena y si dos AFD son equivalentes.

## Gramáticas libres de contexto

Una **gramática** genera cadenas mediante reglas de reescritura.

| Elemento | Significado |
|---|---|
| Variables (no terminales) | símbolos que se reemplazan |
| Terminales | símbolos del alfabeto |
| Producciones | reglas $A \to \alpha$ |
| Símbolo inicial | por donde se empieza |

> [!NOTE]
> **Gramática de $a^n b^n$:**
> $$S \to aSb \mid \varepsilon$$
>
> Derivación de $aabb$: $S \Rightarrow aSb \Rightarrow aaSbb \Rightarrow aabb$.

Una gramática de expresiones aritméticas:

```text
E → E + T | T
T → T * F | F
F → ( E ) | número
```

Los niveles `E`, `T`, `F` codifican la **precedencia**: el producto se agrupa antes que la suma.

### Árbol de derivación

Para `2 + 3 * 4`:

```text
          E
        / | \
       E  +  T
       |   / | \
       T  T  *  F
       |  |     |
       F  F     4
       |  |
       2  3
```

Una gramática es **ambigua** si alguna cadena tiene más de un árbol. La gramática `E → E + E | E * E | número` lo es: `2 + 3 * 4` admite dos árboles, con resultados distintos.

La notación **BNF** (y su extensión EBNF) es la forma habitual de escribir la gramática de un lenguaje de programación.

## Autómata de pila

Es un autómata finito con una **pila** de capacidad ilimitada. Reconoce exactamente los **lenguajes libres de contexto**.

Para $a^n b^n$: apila un símbolo por cada $a$, desapila uno por cada $b$, y acepta si la pila queda vacía al final.

Tampoco alcanza para todo: $\{a^n b^n c^n\}$ no es libre de contexto.

## Jerarquía de Chomsky

| Tipo | Lenguajes | Máquina que los reconoce | Forma de las reglas |
|---|---|---|---|
| 3 | regulares | autómata finito | $A \to aB$, $A \to a$ |
| 2 | libres de contexto | autómata de pila | $A \to \alpha$ |
| 1 | sensibles al contexto | autómata linealmente acotado | $\alpha A \beta \to \alpha \gamma \beta$ |
| 0 | recursivamente enumerables | máquina de Turing | sin restricciones |

Cada clase contiene a la anterior:

$$\text{regulares} \subset \text{libres de contexto} \subset \text{sensibles al contexto} \subset \text{recursivamente enumerables}$$

## Aplicaciones

| Herramienta teórica | Uso |
|---|---|
| Autómatas finitos, expresiones regulares | analizadores léxicos, búsqueda de texto, validación, protocolos |
| Gramáticas libres de contexto | analizadores sintácticos de lenguajes de programación, JSON, XML |
| Máquinas de estados | interfaces, controladores, videojuegos, circuitos secuenciales |

> [!TIP]
> Para decidir qué herramienta usar: si hay que **contar o anidar** sin límite (paréntesis, bloques, etiquetas), una expresión regular no alcanza y hace falta un analizador sintáctico.
