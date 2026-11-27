+++
title = "Óptica geométrica"
summary = "Reflexión y refracción, ley de Snell, reflexión total, espejos, lentes delgadas, formación de imágenes e instrumentos ópticos."
branch = "Ondas y óptica"
order = 3
level = "Intermedio"
tags = ["reflexión", "refracción", "ley de Snell", "lentes", "espejos"]
+++

## El modelo de rayos

Cuando los objetos son mucho mayores que la longitud de onda, la luz se describe con **rayos** que viajan en línea recta en un medio homogéneo.

## Reflexión

> [!IMPORTANT]
> El ángulo de incidencia es igual al de reflexión: $\theta_i = \theta_r$ (medidos desde la **normal** a la superficie).

- **Reflexión especular**: en superficies lisas; forma imágenes.
- **Reflexión difusa**: en superficies rugosas; permite ver los objetos desde cualquier lado.

Un **espejo plano** forma una imagen virtual, derecha, del mismo tamaño y a la misma distancia detrás del espejo.

## Refracción

Al pasar de un medio a otro, la luz cambia de velocidad y se desvía. El **índice de refracción** es

$$n = \frac{c}{v}$$

| Medio | $n$ |
|---|---|
| Vacío / aire | 1,00 |
| Agua | 1,33 |
| Vidrio | ~1,5 |
| Diamante | 2,42 |

> [!IMPORTANT]
> **Ley de Snell**:
> $$n_1\sin\theta_1 = n_2\sin\theta_2$$

- Al entrar en un medio de **mayor** índice, el rayo se **acerca** a la normal.
- Al pasar a uno de menor índice, se aleja.

> [!NOTE]
> Un rayo incide desde el aire sobre agua con $45°$.
>
> $\sin\theta_2 = \dfrac{\sin 45°}{1{,}33} = 0{,}532 \Rightarrow \theta_2 = 32{,}1°$.

Por la refracción, una pileta parece menos profunda y una cuchara en un vaso parece quebrada.

### Reflexión total interna

Cuando la luz va de un medio de mayor índice a uno de menor, existe un **ángulo crítico**:

$$\sin\theta_c = \frac{n_2}{n_1}$$

Para ángulos mayores, **toda** la luz se refleja. Para agua-aire, $\theta_c = 48{,}8°$; para vidrio-aire, cerca de $42°$.

Es el principio de la **fibra óptica**: la luz queda atrapada en el núcleo y viaja kilómetros casi sin pérdidas. También explica el brillo del diamante.

### Dispersión

El índice de refracción depende levemente de la longitud de onda: el violeta se desvía más que el rojo. Por eso un **prisma** descompone la luz blanca, y las gotas de lluvia forman el **arco iris**.

## Espejos esféricos

| Espejo | Forma | Foco |
|---|---|---|
| Cóncavo | superficie reflectante hacia adentro | real, $f > 0$ |
| Convexo | hacia afuera | virtual, $f < 0$ |

La distancia focal es la mitad del radio de curvatura: $f = \dfrac{R}{2}$.

## Lentes delgadas

| Lente | Forma | Efecto | Foco |
|---|---|---|---|
| Convergente | más gruesa en el centro | junta los rayos | $f > 0$ |
| Divergente | más delgada en el centro | separa los rayos | $f < 0$ |

La **potencia** de una lente es $P = \dfrac{1}{f}$, en **dioptrías** si $f$ está en metros.

## Ecuación de espejos y lentes

> [!IMPORTANT]
> $$\frac{1}{s} + \frac{1}{s'} = \frac{1}{f} \qquad m = \frac{h'}{h} = -\frac{s'}{s}$$

$s$ es la distancia del objeto, $s'$ la de la imagen y $m$ el **aumento lateral**.

| Signo | Significado |
|---|---|
| $s' > 0$ | imagen **real** (se puede proyectar en una pantalla) |
| $s' < 0$ | imagen **virtual** (solo se ve mirando a través del sistema) |
| $m > 0$ | imagen derecha |
| $m < 0$ | imagen invertida |
| $\lvert m \rvert > 1$ | imagen ampliada |

Las imágenes reales se forman del lado al que **sale** la luz: frente al espejo, o detrás de la lente.

> [!NOTE]
> Un objeto a $30$ cm de una lente convergente de $f = 10$ cm.
>
> $\dfrac{1}{s'} = \dfrac{1}{10} - \dfrac{1}{30} = \dfrac{2}{30} \Rightarrow s' = 15$ cm. $m = -\dfrac{15}{30} = -0{,}5$.
>
> Imagen real, invertida y de la mitad del tamaño.

> [!NOTE]
> El mismo objeto a $5$ cm (más cerca que el foco):
>
> $\dfrac{1}{s'} = \dfrac{1}{10} - \dfrac{1}{5} = -\dfrac{1}{10} \Rightarrow s' = -10$ cm. $m = +2$.
>
> Imagen virtual, derecha y al doble: es una **lupa**.

### Lente convergente según la posición del objeto

| Objeto | Imagen | Uso |
|---|---|---|
| Más allá de $2f$ | real, invertida, reducida | cámara, ojo |
| En $2f$ | real, invertida, igual tamaño | |
| Entre $f$ y $2f$ | real, invertida, ampliada | proyector |
| En $f$ | en el infinito | faro, colimador |
| Entre $f$ y la lente | virtual, derecha, ampliada | lupa |

Una lente **divergente** y un espejo **convexo** siempre dan imagen virtual, derecha y reducida (espejos retrovisores y de seguridad).

### Trazado de rayos

Para una lente convergente, tres rayos principales localizan la imagen:

1. El que llega **paralelo** al eje sale pasando por el foco.
2. El que pasa por el **centro** no se desvía.
3. El que pasa por el **foco** sale paralelo al eje.

**Ecuación del constructor de lentes**:

$$\frac{1}{f} = (n - 1)\left(\frac{1}{R_1} - \frac{1}{R_2}\right)$$

## El ojo

La córnea y el cristalino forman una imagen real e invertida sobre la retina. El cristalino cambia su curvatura para enfocar (**acomodación**).

| Defecto | Problema | Corrección |
|---|---|---|
| Miopía | la imagen se forma delante de la retina; no se ve bien de lejos | lente divergente |
| Hipermetropía | se forma detrás; no se ve bien de cerca | lente convergente |
| Presbicia | el cristalino pierde elasticidad con la edad | lente convergente |
| Astigmatismo | córnea con curvatura desigual | lente cilíndrica |

## Instrumentos ópticos

| Instrumento | Aumento |
|---|---|
| Lupa | $M = \dfrac{25\ \text{cm}}{f}$ |
| Microscopio | producto de los aumentos del objetivo y el ocular |
| Telescopio | $M = -\dfrac{f_{objetivo}}{f_{ocular}}$ |

> [!WARNING]
> Hay varias convenciones de signos. La usada aquí: distancias positivas para objetos e imágenes reales. Mantener una sola en todo el problema.

> [!TIP]
> Antes de calcular, hacer el trazado de rayos: muestra enseguida si la imagen es real o virtual y evita errores de signo.
