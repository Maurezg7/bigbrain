+++
title = "Sistemas de ecuaciones diferenciales"
summary = "Sistemas lineales, solución con autovalores, plano de fases, estabilidad de los puntos de equilibrio y sistemas no lineales."
branch = "Ecuaciones diferenciales"
order = 4
level = "Avanzado"
tags = ["sistemas de EDO", "plano de fases", "estabilidad", "autovalores"]
+++

## Varias incógnitas acopladas

Muchos fenómenos involucran varias magnitudes que se influyen entre sí: poblaciones de presas y depredadores, corrientes en mallas de un circuito, concentraciones en tanques conectados.

Un **sistema lineal** de primer orden con coeficientes constantes:

$$\begin{cases} x' = ax + by \\ y' = cx + dy \end{cases} \quad\Longleftrightarrow\quad \vec{x}\,' = A\vec{x}$$

### De orden superior a sistema

Toda ecuación de orden $n$ se convierte en un sistema de $n$ ecuaciones de primer orden. Para $y'' + 3y' + 2y = 0$, con $x_1 = y$, $x_2 = y'$:

$$\begin{cases} x_1' = x_2 \\ x_2' = -2x_1 - 3x_2 \end{cases}$$

Así trabajan los métodos numéricos y la teoría de control (variables de estado).

## Solución con autovalores

Probando $\vec{x} = \vec{v}e^{\lambda t}$ en $\vec{x}\,' = A\vec{x}$ resulta $A\vec{v} = \lambda\vec{v}$: $\lambda$ es autovalor y $\vec{v}$ autovector.

> [!IMPORTANT]
> Si $A$ tiene autovalores reales distintos:
> $$\vec{x}(t) = C_1\vec{v}_1e^{\lambda_1t} + C_2\vec{v}_2e^{\lambda_2t}$$

> [!NOTE]
> $\vec{x}\,' = \begin{pmatrix} 1 & 2 \\ 2 & 1 \end{pmatrix}\vec{x}$.
>
> $\lambda^2 - 2\lambda - 3 = 0 \Rightarrow \lambda = 3, -1$. Autovectores: $(1, 1)$ y $(1, -1)$.
>
> $$\vec{x}(t) = C_1\begin{pmatrix} 1 \\ 1 \end{pmatrix}e^{3t} + C_2\begin{pmatrix} 1 \\ -1 \end{pmatrix}e^{-t}$$

Con autovalores **complejos** $\alpha \pm \beta i$, las soluciones son oscilaciones $e^{\alpha t}\cos\beta t$ y $e^{\alpha t}\sin\beta t$.

En general, la solución es $\vec{x}(t) = e^{At}\vec{x}(0)$, con la **exponencial matricial** $e^{At} = I + At + \dfrac{(At)^2}{2!} + \dots$

## Plano de fases

En lugar de graficar $x(t)$ e $y(t)$ por separado, se dibuja la trayectoria $(x(t), y(t))$ en el plano $xy$. El tiempo queda implícito como el recorrido de la curva.

El origen es un **punto de equilibrio** (allí $\vec{x}\,' = \vec{0}$). Su tipo depende de los autovalores:

| Autovalores | Tipo | Estabilidad |
|---|---|---|
| Reales negativos | nodo | estable |
| Reales positivos | nodo | inestable |
| Reales de signo opuesto | punto silla | inestable |
| Complejos con parte real negativa | foco (espiral) | estable |
| Complejos con parte real positiva | foco (espiral) | inestable |
| Imaginarios puros | centro | órbitas cerradas |

> [!IMPORTANT]
> El equilibrio es **asintóticamente estable** si y solo si todos los autovalores tienen **parte real negativa**.

Para una matriz $2 \times 2$, con traza $T$ y determinante $D$: es estable si $T < 0$ y $D > 0$.

## Sistemas no lineales

$$\begin{cases} x' = f(x, y) \\ y' = g(x, y) \end{cases}$$

Rara vez tienen solución exacta. Se estudian cualitativamente:

1. Hallar los **equilibrios**: $f = 0$ y $g = 0$.
2. **Linealizar** cerca de cada uno con la matriz jacobiana:

$$J = \begin{pmatrix} f_x & f_y \\ g_x & g_y \end{pmatrix}$$

3. Los autovalores de $J$ en el equilibrio indican su tipo (salvo en el caso de centros, que no es concluyente).

### Modelo presa-depredador (Lotka-Volterra)

$$\begin{cases} x' = ax - bxy \\ y' = -cy + dxy \end{cases}$$

$x$ son las presas e $y$ los depredadores. Tiene un equilibrio en $\left(\dfrac{c}{d}, \dfrac{a}{b}\right)$, alrededor del cual las poblaciones oscilan en ciclos desfasados.

### El péndulo

$\ddot{\theta} + \dfrac{g}{L}\sin\theta = 0$ se escribe como $\theta' = \omega$, $\omega' = -\dfrac{g}{L}\sin\theta$.

- $(0, 0)$: péndulo colgando. Es un **centro**: equilibrio estable.
- $(\pi, 0)$: péndulo invertido. Es un **punto silla**: inestable.

## Métodos numéricos

Cuando no hay solución analítica se avanza paso a paso con un intervalo $h$.

**Método de Euler**:

$$\vec{x}_{n+1} = \vec{x}_n + h\,\vec{f}(t_n, \vec{x}_n)$$

Es simple pero poco preciso (error proporcional a $h$). El **Runge-Kutta de cuarto orden** evalúa la pendiente en cuatro puntos por paso y su error es proporcional a $h^4$; es el método estándar.

> [!NOTE]
> $y' = y$, $y(0) = 1$, con $h = 0{,}1$. Euler: $y_1 = 1 + 0{,}1(1) = 1{,}1$; $y_2 = 1{,}21$; $\dots$; $y_{10} = 1{,}1^{10} \approx 2{,}594$.
>
> El valor exacto es $e \approx 2{,}718$: un error del $4{,}6\,\%$.

## Aplicaciones

| Sistema | Variables |
|---|---|
| Masas acopladas por resortes | posiciones y velocidades |
| Circuitos con varias mallas | corrientes y tensiones |
| Tanques interconectados | cantidad de soluto en cada uno |
| Epidemias (modelo SIR) | susceptibles, infectados, recuperados |
| Reacciones químicas | concentraciones |
| Control automático | variables de estado |

> [!WARNING]
> Los sistemas no lineales de tres o más variables pueden ser **caóticos**: deterministas pero imposibles de predecir a largo plazo, porque una diferencia mínima en las condiciones iniciales crece exponencialmente.

> [!TIP]
> Antes de buscar una fórmula, preguntarse dónde están los equilibrios y si son estables. A menudo eso es todo lo que hace falta saber.
