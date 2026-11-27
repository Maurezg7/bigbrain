+++
title = "Campo magnético"
summary = "Fuerza magnética sobre cargas y corrientes, movimiento de partículas, ley de Biot-Savart, ley de Ampère y materiales magnéticos."
branch = "Electromagnetismo"
order = 4
level = "Intermedio"
tags = ["campo magnético", "fuerza de Lorentz", "ley de Ampère", "solenoide"]
+++

## Imanes y campo magnético

Todo imán tiene dos polos, **norte** y **sur**. Polos iguales se repelen y distintos se atraen. A diferencia de las cargas, los polos **no pueden aislarse**: al partir un imán se obtienen dos imanes completos.

El **campo magnético** $\vec{B}$ se mide en **teslas** (T). $1\ \text{T} = 10^4$ gauss.

| Fuente | Campo |
|---|---|
| Campo terrestre | ~50 μT |
| Imán de heladera | ~5 mT |
| Imán de neodimio | ~1 T |
| Resonador magnético | 1,5–3 T |

Las líneas de campo salen del polo norte, entran por el sur y **siempre son cerradas**.

El origen de todo magnetismo son las **cargas en movimiento**.

## Fuerza sobre una carga

> [!IMPORTANT]
> **Fuerza de Lorentz**:
> $$\vec{F} = q\,\vec{v} \times \vec{B}, \qquad F = \lvert q \rvert vB\sin\theta$$

- Es perpendicular tanto a $\vec{v}$ como a $\vec{B}$ (**regla de la mano derecha**; para cargas negativas, el sentido opuesto).
- Es cero si la carga está quieta o se mueve paralela al campo.
- Al ser perpendicular a la velocidad, **no realiza trabajo**: cambia la dirección, nunca la rapidez.

### Movimiento circular

Una carga que entra perpendicular a un campo uniforme describe una circunferencia:

$$qvB = \frac{mv^2}{r} \quad\Longrightarrow\quad r = \frac{mv}{qB}$$

El período $T = \dfrac{2\pi m}{qB}$ no depende de la velocidad. Si la velocidad tiene una componente paralela al campo, la trayectoria es una **hélice**.

> [!NOTE]
> Un protón a $2 \times 10^6$ m/s entra perpendicular a un campo de $0{,}5$ T.
>
> $r = \dfrac{1{,}67 \times 10^{-27} \cdot 2 \times 10^6}{1{,}6 \times 10^{-19} \cdot 0{,}5} = 4{,}2$ cm.

| Aplicación | Principio |
|---|---|
| Espectrómetro de masas | el radio depende de $m/q$: separa isótopos |
| Ciclotrón | acelera partículas en espiral |
| Selector de velocidades | con $\vec{E}$ y $\vec{B}$ cruzados pasa solo $v = E/B$ |
| Auroras | el campo terrestre guía partículas hacia los polos |
| Efecto Hall | mide campos y el signo de los portadores |

## Fuerza sobre una corriente

Un conductor de longitud $L$ con corriente $I$:

$$\vec{F} = I\,\vec{L} \times \vec{B}, \qquad F = ILB\sin\theta$$

Una **espira** con corriente en un campo recibe un par $\tau = NIAB\sin\theta$ que tiende a alinearla. Es el principio del **motor eléctrico** y del galvanómetro.

> [!NOTE]
> Un cable de $0{,}5$ m con $10$ A, perpendicular a un campo de $0{,}2$ T: $F = 10 \cdot 0{,}5 \cdot 0{,}2 = 1$ N.

## Campo creado por corrientes

Oersted descubrió en 1820 que una corriente desvía una brújula.

**Ley de Biot-Savart**:

$$d\vec{B} = \frac{\mu_0}{4\pi}\,\frac{I\,d\vec{l} \times \hat{r}}{r^2}, \qquad \mu_0 = 4\pi \times 10^{-7}\ \text{T·m/A}$$

| Configuración | Campo |
|---|---|
| Conductor recto largo | $B = \dfrac{\mu_0I}{2\pi r}$ |
| Centro de una espira | $B = \dfrac{\mu_0I}{2R}$ |
| Interior de un solenoide | $B = \mu_0nI$ |
| Interior de un toroide | $B = \dfrac{\mu_0NI}{2\pi r}$ |

En el solenoide, $n$ es la cantidad de espiras por metro. El campo interior es uniforme y, fuera, casi nulo.

**Regla de la mano derecha**: con el pulgar en el sentido de la corriente, los dedos indican cómo circula el campo alrededor del cable.

> [!NOTE]
> A $5$ cm de un cable con $20$ A: $B = \dfrac{4\pi \times 10^{-7} \cdot 20}{2\pi \cdot 0{,}05} = 80\ \mu\text{T}$, comparable al campo terrestre.

### Fuerza entre conductores paralelos

$$\frac{F}{L} = \frac{\mu_0I_1I_2}{2\pi d}$$

Corrientes en el **mismo** sentido se **atraen**; en sentidos opuestos se repelen.

## Ley de Ampère

> [!IMPORTANT]
> $$\oint\vec{B} \cdot d\vec{l} = \mu_0I_{enc}$$

La circulación del campo a lo largo de una curva cerrada es proporcional a la corriente que la atraviesa. Es el análogo magnético de la ley de Gauss y permite calcular $B$ cuando hay simetría.

**Ley de Gauss del magnetismo**: $\displaystyle\oint\vec{B} \cdot d\vec{A} = 0$. No existen monopolos magnéticos.

## Materiales magnéticos

| Tipo | Comportamiento | Ejemplos |
|---|---|---|
| Diamagnético | repelido muy débilmente | cobre, agua, bismuto |
| Paramagnético | atraído débilmente | aluminio, platino |
| Ferromagnético | atraído con fuerza; puede quedar imantado | hierro, níquel, cobalto |

En los ferromagnéticos, los átomos se alinean en **dominios**. Un campo externo los orienta y multiplica el campo miles de veces: por eso los electroimanes y transformadores tienen núcleo de hierro.

- **Histéresis**: la imantación depende de la historia. Los materiales «duros» retienen la imantación (imanes permanentes); los «blandos» la pierden (núcleos de transformadores).
- Por encima de la **temperatura de Curie** ($770$ °C para el hierro) el ferromagnetismo desaparece.

> [!WARNING]
> La fuerza magnética nunca cambia la energía cinética de una partícula. Para acelerar hace falta un campo eléctrico.

> [!TIP]
> Los campos eléctricos nacen en cargas; los magnéticos rodean a las corrientes. Las líneas de $\vec{E}$ empiezan y terminan; las de $\vec{B}$ se cierran sobre sí mismas.
