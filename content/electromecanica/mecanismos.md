+++
title = "Mecanismos"
summary = "Pares cinemáticos, grados de libertad, cuadrilátero articulado, biela-manivela, levas y análisis de velocidades."
branch = "Mecanismos y tecnología mecánica"
order = 1
level = "Intermedio"
tags = ["mecanismos", "grados de libertad", "biela-manivela", "levas", "cinemática"]
+++

## Qué es un mecanismo

Un mecanismo es un conjunto de piezas rígidas (**eslabones**) unidas por articulaciones (**pares cinemáticos**) que transforma un movimiento en otro: giro en vaivén, giro continuo en intermitente, y así.

| Par | Movimiento que permite | Grados de libertad |
|---|---|---|
| Rotación (articulación) | giro | 1 |
| Prismático (corredera) | deslizamiento | 1 |
| Leva o engrane | rodadura con deslizamiento | 2 |

## Grados de libertad

Es el número de entradas independientes que hay que fijar para determinar la posición de todo el mecanismo. En el plano, con $n$ eslabones (incluido el fijo), $j_1$ pares de un grado de libertad y $j_2$ de dos, el criterio de Grübler da

$$GDL = 3\,(n - 1) - 2\,j_1 - j_2$$

Si resulta 1, basta un motor; si resulta 0, es una estructura.

## Cuadrilátero articulado

![Mecanismo de cuatro barras con manivela, biela, balancín y bastidor](/media/electromecanica/cuatro-barras.svg)

*Cuadrilátero articulado: cuatro eslabones y cuatro articulaciones, con un grado de libertad.*

Es el mecanismo más simple: $n = 4$, $j_1 = 4$, $GDL = 1$. La **ley de Grashof** dice que alguna barra podrá dar vueltas completas si la suma de la más corta $s$ y la más larga $l$ no supera la de las otras dos:

$$s + l\le p + q$$

| Eslabón fijo | Mecanismo resultante |
|---|---|
| Adyacente a la barra más corta | manivela-balancín |
| La barra más corta | doble manivela |
| Opuesto a la más corta | doble balancín |

El **ángulo de transmisión** entre biela y balancín indica la calidad del mecanismo: conviene que se mantenga entre 40° y 140°.

## Biela-manivela

Convierte rotación en movimiento alternativo, o al revés: es el mecanismo de motores, compresores y bombas de pistón. Con manivela de radio $r$, biela de longitud $l$ y $\lambda = r/l$, la posición del pistón medida desde el punto muerto superior es aproximadamente

$$x\approx r\left[(1 - \cos\theta) + \frac{\lambda}{2}\sin^2\theta\right]$$

y derivando,

$$v\approx r\omega\left(\sin\theta + \frac{\lambda}{2}\sin 2\theta\right),\qquad a\approx r\omega^2\left(\cos\theta + \lambda\cos 2\theta\right)$$

La carrera es $2r$. La aceleración máxima ocurre en el punto muerto superior y produce las **fuerzas de inercia** que cargan a la biela y hacen vibrar al motor.

> [!NOTE]
> Motor con carrera de 80 mm ($r = 40$ mm), biela de 140 mm ($\lambda = 0{,}286$) a 6000 rpm ($\omega = 628$ rad/s).
>
> $a_{max} = 0{,}04\cdot 628^2\cdot 1{,}286\approx 20\,300\ \text{m/s}^2$, más de 2000 veces la gravedad. Un pistón de 0,4 kg tira de la biela con unos 8 kN.

## Análisis de velocidades

- **Centro instantáneo de rotación**: en cada instante, un eslabón gira alrededor de un punto; la velocidad de cualquier punto suyo es perpendicular al radio y proporcional a él.
- **Método de las velocidades relativas**: $\vec v_B = \vec v_A + \vec\omega\times\vec r_{AB}$, resuelto gráficamente o por componentes.
- **Teorema de Kennedy**: los tres centros instantáneos relativos de tres cuerpos están alineados.

## Levas

Una leva impone a su **seguidor** una ley de movimiento arbitraria en cada vuelta: subida, reposo, bajada y reposo. La elección de esa ley es lo esencial:

| Ley de desplazamiento | Aceleración | Uso |
|---|---|---|
| Velocidad constante | infinita en los extremos | inadmisible |
| Armónica simple | salto finito en los extremos | baja velocidad |
| Cicloidal | continua, nula en los extremos | alta velocidad |
| Polinómica 3-4-5 | continua | alta velocidad |

Las discontinuidades de aceleración producen golpes, ruido y desgaste. El **ángulo de presión** entre la fuerza y el movimiento del seguidor no debe superar unos 30°; se reduce agrandando la leva.

## Movimiento intermitente

| Mecanismo | Función |
|---|---|
| Cruz de Malta (Ginebra) | giro continuo en avances por pasos con pausa |
| Trinquete | permite el giro en un solo sentido |
| Retorno rápido (limadora) | carrera útil lenta y vuelta rápida |

## Síntesis

El **análisis** estudia un mecanismo dado; la **síntesis** busca las dimensiones que producen el movimiento deseado, sea una función, una trayectoria o una serie de posiciones. Hoy se resuelve con programas de simulación multicuerpo, pero los criterios anteriores siguen guiando el diseño.
