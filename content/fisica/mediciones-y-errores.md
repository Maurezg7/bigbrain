+++
title = "Mediciones, unidades y errores"
summary = "Sistema Internacional, análisis dimensional, cifras significativas, errores sistemáticos y aleatorios, propagación de incertidumbres y ajuste por mínimos cuadrados."
branch = "Mecánica"
order = 0
level = "Básico"
tags = ["mediciones", "unidades", "incertidumbre", "propagación de errores", "mínimos cuadrados", "laboratorio"]
+++

## Medir

**Medir** es comparar una magnitud con otra de la misma clase tomada como **unidad**. Todo resultado tiene tres partes:

$$x = (\text{valor} \pm \text{incertidumbre})\ \text{unidad}$$

Un número sin unidad no significa nada, y sin incertidumbre no se sabe cuánto confiar en él.

## Sistema Internacional

Tiene siete **unidades de base**, definidas desde 2019 a partir de constantes de la naturaleza.

| Magnitud | Unidad | Símbolo |
|---|---|---|
| Longitud | metro | m |
| Masa | kilogramo | kg |
| Tiempo | segundo | s |
| Corriente eléctrica | ampere | A |
| Temperatura | kelvin | K |
| Cantidad de sustancia | mol | mol |
| Intensidad luminosa | candela | cd |

Las demás son **derivadas**:

| Magnitud | Unidad | En unidades de base |
|---|---|---|
| Fuerza | newton (N) | kg·m/s² |
| Energía | joule (J) | kg·m²/s² |
| Potencia | watt (W) | kg·m²/s³ |
| Presión | pascal (Pa) | kg/(m·s²) |
| Frecuencia | hertz (Hz) | 1/s |
| Carga | coulomb (C) | A·s |
| Tensión | volt (V) | kg·m²/(A·s³) |

### Prefijos

| Prefijo | Símbolo | Factor | | Prefijo | Símbolo | Factor |
|---|---|---|---|---|---|---|
| tera | T | $10^{12}$ | | mili | m | $10^{-3}$ |
| giga | G | $10^{9}$ | | micro | µ | $10^{-6}$ |
| mega | M | $10^{6}$ | | nano | n | $10^{-9}$ |
| kilo | k | $10^{3}$ | | pico | p | $10^{-12}$ |

### Conversión de unidades

Se multiplica por **factores unitarios**, cocientes que valen 1.

> [!NOTE]
> $$90\ \frac{\text{km}}{\text{h}} \times \frac{1000\ \text{m}}{1\ \text{km}} \times \frac{1\ \text{h}}{3600\ \text{s}} = 25\ \frac{\text{m}}{\text{s}}$$
>
> Con potencias, el factor se eleva: $1\ \text{m}^3 = (100\ \text{cm})^3 = 10^6\ \text{cm}^3$.

## Análisis dimensional

Toda magnitud mecánica se expresa con las dimensiones de longitud $[L]$, masa $[M]$ y tiempo $[T]$.

| Magnitud | Dimensión |
|---|---|
| Velocidad | $L\,T^{-1}$ |
| Aceleración | $L\,T^{-2}$ |
| Fuerza | $M\,L\,T^{-2}$ |
| Energía | $M\,L^2\,T^{-2}$ |

> [!IMPORTANT]
> Una ecuación física debe ser **dimensionalmente homogénea**: todos sus términos tienen la misma dimensión. Si no lo es, está mal.

> [!NOTE]
> **Deducir una fórmula.** El período $T$ de un péndulo podría depender de la longitud $l$, la masa $m$ y la gravedad $g$: $T = k\, l^a m^b g^c$.
> $$[T] = L^a M^b (L\,T^{-2})^c = L^{a + c}\, M^b\, T^{-2c}$$
> Igualando exponentes: $b = 0$, $c = -\tfrac{1}{2}$, $a = \tfrac{1}{2}$.
> $$T = k \sqrt{\frac{l}{g}}$$
> El análisis muestra que el período no depende de la masa. La constante $k = 2\pi$ no puede obtenerse así.

## Cifras significativas

Son los dígitos que aportan información: los seguros más uno dudoso.

| Número | Cifras significativas | Observación |
|---|---|---|
| 25,3 | 3 | |
| 0,0025 | 2 | los ceros a la izquierda no cuentan |
| 2,50 | 3 | el cero final sí cuenta |
| 1500 | ambiguo | mejor $1{,}5 \times 10^3$ (2) o $1{,}500 \times 10^3$ (4) |

| Operación | Regla |
|---|---|
| Multiplicación y división | tantas cifras significativas como el dato que menos tiene |
| Suma y resta | tantos **decimales** como el dato que menos tiene |

> [!NOTE]
> $2{,}5 \times 3{,}42 = 8{,}55 \to 8{,}6$ (dos cifras).
>
> $12{,}11 + 0{,}3 = 12{,}41 \to 12{,}4$ (un decimal).

