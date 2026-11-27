+++
title = "Análisis real"
summary = "Axioma del supremo, sucesiones y convergencia, definición épsilon-delta de límite y continuidad, teoremas fundamentales, continuidad uniforme, integral de Riemann y convergencia uniforme."
branch = "Cálculo"
order = 9
level = "Avanzado"
tags = ["análisis real", "supremo", "épsilon-delta", "continuidad uniforme", "sucesiones de Cauchy", "convergencia uniforme"]
+++

## Por qué hace falta

El cálculo enseña a **operar** con límites, derivadas e integrales. El análisis real **demuestra** por qué funcionan, y delimita cuándo dejan de hacerlo.

Todo descansa en una propiedad de los números reales.

## El axioma del supremo

Un conjunto $A \subseteq \mathbb{R}$ está **acotado superiormente** si existe $M$ con $a \le M$ para todo $a \in A$. Ese $M$ es una **cota superior**.

El **supremo** de $A$ es su **menor** cota superior.

> [!IMPORTANT]
> **Axioma de completitud.** Todo conjunto no vacío de números reales acotado superiormente tiene supremo en $\mathbb{R}$.

Es lo que distingue a $\mathbb{R}$ de $\mathbb{Q}$: el conjunto $\{x \in \mathbb{Q} : x^2 < 2\}$ está acotado pero no tiene supremo racional. Los racionales tienen «huecos»; los reales no.

| Concepto | Definición | Ejemplo: $A = [0, 1)$ |
|---|---|---|
| Supremo | menor cota superior | 1 |
| Máximo | supremo que pertenece al conjunto | no existe |
| Ínfimo | mayor cota inferior | 0 |
| Mínimo | ínfimo que pertenece al conjunto | 0 |

**Caracterización útil**: $s = \sup A$ si y solo si $s$ es cota superior y, para todo $\varepsilon > 0$, existe $a \in A$ con $a > s - \varepsilon$.

### Consecuencias

| Propiedad | Enunciado |
|---|---|
| Arquimediana | para todo real $x$ existe un natural $n > x$ |
| Densidad de $\mathbb{Q}$ | entre dos reales distintos hay un racional (y un irracional) |
| Existencia de raíces | todo real positivo tiene raíz $n$-ésima |

$\mathbb{Q}$ es **numerable**; $\mathbb{R}$ no lo es (argumento diagonal de Cantor).

## Sucesiones

La sucesión $(a_n)$ **converge** a $L$ si

$$\forall \varepsilon > 0\ \ \exists N \in \mathbb{N}\ \ \text{tal que}\ \ n > N \implies \lvert a_n - L \rvert < \varepsilon$$

Por pequeño que sea el margen $\varepsilon$, a partir de algún término todos están dentro de él.

> [!NOTE]
> **Demostrar que $\dfrac{1}{n} \to 0$.** Dado $\varepsilon > 0$, por la propiedad arquimediana existe $N > 1/\varepsilon$. Si $n > N$:
> $$\left\lvert \frac{1}{n} - 0 \right\rvert = \frac{1}{n} < \frac{1}{N} < \varepsilon$$

> [!NOTE]
> **Demostrar que $\dfrac{2n + 1}{n + 3} \to 2$.**
> $$\left\lvert \frac{2n + 1}{n + 3} - 2 \right\rvert = \frac{5}{n + 3} < \frac{5}{n}$$
> Basta tomar $N \ge 5/\varepsilon$.

### Teoremas

| Teorema | Enunciado |
|---|---|
| Unicidad | el límite, si existe, es único |
| Acotación | toda sucesión convergente es acotada |
| **Convergencia monótona** | toda sucesión monótona y acotada converge |
| Del emparedado | si $a_n \le b_n \le c_n$ y $a_n, c_n \to L$, entonces $b_n \to L$ |
| **Bolzano-Weierstrass** | toda sucesión acotada tiene una subsucesión convergente |

El teorema de convergencia monótona se deduce del axioma del supremo: una sucesión creciente y acotada converge a su supremo.

> [!NOTE]
> **Ejemplo.** $a_1 = \sqrt{2}$, $a_{n+1} = \sqrt{2 + a_n}$. Por inducción es creciente y está acotada por 2, luego converge. El límite cumple $L = \sqrt{2 + L}$, de donde $L = 2$.

### Sucesiones de Cauchy

$(a_n)$ es **de Cauchy** si sus términos se acercan entre sí:

$$\forall \varepsilon > 0\ \ \exists N\ \ \text{tal que}\ \ m, n > N \implies \lvert a_m - a_n \rvert < \varepsilon$$

> [!IMPORTANT]
> En $\mathbb{R}$, una sucesión converge **si y solo si** es de Cauchy.

Permite probar la convergencia sin conocer el límite. En $\mathbb{Q}$ falla: las aproximaciones decimales de $\sqrt{2}$ son de Cauchy, pero no convergen a un racional.

## Límite de funciones

