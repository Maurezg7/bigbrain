+++
title = "Inferencia estadística"
summary = "Estimación puntual, intervalos de confianza, pruebas de hipótesis, valor p y errores de tipo I y II."
branch = "Probabilidad y estadística"
order = 4
level = "Avanzado"
tags = ["intervalo de confianza", "prueba de hipótesis", "valor p", "t de Student"]
+++

## De la muestra a la población

La **inferencia** usa una muestra para sacar conclusiones sobre toda la población, midiendo además cuánta incertidumbre tienen.

| Parámetro (población) | Estimador (muestra) |
|---|---|
| Media $\mu$ | $\bar{x}$ |
| Desviación estándar $\sigma$ | $s$ |
| Proporción $p$ | $\hat{p}$ |

Un buen estimador es **insesgado** (en promedio acierta) y de **baja varianza**.

La muestra debe ser **aleatoria**: una muestra sesgada no se arregla con más datos.

## Intervalos de confianza

Una estimación puntual casi nunca coincide con el valor real. Un **intervalo de confianza** da un rango de valores plausibles:

$$\text{estimador} \pm (\text{valor crítico}) \times (\text{error estándar})$$

### Para la media

> [!IMPORTANT]
> $$\bar{x} \pm t_{\alpha/2,\,n-1}\,\frac{s}{\sqrt{n}}$$

Se usa la distribución **$t$ de Student** con $n - 1$ grados de libertad porque $\sigma$ se estima con $s$. Para $n$ grande, $t$ se aproxima a $z$.

| Confianza | $z_{\alpha/2}$ |
|---|---|
| 90 % | 1,645 |
| 95 % | 1,960 |
| 99 % | 2,576 |

> [!NOTE]
> En 36 mediciones de resistencia se obtuvo $\bar{x} = 250$ MPa y $s = 12$ MPa. Intervalo del 95 % (con $t \approx 2{,}03$):
>
> $250 \pm 2{,}03\,\dfrac{12}{6} = 250 \pm 4{,}1$, es decir $(245{,}9;\ 254{,}1)$ MPa.

### Para una proporción

$$\hat{p} \pm z_{\alpha/2}\sqrt{\frac{\hat{p}(1 - \hat{p})}{n}}$$

> [!NOTE]
> En una encuesta a 400 personas, 180 apoyan una medida: $\hat{p} = 0{,}45$.
>
> $0{,}45 \pm 1{,}96\sqrt{\dfrac{0{,}45 \cdot 0{,}55}{400}} = 0{,}45 \pm 0{,}049$. Margen de error: $\pm 4{,}9$ puntos.

### Tamaño de muestra

Para un margen de error $E$ al estimar una media: $n = \left(\dfrac{z_{\alpha/2}\,\sigma}{E}\right)^2$.

> [!WARNING]
> «95 % de confianza» **no** significa que haya 95 % de probabilidad de que $\mu$ esté en ese intervalo. Significa que el **método** acierta el 95 % de las veces que se lo aplica.

El intervalo se angosta al aumentar $n$ y se ensancha al pedir más confianza.

## Pruebas de hipótesis

Sirven para decidir si los datos son compatibles con una afirmación.

1. **Hipótesis nula** $H_0$: la afirmación de «no hay efecto» ($\mu = \mu_0$).
2. **Hipótesis alternativa** $H_1$: lo que se quiere demostrar ($\mu \neq \mu_0$, $\mu > \mu_0$ o $\mu < \mu_0$).
3. Elegir el **nivel de significación** $\alpha$ (habitualmente $0{,}05$).
4. Calcular el **estadístico de prueba**.
5. Hallar el **valor p** y decidir.

Para una media:

$$t = \frac{\bar{x} - \mu_0}{s/\sqrt{n}}$$

### Valor p

> [!IMPORTANT]
> El **valor p** es la probabilidad de obtener un resultado tan extremo como el observado (o más) **si $H_0$ fuera cierta**.
>
> Si $p < \alpha$, se **rechaza** $H_0$: el resultado es estadísticamente significativo.

> [!NOTE]
> Un fabricante afirma que sus lámparas duran $\mu = 1000$ h. En una muestra de 25: $\bar{x} = 980$ h, $s = 40$ h. ¿Duran menos?
>
> $H_0: \mu = 1000$; $H_1: \mu < 1000$.
>
> $t = \dfrac{980 - 1000}{40/5} = -2{,}5$. Con 24 grados de libertad, $p \approx 0{,}01 < 0{,}05$.
>
> Se rechaza $H_0$: hay evidencia de que duran menos de lo declarado.

### Errores

| | $H_0$ es verdadera | $H_0$ es falsa |
|---|---|---|
| Se rechaza $H_0$ | **error de tipo I** (prob. $\alpha$) | decisión correcta (potencia $1 - \beta$) |
| No se rechaza $H_0$ | decisión correcta | **error de tipo II** (prob. $\beta$) |

- Tipo I: falso positivo. Tipo II: falso negativo.
- Bajar $\alpha$ aumenta $\beta$. Para reducir ambos hay que aumentar $n$.

> [!WARNING]
> «No rechazar $H_0$» no es «demostrar $H_0$»: solo significa que no hubo evidencia suficiente.
>
> Y el valor p **no** es la probabilidad de que $H_0$ sea cierta.

## Pruebas más usadas

| Situación | Prueba |
|---|---|
| Una media | $t$ de una muestra |
| Dos medias, grupos independientes | $t$ de dos muestras |
| Antes y después en los mismos sujetos | $t$ para datos apareados |
| Tres o más medias | ANOVA |
| Una o dos proporciones | prueba $z$ |
| Asociación entre variables categóricas | chi-cuadrado ($\chi^2$) |
| Ajuste a una distribución teórica | chi-cuadrado de bondad de ajuste |

### Chi-cuadrado

Compara frecuencias observadas ($O$) con las esperadas ($E$) bajo $H_0$:

$$\chi^2 = \sum\frac{(O - E)^2}{E}$$

Un valor grande indica que los datos se apartan de lo esperado.

## Significación estadística y práctica

Con una muestra enorme, una diferencia insignificante puede dar $p < 0{,}05$. Por eso se informa también el **tamaño del efecto** y el intervalo de confianza: indican si la diferencia importa en la práctica.

> [!TIP]
> Un intervalo de confianza dice más que un valor p: muestra la magnitud del efecto y su incertidumbre a la vez.
