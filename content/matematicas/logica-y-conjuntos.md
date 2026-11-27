+++
title = "Lógica proposicional y conjuntos"
summary = "Conectivos, tablas de verdad y operaciones entre conjuntos: la gramática de las demostraciones."
branch = "Matemática discreta"
order = 1
level = "Básico"
tags = ["lógica", "conjuntos", "tablas de verdad", "demostración"]
+++

## Proposiciones y conectivos

Una **proposición** es una afirmación que es verdadera (V) o falsa (F). Se combinan con conectivos:

| $p$ | $q$ | $\neg p$ | $p \land q$ | $p \lor q$ | $p \Rightarrow q$ | $p \Leftrightarrow q$ |
|---|---|---|---|---|---|---|
| V | V | F | V | V | V | V |
| V | F | F | F | V | F | F |
| F | V | V | F | V | V | F |
| F | F | V | F | F | V | V |

> [!WARNING]
> La implicación $p \Rightarrow q$ solo es falsa cuando $p$ es verdadera y $q$ falsa. Si $p$ es falsa, la implicación es verdadera sin importar $q$.

## Equivalencias importantes

- **Leyes de De Morgan**: $\neg(p \land q) \equiv \neg p \lor \neg q$ y $\neg(p \lor q) \equiv \neg p \land \neg q$
- **Contrarrecíproco**: $p \Rightarrow q \equiv \neg q \Rightarrow \neg p$
- **Implicación**: $p \Rightarrow q \equiv \neg p \lor q$

Los cuantificadores $\forall$ ("para todo") y $\exists$ ("existe") se niegan intercambiándolos: $\neg(\forall x\, P(x)) \equiv \exists x\, \neg P(x)$.

## Conjuntos

Un **conjunto** es una colección de elementos. $x \in A$ significa que $x$ pertenece a $A$, y $A \subseteq B$ que todo elemento de $A$ está en $B$.

![Diagramas de Venn de unión, intersección y diferencia](/media/matematicas/venn.svg)

*Unión, intersección y diferencia de dos conjuntos.*

| Operación | Definición |
|---|---|
| Unión | $A \cup B = \{x : x \in A \lor x \in B\}$ |
| Intersección | $A \cap B = \{x : x \in A \land x \in B\}$ |
| Diferencia | $A \setminus B = \{x : x \in A \land x \notin B\}$ |
| Complemento | $A^c = \{x : x \notin A\}$ |

Cada operación de conjuntos corresponde a un conectivo lógico, y por eso valen las mismas leyes: $(A \cup B)^c = A^c \cap B^c$.

## Métodos de demostración

- **Directa**: se asume $p$ y se deduce $q$.
- **Por contrarrecíproco**: se asume $\neg q$ y se deduce $\neg p$.
- **Por el absurdo**: se asume lo contrario de lo que se quiere probar y se llega a una contradicción.
- **Por inducción**: se prueba el caso base $P(1)$ y que $P(n) \Rightarrow P(n+1)$.

> [!NOTE]
> Probar por inducción que $1 + 2 + \dots + n = \dfrac{n(n+1)}{2}$.
>
> Caso base: para $n=1$, $1 = \frac{1\cdot 2}{2}$. Paso inductivo: si vale para $n$, entonces
> $1 + \dots + n + (n+1) = \frac{n(n+1)}{2} + (n+1) = \frac{(n+1)(n+2)}{2}$, que es la fórmula para $n+1$.