$$\lim_{x \to a} f(x) = L \iff \forall \varepsilon > 0\ \ \exists \delta > 0\ \ \text{tal que}\ \ 0 < \lvert x - a \rvert < \delta \implies \lvert f(x) - L \rvert < \varepsilon$$

Dado el margen $\varepsilon$ para la salida, hay que encontrar un margen $\delta$ para la entrada.

> [!NOTE]
> **Demostrar que $\lim_{x \to 2} (3x - 1) = 5$.**
> $$\lvert (3x - 1) - 5 \rvert = 3 \lvert x - 2 \rvert$$
> Tomando $\delta = \varepsilon / 3$: si $\lvert x - 2 \rvert < \delta$, entonces $3\lvert x - 2 \rvert < \varepsilon$.

> [!NOTE]
> **Demostrar que $\lim_{x \to 3} x^2 = 9$.**
> $$\lvert x^2 - 9 \rvert = \lvert x + 3 \rvert \, \lvert x - 3 \rvert$$
> Si $\lvert x - 3 \rvert < 1$, entonces $\lvert x + 3 \rvert < 7$. Tomando $\delta = \min(1, \varepsilon/7)$ resulta $\lvert x^2 - 9 \rvert < 7 \cdot \varepsilon/7 = \varepsilon$.

**Criterio por sucesiones**: $\lim_{x \to a} f(x) = L$ si y solo si $f(x_n) \to L$ para toda sucesión $x_n \to a$ con $x_n \ne a$. Sirve para probar que un límite **no** existe: basta hallar dos sucesiones con límites distintos.

> [!NOTE]
> $\sin(1/x)$ no tiene límite en 0: con $x_n = \dfrac{1}{n\pi}$ vale siempre 0, y con $x_n = \dfrac{1}{\pi/2 + 2n\pi}$ vale siempre 1.

## Continuidad

$f$ es **continua en $a$** si $\lim_{x \to a} f(x) = f(a)$:

$$\forall \varepsilon > 0\ \ \exists \delta > 0\ \ \text{tal que}\ \ \lvert x - a \rvert < \delta \implies \lvert f(x) - f(a) \rvert < \varepsilon$$

### Teoremas sobre intervalos cerrados

Para $f$ continua en $[a, b]$:

| Teorema | Enunciado |
|---|---|
| **Bolzano** | si $f(a)$ y $f(b)$ tienen signos opuestos, existe $c \in (a, b)$ con $f(c) = 0$ |
| **Valor intermedio** | $f$ toma todos los valores entre $f(a)$ y $f(b)$ |
| **Weierstrass** | $f$ alcanza un máximo y un mínimo |

Los tres dependen de la completitud, y fallan si se quita alguna hipótesis.

| Sin la hipótesis | Contraejemplo |
|---|---|
| intervalo cerrado | $f(x) = 1/x$ en $(0, 1]$ no está acotada |
| continuidad | una función con salto no toma los valores intermedios |

> [!NOTE]
> **Aplicación.** $f(x) = x^3 + x - 1$ es continua, $f(0) = -1 < 0$ y $f(1) = 1 > 0$: tiene una raíz en $(0, 1)$. El método de bisección la aproxima.

### Continuidad uniforme

En la definición de continuidad, $\delta$ puede depender del punto $a$. Si un mismo $\delta$ sirve para **todos** los puntos, $f$ es **uniformemente continua**:

$$\forall \varepsilon > 0\ \ \exists \delta > 0\ \ \text{tal que}\ \ \lvert x - y \rvert < \delta \implies \lvert f(x) - f(y) \rvert < \varepsilon$$

| Función | Dominio | ¿Uniformemente continua? |
|---|---|---|
| $x^2$ | $[0, 1]$ | sí |
| $x^2$ | $\mathbb{R}$ | no: crece cada vez más rápido |
| $1/x$ | $(0, 1)$ | no: se dispara cerca de 0 |
| $\sin x$ | $\mathbb{R}$ | sí |

**Teorema de Heine-Cantor**: toda función continua en un intervalo **cerrado y acotado** es uniformemente continua.

Una función **lipschitziana**, $\lvert f(x) - f(y) \rvert \le K \lvert x - y \rvert$, es uniformemente continua. Toda función con derivada acotada lo es.

## Derivabilidad

$$f'(a) = \lim_{h \to 0} \frac{f(a + h) - f(a)}{h}$$

Derivable implica continua; la recíproca es falsa ($\lvert x \rvert$ en 0). Existen incluso funciones continuas en todo punto y derivables en **ninguno** (la función de Weierstrass).

| Teorema | Enunciado |
|---|---|
| Fermat | en un extremo local interior, si $f$ es derivable, $f' = 0$ |
| **Rolle** | si $f(a) = f(b)$, existe $c$ con $f'(c) = 0$ |
| **Valor medio** (Lagrange) | existe $c \in (a, b)$ con $f'(c) = \dfrac{f(b) - f(a)}{b - a}$ |

Del teorema del valor medio se deducen: si $f' = 0$ en un intervalo, $f$ es constante; si $f' > 0$, es estrictamente creciente; la regla de L'Hôpital; y la fórmula de Taylor con resto.

