+++
title = "Rotación y momento angular"
summary = "Momento de una fuerza, momento de inercia, dinámica de rotación, energía de rotación, rodadura y conservación del momento angular."
branch = "Mecánica"
order = 6
level = "Intermedio"
tags = ["torque", "momento de inercia", "momento angular", "rodadura"]
+++

## De la traslación a la rotación

Un cuerpo rígido puede trasladarse y girar. La rotación tiene magnitudes análogas a las de la traslación:

| Traslación | Rotación | Relación |
|---|---|---|
| posición $x$ | ángulo $\theta$ | $s = r\theta$ |
| velocidad $v$ | velocidad angular $\omega$ | $v = r\omega$ |
| aceleración $a$ | aceleración angular $\alpha$ | $a_t = r\alpha$ |
| masa $m$ | momento de inercia $I$ | $I = \sum mr^2$ |
| fuerza $F$ | momento (torque) $\tau$ | $\tau = rF\sin\varphi$ |
| $F = ma$ | $\tau = I\alpha$ | |
| $p = mv$ | $L = I\omega$ | |
| $E_c = \tfrac{1}{2}mv^2$ | $E_c = \tfrac{1}{2}I\omega^2$ | |

## Momento de una fuerza

> [!IMPORTANT]
> $$\vec{\tau} = \vec{r} \times \vec{F}, \qquad \tau = rF\sin\varphi = F \cdot d$$

$d$ es el **brazo de palanca**: la distancia perpendicular del eje a la línea de acción de la fuerza. Se mide en N·m.

El torque mide la capacidad de una fuerza para hacer girar. Por eso el picaporte está lejos de las bisagras y una llave larga afloja mejor una tuerca.

### Equilibrio

Un cuerpo rígido está en equilibrio si:

$$\sum\vec{F} = 0 \qquad \sum\vec{\tau} = 0$$

> [!NOTE]
> Un subibaja: un niño de $30$ kg se sienta a $2$ m del apoyo. ¿Dónde debe sentarse otro de $40$ kg?
>
> $30 \cdot g \cdot 2 = 40 \cdot g \cdot d \Rightarrow d = 1{,}5$ m.

## Momento de inercia

Mide la resistencia a cambiar el estado de rotación. Depende de la masa y de **cómo está distribuida** respecto del eje:

$$I = \sum m_ir_i^2 = \int r^2\,dm$$

| Cuerpo | Eje | $I$ |
|---|---|---|
| Aro o cilindro hueco de pared delgada | central | $MR^2$ |
| Disco o cilindro macizo | central | $\tfrac{1}{2}MR^2$ |
| Esfera maciza | por el centro | $\tfrac{2}{5}MR^2$ |
| Esfera hueca de pared delgada | por el centro | $\tfrac{2}{3}MR^2$ |
| Varilla | por el centro | $\tfrac{1}{12}ML^2$ |
| Varilla | por un extremo | $\tfrac{1}{3}ML^2$ |

**Teorema de Steiner** (ejes paralelos): $I = I_{cm} + Md^2$.

## Dinámica de rotación

> [!IMPORTANT]
> $$\sum\tau = I\alpha$$

> [!NOTE]
> Una polea maciza de $M = 4$ kg y $R = 0{,}2$ m tiene enrollada una cuerda de la que cuelga un bloque de $m = 2$ kg.
>
> Bloque: $mg - T = ma$. Polea: $TR = \tfrac{1}{2}MR^2\alpha$, con $a = R\alpha$, así que $T = \tfrac{1}{2}Ma$.
>
> $a = \dfrac{mg}{m + M/2} = \dfrac{19{,}6}{4} = 4{,}9\ \text{m/s}^2$.
>
> Sin la inercia de la polea caería con $g$.

## Energía y trabajo

$$E_c = \tfrac{1}{2}I\omega^2 \qquad W = \int\tau\,d\theta \qquad P = \tau\,\omega$$

Con par constante, $W = \tau\,\Delta\theta$.

La última es la relación entre potencia, par y velocidad de cualquier motor.

### Rodadura

Un cuerpo que rueda **sin deslizar** cumple $v_{cm} = R\omega$ y tiene energía de traslación y de rotación:

$$E_c = \tfrac{1}{2}mv_{cm}^2 + \tfrac{1}{2}I_{cm}\omega^2$$

> [!NOTE]
> ¿Con qué velocidad llega al pie de un plano de altura $h$ un cuerpo que rueda?
>
> $mgh = \tfrac{1}{2}mv^2 + \tfrac{1}{2}I\dfrac{v^2}{R^2}$. Con $I = kmR^2$:
>
> $$v = \sqrt{\frac{2gh}{1 + k}}$$
>
> Esfera maciza ($k = \tfrac{2}{5}$): $v = \sqrt{\tfrac{10}{7}gh}$. Aro ($k = 1$): $v = \sqrt{gh}$.
>
> La esfera llega primero, **sin importar** masa ni radio: cuanto más concentrada está la masa cerca del eje, menos energía se va a rotación.

## Momento angular

Para una partícula: $\vec{L} = \vec{r} \times \vec{p}$. Para un cuerpo rígido que gira sobre un eje fijo: $L = I\omega$.

$$\sum\vec{\tau}_{ext} = \frac{d\vec{L}}{dt}$$

> [!IMPORTANT]
> **Conservación**: si el torque externo neto es cero, $L = I\omega$ es constante.

Si $I$ disminuye, $\omega$ aumenta:

- Una patinadora gira más rápido al pegar los brazos al cuerpo.
- Un clavadista se encoge para dar más vueltas.
- Una estrella que colapsa en púlsar pasa a girar cientos de veces por segundo.
- Los planetas van más rápido cuando están cerca del Sol (segunda ley de Kepler).

> [!NOTE]
> Una patinadora gira a $2$ vueltas por segundo con $I = 4\ \text{kg·m}^2$. Al cerrar los brazos, $I = 1{,}6\ \text{kg·m}^2$.
>
> $\omega_2 = \dfrac{4}{1{,}6}(2) = 5$ vueltas por segundo.
>
> La energía cinética **aumenta** (de $\tfrac{1}{2}I\omega^2$): el trabajo lo hacen sus músculos.

### Giróscopo

Un cuerpo que gira rápido tiende a mantener la dirección de su eje. Si se le aplica un torque, el eje no cae: **precesa** perpendicularmente. Explica la estabilidad de la bicicleta y el trompo, y es la base de los sistemas de navegación.

> [!WARNING]
> El momento de inercia no es una propiedad fija del cuerpo: cambia según el eje elegido.

> [!TIP]
> Toda fórmula de traslación tiene su gemela en rotación: basta cambiar $m$ por $I$, $v$ por $\omega$ y $F$ por $\tau$.
