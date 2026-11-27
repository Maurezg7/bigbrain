+++
title = "Cadenas de Markov y procesos estocásticos"
summary = "Propiedad de Markov, matriz de transición, clasificación de estados, distribución estacionaria, cadenas absorbentes, proceso de Poisson y colas M/M/1."
branch = "Probabilidad y estadística"
order = 6
level = "Avanzado"
tags = ["cadenas de Markov", "matriz de transición", "distribución estacionaria", "proceso de Poisson", "teoría de colas"]
+++

## Procesos estocásticos

Un **proceso estocástico** es una familia de variables aleatorias $X_t$ indexadas por el tiempo: describe un sistema que evoluciona al azar.

| | Tiempo discreto | Tiempo continuo |
|---|---|---|
| **Estados discretos** | cadena de Markov | proceso de Poisson, colas |
| **Estados continuos** | series temporales | movimiento browniano |

## Propiedad de Markov

Una **cadena de Markov** es un proceso en el que el futuro depende solo del **estado presente**, no de cómo se llegó a él:

$$P(X_{n+1} = j \mid X_n = i, X_{n-1}, \dots, X_0) = P(X_{n+1} = j \mid X_n = i)$$

Se dice que el proceso **no tiene memoria**.

Si esas probabilidades no cambian con el tiempo, la cadena es **homogénea** y queda descrita por las **probabilidades de transición**:

$$p_{ij} = P(X_{n+1} = j \mid X_n = i)$$

## Matriz de transición

Las $p_{ij}$ se ordenan en la matriz $P$: la fila $i$ contiene las probabilidades de ir desde $i$ a cada estado.

> [!IMPORTANT]
> $P$ es una **matriz estocástica**: sus elementos son no negativos y **cada fila suma 1**.

> [!NOTE]
> **Ejemplo: el clima.** Estados: soleado (S) y lluvioso (L). Tras un día soleado, el siguiente es soleado con probabilidad 0,8; tras uno lluvioso, llueve de nuevo con probabilidad 0,6.
>
> $$P = \begin{pmatrix} 0{,}8 & 0{,}2 \\ 0{,}4 & 0{,}6 \end{pmatrix}$$
>
> ```text
>         0,2
>    ┌──────────▶┐
>  ( S )        ( L )
>    └◀──────────┘
>         0,4
>  S→S: 0,8      L→L: 0,6
> ```

### Varios pasos

La distribución en el instante $n$ es un vector fila $\boldsymbol{\pi}^{(n)}$. Evoluciona así:

$$\boldsymbol{\pi}^{(n+1)} = \boldsymbol{\pi}^{(n)} P \qquad\Rightarrow\qquad \boldsymbol{\pi}^{(n)} = \boldsymbol{\pi}^{(0)} P^n$$

El elemento $(i, j)$ de $P^n$ es la probabilidad de pasar de $i$ a $j$ en exactamente $n$ pasos. La relación $P^{m+n} = P^m P^n$ es la **ecuación de Chapman-Kolmogorov**.

> [!NOTE]
> **Dos días después.**
> $$P^2 = \begin{pmatrix} 0{,}72 & 0{,}28 \\ 0{,}56 & 0{,}44 \end{pmatrix}$$
> Si hoy está soleado, pasado mañana lo estará con probabilidad 0,72 (por S→S→S, $0{,}64$, más S→L→S, $0{,}08$).

## Clasificación de estados

| Concepto | Definición |
|---|---|
| $j$ es **accesible** desde $i$ | se puede llegar de $i$ a $j$ con probabilidad positiva |
| $i$ y $j$ se **comunican** | cada uno es accesible desde el otro |
| Cadena **irreducible** | todos los estados se comunican |
| Estado **recurrente** | se vuelve a él con probabilidad 1 |
| Estado **transitorio** | hay probabilidad positiva de no volver nunca |
| Estado **absorbente** | una vez que se entra, no se sale: $p_{ii} = 1$ |
| **Período** de un estado | máximo común divisor de las cantidades de pasos en que se puede volver |
| Estado **aperiódico** | de período 1 |

En una cadena finita e irreducible, todos los estados son recurrentes. Una cadena irreducible y aperiódica se llama **ergódica**.

> [!NOTE]
> La cadena con $P = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix}$ alterna entre dos estados: tiene período 2 y sus potencias no convergen.

## Distribución estacionaria

Es una distribución $\boldsymbol{\pi}$ que no cambia al avanzar un paso:

$$\boldsymbol{\pi} = \boldsymbol{\pi} P \qquad \sum_i \pi_i = 1$$

Es un **autovector por izquierda** de $P$ con autovalor 1.