## Integral de Riemann

Para $f$ acotada en $[a, b]$ y una **partición** $P = \{x_0 < x_1 < \dots < x_n\}$, con $M_i$ y $m_i$ el supremo y el ínfimo de $f$ en cada subintervalo:

$$U(f, P) = \sum M_i \, \Delta x_i \qquad L(f, P) = \sum m_i \, \Delta x_i$$

Son las sumas **superior** e **inferior**. Siempre $L(f, P) \le U(f, Q)$.

$f$ es **integrable** si el ínfimo de las sumas superiores coincide con el supremo de las inferiores. Equivalentemente:

$$\forall \varepsilon > 0\ \ \exists P\ \ \text{tal que}\ \ U(f, P) - L(f, P) < \varepsilon$$

| Función en $[a, b]$ | ¿Integrable? |
|---|---|
| continua | sí |
| monótona | sí |
| acotada con finitas discontinuidades | sí |
| función de Dirichlet (1 en los racionales, 0 en los irracionales) | **no** |

Para la función de Dirichlet, toda suma superior vale $b - a$ y toda inferior vale 0.

**Criterio de Lebesgue**: una función acotada es integrable Riemann si y solo si su conjunto de discontinuidades tiene medida nula.

### Teorema fundamental del cálculo

1. Si $f$ es continua en $[a, b]$, entonces $F(x) = \displaystyle\int_a^x f(t)\,dt$ es derivable y $F' = f$.
2. Si $F' = f$ y $f$ es integrable, entonces $\displaystyle\int_a^b f(x)\,dx = F(b) - F(a)$.

## Sucesiones de funciones

Sea $f_n: A \to \mathbb{R}$.

| Convergencia | Definición |
|---|---|
| **Puntual** | para cada $x$, $f_n(x) \to f(x)$ |
| **Uniforme** | $\sup_{x \in A} \lvert f_n(x) - f(x) \rvert \to 0$ |

En la uniforme, un mismo $N$ sirve para todos los $x$.

> [!NOTE]
> **Ejemplo.** $f_n(x) = x^n$ en $[0, 1]$ converge puntualmente a
> $$f(x) = \begin{cases} 0 & \text{si } 0 \le x < 1 \\ 1 & \text{si } x = 1 \end{cases}$$
> Cada $f_n$ es continua y el límite **no** lo es. La convergencia no es uniforme: $\sup \lvert f_n - f \rvert = 1$ para todo $n$.

> [!IMPORTANT]
> La convergencia **uniforme** preserva lo que la puntual no:
>
> - el límite uniforme de funciones continuas es **continuo**;
> - se puede intercambiar el límite con la **integral**: $\lim \int f_n = \int \lim f_n$;
> - con convergencia uniforme de las derivadas, se puede derivar término a término.

**Criterio M de Weierstrass**: si $\lvert f_n(x) \rvert \le M_n$ para todo $x$ y $\sum M_n$ converge, entonces $\sum f_n$ converge uniformemente.

> [!NOTE]
> $\displaystyle\sum_{n=1}^{\infty} \frac{\sin(nx)}{n^2}$ converge uniformemente en $\mathbb{R}$, porque $\left\lvert \dfrac{\sin(nx)}{n^2} \right\rvert \le \dfrac{1}{n^2}$. Su suma es una función continua.

Las **series de potencias** convergen uniformemente en todo intervalo cerrado dentro de su intervalo de convergencia: por eso pueden derivarse e integrarse término a término.

## Nociones de topología

| Concepto | Definición |
|---|---|
| Conjunto **abierto** | cada punto tiene un entorno contenido en el conjunto |
| Conjunto **cerrado** | su complemento es abierto; contiene a todos sus puntos de acumulación |
| Punto de acumulación | todo entorno suyo contiene otros puntos del conjunto |
| Conjunto **compacto** | todo cubrimiento por abiertos admite un subcubrimiento finito |

**Teorema de Heine-Borel**: en $\mathbb{R}^n$, un conjunto es compacto si y solo si es **cerrado y acotado**.

La compacidad es la razón de fondo de los teoremas sobre $[a, b]$: una función continua sobre un compacto es acotada, alcanza sus extremos y es uniformemente continua.

## Cómo leer y escribir demostraciones

| Hay que probar | Estrategia |
|---|---|
| «para todo $\varepsilon > 0$ existe...» | empezar con «sea $\varepsilon > 0$» y **construir** lo pedido |
| «existe...» | exhibirlo |
| que algo **no** converge | negar la definición: existe $\varepsilon_0$ tal que para todo $N$... |
| una unicidad | suponer dos y probar que son iguales |

Para negar un enunciado con cuantificadores, se intercambian $\forall$ y $\exists$ y se niega la conclusión.

> [!TIP]
> En una demostración $\varepsilon$-$\delta$, el trabajo real se hace en borrador y **al revés**: se parte de $\lvert f(x) - L \rvert < \varepsilon$ y se despeja qué $\delta$ hace falta. La demostración final lo presenta en el orden lógico.
