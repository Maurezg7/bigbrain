+++
title = "Aritmética modular y teoría de números"
summary = "Divisibilidad, números primos, máximo común divisor, congruencias, y su uso en criptografía."
branch = "Matemática discreta"
order = 5
level = "Intermedio"
tags = ["divisibilidad", "primos", "congruencias", "algoritmo de Euclides", "RSA"]
+++

## Divisibilidad

$a$ **divide** a $b$ (se escribe $a \mid b$) si existe un entero $k$ con $b = ak$.

**Algoritmo de la división**: dados $a$ y $b > 0$, existen únicos $q$ (cociente) y $r$ (resto) tales que

$$a = bq + r, \qquad 0 \le r < b$$

### Criterios de divisibilidad

| Por | Condición |
|---|---|
| 2 | termina en cifra par |
| 3 | la suma de sus cifras es múltiplo de 3 |
| 4 | sus dos últimas cifras forman un múltiplo de 4 |
| 5 | termina en 0 o 5 |
| 9 | la suma de sus cifras es múltiplo de 9 |
| 11 | la suma alternada de sus cifras es múltiplo de 11 |

## Números primos

Un **primo** es un entero mayor que 1 cuyos únicos divisores positivos son 1 y él mismo: $2, 3, 5, 7, 11, 13, \dots$

> [!IMPORTANT]
> **Teorema fundamental de la aritmética**: todo entero mayor que 1 se escribe de forma única como producto de primos.

$$360 = 2^3 \cdot 3^2 \cdot 5$$

- Hay **infinitos** primos (Euclides).
- Para saber si $n$ es primo, basta probar divisores hasta $\sqrt{n}$.
- La **criba de Eratóstenes** halla todos los primos hasta $n$ tachando múltiplos.

## MCD y MCM

- **Máximo común divisor** $\text{mcd}(a, b)$: el mayor entero que divide a ambos.
- **Mínimo común múltiplo** $\text{mcm}(a, b)$: el menor múltiplo positivo de ambos.

$$\text{mcd}(a, b) \cdot \text{mcm}(a, b) = a \cdot b$$

Si $\text{mcd}(a, b) = 1$, son **coprimos**.

### Algoritmo de Euclides

Se basa en que $\text{mcd}(a, b) = \text{mcd}(b, a \bmod b)$. Se divide sucesivamente hasta obtener resto cero; el último resto no nulo es el MCD.

> [!NOTE]
> $\text{mcd}(252, 198)$:
>
> $252 = 198 \cdot 1 + 54$
>
> $198 = 54 \cdot 3 + 36$
>
> $54 = 36 \cdot 1 + 18$
>
> $36 = 18 \cdot 2 + 0$
>
> $\text{mcd} = 18$, y $\text{mcm} = \dfrac{252 \cdot 198}{18} = 2772$.

Es mucho más rápido que factorizar: lleva una cantidad de pasos proporcional al número de cifras.

### Identidad de Bézout

Existen enteros $x, y$ tales que

$$ax + by = \text{mcd}(a, b)$$

Se obtienen recorriendo el algoritmo de Euclides hacia atrás. La ecuación $ax + by = c$ tiene soluciones enteras si y solo si $\text{mcd}(a, b) \mid c$.

## Congruencias

> [!IMPORTANT]
> $a \equiv b \pmod{n}$ si $n$ divide a $a - b$, es decir, si $a$ y $b$ dan el **mismo resto** al dividir por $n$.

Es la aritmética del reloj: 5 horas después de las 10 son las 3, porque $15 \equiv 3 \pmod{12}$.

Las congruencias se pueden sumar, restar y multiplicar. Si $a \equiv b$ y $c \equiv d \pmod{n}$:

$$a + c \equiv b + d \qquad a \cdot c \equiv b \cdot d \qquad a^k \equiv b^k \pmod{n}$$

> [!NOTE]
> Última cifra de $7^{100}$, es decir $7^{100} \bmod 10$.
>
> $7^2 = 49 \equiv -1 \pmod{10}$, así que $7^{100} = (7^2)^{50} \equiv (-1)^{50} = 1$. Termina en **1**.

> [!NOTE]
> Hoy es martes. ¿Qué día será dentro de 100 días? $100 = 14 \cdot 7 + 2$, así que $100 \equiv 2 \pmod 7$: jueves.

> [!WARNING]
> No siempre se puede dividir: $6 \equiv 2 \pmod 4$, pero dividiendo por 2 daría $3 \equiv 1 \pmod 4$, que es falso. Solo se puede simplificar un factor coprimo con $n$.

### Inverso modular

El inverso de $a$ módulo $n$ es un $x$ con $ax \equiv 1 \pmod{n}$. Existe si y solo si $\text{mcd}(a, n) = 1$, y se calcula con Bézout.

> [!NOTE]
> Inverso de 3 módulo 7: $3 \cdot 5 = 15 \equiv 1$, así que es 5.
>
> Resolver $3x \equiv 4 \pmod 7$: $x \equiv 5 \cdot 4 = 20 \equiv 6$.

## Teoremas de Fermat y Euler

> [!IMPORTANT]
> **Pequeño teorema de Fermat**: si $p$ es primo y no divide a $a$, entonces $a^{p-1} \equiv 1 \pmod{p}$.

La **función de Euler** $\varphi(n)$ cuenta los enteros entre 1 y $n$ coprimos con $n$. Para $p$ primo, $\varphi(p) = p - 1$; para $n = pq$ con $p, q$ primos distintos, $\varphi(n) = (p - 1)(q - 1)$.

**Teorema de Euler**: si $\text{mcd}(a, n) = 1$, entonces $a^{\varphi(n)} \equiv 1 \pmod{n}$.

> [!NOTE]
> $3^{100} \bmod 7$: por Fermat, $3^6 \equiv 1$. Como $100 = 6 \cdot 16 + 4$: $3^{100} \equiv 3^4 = 81 \equiv 4 \pmod 7$.

**Teorema chino del resto**: un sistema de congruencias con módulos coprimos entre sí tiene solución única módulo el producto de los módulos.

## Aplicaciones

### Criptografía RSA

Se basa en que multiplicar dos primos grandes es fácil, pero factorizar el producto es prácticamente imposible.

1. Elegir dos primos grandes $p$ y $q$; calcular $n = pq$ y $\varphi(n) = (p - 1)(q - 1)$.
2. Elegir $e$ coprimo con $\varphi(n)$. La **clave pública** es $(n, e)$.
3. Calcular $d$, el inverso de $e$ módulo $\varphi(n)$. Es la **clave privada**.
4. **Cifrar**: $c = m^e \bmod n$. **Descifrar**: $m = c^d \bmod n$.

Funciona por el teorema de Euler: $m^{ed} \equiv m \pmod{n}$.

### Otras

| Uso | Cómo |
|---|---|
| Dígitos verificadores (CUIT, ISBN, tarjetas) | suma ponderada módulo 10 u 11 |
| Tablas hash | índice = clave $\bmod$ tamaño |
| Números pseudoaleatorios | $x_{n+1} = (ax_n + c) \bmod m$ |
| Detección de errores (CRC) | resto de una división de polinomios |
| Calendarios | cálculo del día de la semana módulo 7 |

> [!TIP]
> Trabajar módulo $n$ es quedarse solo con los restos: los números enormes se vuelven pequeños y los cálculos, manejables.
