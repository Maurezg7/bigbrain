+++
title = "Inducción y recurrencias"
summary = "El principio de inducción matemática, sumatorias, sucesiones definidas por recurrencia y cómo resolverlas."
branch = "Matemática discreta"
order = 3
level = "Intermedio"
tags = ["inducción", "recurrencias", "sumatorias", "Fibonacci"]
+++

## Inducción matemática

Sirve para demostrar que una propiedad $P(n)$ vale para **todos** los naturales. Es como una fila de fichas de dominó: si cae la primera y cada una tira a la siguiente, caen todas.

> [!IMPORTANT]
> 1. **Caso base**: probar $P(1)$ (o el primer valor).
> 2. **Paso inductivo**: suponer que vale $P(k)$ (**hipótesis inductiva**) y demostrar $P(k + 1)$.

> [!NOTE]
> Demostrar que $1 + 2 + \dots + n = \dfrac{n(n + 1)}{2}$.
>
> **Base**: $n = 1$: $1 = \dfrac{1 \cdot 2}{2}$. ✓
>
> **Paso**: supongamos $1 + \dots + k = \dfrac{k(k + 1)}{2}$. Entonces
>
> $$1 + \dots + k + (k + 1) = \frac{k(k + 1)}{2} + (k + 1) = \frac{(k + 1)(k + 2)}{2}$$
>
> que es la fórmula para $n = k + 1$. ✓

> [!NOTE]
> Demostrar que $n^3 - n$ es divisible por 3.
>
> **Base**: $1 - 1 = 0$. ✓
>
> **Paso**: $(k + 1)^3 - (k + 1) = (k^3 - k) + 3(k^2 + k)$. El primer término es múltiplo de 3 por hipótesis; el segundo, evidentemente. ✓

> [!WARNING]
> Los dos pasos son obligatorios. Sin caso base se «demuestran» cosas falsas; y comprobar muchos casos particulares **no** es una demostración.

**Inducción fuerte**: en el paso inductivo se supone que la propiedad vale para **todos** los valores hasta $k$, no solo para $k$. Se usa, por ejemplo, para probar que todo entero mayor que 1 es producto de primos.

## Sumatorias

$$\sum_{i=1}^{n} a_i = a_1 + a_2 + \dots + a_n$$

Propiedades: $\sum(a_i + b_i) = \sum a_i + \sum b_i$ y $\sum c\,a_i = c\sum a_i$.

| Suma | Fórmula |
|---|---|
| $\displaystyle\sum_{i=1}^{n} 1$ | $n$ |
| $\displaystyle\sum_{i=1}^{n} i$ | $\dfrac{n(n + 1)}{2}$ |
| $\displaystyle\sum_{i=1}^{n} i^2$ | $\dfrac{n(n + 1)(2n + 1)}{6}$ |
| $\displaystyle\sum_{i=1}^{n} i^3$ | $\left[\dfrac{n(n + 1)}{2}\right]^2$ |
| $\displaystyle\sum_{i=0}^{n} r^i$ | $\dfrac{r^{n+1} - 1}{r - 1}$ |
| $\displaystyle\sum_{i=0}^{n} 2^i$ | $2^{n+1} - 1$ |

Una **suma telescópica** se cancela término a término: $\displaystyle\sum_{i=1}^{n}\left(\frac{1}{i} - \frac{1}{i + 1}\right) = 1 - \frac{1}{n + 1}$.

## Recurrencias

Una sucesión se define por **recurrencia** cuando cada término se calcula a partir de los anteriores, más una o varias **condiciones iniciales**.

| Sucesión | Recurrencia | Fórmula cerrada |
|---|---|---|
| Aritmética | $a_n = a_{n-1} + d$ | $a_1 + (n - 1)d$ |
| Geométrica | $a_n = r\,a_{n-1}$ | $a_1r^{n-1}$ |
| Factorial | $a_n = n\,a_{n-1}$, $a_0 = 1$ | $n!$ |
| Fibonacci | $F_n = F_{n-1} + F_{n-2}$ | fórmula de Binet |

### Fibonacci

$F_1 = F_2 = 1$: la sucesión es $1, 1, 2, 3, 5, 8, 13, 21, 34, \dots$ El cociente de términos consecutivos tiende al **número áureo** $\varphi = \dfrac{1 + \sqrt{5}}{2} \approx 1{,}618$.

### Torres de Hanói

Para mover $n$ discos: mover $n - 1$ al poste auxiliar, el mayor al destino, y otra vez los $n - 1$.

$$T_n = 2T_{n-1} + 1, \quad T_1 = 1 \quad\Longrightarrow\quad T_n = 2^n - 1$$

## Resolver recurrencias lineales

Para una recurrencia **lineal homogénea** de segundo orden con coeficientes constantes:

$$a_n = c_1a_{n-1} + c_2a_{n-2}$$

se prueba $a_n = r^n$, lo que da la **ecuación característica**:

$$r^2 - c_1r - c_2 = 0$$

| Raíces | Solución general |
|---|---|
| Distintas $r_1 \neq r_2$ | $a_n = Ar_1^n + Br_2^n$ |
| Doble $r$ | $a_n = (A + Bn)r^n$ |

$A$ y $B$ salen de las condiciones iniciales. El método es análogo al de las ecuaciones diferenciales lineales.

> [!NOTE]
> $a_n = 5a_{n-1} - 6a_{n-2}$, con $a_0 = 1$, $a_1 = 4$.
>
> $r^2 - 5r + 6 = 0 \Rightarrow r = 2, 3$. $a_n = A \cdot 2^n + B \cdot 3^n$.
>
> $A + B = 1$ y $2A + 3B = 4 \Rightarrow A = -1$, $B = 2$.
>
> $a_n = 2 \cdot 3^n - 2^n$. Comprobación: $a_2 = 18 - 4 = 14 = 5(4) - 6(1)$. ✓

Para Fibonacci, $r^2 - r - 1 = 0$ da $r = \dfrac{1 \pm \sqrt{5}}{2}$ y la **fórmula de Binet**:

$$F_n = \frac{\varphi^n - (1 - \varphi)^n}{\sqrt{5}}$$

## Recurrencias en algoritmos

El tiempo de un algoritmo recursivo se expresa con una recurrencia.

| Algoritmo | Recurrencia | Complejidad |
|---|---|---|
| Búsqueda binaria | $T(n) = T(n/2) + 1$ | $O(\log n)$ |
| Recorrido lineal | $T(n) = T(n - 1) + 1$ | $O(n)$ |
| Ordenamiento por mezcla | $T(n) = 2T(n/2) + n$ | $O(n\log n)$ |
| Ordenamiento por selección | $T(n) = T(n - 1) + n$ | $O(n^2)$ |
| Torres de Hanói | $T(n) = 2T(n - 1) + 1$ | $O(2^n)$ |

El **teorema maestro** resuelve directamente las recurrencias de la forma $T(n) = aT(n/b) + f(n)$, típicas de «divide y vencerás».

## Inducción y recursión

Son dos caras de la misma idea:

| | Inducción | Recursión |
|---|---|---|
| Parte inicial | caso base | caso base |
| Paso | de $k$ a $k + 1$ | de $n$ a un problema menor |
| Sirve para | demostrar | calcular |

La corrección de un algoritmo recursivo se demuestra por inducción.

> [!TIP]
> Para encontrar una fórmula cerrada: calcular los primeros términos, conjeturar el patrón y demostrarlo por inducción.
