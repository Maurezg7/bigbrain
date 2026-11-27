+++
title = "Sonido"
summary = "Naturaleza de las ondas sonoras, velocidad, intensidad y decibeles, ondas estacionarias en cuerdas y tubos, batidos y efecto Doppler."
branch = "Ondas y óptica"
order = 2
level = "Intermedio"
tags = ["sonido", "decibeles", "efecto Doppler", "ondas estacionarias", "armónicos"]
+++

## Ondas sonoras

El sonido es una onda **mecánica longitudinal**: las partículas del medio oscilan en la misma dirección en que avanza la onda, formando zonas de compresión y de enrarecimiento. Necesita un medio material: no se propaga en el vacío.

| Rango | Frecuencia |
|---|---|
| Infrasonido | < 20 Hz |
| Audible | 20 Hz – 20 kHz |
| Ultrasonido | > 20 kHz |

### Velocidad

Depende del medio, no de la frecuencia ni de la amplitud. En general, es mayor cuanto más rígido es el medio.

| Medio | Velocidad (m/s) |
|---|---|
| Aire (20 °C) | 343 |
| Agua | 1480 |
| Acero | ~5900 |

En el aire aumenta con la temperatura: $v \approx 331 + 0{,}6\,T_C$ m/s (con $T_C$ en grados Celsius).

> [!NOTE]
> Se ve un relámpago y el trueno llega $6$ s después. La tormenta está a $343 \cdot 6 \approx 2$ km.

## Cualidades del sonido

| Cualidad | Magnitud física | Distingue |
|---|---|---|
| Tono | frecuencia | graves de agudos |
| Intensidad | amplitud | fuertes de débiles |
| Timbre | forma de onda (armónicos) | un violín de una flauta |

## Intensidad y decibeles

La **intensidad** es la potencia por unidad de área (W/m²). Para una fuente puntual que emite en todas direcciones, sin absorción:

$$I = \frac{P}{4\pi r^2}$$

Al duplicar la distancia, la intensidad cae a la cuarta parte.

El oído percibe un rango enorme, de $10^{-12}$ a $1\ \text{W/m}^2$, y responde de forma logarítmica. Por eso se usa el **nivel de intensidad**:

> [!IMPORTANT]
> $$\beta = 10\log_{10}\frac{I}{I_0}\ \text{dB}, \qquad I_0 = 10^{-12}\ \text{W/m}^2$$

| Fuente | Nivel (dB) |
|---|---|
| Umbral de audición | 0 |
| Susurro | 20 |
| Conversación | 60 |
| Tránsito intenso | 80 |
| Recital | 110 |
| Umbral de dolor | 120–130 |

- Multiplicar la intensidad por 10 **suma** 10 dB.
- Duplicar la intensidad suma 3 dB.

> [!NOTE]
> Una máquina produce $80$ dB. Dos máquinas iguales producen $80 + 10\log 2 = 83$ dB, no $160$.

> [!WARNING]
> Los decibeles no se suman directamente: se suman las intensidades. La exposición prolongada por encima de unos 85 dB daña el oído de forma permanente.

## Ondas estacionarias

Cuando dos ondas iguales viajan en sentidos opuestos (una onda y su reflexión), se forma una **onda estacionaria**, con **nodos** (puntos quietos) y **vientres** (máxima amplitud). Solo ciertas frecuencias son posibles: los **armónicos**.

### Cuerda fija en ambos extremos

$$f_n = n\,\frac{v}{2L}, \qquad n = 1, 2, 3, \dots \qquad v = \sqrt{\frac{T}{\mu}}$$

con $T$ la tensión y $\mu$ la masa por unidad de longitud. $f_1$ es la **frecuencia fundamental**.

Una cuerda suena más aguda si es más corta, más tensa o más liviana: así se afina y se toca una guitarra.

### Tubos

| Tubo | Frecuencias | Armónicos |
|---|---|---|
| Abierto en ambos extremos | $f_n = n\,\dfrac{v}{2L}$ | todos |
| Cerrado en un extremo | $f_n = n\,\dfrac{v}{4L}$ | solo impares ($n = 1, 3, 5, \dots$) |

En un extremo abierto hay un vientre de desplazamiento; en uno cerrado, un nodo.

> [!NOTE]
> Un tubo abierto de $0{,}5$ m: $f_1 = \dfrac{343}{2 \cdot 0{,}5} = 343$ Hz.
>
> El mismo tubo cerrado en un extremo: $f_1 = \dfrac{343}{4 \cdot 0{,}5} = 171{,}5$ Hz, una octava más grave.

El **timbre** de un instrumento depende de qué armónicos acompañan a la fundamental y con qué intensidad.

## Batidos

Dos sonidos de frecuencias cercanas producen una variación periódica de volumen:

$$f_{batido} = \lvert f_1 - f_2 \rvert$$

Se usa para afinar: los batidos desaparecen cuando las frecuencias coinciden.

## Efecto Doppler

Cuando la fuente o el observador se mueven, la frecuencia percibida cambia.

> [!IMPORTANT]
> $$f' = f\,\frac{v \pm v_o}{v \mp v_f}$$
> Signos superiores si se **acercan**; inferiores si se **alejan**. $v$ es la velocidad del sonido, $v_o$ la del observador y $v_f$ la de la fuente, medidas respecto del aire.

Al acercarse, el sonido se oye más agudo; al alejarse, más grave.

> [!NOTE]
> Una ambulancia emite $1000$ Hz y se acerca a $30$ m/s a un peatón quieto.
>
> $f' = 1000 \cdot \dfrac{343}{343 - 30} = 1096$ Hz. Al alejarse: $1000 \cdot \dfrac{343}{343 + 30} = 920$ Hz.

Aplicaciones: radares de velocidad, ecografía Doppler (flujo sanguíneo), y en astronomía el **corrimiento al rojo** de las galaxias que se alejan.

### Ondas de choque

Si la fuente supera la velocidad del sonido, las ondas se apilan en un cono y producen un **estampido sónico**. El **número de Mach** es $M = \dfrac{v_f}{v}$, y el semiángulo del cono cumple $\sin\theta = \dfrac{1}{M}$.

## Ultrasonido

| Aplicación | Principio |
|---|---|
| Ecografía | los ecos en los tejidos forman una imagen |
| Sonar | el tiempo de ida y vuelta da la distancia |
| Ensayos no destructivos | los ecos revelan fisuras en piezas |
| Limpieza | la cavitación desprende la suciedad |

> [!TIP]
> En cualquier onda vale $v = \lambda f$. El medio fija la velocidad y la fuente fija la frecuencia; la longitud de onda se acomoda.
