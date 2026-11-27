+++
title = "Ecuaciones de Maxwell y ondas electromagnéticas"
summary = "Las cuatro ecuaciones de Maxwell, la corriente de desplazamiento, propiedades de las ondas electromagnéticas y el espectro."
branch = "Electromagnetismo"
order = 6
level = "Avanzado"
tags = ["Maxwell", "ondas electromagnéticas", "espectro electromagnético", "velocidad de la luz"]
+++

## Las ecuaciones de Maxwell

Hacia 1865, Maxwell reunió todo el electromagnetismo en cuatro ecuaciones.

| Ley | Forma integral | Significado |
|---|---|---|
| Gauss (eléctrica) | $\displaystyle\oint\vec{E} \cdot d\vec{A} = \frac{Q_{enc}}{\varepsilon_0}$ | las cargas son fuentes del campo eléctrico |
| Gauss (magnética) | $\displaystyle\oint\vec{B} \cdot d\vec{A} = 0$ | no hay monopolos magnéticos |
| Faraday | $\displaystyle\oint\vec{E} \cdot d\vec{l} = -\frac{d\Phi_B}{dt}$ | un campo magnético variable crea un campo eléctrico |
| Ampère-Maxwell | $\displaystyle\oint\vec{B} \cdot d\vec{l} = \mu_0I + \mu_0\varepsilon_0\frac{d\Phi_E}{dt}$ | las corrientes y los campos eléctricos variables crean campo magnético |

Junto con la fuerza de Lorentz, $\vec{F} = q(\vec{E} + \vec{v} \times \vec{B})$, describen todos los fenómenos eléctricos, magnéticos y ópticos clásicos.

### La corriente de desplazamiento

El aporte de Maxwell fue el término $\mu_0\varepsilon_0\dfrac{d\Phi_E}{dt}$. Entre las placas de un capacitor que se carga no circula corriente, pero el campo eléctrico cambia, y ese cambio produce campo magnético igual que una corriente.

Con ese término aparece una simetría: un $\vec{B}$ variable crea $\vec{E}$, y un $\vec{E}$ variable crea $\vec{B}$. Los campos pueden sostenerse mutuamente y **propagarse** por el vacío.

## Ondas electromagnéticas

> [!IMPORTANT]
> Las ecuaciones de Maxwell predicen ondas que viajan en el vacío con velocidad
> $$c = \frac{1}{\sqrt{\mu_0\varepsilon_0}} = 299\,792\,458\ \text{m/s}$$

El valor coincidía con la velocidad medida de la luz. Maxwell concluyó que **la luz es una onda electromagnética**. Hertz las produjo y detectó en 1887.

### Propiedades

- Son **transversales**: $\vec{E}$ y $\vec{B}$ son perpendiculares entre sí y a la dirección de propagación (que es la de $\vec{E} \times \vec{B}$).
- Los campos oscilan **en fase**, con $E = cB$.
- **No necesitan medio**: se propagan en el vacío.
- Las produce toda carga **acelerada**.
- Cumplen $c = \lambda f$.

En un medio de índice de refracción $n$ viajan más lento: $v = \dfrac{c}{n}$. La frecuencia no cambia; la longitud de onda se acorta.

> [!NOTE]
> Una radio FM emite a $100$ MHz: $\lambda = \dfrac{3 \times 10^8}{10^8} = 3$ m.
>
> Luz verde de $550$ nm: $f = \dfrac{3 \times 10^8}{550 \times 10^{-9}} = 5{,}45 \times 10^{14}$ Hz.

### Energía e intensidad

La onda transporta energía, repartida por igual entre ambos campos. El **vector de Poynting** da la potencia por unidad de área:

$$\vec{S} = \frac{1}{\mu_0}\,\vec{E} \times \vec{B}$$

La **intensidad** es su promedio:

$$I = \frac{E_0B_0}{2\mu_0} = \tfrac{1}{2}c\,\varepsilon_0E_0^2$$

Para una fuente puntual de potencia $P$ que emite en todas direcciones:

$$I = \frac{P}{4\pi r^2}$$

> [!NOTE]
> La luz solar llega a la Tierra con $I \approx 1360\ \text{W/m}^2$.
>
> $E_0 = \sqrt{\dfrac{2I}{c\,\varepsilon_0}} = \sqrt{\dfrac{2 \cdot 1360}{3 \times 10^8 \cdot 8{,}85 \times 10^{-12}}} \approx 1010$ V/m.

Las ondas también transportan cantidad de movimiento y ejercen **presión de radiación**: $p = \dfrac{I}{c}$ sobre una superficie absorbente y el doble sobre una reflectante. Es minúscula, pero empuja las colas de los cometas y las velas solares.

## El espectro electromagnético

Todas son la misma clase de onda; solo difieren en frecuencia.

| Región | Longitud de onda | Fuente o uso |
|---|---|---|
| Ondas de radio | > 1 m | radio, TV |
| Microondas | 1 mm – 1 m | telefonía, wifi, radar, hornos |
| Infrarrojo | 700 nm – 1 mm | calor, controles remotos, fibra óptica |
| Visible | 400 – 700 nm | visión |
| Ultravioleta | 10 – 400 nm | esterilización, bronceado |
| Rayos X | 0,01 – 10 nm | radiografías |
| Rayos gamma | < 0,01 nm | núcleos atómicos, radioterapia |

La luz visible es una franja mínima: del **violeta** (400 nm) al **rojo** (700 nm).

La energía de cada fotón es $E = hf$. Por eso las radiaciones de alta frecuencia (ultravioleta lejano, rayos X y gamma) son **ionizantes**: pueden arrancar electrones y dañar el ADN. Las de baja frecuencia solo calientan.

## Polarización

La dirección en que oscila $\vec{E}$ es la **polarización** de la onda.

- La luz natural no está polarizada: oscila en todas las direcciones perpendiculares al avance.
- Un **polarizador** deja pasar solo una dirección.

**Ley de Malus**: al atravesar un segundo polarizador girado un ángulo $\theta$:

$$I = I_0\cos^2\theta$$

Dos polarizadores cruzados ($90°$) bloquean toda la luz.

La luz reflejada en superficies horizontales queda parcialmente polarizada; los anteojos polarizados eliminan ese reflejo. Las pantallas LCD funcionan controlando la polarización.

## Antenas

Una antena emisora tiene cargas que oscilan y radian. En la receptora, el campo eléctrico de la onda mueve los electrones y genera una señal. El tamaño óptimo es del orden de la longitud de onda (típicamente $\lambda/2$ o $\lambda/4$).

> [!NOTE]
> Una antena de cuarto de onda para wifi de $2{,}4$ GHz: $\lambda = 12{,}5$ cm, así que mide unos $3$ cm.

## Importancia

Las ecuaciones de Maxwell unificaron electricidad, magnetismo y óptica. Como predicen la misma velocidad $c$ para todos los observadores, llevaron a Einstein a la relatividad especial.

> [!TIP]
> Toda la tecnología de comunicaciones (radio, telefonía, wifi, fibra óptica, GPS) es una aplicación de estas cuatro ecuaciones.
