+++
title = "Estructuras algebraicas"
summary = "Operaciones binarias, grupos, subgrupos, grupos cíclicos y de permutaciones, teorema de Lagrange, homomorfismos, anillos, cuerpos y cuerpos finitos."
branch = "Matemática discreta"
order = 6
level = "Avanzado"
tags = ["grupos", "anillos", "cuerpos", "homomorfismos", "Lagrange", "álgebra abstracta"]
+++

## La idea

El álgebra abstracta estudia **conjuntos con operaciones**, atendiendo solo a las propiedades que cumplen. Lo que se demuestra para un grupo vale a la vez para los enteros con la suma, las rotaciones de un cuadrado y las matrices invertibles.

Una **operación binaria** en un conjunto $G$ asigna a cada par $(a, b)$ un elemento $a * b$ de $G$. Que el resultado esté en $G$ se llama **clausura**.

| Propiedad | Definición |
|---|---|
| Asociativa | $(a * b) * c = a * (b * c)$ |
| Conmutativa | $a * b = b * a$ |
| Elemento neutro $e$ | $a * e = e * a = a$ |
| Inverso de $a$ | $a * a^{-1} = a^{-1} * a = e$ |

## Grupos

Un **grupo** $(G, *)$ es un conjunto con una operación **asociativa**, con **neutro**, y donde cada elemento tiene **inverso**.

Si además es conmutativa, el grupo es **abeliano**.

| Estructura | Asociativa | Neutro | Inversos | Conmutativa |
|---|---|---|---|---|
| Semigrupo | sí | | | |
| Monoide | sí | sí | | |
| Grupo | sí | sí | sí | |
| Grupo abeliano | sí | sí | sí | sí |

### Ejemplos

| Conjunto y operación | ¿Grupo? | Observación |
|---|---|---|
| $(\mathbb{Z}, +)$ | sí, abeliano | neutro 0; inverso $-a$ |
| $(\mathbb{N}, +)$ | no | faltan los inversos |
| $(\mathbb{Z}, \cdot)$ | no | solo $\pm 1$ tienen inverso |
| $(\mathbb{Q} \setminus \{0\}, \cdot)$ | sí, abeliano | inverso $1/a$ |
| $(\mathbb{Z}_n, +)$ | sí, abeliano | suma módulo $n$ |
| $(\mathbb{Z}_n^*, \cdot)$ | sí, abeliano | los coprimos con $n$, con el producto módulo $n$ |
| Matrices invertibles $n \times n$ | sí, **no** abeliano | grupo lineal $GL_n$ |
| Permutaciones de $n$ elementos | sí, no abeliano si $n \ge 3$ | grupo simétrico $S_n$ |
| Simetrías de un polígono regular | sí | grupo diédrico $D_n$ |

### Propiedades básicas

En todo grupo:

- el neutro es **único**, y el inverso de cada elemento también;
- $(a * b)^{-1} = b^{-1} * a^{-1}$ (se invierte el orden);
- vale la **cancelación**: si $a * b = a * c$, entonces $b = c$;
- la ecuación $a * x = b$ tiene solución única, $x = a^{-1} * b$.

En la **tabla de Cayley** de un grupo finito, cada elemento aparece exactamente una vez en cada fila y en cada columna.

> [!NOTE]
> **$(\mathbb{Z}_4, +)$:**
>
> | + | 0 | 1 | 2 | 3 |
> |---|---|---|---|---|
> | **0** | 0 | 1 | 2 | 3 |
> | **1** | 1 | 2 | 3 | 0 |
> | **2** | 2 | 3 | 0 | 1 |
> | **3** | 3 | 0 | 1 | 2 |

## Orden y grupos cíclicos

El **orden de un grupo** es su cantidad de elementos, $\lvert G \rvert$. El **orden de un elemento** $a$ es el menor entero positivo $k$ con $a^k = e$.

Un grupo es **cíclico** si todos sus elementos son potencias de uno solo, el **generador**: $G = \langle g \rangle$.

> [!NOTE]
> **Ejemplo.** En $\mathbb{Z}_7^* = \{1, 2, 3, 4, 5, 6\}$ con el producto módulo 7, las potencias de 3 son
> $$3^1 = 3,\quad 3^2 = 2,\quad 3^3 = 6,\quad 3^4 = 4,\quad 3^5 = 5,\quad 3^6 = 1$$
> El 3 genera todo el grupo. En cambio, $2^3 = 8 \equiv 1$: el 2 tiene orden 3 y genera solo $\{1, 2, 4\}$.

Todo grupo cíclico es abeliano. Todo grupo cíclico infinito se comporta como $(\mathbb{Z}, +)$, y todo cíclico de orden $n$, como $(\mathbb{Z}_n, +)$.