> [!NOTE]
> **Para el clima.** Con $\boldsymbol{\pi} = (\pi_S, \pi_L)$:
> $$\pi_S = 0{,}8\,\pi_S + 0{,}4\,\pi_L \quad\Rightarrow\quad 0{,}2\,\pi_S = 0{,}4\,\pi_L \quad\Rightarrow\quad \pi_S = 2\pi_L$$
> Con $\pi_S + \pi_L = 1$:
> $$\boldsymbol{\pi} = \left( \tfrac{2}{3},\ \tfrac{1}{3} \right)$$
> A largo plazo, dos de cada tres días son soleados.

> [!IMPORTANT]
> **Teorema de convergencia.** Si la cadena es finita, irreducible y aperiódica, existe una única distribución estacionaria y, **cualquiera sea el estado inicial**,
> $$\lim_{n \to \infty} P^n = \begin{pmatrix} \boldsymbol{\pi} \\ \vdots \\ \boldsymbol{\pi} \end{pmatrix}$$

Interpretaciones de $\pi_i$:

- la probabilidad de hallar al sistema en $i$ después de mucho tiempo;
- la **fracción del tiempo** que el sistema pasa en $i$;
- el tiempo medio de retorno a $i$ es $1/\pi_i$.

La velocidad de convergencia la da el segundo autovalor de mayor módulo. En el ejemplo, los autovalores son 1 y 0,4: el error se multiplica por 0,4 en cada paso.

### Balance detallado

Si existe $\boldsymbol{\pi}$ con

$$\pi_i\, p_{ij} = \pi_j\, p_{ji} \quad \text{para todo } i, j$$

entonces $\boldsymbol{\pi}$ es estacionaria y la cadena es **reversible**. Es una condición más fuerte y a menudo más fácil de verificar.

## Cadenas absorbentes

Una cadena es **absorbente** si tiene al menos un estado absorbente, y desde todo estado se puede llegar a alguno. Ordenando primero los $t$ estados transitorios:

$$P = \begin{pmatrix} Q & R \\ 0 & I \end{pmatrix}$$

$Q$ contiene las transiciones entre transitorios y $R$, las de transitorios a absorbentes. La **matriz fundamental** es

$$N = (I - Q)^{-1}$$

| Cantidad | Fórmula |
|---|---|
| Número esperado de visitas a $j$ partiendo de $i$ | $N_{ij}$ |
| Número esperado de pasos hasta la absorción | $\mathbf{t} = N \mathbf{1}$ |
| Probabilidad de terminar en cada estado absorbente | $B = N R$ |

> [!NOTE]
> **La ruina del jugador.** Un jugador tiene \$1 y apuesta \$1 por vez, con probabilidad ½ de ganar. Se retira al llegar a \$3 o al quedar en \$0. Estados transitorios: 1 y 2. Absorbentes: 0 y 3.
>
> $$Q = \begin{pmatrix} 0 & ½ \\ ½ & 0 \end{pmatrix} \qquad R = \begin{pmatrix} ½ & 0 \\ 0 & ½ \end{pmatrix}$$
>
> $$N = (I - Q)^{-1} = \frac{4}{3} \begin{pmatrix} 1 & ½ \\ ½ & 1 \end{pmatrix} = \begin{pmatrix} 4/3 & 2/3 \\ 2/3 & 4/3 \end{pmatrix}$$
>
> $$B = N R = \begin{pmatrix} 2/3 & 1/3 \\ 1/3 & 2/3 \end{pmatrix}$$
>
> Empezando con \$1: se arruina con probabilidad $2/3$ y llega a \$3 con probabilidad $1/3$. El juego dura en promedio $4/3 + 2/3 = 2$ apuestas.

En general, con juego equilibrado, capital inicial $i$ y meta $M$, la probabilidad de alcanzar la meta es $i/M$.

## Paseo aleatorio

En cada paso, la posición aumenta o disminuye en 1, con probabilidades $p$ y $1 - p$.

| Propiedad (caso simétrico, $p = ½$) | |
|---|---|
| Posición esperada tras $n$ pasos | 0 |
| Varianza | $n$ |
| Distancia típica al origen | $\sqrt{n}$ |
| Retorno al origen | seguro en 1 y 2 dimensiones; no en 3 o más |

Es el modelo básico de la **difusión**, y en el límite continuo da el **movimiento browniano**.

## Proceso de Poisson

Cuenta sucesos que ocurren al azar en **tiempo continuo**, con tasa media $\lambda$ por unidad de tiempo: llamadas, llegadas de clientes, fallas, desintegraciones.

