+++
title = "Variables aleatorias y distribuciones"
summary = "Esperanza y varianza, distribuciones binomial, de Poisson, exponencial y normal, y el teorema central del límite."
branch = "Probabilidad y estadística"
order = 3
level = "Intermedio"
tags = ["distribución normal", "binomial", "Poisson", "esperanza", "teorema central del límite"]
+++

## Variable aleatoria

Una **variable aleatoria** $X$ asigna un número a cada resultado de un experimento aleatorio.

| | Discreta | Continua |
|---|---|---|
| Valores | aislados (conteos) | un intervalo (mediciones) |
| Se describe con | función de probabilidad $p(x) = P(X = x)$ | densidad $f(x)$ |
| Condición | $\sum p(x) = 1$ | $\int f(x)\,dx = 1$ |
| Probabilidad de un intervalo | suma | área bajo la curva |

En una variable continua, $P(X = a) = 0$: solo los intervalos tienen probabilidad.

**Función de distribución acumulada**: $F(x) = P(X \le x)$.

## Esperanza y varianza

> [!IMPORTANT]
> $$E(X) = \mu = \sum x\,p(x) \quad\text{o}\quad \int x\,f(x)\,dx$$
> $$\text{Var}(X) = \sigma^2 = E[(X - \mu)^2] = E(X^2) - \mu^2$$

La **esperanza** es el promedio a largo plazo; la **desviación estándar** $\sigma$ mide la dispersión.

Propiedades:

$$E(aX + b) = aE(X) + b \qquad \text{Var}(aX + b) = a^2\,\text{Var}(X)$$

$$E(X + Y) = E(X) + E(Y)$$

Si $X$ e $Y$ son **independientes**: $\text{Var}(X + Y) = \text{Var}(X) + \text{Var}(Y)$.

> [!NOTE]
> Un juego paga 100 pesos con probabilidad $0{,}1$ y cuesta 15 pesos jugar.
>
> $E(\text{ganancia}) = 100(0{,}1) - 15 = -5$. En promedio se pierden 5 pesos por jugada.

## Distribuciones discretas

### Binomial

Cuenta los éxitos en $n$ ensayos independientes, cada uno con probabilidad $p$.

$$P(X = k) = \binom{n}{k}p^k(1 - p)^{n-k} \qquad \mu = np \qquad \sigma^2 = np(1 - p)$$

> [!NOTE]
> El 5 % de las piezas es defectuoso. En una muestra de 10, probabilidad de exactamente una defectuosa:
>
> $P(X = 1) = \binom{10}{1}(0{,}05)(0{,}95)^9 \approx 0{,}315$.

### Poisson

Cuenta sucesos en un intervalo de tiempo o espacio, cuando ocurren a una tasa media $\lambda$.

$$P(X = k) = \frac{e^{-\lambda}\lambda^k}{k!} \qquad \mu = \sigma^2 = \lambda$$

Ejemplos: llamadas por hora, fallas por kilómetro de cable. Aproxima a la binomial cuando $n$ es grande y $p$ pequeña, con $\lambda = np$.

> [!NOTE]
> Llegan en promedio 3 clientes por minuto. Probabilidad de que no llegue ninguno: $P(X = 0) = e^{-3} \approx 0{,}05$.

### Otras

| Distribución | Modela | Media |
|---|---|---|
| Bernoulli | un solo ensayo | $p$ |
| Geométrica | ensayos hasta el primer éxito | $1/p$ |
| Hipergeométrica | extracciones sin reposición | $nK/N$ |

## Distribuciones continuas

### Uniforme

Todos los valores de $[a, b]$ son igualmente probables: $f(x) = \dfrac{1}{b - a}$, $\mu = \dfrac{a + b}{2}$, $\sigma^2 = \dfrac{(b - a)^2}{12}$.

### Exponencial

Tiempo de espera hasta el próximo suceso de un proceso de Poisson.

$$f(x) = \lambda e^{-\lambda x}, \quad x \ge 0 \qquad P(X > t) = e^{-\lambda t} \qquad \mu = \frac{1}{\lambda}$$

No tiene memoria: el tiempo ya esperado no cambia la probabilidad de lo que falta. Se usa en confiabilidad.

### Normal

> [!IMPORTANT]
> $$f(x) = \frac{1}{\sigma\sqrt{2\pi}}\,e^{-\frac{(x - \mu)^2}{2\sigma^2}}$$

La «campana de Gauss»: simétrica alrededor de $\mu$, con ancho determinado por $\sigma$. Describe errores de medición, alturas, y toda magnitud que sea suma de muchos efectos pequeños.

Para calcular probabilidades se **estandariza**:

$$Z = \frac{X - \mu}{\sigma} \sim N(0, 1)$$

y se usa la tabla de la normal estándar.

| $z$ | $P(Z \le z)$ |
|---|---|
| 0 | 0,5000 |
| 1 | 0,8413 |
| 1,645 | 0,9500 |
| 1,96 | 0,9750 |
| 2 | 0,9772 |
| 3 | 0,9987 |

> [!NOTE]
> El diámetro de un eje es normal con $\mu = 20$ mm y $\sigma = 0{,}1$ mm. ¿Qué proporción supera $20{,}2$ mm?
>
> $z = \dfrac{20{,}2 - 20}{0{,}1} = 2$. $P(Z > 2) = 1 - 0{,}9772 = 0{,}0228$: el 2,3 %.

## Teorema central del límite

> [!IMPORTANT]
> Si $X_1, \dots, X_n$ son independientes con la misma distribución (de media $\mu$ y desviación $\sigma$), para $n$ grande la **media muestral** es aproximadamente normal:
> $$\bar{X} \approx N\left(\mu, \frac{\sigma}{\sqrt{n}}\right)$$

Vale **sea cual sea** la distribución original. En la práctica suele bastar $n \ge 30$.

Consecuencias:

- Explica por qué la normal aparece en todas partes.
- $\dfrac{\sigma}{\sqrt{n}}$ es el **error estándar**: para reducir el error a la mitad hay que cuadruplicar la muestra.
- Justifica la inferencia estadística.

**Ley de los grandes números**: al crecer $n$, la media muestral converge a $\mu$.

> [!WARNING]
> La ley de los grandes números no «compensa» resultados pasados. Tras cinco caras seguidas, la probabilidad de cara en una moneda justa sigue siendo $0{,}5$ (falacia del jugador).

> [!TIP]
> Para elegir distribución: ¿se cuentan éxitos en $n$ intentos? Binomial. ¿Sucesos en un intervalo? Poisson. ¿Tiempo de espera? Exponencial. ¿Suma o promedio de muchas cosas? Normal.
