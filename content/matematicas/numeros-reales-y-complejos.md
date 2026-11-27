+++
title = "Números reales y complejos"
summary = "Los conjuntos numéricos, intervalos, potencias y raíces, y los números complejos en forma binómica, polar y exponencial."
branch = "Álgebra"
order = 0
level = "Básico"
tags = ["números reales", "complejos", "intervalos", "De Moivre"]
+++

## Los conjuntos numéricos

Cada conjunto amplía al anterior para poder resolver una operación que antes no tenía respuesta.

| Conjunto | Símbolo | Contiene | Permite |
|---|---|---|---|
| Naturales | $\mathbb{N}$ | $1, 2, 3, \dots$ | contar, sumar |
| Enteros | $\mathbb{Z}$ | $\dots, -2, -1, 0, 1, 2, \dots$ | restar siempre |
| Racionales | $\mathbb{Q}$ | fracciones $p/q$ con $q \neq 0$ | dividir |
| Reales | $\mathbb{R}$ | racionales e irracionales | medir cualquier longitud |
| Complejos | $\mathbb{C}$ | $a + bi$ | sacar raíces de negativos |

$$\mathbb{N} \subset \mathbb{Z} \subset \mathbb{Q} \subset \mathbb{R} \subset \mathbb{C}$$

Un número es **racional** si su expresión decimal es finita o periódica ($0{,}75$; $0{,}333\dots$). Es **irracional** si tiene infinitas cifras sin período: $\sqrt{2}$, $\pi$, $e$.

## Propiedades de los reales

Para todo $a, b, c \in \mathbb{R}$:

| Propiedad | Suma | Producto |
|---|---|---|
| Conmutativa | $a + b = b + a$ | $ab = ba$ |
| Asociativa | $(a+b)+c = a+(b+c)$ | $(ab)c = a(bc)$ |
| Neutro | $a + 0 = a$ | $a \cdot 1 = a$ |
| Inverso | $a + (-a) = 0$ | $a \cdot a^{-1} = 1$ si $a \neq 0$ |
| Distributiva | $a(b + c) = ab + ac$ | |

Además $\mathbb{R}$ está **ordenado** (dados dos reales distintos, uno es menor) y es **completo**: no tiene «huecos» en la recta.

## Intervalos

| Notación | Conjunto | Nombre |
|---|---|---|
| $(a, b)$ | $a < x < b$ | abierto |
| $[a, b]$ | $a \le x \le b$ | cerrado |
| $[a, b)$ | $a \le x < b$ | semiabierto |
| $(a, +\infty)$ | $x > a$ | semirrecta |
| $(-\infty, b]$ | $x \le b$ | semirrecta |

El infinito nunca lleva corchete: no es un número.

## Potencias y raíces

$$a^m a^n = a^{m+n} \qquad \frac{a^m}{a^n} = a^{m-n} \qquad (a^m)^n = a^{mn} \qquad (ab)^n = a^n b^n$$

$$a^0 = 1 \qquad a^{-n} = \frac{1}{a^n} \qquad a^{m/n} = \sqrt[n]{a^m}$$

> [!WARNING]
> La potencia **no** se distribuye en la suma: $(a + b)^2 \neq a^2 + b^2$. Lo correcto es $(a+b)^2 = a^2 + 2ab + b^2$. Tampoco $\sqrt{a + b} = \sqrt{a} + \sqrt{b}$.

Además, $\sqrt{x^2} = |x|$, no $x$.

**Racionalizar** es eliminar raíces del denominador multiplicando por el conjugado:

$$\frac{1}{\sqrt{3} - 1} = \frac{1}{\sqrt{3} - 1} \cdot \frac{\sqrt{3} + 1}{\sqrt{3} + 1} = \frac{\sqrt{3} + 1}{2}$$

## Números complejos

La ecuación $x^2 = -1$ no tiene solución real. Se define la **unidad imaginaria** $i$ con $i^2 = -1$.

> [!IMPORTANT]
> Un número complejo es $z = a + bi$, con $a, b \in \mathbb{R}$. $a$ es la **parte real** y $b$ la **parte imaginaria**.

Las potencias de $i$ se repiten cada cuatro: $i^1 = i$, $i^2 = -1$, $i^3 = -i$, $i^4 = 1$.

### Operaciones en forma binómica

- **Suma**: $(a + bi) + (c + di) = (a + c) + (b + d)i$
- **Producto**: $(a + bi)(c + di) = (ac - bd) + (ad + bc)i$
- **Conjugado**: $\bar{z} = a - bi$, y se cumple $z \bar{z} = a^2 + b^2$
- **División**: se multiplica y divide por el conjugado del denominador.

> [!NOTE]
> Calcular $\dfrac{3 + 2i}{1 - i}$.
>
> $$\frac{(3 + 2i)(1 + i)}{(1 - i)(1 + i)} = \frac{3 + 3i + 2i + 2i^2}{1 + 1} = \frac{1 + 5i}{2} = \frac{1}{2} + \frac{5}{2}i$$

### Forma polar

Un complejo es un punto $(a, b)$ del plano. Se lo puede describir con su distancia al origen y su ángulo:

$$|z| = r = \sqrt{a^2 + b^2} \qquad \tan\theta = \frac{b}{a}$$

$$z = r(\cos\theta + i \sin\theta) = r e^{i\theta}$$

La última igualdad usa la **fórmula de Euler**: $e^{i\theta} = \cos\theta + i\sin\theta$. De ella sale $e^{i\pi} + 1 = 0$.

En forma polar, multiplicar es fácil: **se multiplican los módulos y se suman los ángulos**.

$$z_1 z_2 = r_1 r_2 \, e^{i(\theta_1 + \theta_2)} \qquad \frac{z_1}{z_2} = \frac{r_1}{r_2} e^{i(\theta_1 - \theta_2)}$$

### Potencias y raíces

**Fórmula de De Moivre**:

$$z^n = r^n (\cos n\theta + i \sin n\theta)$$

Todo complejo no nulo tiene exactamente $n$ **raíces enésimas**:

$$\sqrt[n]{z} = \sqrt[n]{r} \left[ \cos\frac{\theta + 2k\pi}{n} + i\sin\frac{\theta + 2k\pi}{n} \right], \quad k = 0, 1, \dots, n-1$$

En el plano son los vértices de un polígono regular de $n$ lados.

> [!NOTE]
> Raíces cúbicas de $8$. Aquí $r = 8$, $\theta = 0$. Módulo: $\sqrt[3]{8} = 2$. Ángulos: $0°$, $120°$, $240°$.
>
> $$2, \qquad -1 + \sqrt{3}\,i, \qquad -1 - \sqrt{3}\,i$$

> [!TIP]
> Forma binómica para sumar y restar; forma polar para multiplicar, dividir, potenciar y sacar raíces. En ingeniería eléctrica los complejos representan tensiones y corrientes alternas (fasores).
