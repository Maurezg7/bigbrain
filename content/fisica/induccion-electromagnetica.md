+++
title = "Inducción electromagnética"
summary = "Flujo magnético, ley de Faraday, ley de Lenz, fem de movimiento, generadores, inductancia y transformadores."
branch = "Electromagnetismo"
order = 5
level = "Intermedio"
tags = ["ley de Faraday", "ley de Lenz", "inductancia", "generador", "transformador"]
+++

## Flujo magnético

$$\Phi_B = \int\vec{B} \cdot d\vec{A}$$

Para un campo uniforme que forma un ángulo $\theta$ con la normal a una superficie plana de área $A$: $\Phi_B = BA\cos\theta$. Se mide en **webers** (Wb = T·m²).

Mide cuántas líneas de campo atraviesan la superficie.

## Ley de Faraday

Faraday descubrió en 1831 que un campo magnético **variable** produce corriente.

> [!IMPORTANT]
> $$\varepsilon = -N\,\frac{d\Phi_B}{dt}$$
> La fem inducida en una bobina de $N$ espiras es igual a la rapidez con que cambia el flujo que la atraviesa.

El flujo puede cambiar de tres maneras:

| Qué cambia | Ejemplo |
|---|---|
| El campo $B$ | acercar o alejar un imán; variar la corriente de una bobina vecina |
| El área $A$ | una barra que se desliza sobre rieles |
| El ángulo $\theta$ | una espira que gira (generador) |

Un flujo constante, por grande que sea, **no** induce nada.

> [!NOTE]
> Una bobina de $200$ espiras y $50\ \text{cm}^2$ está perpendicular a un campo que pasa de $0$ a $0{,}4$ T en $0{,}1$ s.
>
> $\lvert\varepsilon\rvert = 200 \cdot \dfrac{0{,}4 \cdot 0{,}005}{0{,}1} = 4$ V.

## Ley de Lenz

> [!IMPORTANT]
> La corriente inducida circula en el sentido que **se opone al cambio** de flujo que la produce.

Es el significado del signo menos, y una consecuencia de la conservación de la energía.

- Si el flujo **aumenta**, la corriente inducida crea un campo opuesto al original.
- Si el flujo **disminuye**, crea un campo en el mismo sentido, para sostenerlo.

Al acercar el polo norte de un imán a una espira, esta se comporta como un polo norte enfrentado: **repele** al imán. Hay que hacer trabajo para acercarlo, y ese trabajo es la energía eléctrica que aparece.

> [!WARNING]
> La corriente inducida no se opone al flujo, sino a su **variación**.

## Fem de movimiento

Una barra conductora de longitud $L$ que se mueve con velocidad $v$ perpendicular a un campo $B$:

$$\varepsilon = BLv$$

> [!NOTE]
> Una barra de $0{,}5$ m se desliza a $4$ m/s sobre rieles en un campo de $0{,}3$ T, cerrando un circuito de $2\ \Omega$.
>
> $\varepsilon = 0{,}3 \cdot 0{,}5 \cdot 4 = 0{,}6$ V; $I = 0{,}3$ A.
>
> Fuerza que frena la barra: $F = ILB = 0{,}045$ N. La potencia mecánica $Fv = 0{,}18$ W es igual a la disipada $I^2R$.

### Corrientes parásitas

En un bloque de metal sometido a flujo variable se inducen remolinos de corriente (**corrientes de Foucault**).

| Efecto | Uso o solución |
|---|---|
| Frenan el movimiento | frenos magnéticos de trenes |
| Calientan el metal | cocinas y hornos de inducción |
| Provocan pérdidas | los núcleos de transformadores se hacen de chapas aisladas |

## Generadores

Una bobina de $N$ espiras y área $A$ que gira con velocidad angular $\omega$ en un campo $B$:

$$\varepsilon = NBA\omega\sin(\omega t)$$

Produce **corriente alterna** sinusoidal de amplitud $\varepsilon_0 = NBA\omega$. Así se genera casi toda la electricidad del mundo: una turbina (de vapor, agua o viento) hace girar el generador.

Un **motor** es un generador funcionando al revés. Al girar, además, induce una **fuerza contraelectromotriz** que limita su corriente; por eso un motor trabado se quema.

## Inductancia

Una bobina se opone a los cambios de su propia corriente (**autoinducción**):

> [!IMPORTANT]
> $$\varepsilon = -L\,\frac{dI}{dt}$$

La **inductancia** $L$ se mide en **henrys** (H). Para un solenoide de $N$ espiras, longitud $l$ y sección $A$:

$$L = \frac{\mu_0N^2A}{l}$$

Un inductor es la «inercia» de un circuito: no deja que la corriente cambie bruscamente.

**Energía** almacenada en el campo magnético:

$$U = \tfrac{1}{2}LI^2$$

con densidad $u = \dfrac{B^2}{2\mu_0}$.

### Circuito RL

Al conectar una fuente, la corriente crece gradualmente:

$$I(t) = \frac{\varepsilon}{R}\left(1 - e^{-t/\tau}\right), \qquad \tau = \frac{L}{R}$$

Al interrumpir bruscamente un circuito inductivo aparece una sobretensión grande (chispa en el interruptor). Es el principio de la bobina de encendido de un auto.

### Circuito LC

Un capacitor cargado conectado a un inductor oscila: la energía pasa del campo eléctrico al magnético y vuelve.

$$\omega = \frac{1}{\sqrt{LC}}$$

Es el análogo eléctrico de la masa con resorte, y la base de la sintonía de radio.

| Mecánica | Eléctrica |
|---|---|
| masa $m$ | inductancia $L$ |
| constante $k$ | $1/C$ |
| posición $x$ | carga $q$ |
| velocidad $v$ | corriente $I$ |
| rozamiento $b$ | resistencia $R$ |

## Transformadores

Dos bobinas sobre un mismo núcleo de hierro. La corriente alterna del primario crea un flujo variable que induce una fem en el secundario (**inducción mutua**).

> [!IMPORTANT]
> $$\frac{V_2}{V_1} = \frac{N_2}{N_1} \qquad \frac{I_2}{I_1} = \frac{N_1}{N_2}$$

En un transformador ideal la potencia se conserva: $V_1I_1 = V_2I_2$. Si eleva la tensión, reduce la corriente.

> [!NOTE]
> Un cargador con $N_1 = 2200$ y $N_2 = 120$ espiras conectado a $220$ V: $V_2 = 220 \cdot \dfrac{120}{2200} = 12$ V.

> [!WARNING]
> Un transformador **no funciona con corriente continua**: sin variación de flujo no hay inducción.

Los transformadores permiten transmitir la energía en alta tensión (pocas pérdidas) y usarla en baja tensión (segura). Es la razón por la que las redes son de corriente alterna.

> [!TIP]
> Para hallar el sentido de la corriente inducida: (1) ¿el flujo aumenta o disminuye?; (2) el campo inducido se opone a ese cambio; (3) regla de la mano derecha.
