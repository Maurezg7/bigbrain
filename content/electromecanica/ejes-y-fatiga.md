+++
title = "Ejes, árboles y fatiga"
summary = "Diseño de ejes: cargas, concentración de tensiones, curva de Wöhler, límite de fatiga, criterio de Goodman y chavetas."
branch = "Mecánica y elementos de máquinas"
order = 3
level = "Avanzado"
tags = ["ejes", "fatiga", "Wöhler", "concentración de tensiones", "chaveta"]
+++

## Qué carga soporta un eje

Un **árbol** transmite par entre elementos (poleas, engranajes, acoplamientos); un **eje** propiamente dicho solo los soporta. En general actúan a la vez:

- **torsión**, por la potencia transmitida: $T = P/\omega$;
- **flexión**, por las fuerzas de engranajes y correas y por los pesos;
- a veces cargas axiales.

Como el eje gira, cada fibra pasa de tracción a compresión en cada vuelta: la flexión es **alternante** aunque las cargas sean constantes. Por eso los ejes fallan casi siempre por fatiga.

## Fatiga

![Curva de Wöhler con la tensión en función del número de ciclos y el límite de fatiga](/media/electromecanica/wohler.svg)

*Curva S-N del acero: por debajo del límite de fatiga la pieza soporta infinitos ciclos.*

Un material sometido a tensiones variables puede romperse con valores muy inferiores a su resistencia estática. La rotura tiene tres etapas: nace una fisura en un punto de concentración de tensiones, crece un poco en cada ciclo y, cuando la sección restante no alcanza, rompe de golpe y **sin deformación previa**.

La **curva de Wöhler** relaciona la amplitud de tensión con el número de ciclos hasta la rotura. Los aceros tienen un **límite de fatiga** $S_e'$ por debajo del cual la vida es infinita, aproximadamente

$$S_e'\approx 0{,}5\,\sigma_R$$

El aluminio no tiene ese límite: siempre termina fallando.

## Del ensayo a la pieza real

El límite de la probeta pulida se reduce por varios factores:

$$S_e = k_a\,k_b\,k_c\,k_d\,k_e\,S_e'$$

| Factor | Considera |
|---|---|
| $k_a$ | terminación superficial: una superficie rugosa inicia fisuras |
| $k_b$ | tamaño: las piezas grandes resisten menos |
| $k_c$ | tipo de carga: flexión, axial o torsión |
| $k_d$ | temperatura |
| $k_e$ | confiabilidad exigida |

## Concentración de tensiones

En cambios de sección, chaveteros, agujeros y roscas la tensión local supera a la nominal:

$$\sigma_{max} = K_t\,\sigma_{nom}$$

$K_t$ depende solo de la geometría y puede valer 2 o 3. En fatiga se usa $K_f = 1 + q\,(K_t - 1)$, con $q$ la sensibilidad a la entalla del material.

> [!TIP]
> El recurso más barato para alargar la vida de un eje es aumentar los **radios de acuerdo** en los cambios de diámetro y cuidar la terminación superficial.

## Tensión media y criterio de Goodman

Cuando además de la componente alternante $\sigma_a$ hay una tensión media $\sigma_m$, el coeficiente de seguridad $n$ se obtiene de

$$\frac{\sigma_a}{S_e} + \frac{\sigma_m}{\sigma_R} = \frac{1}{n}$$

## Diámetro de un eje

Para flexión alternante $M$ y torsión constante $T$, con von Mises y Goodman:

$$d = \sqrt[3]{\frac{16\,n}{\pi}\left(\frac{2K_f M}{S_e} + \frac{\sqrt3\,K_{fs} T}{\sigma_R}\right)}$$

> [!NOTE]
> Eje con $M = 200$ N·m, $T = 300$ N·m, acero con $\sigma_R = 600$ MPa y $S_e = 200$ MPa, $K_f = 1{,}8$, $K_{fs} = 1{,}5$ y $n = 2$.
>
> $\dfrac{2\cdot 1{,}8\cdot 200\,000}{200} = 3600$ y $\dfrac{\sqrt3\cdot 1{,}5\cdot 300\,000}{600}\approx 1299$ (en $\text{mm}^3$).
>
> $d = \sqrt[3]{\dfrac{32}{\pi}\cdot 4899}\approx 36{,}8$ mm; se adopta 40 mm.

Además de la resistencia se verifican la **rigidez** (flecha e inclinación en los apoyos, ángulo de torsión) y la **velocidad crítica**.

## Chavetas y uniones eje-cubo

La chaveta transmite el par trabajando a corte y a aplastamiento. Con diámetro $d$, ancho $b$, alto $h$ y longitud $l$:

$$\tau = \frac{2T}{d\,b\,l},\qquad \sigma_{apl} = \frac{4T}{d\,h\,l}$$

Se la diseña para que sea el elemento más débil: es barata y fácil de cambiar. Otras uniones son los ejes estriados, los ajustes por interferencia y los anillos de apriete cónicos.
