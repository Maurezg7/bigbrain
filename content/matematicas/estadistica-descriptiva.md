+++
title = "Estadística descriptiva"
summary = "Tipos de variables, tablas de frecuencia, medidas de posición y dispersión, y gráficos para resumir datos."
branch = "Probabilidad y estadística"
order = 2
level = "Básico"
tags = ["estadística", "media", "desviación estándar", "histograma"]
+++

## Población y muestra

- **Población**: el conjunto completo que interesa estudiar.
- **Muestra**: el subconjunto que efectivamente se observa.
- **Parámetro**: número que describe a la población ($\mu$, $\sigma$). **Estadístico**: número calculado con la muestra ($\bar{x}$, $s$).

| Tipo de variable | Ejemplo |
|---|---|
| Cualitativa nominal | carrera, color |
| Cualitativa ordinal | nivel de satisfacción |
| Cuantitativa discreta | cantidad de materias aprobadas |
| Cuantitativa continua | altura, tiempo |

## Tablas de frecuencia

| Frecuencia | Significado |
|---|---|
| Absoluta $f_i$ | cuántas veces aparece el valor |
| Relativa $f_i/n$ | proporción del total |
| Acumulada $F_i$ | cuántos datos son menores o iguales |

Los datos continuos se agrupan en **intervalos de clase**; una regla práctica es usar cerca de $\sqrt{n}$ intervalos.

## Medidas de posición

> [!IMPORTANT]
> **Media**: $\bar{x} = \dfrac{1}{n}\displaystyle\sum_{i=1}^{n} x_i$

- **Mediana**: el valor central de los datos ordenados (o el promedio de los dos centrales si $n$ es par).
- **Moda**: el valor más frecuente.
- **Cuartiles** $Q_1$, $Q_2$, $Q_3$: dividen los datos ordenados en cuatro partes iguales. $Q_2$ es la mediana.
- **Percentil** $p$: deja por debajo el $p\,\%$ de los datos.

> [!NOTE]
> Sueldos (en miles): 300, 320, 350, 380, 400, 2000.
>
> Media: $625$. Mediana: $\dfrac{350 + 380}{2} = 365$.
>
> Un solo valor extremo arrastra la media; la mediana representa mejor al grupo.

> [!TIP]
> La media es sensible a valores atípicos; la mediana es **robusta**. En distribuciones asimétricas (ingresos, precios) conviene informar la mediana.

## Medidas de dispersión

Dos conjuntos con la misma media pueden ser muy distintos: $\{49, 50, 51\}$ y $\{0, 50, 100\}$.

| Medida | Fórmula |
|---|---|
| Rango | $x_{máx} - x_{mín}$ |
| Rango intercuartílico | $RIC = Q_3 - Q_1$ |
| Varianza muestral | $s^2 = \dfrac{1}{n - 1}\displaystyle\sum (x_i - \bar{x})^2$ |
| Desviación estándar | $s = \sqrt{s^2}$ |
| Coeficiente de variación | $CV = \dfrac{s}{\bar{x}}$ |

La desviación estándar tiene las mismas unidades que los datos. El $CV$ no tiene unidades y permite comparar la variabilidad de magnitudes distintas.

> [!WARNING]
> En una **muestra** se divide por $n - 1$ (corrección de Bessel), no por $n$. Dividir por $n$ subestima la variabilidad de la población.

> [!NOTE]
> Datos: 2, 4, 4, 6, 9. Media: $5$.
>
> Desvíos al cuadrado: $9, 1, 1, 1, 16$; suma $= 28$.
>
> $s^2 = \dfrac{28}{4} = 7$, $s \approx 2{,}65$.

### Regla empírica

Para datos con distribución aproximadamente normal:

| Intervalo | Proporción de datos |
|---|---|
| $\bar{x} \pm s$ | 68 % |
| $\bar{x} \pm 2s$ | 95 % |
| $\bar{x} \pm 3s$ | 99,7 % |

### Puntuación estándar

$$z = \frac{x - \bar{x}}{s}$$

Indica a cuántas desviaciones estándar está un dato de la media. Permite comparar valores de escalas distintas.

## Forma de la distribución

- **Simétrica**: media ≈ mediana.
- **Asimetría positiva** (cola a la derecha): media > mediana.
- **Asimetría negativa** (cola a la izquierda): media < mediana.
- **Curtosis**: mide cuán pesadas son las colas.

## Gráficos

| Gráfico | Para qué |
|---|---|
| Barras | variables cualitativas o discretas |
| Sectores | partes de un total (pocas categorías) |
| Histograma | distribución de una variable continua |
| Diagrama de caja | mediana, cuartiles y atípicos; comparar grupos |
| Dispersión | relación entre dos variables |
| Líneas | evolución en el tiempo |

### Diagrama de caja

Muestra cinco números: mínimo, $Q_1$, mediana, $Q_3$ y máximo. Los datos más allá de $Q_1 - 1{,}5\,RIC$ o $Q_3 + 1{,}5\,RIC$ se marcan como **atípicos**.

## Datos agrupados

Si solo se tiene la tabla de frecuencias, con marcas de clase $x_i$:

$$\bar{x} = \frac{\sum f_ix_i}{n} \qquad s^2 = \frac{\sum f_i(x_i - \bar{x})^2}{n - 1}$$

## Transformaciones lineales

Si $y = a + bx$: $\bar{y} = a + b\bar{x}$ y $s_y = |b|\,s_x$. Sumar una constante desplaza la media pero no cambia la dispersión.

> [!WARNING]
> Un gráfico puede engañar: ejes que no empiezan en cero, escalas distintas o áreas desproporcionadas exageran diferencias. Mirar siempre los ejes.

> [!TIP]
> Nunca informar un promedio solo. Un valor central sin una medida de dispersión dice muy poco.
