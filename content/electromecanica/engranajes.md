+++
title = "Engranajes"
summary = "Ley del engrane, perfil de evolvente, módulo, relación de transmisión, trenes, fuerzas en los dientes y verificación."
branch = "Mecánica y elementos de máquinas"
order = 4
level = "Intermedio"
tags = ["engranajes", "módulo", "relación de transmisión", "evolvente", "reductor"]
+++

## Para qué sirven

Los engranajes transmiten movimiento entre ejes con relación de velocidades **exacta**, sin deslizamiento. Casi siempre se usan para reducir la velocidad de un motor y multiplicar su par.

| Tipo | Ejes | Características |
|---|---|---|
| Cilíndricos de dientes rectos | paralelos | simples; ruidosos a alta velocidad |
| Cilíndricos helicoidales | paralelos | silenciosos, más carga; generan empuje axial |
| Cónicos | que se cortan | cambian la dirección |
| Sinfín y corona | que se cruzan | gran reducción en una etapa; rendimiento bajo |
| Planetarios | coaxiales | compactos, mucha reducción y par |

## Geometría

![Dos ruedas dentadas en contacto con sus circunferencias primitivas](/media/electromecanica/engranajes.svg)

*Las circunferencias primitivas ruedan una sobre otra sin deslizar.*

El tamaño de los dientes queda definido por el **módulo**:

$$m = \frac{d}{z}$$

donde $d$ es el diámetro primitivo y $z$ el número de dientes. Dos ruedas solo engranan si tienen el **mismo módulo** y el mismo ángulo de presión (20° en el sistema normalizado).

| Magnitud | Expresión |
|---|---|
| Diámetro primitivo | $d = m\,z$ |
| Paso | $p = \pi m$ |
| Diámetro exterior | $d_e = m\,(z + 2)$ |
| Altura del diente | $h = 2{,}25\,m$ |
| Distancia entre centros | $a = \dfrac{m\,(z_1 + z_2)}{2}$ |

El perfil de los dientes es una **evolvente** de círculo: mantiene constante la relación de velocidades aunque la distancia entre centros varíe un poco, y se talla con herramientas de flancos rectos.

## Relación de transmisión

$$i = \frac{n_1}{n_2} = \frac{z_2}{z_1} = \frac{d_2}{d_1}$$

Despreciando pérdidas, el par aumenta en la misma proporción: $T_2 = i\,T_1\,\eta$. En un **tren** de varias etapas, la relación total es el producto de las parciales.

> [!NOTE]
> Reductor de dos etapas: 20/80 dientes y 25/100 dientes. $i = 4\cdot 4 = 16$.
>
> Un motor de 1450 rpm y 50 N·m produce a la salida 90,6 rpm y, con rendimiento 0,96, unos 768 N·m.

En un tren **planetario** (sol, satélites, portasatélites y corona), con la corona fija y entrada por el sol, la reducción es $i = 1 + z_{corona}/z_{sol}$.

Para evitar la **interferencia** (socavado del pie del diente), un piñón recto de 20° tallado con cremallera necesita al menos 17 dientes.

## Fuerzas en el diente

La fuerza útil es la tangencial; el ángulo de presión $\alpha$ añade una componente radial que carga a los ejes y rodamientos:

$$F_t = \frac{2T}{d},\qquad F_r = F_t\tan\alpha$$

En los helicoidales aparece además una fuerza axial $F_a = F_t\tan\beta$, con $\beta$ el ángulo de la hélice.

## Verificación

Un diente puede fallar de dos maneras y hay que comprobar ambas:

- **rotura por flexión** en la raíz, como una viga en voladizo. La ecuación de Lewis da $\sigma = \dfrac{F_t}{b\,m\,Y}$, con $b$ el ancho y $Y$ el factor de forma, corregida por factores dinámicos y de servicio;
- **picado** (*pitting*) de los flancos por fatiga superficial, gobernado por la presión de contacto de Hertz.

Por eso los engranajes exigidos se fabrican en acero cementado y rectificado: superficie muy dura y núcleo tenaz.

## Rendimiento y lubricación

| Transmisión | Rendimiento por etapa |
|---|---|
| Cilíndricos y cónicos | 96 a 99 % |
| Planetario | 95 a 98 % |
| Sinfín y corona | 40 a 90 %, menor cuanto mayor la reducción |

Las pérdidas se convierten en calor que debe evacuar el aceite. Un sinfín de mucha reducción es **irreversible**: la corona no puede mover al tornillo, lo que sirve como freno en aparatos de elevación.