## Subgrupos

Un **subgrupo** $H \le G$ es un subconjunto que es un grupo con la misma operación. Basta comprobar que $H$ no es vacío y que, si $a, b \in H$, entonces $a * b^{-1} \in H$.

| Grupo | Subgrupo |
|---|---|
| $(\mathbb{Z}, +)$ | los múltiplos de $n$: $n\mathbb{Z}$ |
| $(\mathbb{R} \setminus \{0\}, \cdot)$ | los reales positivos |
| $GL_n$ | las matrices de determinante 1 |

### Teorema de Lagrange

> [!IMPORTANT]
> Si $H$ es un subgrupo de un grupo finito $G$, el orden de $H$ **divide** al orden de $G$.

La demostración parte $G$ en **clases laterales** $aH = \{a * h : h \in H\}$, todas del mismo tamaño que $H$ y disjuntas entre sí.

Consecuencias:

- el orden de cada elemento divide a $\lvert G \rvert$, de modo que $a^{\lvert G \rvert} = e$;
- todo grupo de orden **primo** es cíclico;
- aplicado a $\mathbb{Z}_p^*$, de orden $p - 1$, da el **pequeño teorema de Fermat**:

$$a^{p-1} \equiv 1 \pmod{p}$$

- aplicado a $\mathbb{Z}_n^*$, de orden $\varphi(n)$, da el **teorema de Euler**, base del cifrado RSA.

## Permutaciones

Una **permutación** es una biyección de un conjunto en sí mismo. Las de $n$ elementos forman el **grupo simétrico** $S_n$, de orden $n!$.

En **notación de ciclos**, $(1\ 3\ 2)$ indica $1 \to 3 \to 2 \to 1$.

> [!NOTE]
> **Composición** (de derecha a izquierda). Con $\sigma = (1\ 2)$ y $\tau = (2\ 3)$:
>
> $\sigma\tau$: $1 \to 1 \to 2$, $2 \to 3 \to 3$, $3 \to 2 \to 1$. Resulta $(1\ 2\ 3)$.
>
> $\tau\sigma$: $1 \to 2 \to 3$, $3 \to 3 \to 2$, $2 \to 1 \to 1$. Resulta $(1\ 3\ 2)$.
>
> Son distintas: $S_3$ no es abeliano.

Toda permutación es producto de **trasposiciones** (intercambios de dos elementos). Es **par** o **impar** según la cantidad que requiere; las pares forman el **grupo alternante** $A_n$, de orden $n!/2$.

**Teorema de Cayley**: todo grupo se comporta como un grupo de permutaciones.

### Grupo diédrico

Las simetrías de un polígono regular de $n$ lados forman $D_n$, de orden $2n$: $n$ rotaciones y $n$ reflexiones. Para el cuadrado, $D_4$ tiene 8 elementos.

## Homomorfismos

Un **homomorfismo** es una función $f: G \to H$ entre grupos que respeta la operación:

$$f(a * b) = f(a) \circ f(b)$$

| Concepto | Definición |
|---|---|
| **Núcleo** | $\ker f = \{a \in G : f(a) = e_H\}$ |
| **Imagen** | $\operatorname{Im} f = \{f(a) : a \in G\}$ |
| **Isomorfismo** | homomorfismo biyectivo |

$f$ es inyectivo si y solo si su núcleo es $\{e\}$.

Dos grupos **isomorfos** ($G \cong H$) son el mismo grupo con otros nombres.

| Homomorfismo | De | A |
|---|---|---|
| $f(x) = e^x$ | $(\mathbb{R}, +)$ | $(\mathbb{R}^+, \cdot)$: es un isomorfismo |
| determinante | $GL_n$ | $(\mathbb{R} \setminus \{0\}, \cdot)$ |
| $f(a) = a \bmod n$ | $(\mathbb{Z}, +)$ | $(\mathbb{Z}_n, +)$ |

> [!NOTE]
> El logaritmo convierte productos en sumas, $\ln(ab) = \ln a + \ln b$, porque es un isomorfismo entre $(\mathbb{R}^+, \cdot)$ y $(\mathbb{R}, +)$. Es el principio de la regla de cálculo.

### Grupo cociente

Un subgrupo $N$ es **normal** si $aN = Na$ para todo $a$. Entonces las clases laterales forman un grupo, el **cociente** $G/N$.

**Primer teorema de isomorfía**:

$$G / \ker f \cong \operatorname{Im} f$$

Así, $\mathbb{Z} / n\mathbb{Z} \cong \mathbb{Z}_n$.

## Anillos

Un **anillo** $(R, +, \cdot)$ tiene dos operaciones tales que:

1. $(R, +)$ es un grupo abeliano;
2. el producto es asociativo;
3. vale la **distributiva**: $a(b + c) = ab + ac$ y $(a + b)c = ac + bc$.

| Tipo | Propiedad adicional | Ejemplos |
|---|---|---|
| Conmutativo | $ab = ba$ | $\mathbb{Z}$, polinomios |
| Con unidad | existe el 1 | $\mathbb{Z}$, matrices |
| **Dominio de integridad** | conmutativo, con unidad y sin divisores de cero | $\mathbb{Z}$, $\mathbb{R}[x]$ |
| **Cuerpo** | todo elemento no nulo tiene inverso multiplicativo | $\mathbb{Q}$, $\mathbb{R}$, $\mathbb{C}$, $\mathbb{Z}_p$ |

Un **divisor de cero** es un elemento $a \ne 0$ para el que existe $b \ne 0$ con $ab = 0$.

> [!NOTE]
> En $\mathbb{Z}_6$: $2 \cdot 3 = 6 \equiv 0$. Hay divisores de cero, y por eso la ecuación $2x = 0$ tiene dos soluciones ($x = 0$ y $x = 3$).

Las matrices $n \times n$ forman un anillo no conmutativo con divisores de cero. Los polinomios con coeficientes en un cuerpo, $K[x]$, forman un dominio de integridad donde vale la división con resto, igual que en los enteros.

Un **ideal** es un subconjunto $I$ cerrado bajo la suma y que absorbe productos ($ra \in I$ para todo $r \in R$, $a \in I$). Es el análogo del subgrupo normal: permite construir el anillo cociente $R/I$.

## Cuerpos

Un **cuerpo** es un anillo conmutativo con unidad en el que se puede dividir por cualquier elemento no nulo. Es la estructura donde funciona toda el álgebra lineal.

| Cuerpo | Elementos |
|---|---|
| $\mathbb{Q}$ | racionales |
| $\mathbb{R}$ | reales |
| $\mathbb{C}$ | complejos; algebraicamente cerrado |
| $\mathbb{Z}_p$ | enteros módulo un primo $p$ |

> [!IMPORTANT]
> $\mathbb{Z}_n$ es un cuerpo **si y solo si $n$ es primo**. Si $n$ es compuesto, tiene divisores de cero.

### Cuerpos finitos

Existe un cuerpo finito con $q$ elementos si y solo si $q = p^k$ es potencia de un primo. Es único salvo isomorfismo, y se denota $GF(q)$ o $\mathbb{F}_q$.

Para $k > 1$ **no** es $\mathbb{Z}_{p^k}$: se construye con polinomios sobre $\mathbb{Z}_p$, módulo un polinomio irreducible de grado $k$.

> [!NOTE]
> **$GF(4)$.** Polinomios sobre $\mathbb{Z}_2$ módulo $x^2 + x + 1$. Elementos: $0$, $1$, $x$, $x + 1$.
>
> $x \cdot x = x^2 = x + 1$ (porque $x^2 + x + 1 = 0$ y en $\mathbb{Z}_2$ es $-1 = 1$).
>
> $x \cdot (x + 1) = x^2 + x = (x + 1) + x = 1$: son inversos entre sí.

El grupo multiplicativo de un cuerpo finito siempre es **cíclico**.

## Aplicaciones

| Área | Estructura | Uso |
|---|---|---|
| Criptografía | $\mathbb{Z}_n^*$, curvas elípticas sobre cuerpos finitos | RSA, Diffie-Hellman, firmas |
| Cifrado AES | $GF(2^8)$ | cada byte es un elemento del cuerpo |
| Códigos correctores | cuerpos finitos, polinomios | Reed-Solomon (códigos QR, discos), CRC |
| Física | grupos de Lie | simetrías y leyes de conservación |
| Química | grupos puntuales | simetría molecular, espectroscopía |
| Cristalografía | grupos espaciales | los 230 tipos de simetría cristalina |
| Combinatoria | acciones de grupos | contar configuraciones salvo simetría (lema de Burnside) |
| Informática | monoides | las cadenas con la concatenación; agregación en paralelo |

> [!NOTE]
> **Lema de Burnside.** La cantidad de configuraciones distintas salvo simetría es el promedio de las configuraciones que cada simetría deja fijas.
>
> Collares de 4 cuentas con 2 colores, salvo rotación: la identidad fija $2^4 = 16$; los giros de 90° y 270°, 2 cada uno; el de 180°, 4.
> $$\frac{16 + 2 + 4 + 2}{4} = 6$$

> [!TIP]
> Ante una estructura nueva, preguntarse qué axiomas cumple: si resulta ser un grupo o un cuerpo, todos los teoremas ya demostrados para esas estructuras quedan disponibles sin más trabajo.