Se redondea solo el resultado **final**: en los pasos intermedios se conservan cifras de más.

## Tipos de error

El **error** no es una equivocación: es la diferencia inevitable entre lo medido y el valor verdadero, que nunca se conoce exactamente.

| Tipo | Causa | Efecto | Cómo se trata |
|---|---|---|---|
| **Sistemático** | instrumento descalibrado, error de cero, método defectuoso | desvía todas las medidas en el mismo sentido | calibrar, corregir, cambiar de método |
| **Aleatorio** | fluctuaciones imprevisibles | dispersa las medidas alrededor de un valor | repetir y promediar |
| Grosero | lectura o anotación equivocada | un dato absurdo | descartarlo |

| Cualidad | Significa | Depende de los errores |
|---|---|---|
| **Exactitud** | cercanía al valor verdadero | sistemáticos |
| **Precisión** | cercanía de las medidas entre sí | aleatorios |

Un instrumento puede ser muy preciso e inexacto: repetir muchas veces **no** elimina un error sistemático.

El **error de paralaje** aparece al leer una escala desde un ángulo: hay que mirar de frente.

## Incertidumbre de una medición

### Una sola medida

La incertidumbre la da la **apreciación** del instrumento: la menor división de su escala (o la mitad, si se puede estimar).

| Instrumento | Apreciación típica |
|---|---|
| Regla | 1 mm |
| Calibre | 0,05 mm o 0,02 mm |
| Micrómetro | 0,01 mm |
| Cronómetro manual | 0,01 s, pero el tiempo de reacción agrega unos 0,2 s |
| Multímetro digital | según el manual: un porcentaje de la lectura más algunos dígitos |

### Varias medidas

Con $N$ medidas, el mejor valor es el **promedio**:

$$\bar{x} = \frac{1}{N} \sum x_i$$

La dispersión se mide con la **desviación estándar**:

$$s = \sqrt{\frac{\sum (x_i - \bar{x})^2}{N - 1}}$$

Y la incertidumbre del promedio es el **error estándar**:

$$\Delta x = \frac{s}{\sqrt{N}}$$

Para reducirla a la mitad hay que cuadruplicar las mediciones. La incertidumbre final combina la estadística con la del instrumento:

$$\Delta x = \sqrt{\Delta x_{\text{est}}^2 + \Delta x_{\text{inst}}^2}$$

> [!NOTE]
> **Ejemplo.** Cinco medidas del período de un péndulo, en segundos: 2,01; 1,98; 2,03; 2,00; 1,98.
>
> $\bar{T} = 10{,}00 / 5 = 2{,}000$ s
>
> Desvíos al cuadrado: 0,0001; 0,0004; 0,0009; 0; 0,0004. Suma: 0,0018.
>
> $s = \sqrt{0{,}0018 / 4} = 0{,}021$ s
>
> $\Delta T = 0{,}021 / \sqrt{5} = 0{,}009$ s
>
> Resultado: $T = (2{,}000 \pm 0{,}009)$ s.

### Cómo se informa

- La incertidumbre se expresa con **una** cifra significativa (dos si la primera es 1).
- El valor se redondea a la **misma posición decimal** que la incertidumbre.

| Incorrecto | Correcto |
|---|---|
| $9{,}8143 \pm 0{,}0372$ | $9{,}81 \pm 0{,}04$ |
| $1520 \pm 87$ | $(1{,}52 \pm 0{,}09) \times 10^3$ |
| $3{,}2 \pm 0{,}015$ | $3{,}200 \pm 0{,}015$ |

| Incertidumbre | Definición |
|---|---|
| Absoluta | $\Delta x$, con las unidades de $x$ |
| Relativa | $\Delta x / \lvert x \rvert$, adimensional |
| Porcentual | la relativa por 100 |

La relativa permite comparar la calidad de mediciones distintas: 1 mm de incertidumbre es excelente al medir 10 m y pésimo al medir 5 mm.

## Propagación de incertidumbres

Cuando una magnitud se **calcula** a partir de otras medidas, sus incertidumbres se propagan. Para $f(x, y, \dots)$ con variables independientes:

$$\Delta f = \sqrt{\left( \frac{\partial f}{\partial x} \Delta x \right)^2 + \left( \frac{\partial f}{\partial y} \Delta y \right)^2 + \cdots}$$

| Operación | Regla |
|---|---|
| $f = x \pm y$ | $\Delta f = \sqrt{\Delta x^2 + \Delta y^2}$ (se combinan las **absolutas**) |
| $f = x \cdot y$ o $f = x / y$ | $\dfrac{\Delta f}{\lvert f \rvert} = \sqrt{\left( \dfrac{\Delta x}{x} \right)^2 + \left( \dfrac{\Delta y}{y} \right)^2}$ (se combinan las **relativas**) |
| $f = x^n$ | $\dfrac{\Delta f}{\lvert f \rvert} = \lvert n \rvert \dfrac{\Delta x}{\lvert x \rvert}$ |
| $f = c\,x$ | $\Delta f = \lvert c \rvert\, \Delta x$ |

