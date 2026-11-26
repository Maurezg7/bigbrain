+++
title = "Metrología, ajustes y tolerancias"
summary = "Instrumentos de medición dimensional, sistema ISO de tolerancias y ajustes, tolerancias geométricas y rugosidad."
branch = "Mecanismos y tecnología mecánica"
order = 5
level = "Básico"
tags = ["metrología", "tolerancias", "ajustes", "calibre", "micrómetro"]
+++

## Por qué existen las tolerancias

Ninguna pieza sale exactamente con la medida del plano. Para que cualquier eje entre en cualquier agujero de la misma serie, sin retoques (**intercambiabilidad**), el plano indica entre qué límites puede variar cada medida. Esa franja es la **tolerancia**.

Cuanto más estrecha, más cara: se especifica la más amplia que permita que la pieza funcione.

## Instrumentos

| Instrumento | Apreciación típica | Uso |
|---|---|---|
| Regla y cinta | 1 mm | medidas bastas |
| Calibre (pie de rey) | 0,05 o 0,02 mm | exteriores, interiores y profundidades |
| Micrómetro | 0,01 mm; 0,001 mm con nonio | diámetros y espesores |
| Reloj comparador | 0,01 o 0,001 mm | diferencias, alineación, excentricidad |
| Bloques patrón | décimas de micrómetro | calibración |
| Calibres pasa-no pasa | — | control rápido en producción |
| Máquina de medir por coordenadas | micrómetros | piezas complejas |

El **nonio** del calibre permite leer fracciones de milímetro: se suma a la lectura de la regla fija la división del nonio que coincide con una de la regla.

> [!TIP]
> Las medidas de precisión se refieren a 20 °C. Una barra de acero de 100 mm se alarga 0,012 mm por cada 10 °C; sostener un micrómetro con la mano ya altera la lectura.

Un error típico es el de **Abbe**: aparece cuando la escala del instrumento no está alineada con la dimensión medida. El micrómetro lo evita; el calibre no.

## Sistema ISO de tolerancias

Cada tolerancia se indica con una letra y un número, como **H7** o **g6**.

- La **letra** indica la posición de la franja respecto de la medida nominal: mayúsculas para agujeros y minúsculas para ejes. La H empieza exactamente en la nominal y va hacia arriba; la h termina en la nominal.
- El **número** es la calidad (IT): cuanto menor, más estrecha.

| Calidad | Obtenible con |
|---|---|
| IT5 a IT6 | rectificado fino |
| IT7 a IT8 | rectificado, torneado fino, escariado |
| IT9 a IT11 | torneado y fresado normales |
| IT12 a IT16 | laminado, forja, fundición |

## Ajustes

![Posición de las zonas de tolerancia de eje y agujero en ajustes con juego, indeterminado y con apriete](/media/electromecanica/ajustes.svg)

*Según la posición relativa de las zonas de tolerancia, el ajuste queda con juego, indeterminado o con apriete.*

Un **ajuste** es la relación entre un eje y un agujero de la misma medida nominal.

| Tipo | Condición | Ejemplo en agujero único | Uso |
|---|---|---|---|
| Con juego | el eje siempre es menor | H7/g6, H7/f7 | piezas que deslizan o giran |
| Indeterminado | puede quedar juego o apriete | H7/k6, H7/js6 | centrado preciso, desmontable |
| Con apriete | el eje siempre es mayor | H7/p6, H7/s6 | uniones fijas, montaje a presión o en caliente |

Se prefiere el sistema de **agujero único** (agujero H) porque es más fácil variar la medida de un eje que la de un agujero.

> [!NOTE]
> Ajuste 40 H7/g6. Para 40 mm: H7 va de 0 a +25 µm y g6 de −25 a −9 µm.
>
> Agujero: de 40,000 a 40,025 mm. Eje: de 39,975 a 39,991 mm.
>
> Juego mínimo: $40{,}000 - 39{,}991 = 0{,}009$ mm. Juego máximo: $40{,}025 - 39{,}975 = 0{,}050$ mm.

## Tolerancias geométricas

Una pieza puede tener todas sus medidas dentro de tolerancia y no servir por su forma. Las tolerancias geométricas (GD&T) controlan:

| Grupo | Características |
|---|---|
| Forma | rectitud, planitud, redondez, cilindricidad |
| Orientación | paralelismo, perpendicularidad, angularidad |
| Posición | posición, concentricidad, simetría |
| Oscilación | salto radial y axial |

Se indican en un recuadro con el símbolo, el valor y las **referencias** (datums) desde las que se mide.

## Cadenas de cotas

Cuando varias piezas se apilan, sus tolerancias se acumulan. En el peor caso se suman:

$$T_{total} = \sum T_i$$

Si son muchas e independientes, es más realista la suma estadística $T_{total} = \sqrt{\sum T_i^2}$, que permite tolerancias individuales más amplias.

## Rugosidad

La terminación superficial se especifica con $R_a$ en micrómetros. Afecta el rozamiento, el desgaste, el sellado y la resistencia a la fatiga, y debe ser coherente con la tolerancia: no tiene sentido pedir IT6 con una superficie de desbaste.

## Calibración y trazabilidad

Todo instrumento se compara periódicamente con un patrón más exacto, y este con otro, hasta llegar al patrón nacional (en Argentina, el INTI) y a la definición internacional del metro. Esa cadena documentada es la **trazabilidad**. Como regla, el instrumento debe ser unas diez veces más fino que la tolerancia a controlar.
