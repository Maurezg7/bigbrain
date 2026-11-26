+++
title = "Rodamientos y lubricación"
summary = "Tipos de rodamientos, vida nominal, selección y montaje; cojinetes de deslizamiento, viscosidad y regímenes de lubricación."
branch = "Mecánica y elementos de máquinas"
order = 5
level = "Intermedio"
tags = ["rodamientos", "lubricación", "vida útil", "viscosidad", "cojinetes"]
+++

## Apoyar un eje que gira

Los apoyos deben sostener las cargas con el menor rozamiento posible. Hay dos familias: **rodamientos**, donde hay rodadura, y **cojinetes de deslizamiento**, donde el eje flota sobre una película de aceite.

## Tipos de rodamientos

| Rodamiento | Carga radial | Carga axial | Uso típico |
|---|---|---|---|
| Rígido de bolas | media | media, en ambos sentidos | motores eléctricos, uso general |
| De bolas de contacto angular | media | alta, en un sentido | husillos, bombas |
| De rodillos cilíndricos | alta | nula | motores grandes, reductores |
| De rodillos cónicos | alta | alta, en un sentido | ruedas, reductores |
| Oscilante de rodillos | muy alta | media | ejes que flexionan o desalineados |
| Axial de bolas | nula | alta | ganchos, mesas giratorias |
| De agujas | alta | nula | poco espacio radial |

## Vida nominal

Un rodamiento bien montado falla por **fatiga** de las pistas. La vida $L_{10}$ es la cantidad de millones de vueltas que alcanza o supera el 90 % de un lote:

$$L_{10} = \left(\frac{C}{P}\right)^{p},\qquad L_{10h} = \frac{10^6}{60\,n}\left(\frac{C}{P}\right)^{p}$$

$C$ es la capacidad de carga dinámica (de catálogo), $P$ la carga equivalente y $p = 3$ para bolas o $10/3$ para rodillos. Con cargas combinadas, $P = X F_r + Y F_a$.

> [!NOTE]
> Rodamiento 6208 ($C = 30{,}7$ kN) con $P = 3$ kN a 1500 rpm.
>
> $L_{10} = \left(\dfrac{30{,}7}{3}\right)^3\approx 1072$ millones de vueltas y $L_{10h} = \dfrac{10^6\cdot 1072}{60\cdot 1500}\approx 11\,900$ horas.
>
> Como la vida depende del cubo de la carga, duplicar la carga la reduce a la octava parte.

| Aplicación | Vida recomendada (h) |
|---|---|
| Electrodomésticos, uso intermitente | 2000 a 8000 |
| Máquinas de 8 horas diarias | 20 000 a 30 000 |
| Servicio continuo | 40 000 a 60 000 |
| Máquinas críticas | más de 100 000 |

## Montaje

- Un eje con dos apoyos lleva un rodamiento **fijo** y otro **libre** axialmente, para absorber la dilatación.
- El aro que gira respecto de la carga se monta con **apriete**; el otro, con ajuste deslizante.
- La fuerza de montaje se aplica sobre el aro que entra con apriete, nunca a través de los elementos rodantes.

> [!WARNING]
> La mayoría de los rodamientos no llega a su vida calculada por lubricación deficiente, contaminación, montaje incorrecto o desalineación, no por fatiga. También daña el paso de corriente eléctrica en motores con variador.

## Viscosidad

La propiedad clave de un lubricante es su **viscosidad**, la resistencia a fluir. Baja mucho al subir la temperatura. Los aceites industriales se clasifican por grado ISO VG (viscosidad cinemática en $\text{mm}^2$/s a 40 °C): un ISO VG 68 tiene unos 68 $\text{mm}^2$/s.

Las **grasas** son aceite retenido en un espesante: se quedan en su lugar y sellan, pero evacuan mal el calor. Lubrican la mayoría de los rodamientos.

## Regímenes de lubricación

![Curva de Stribeck del coeficiente de fricción con las zonas de lubricación límite, mixta e hidrodinámica](/media/electromecanica/stribeck.svg)

*Curva de Stribeck: el rozamiento es mínimo cuando recién se forma la película completa.*

| Régimen | Qué ocurre | Coeficiente de fricción |
|---|---|---|
| Límite | las superficies se tocan | 0,1 a 0,3 |
| Mixto | contacto parcial | 0,01 a 0,1 |
| Hidrodinámico | una película completa separa las superficies | 0,001 a 0,01 |

En un cojinete **hidrodinámico**, el propio giro del eje arrastra aceite hacia una cuña convergente y genera la presión que lo sostiene. No hay desgaste en régimen, pero sí en el arranque y la parada. Se usan en turbinas, grandes motores y cigüeñales por su capacidad de carga, amortiguamiento y vida prácticamente ilimitada.

## Sellado

Retenes, laberintos y sellos mecánicos mantienen el lubricante adentro y la suciedad afuera. La limpieza del lubricante condiciona la vida de rodamientos y engranajes tanto como la carga.
