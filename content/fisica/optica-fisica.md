+++
title = "Óptica física"
summary = "Naturaleza ondulatoria de la luz: principio de Huygens, interferencia de Young, películas delgadas, difracción, redes y poder de resolución."
branch = "Ondas y óptica"
order = 4
level = "Avanzado"
tags = ["interferencia", "difracción", "experimento de Young", "red de difracción"]
+++

## La luz como onda

La óptica geométrica no explica qué pasa cuando la luz encuentra obstáculos o aberturas de tamaño comparable a su longitud de onda. Ahí se manifiesta su naturaleza **ondulatoria**.

**Principio de Huygens**: cada punto de un frente de onda actúa como fuente de ondas secundarias; el nuevo frente es la envolvente de esas ondas.

## Interferencia

Cuando dos ondas se superponen, sus amplitudes se suman.

| Tipo | Diferencia de camino | Resultado |
|---|---|---|
| Constructiva | $\Delta r = m\lambda$ | máximo (brillante) |
| Destructiva | $\Delta r = \left(m + \tfrac{1}{2}\right)\lambda$ | mínimo (oscuro) |

con $m = 0, \pm 1, \pm 2, \dots$

Para ver un patrón estable, las fuentes deben ser **coherentes**: misma frecuencia y diferencia de fase constante. Dos lámparas comunes no lo son; un láser sí.

### Experimento de Young

Thomas Young (1801) hizo pasar luz por dos rendijas separadas una distancia $d$ y observó franjas claras y oscuras en una pantalla a distancia $L$. Fue la prueba de que la luz es una onda.

> [!IMPORTANT]
> Máximos: $d\sin\theta = m\lambda$
>
> Para ángulos pequeños, las franjas brillantes están en $y_m = \dfrac{m\lambda L}{d}$, separadas $\Delta y = \dfrac{\lambda L}{d}$.

> [!NOTE]
> Rendijas separadas $0{,}2$ mm, pantalla a $1$ m, luz de $600$ nm.
>
> $\Delta y = \dfrac{600 \times 10^{-9} \cdot 1}{0{,}2 \times 10^{-3}} = 3$ mm entre franjas.

Las franjas se separan más con mayor longitud de onda (rojo) o rendijas más juntas.

### Películas delgadas

Los colores de una pompa de jabón o de una mancha de aceite se deben a la interferencia entre la luz reflejada en las dos caras de la película.

Hay que tener en cuenta que:

- Dentro de la película, la longitud de onda es $\lambda_n = \dfrac{\lambda}{n}$.
- Al reflejarse en un medio de **mayor** índice, la onda sufre un **cambio de fase de media longitud de onda**.

Para una película de espesor $t$ e índice $n$ con incidencia normal, si **solo una** de las dos reflexiones tiene cambio de fase (como en una pompa de jabón en aire):

$$2nt = \left(m + \tfrac{1}{2}\right)\lambda \ \text{(constructiva)} \qquad 2nt = m\lambda \ \text{(destructiva)}$$

Si ambas o ninguna lo tienen, las condiciones se intercambian.

> [!NOTE]
> **Capa antirreflectante**: una capa de $n = 1{,}38$ sobre vidrio ($n = 1{,}5$). Ambas reflexiones cambian de fase, así que la mínima reflexión se da con $2nt = \tfrac{\lambda}{2}$.
>
> Para $\lambda = 550$ nm: $t = \dfrac{550}{4 \cdot 1{,}38} = 100$ nm.

## Difracción

Es la desviación de las ondas al pasar por una abertura o rodear un obstáculo. Es notable cuando la abertura es comparable a $\lambda$.

### Una rendija

Para una rendija de ancho $a$, los **mínimos** están en:

$$a\sin\theta = m\lambda, \qquad m = \pm 1, \pm 2, \dots$$

El máximo central es el doble de ancho que los demás y concentra casi toda la luz. Cuanto **más angosta** la rendija, **más se abre** el haz.

> [!WARNING]
> No confundir: $d\sin\theta = m\lambda$ da los **máximos** de dos rendijas, mientras que $a\sin\theta = m\lambda$ da los **mínimos** de una sola.

### Red de difracción

Muchas rendijas paralelas, separadas una distancia $d$. Los máximos están en

$$d\sin\theta = m\lambda$$

pero son muy **angostos y brillantes**. Como el ángulo depende de $\lambda$, la red separa los colores mucho mejor que un prisma. Es la base de los **espectrómetros**, que identifican elementos por su luz.

> [!NOTE]
> Una red de $500$ líneas/mm ($d = 2 \times 10^{-6}$ m) con luz de $600$ nm.
>
> Primer orden: $\sin\theta = \dfrac{600 \times 10^{-9}}{2 \times 10^{-6}} = 0{,}3 \Rightarrow \theta = 17{,}5°$.
>
> Orden máximo: $m \le \dfrac{d}{\lambda} = 3{,}3$, es decir, hasta el tercero.

Los surcos de un CD actúan como red de difracción; por eso se ven colores.

### Poder de resolución

La difracción impide formar imágenes perfectamente nítidas. Para una abertura circular de diámetro $D$, dos puntos se distinguen si su separación angular supera

> [!IMPORTANT]
> **Criterio de Rayleigh**: $\theta_{mín} = 1{,}22\,\dfrac{\lambda}{D}$

- Los telescopios se hacen con espejos enormes para resolver más detalle.
- Los microscopios ópticos no distinguen detalles menores que unos $200$ nm; para ir más allá se usan electrones, de longitud de onda mucho menor.

> [!NOTE]
> El ojo, con pupila de $3$ mm y luz de $550$ nm: $\theta_{mín} = 1{,}22 \cdot \dfrac{550 \times 10^{-9}}{0{,}003} = 2{,}2 \times 10^{-4}$ rad.
>
> A $10$ m distingue puntos separados unos $2$ mm.

### Difracción de rayos X

Los átomos de un cristal están separados una distancia comparable a la longitud de onda de los rayos X y actúan como red tridimensional.

**Ley de Bragg**: $2d\sin\theta = m\lambda$

Permite determinar la estructura de cristales y moléculas; así se descubrió la doble hélice del ADN.

## Polarización

Demuestra que la luz es una onda **transversal**. Un polarizador deja pasar una sola dirección de oscilación del campo eléctrico.

| Fenómeno | Descripción |
|---|---|
| Ley de Malus | $I = I_0\cos^2\theta$ entre dos polarizadores |
| Ángulo de Brewster | con $\tan\theta_B = \dfrac{n_2}{n_1}$, la luz reflejada queda totalmente polarizada |
| Birrefringencia | cristales con dos índices según la polarización |
| Fotoelasticidad | materiales bajo tensión muestran franjas entre polarizadores |

## Holografía e interferometría

- Un **holograma** registra el patrón de interferencia entre la luz de un objeto y un haz de referencia; al iluminarlo reconstruye la imagen tridimensional.
- Un **interferómetro** (como el de Michelson) mide distancias con precisión de fracciones de longitud de onda. Con uno gigante se detectaron las ondas gravitacionales.

> [!TIP]
> Óptica geométrica cuando los objetos son mucho mayores que $\lambda$; óptica física cuando son comparables. La luz visible tiene $\lambda \approx 0{,}5\ \mu\text{m}$.