| Propiedad | Enunciado |
|---|---|
| Cantidad de sucesos en un intervalo $t$ | Poisson: $P(N(t) = k) = \dfrac{(\lambda t)^k e^{-\lambda t}}{k!}$ |
| Tiempo entre sucesos | exponencial de media $1/\lambda$ |
| Incrementos independientes | lo que pasa en intervalos disjuntos es independiente |
| Superposición | la suma de dos procesos de tasas $\lambda_1$ y $\lambda_2$ es otro de tasa $\lambda_1 + \lambda_2$ |

La distribución exponencial **no tiene memoria**: $P(T > s + t \mid T > s) = P(T > t)$. El tiempo que falta para el próximo suceso no depende de cuánto se lleva esperando.

> [!NOTE]
> **Ejemplo.** A un servidor llegan en promedio 3 peticiones por segundo.
>
> Probabilidad de que no llegue ninguna en un segundo: $e^{-3} = 0{,}050$.
>
> Probabilidad de que lleguen exactamente 2: $\dfrac{3^2 e^{-3}}{2!} = 0{,}224$.
>
> Probabilidad de esperar más de 0,5 s hasta la próxima: $e^{-3 \times 0{,}5} = 0{,}223$.

## Teoría de colas

La cola **M/M/1** tiene llegadas de Poisson con tasa $\lambda$, tiempos de servicio exponenciales con tasa $\mu$ y un solo servidor. Es una cadena de Markov en tiempo continuo, cuyo estado es la cantidad de clientes en el sistema.

El **factor de utilización** es

$$\rho = \frac{\lambda}{\mu}$$

La cola es estable solo si $\rho < 1$. En régimen estacionario:

| Cantidad | Fórmula |
|---|---|
| Probabilidad de $n$ clientes en el sistema | $\pi_n = (1 - \rho)\, \rho^n$ |
| Probabilidad de sistema vacío | $1 - \rho$ |
| Clientes en el sistema (media) | $L = \dfrac{\rho}{1 - \rho}$ |
| Clientes en la cola | $L_q = \dfrac{\rho^2}{1 - \rho}$ |
| Tiempo en el sistema | $W = \dfrac{1}{\mu - \lambda}$ |
| Tiempo en la cola | $W_q = \dfrac{\rho}{\mu - \lambda}$ |

La **ley de Little**, $L = \lambda W$, vale para cualquier sistema de colas estable.

> [!NOTE]
> **Ejemplo.** Llegan 8 clientes por hora y el servidor atiende 10 por hora: $\rho = 0{,}8$.
> $$L = \frac{0{,}8}{0{,}2} = 4 \text{ clientes} \qquad W = \frac{1}{10 - 8} = 0{,}5\ \text{h} = 30 \text{ min}$$
> Si las llegadas suben a 9 por hora ($\rho = 0{,}9$): $L = 9$ y $W = 60$ min.

> [!WARNING]
> Los tiempos de espera crecen de manera **explosiva** cuando $\rho$ se acerca a 1. Un sistema dimensionado para trabajar al 95 % de su capacidad tendrá colas enormes: hay que dejar margen.

## Aplicaciones

| Campo | Uso |
|---|---|
| Buscadores | **PageRank**: la importancia de una página es su probabilidad estacionaria en un paseo aleatorio por los enlaces |
| Estadística | **Montecarlo con cadenas de Markov**: construir una cadena cuya distribución estacionaria sea la que se quiere muestrear |
| Lenguaje | modelos de $n$-gramas: la palabra siguiente depende de las anteriores |
| Reconocimiento de voz, bioinformática | modelos ocultos de Markov |
| Confiabilidad | estados de funcionamiento y falla; disponibilidad |
| Finanzas | calificación crediticia, valuación de opciones |
| Redes y servidores | dimensionamiento con teoría de colas |
| Inteligencia artificial | procesos de decisión de Markov, base del aprendizaje por refuerzo |
| Física y química | difusión, cinética de reacciones |

```python
import numpy as np

P = np.array([[0.8, 0.2],
              [0.4, 0.6]])

print(np.linalg.matrix_power(P, 20)[0])          # [0.6667 0.3333]

# Distribución estacionaria: autovector por izquierda de autovalor 1
valores, vectores = np.linalg.eig(P.T)
pi = np.real(vectores[:, np.argmax(np.real(valores))])
print(pi / pi.sum())                             # [0.6667 0.3333]
```

> [!TIP]
> Para modelar con una cadena de Markov, la clave es elegir bien el **estado**: debe contener toda la información del pasado que influye en el futuro. Si no alcanza, se amplía el estado (por ejemplo, incluyendo los dos últimos días en lugar de uno).
