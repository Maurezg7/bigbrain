+++
title = "Combinatoria"
summary = "Principios de conteo, permutaciones, variaciones, combinaciones, binomio de Newton y principio de inclusión-exclusión."
branch = "Matemática discreta"
order = 2
level = "Intermedio"
tags = ["combinatoria", "permutaciones", "combinaciones", "binomio de Newton"]
+++

## Contar sin enumerar

La combinatoria responde «¿de cuántas maneras?» sin listar todos los casos. Es la base del cálculo de probabilidades y del análisis de algoritmos.

### Principios básicos

> [!IMPORTANT]
> - **Principio del producto**: si una tarea tiene $m$ opciones **y** otra $n$, juntas tienen $m \cdot n$.
> - **Principio de la suma**: si se elige una opción de un grupo de $m$ **o** de otro de $n$ (sin elementos comunes), hay $m + n$.

> [!NOTE]
> Con 3 remeras y 4 pantalones se forman $3 \cdot 4 = 12$ conjuntos.
>
> Una patente con 2 letras y 3 dígitos: $26^2 \cdot 10^3 = 676\,000$ combinaciones.

### Factorial

$$n! = n(n - 1)(n - 2)\cdots 2 \cdot 1, \qquad 0! = 1$$

Crece rapidísimo: $10! = 3\,628\,800$ y $20! \approx 2{,}4 \times 10^{18}$.

## Las dos preguntas

Para elegir la fórmula hay que responder:

1. ¿**Importa el orden**?
2. ¿Se pueden **repetir** elementos?

| | Sin repetición | Con repetición |
|---|---|---|
| **Importa el orden** | $V_{n,k} = \dfrac{n!}{(n - k)!}$ | $n^k$ |
| **No importa el orden** | $\dbinom{n}{k} = \dfrac{n!}{k!(n - k)!}$ | $\dbinom{n + k - 1}{k}$ |

## Permutaciones

Ordenar **todos** los elementos de un conjunto de $n$:

$$P_n = n!$$

> [!NOTE]
> Cinco personas en una fila: $5! = 120$ formas.

**Con elementos repetidos**: si hay $n_1$ iguales de un tipo, $n_2$ de otro, etc.:

$$\frac{n!}{n_1!\,n_2!\cdots n_k!}$$

> [!NOTE]
> Anagramas de «BANANA» (3 A, 2 N, 1 B): $\dfrac{6!}{3!\,2!\,1!} = 60$.

**Circulares**: $n$ personas alrededor de una mesa se ordenan de $(n - 1)!$ formas.

## Variaciones

Elegir y ordenar $k$ elementos de $n$:

> [!NOTE]
> Podio (oro, plata, bronce) entre 8 corredores: $8 \cdot 7 \cdot 6 = 336$.
>
> Claves de 4 dígitos (con repetición): $10^4 = 10\,000$.

## Combinaciones

Elegir $k$ elementos de $n$ **sin importar el orden**. El **número combinatorio**:

> [!IMPORTANT]
> $$\binom{n}{k} = \frac{n!}{k!\,(n - k)!}$$

> [!NOTE]
> Comisiones de 3 personas entre 8: $\dbinom{8}{3} = \dfrac{8 \cdot 7 \cdot 6}{3 \cdot 2 \cdot 1} = 56$.
>
> Es el podio dividido por $3! = 6$, porque cada comisión se contó en todos sus órdenes.

Propiedades:

$$\binom{n}{k} = \binom{n}{n - k} \qquad \binom{n}{0} = \binom{n}{n} = 1 \qquad \binom{n}{k} = \binom{n - 1}{k - 1} + \binom{n - 1}{k}$$

La última genera el **triángulo de Pascal**: cada número es la suma de los dos de arriba.

### Con repetición

Elegir $k$ elementos de $n$ tipos, pudiendo repetir: $\dbinom{n + k - 1}{k}$.

> [!NOTE]
> Formas de elegir 3 bochas de helado entre 5 gustos: $\dbinom{7}{3} = 35$.

## Binomio de Newton

$$(a + b)^n = \sum_{k=0}^{n}\binom{n}{k}a^{n-k}b^k$$

Los coeficientes son la fila $n$ del triángulo de Pascal.

$$(a + b)^4 = a^4 + 4a^3b + 6a^2b^2 + 4ab^3 + b^4$$

Con $a = b = 1$: $\displaystyle\sum_{k=0}^{n}\binom{n}{k} = 2^n$, la cantidad de **subconjuntos** de un conjunto de $n$ elementos.

## Principio de inclusión-exclusión

Para contar la unión de conjuntos que se superponen:

$$|A \cup B| = |A| + |B| - |A \cap B|$$

$$|A \cup B \cup C| = |A| + |B| + |C| - |A \cap B| - |A \cap C| - |B \cap C| + |A \cap B \cap C|$$

> [!NOTE]
> ¿Cuántos números del 1 al 100 son múltiplos de 2 o de 3?
>
> Múltiplos de 2: 50. De 3: 33. De 6: 16. Total: $50 + 33 - 16 = 67$.

## Contar por el complemento

A veces es más fácil contar lo que **no** se quiere y restarlo del total.

> [!NOTE]
> Claves de 4 dígitos con **al menos** un dígito repetido: total menos las que no repiten.
>
> $10^4 - 10 \cdot 9 \cdot 8 \cdot 7 = 10\,000 - 5040 = 4960$.

## Principio del palomar

Si se reparten $n + 1$ objetos en $n$ cajas, alguna caja tiene al menos dos.

En un grupo de 367 personas, dos cumplen años el mismo día. Con solo 23 personas, la probabilidad ya supera el 50 % (paradoja del cumpleaños).

## Aplicación a la probabilidad

Con resultados igualmente probables:

$$P(A) = \frac{\text{casos favorables}}{\text{casos posibles}}$$

> [!NOTE]
> Probabilidad de acertar 6 números de 45 en un sorteo: $\dfrac{1}{\binom{45}{6}} = \dfrac{1}{8\,145\,060}$.

> [!WARNING]
> El error típico es contar de más por tratar como distintos casos que son el mismo. Preguntarse siempre: ¿«A, B» es lo mismo que «B, A»?

> [!TIP]
> Si cambiar el orden da un resultado distinto (podio, clave, fila), es variación o permutación. Si da lo mismo (comisión, mano de cartas), es combinación.