> [!WARNING]
> Al **restar** dos valores parecidos, la incertidumbre relativa se dispara: $(10{,}0 \pm 0{,}1) - (9{,}5 \pm 0{,}1) = 0{,}5 \pm 0{,}14$, un 28 %.

> [!NOTE]
> **Ejemplo: medir $g$ con un péndulo.** De $T = 2\pi\sqrt{l/g}$:
> $$g = \frac{4\pi^2 l}{T^2}$$
> Con $l = (1{,}000 \pm 0{,}002)$ m y $T = (2{,}000 \pm 0{,}009)$ s:
> $$g = \frac{4\pi^2 \times 1{,}000}{2{,}000^2} = 9{,}870\ \text{m/s}^2$$
> $$\frac{\Delta g}{g} = \sqrt{\left( \frac{0{,}002}{1{,}000} \right)^2 + \left( 2 \times \frac{0{,}009}{2{,}000} \right)^2} = \sqrt{(0{,}002)^2 + (0{,}009)^2} = 0{,}0092$$
> $$\Delta g = 0{,}0092 \times 9{,}870 = 0{,}09\ \text{m/s}^2$$
> Resultado: $g = (9{,}87 \pm 0{,}09)\ \text{m/s}^2$.
>
> El período domina la incertidumbre, porque está al cuadrado. Para mejorarla conviene medir el tiempo de 20 oscilaciones y dividir: el error de reacción se reparte entre todas.

### Comparar resultados

Dos resultados son **compatibles** si su diferencia es menor que unas dos veces la incertidumbre combinada:

$$\lvert x_1 - x_2 \rvert \lesssim 2 \sqrt{\Delta x_1^2 + \Delta x_2^2}$$

El valor $g = 9{,}87 \pm 0{,}09$ es compatible con el aceptado de 9,80: difieren en menos de una incertidumbre.

## Gráficos y ajuste lineal

Un gráfico revela la relación entre dos magnitudes mejor que una tabla.

- La variable **independiente** va en el eje horizontal.
- Ejes rotulados con magnitud y unidad.
- Los puntos llevan **barras de error**.
- No se unen los puntos con segmentos: se traza la curva que mejor los representa.

### Mínimos cuadrados

La recta $y = mx + b$ que mejor ajusta $N$ puntos minimiza la suma de los cuadrados de las diferencias verticales:

$$m = \frac{N \sum x_i y_i - \sum x_i \sum y_i}{N \sum x_i^2 - \left( \sum x_i \right)^2} \qquad b = \frac{\sum y_i - m \sum x_i}{N}$$

La pendiente y la ordenada suelen tener significado físico.

> [!NOTE]
> **Ley de Hooke.** Se cuelgan pesos de un resorte y se mide el estiramiento:
>
> | $x$ (m) | 0,020 | 0,041 | 0,059 | 0,082 | 0,100 |
> |---|---|---|---|---|---|
> | $F$ (N) | 0,49 | 0,98 | 1,47 | 1,96 | 2,45 |
>
> El ajuste da $F = 24{,}3\,x - 0{,}001$. La pendiente es la constante del resorte: $k \approx 24$ N/m.

### Linealización

Si la relación no es lineal, se cambian las variables para obtener una recta.

| Relación | Se grafica | Pendiente | Ordenada |
|---|---|---|---|
| $y = a x^2$ | $y$ en función de $x^2$ | $a$ | 0 |
| $y = a / x$ | $y$ en función de $1/x$ | $a$ | 0 |
| $y = a x^n$ | $\log y$ en función de $\log x$ | $n$ | $\log a$ |
| $y = a e^{bx}$ | $\ln y$ en función de $x$ | $b$ | $\ln a$ |

> [!NOTE]
> Para el péndulo, $T^2 = \dfrac{4\pi^2}{g}\, l$. Graficando $T^2$ en función de $l$ se obtiene una recta cuya pendiente da $g = 4\pi^2 / m$. Usar muchos largos distintos es mejor que repetir uno solo: pone a prueba el modelo y promedia los errores.

## El informe de laboratorio

| Sección | Contenido |
|---|---|
| Objetivo | qué se quiere medir o verificar |
| Fundamento | el modelo y las ecuaciones |
| Materiales y método | instrumentos, con su apreciación; procedimiento |
| Datos | tablas con unidades e incertidumbres |
| Análisis | cálculos, gráficos, propagación |
| Resultados | valor ± incertidumbre |
| Discusión | comparación con lo esperado; fuentes de error; mejoras |
| Conclusión | si se cumplió el objetivo |

> [!TIP]
> Antes de medir, estimar qué magnitud aporta más incertidumbre al resultado (la de mayor error relativo, sobre todo si está elevada a una potencia) y concentrar ahí el esfuerzo. Mejorar las demás no cambia nada.
