+++
title = "Turbinas hidráulicas"
summary = "Pelton, Francis y Kaplan: potencia de un salto, triángulos de velocidades, selección y aprovechamientos hidroeléctricos."
branch = "Máquinas térmicas e hidráulicas"
order = 2
level = "Intermedio"
tags = ["turbinas", "Pelton", "Francis", "Kaplan", "hidroelectricidad"]
+++

## Potencia de un salto

Una turbina hidráulica convierte la energía del agua en trabajo en un eje. La potencia disponible depende del caudal $Q$ y del salto neto $H$:

$$P = \eta\,\rho\,g\,Q\,H$$

El **salto neto** es el desnivel bruto menos las pérdidas en la conducción. El rendimiento de las turbinas grandes supera el 90 %.

> [!NOTE]
> Un salto neto de 120 m con $15\ \text{m}^3$/s y rendimiento global 0,88 entrega
>
> $P = 0{,}88\cdot 1000\cdot 9{,}81\cdot 15\cdot 120\approx 15{,}5$ MW.

## Acción y reacción

| | De acción | De reacción |
|---|---|---|
| Dónde se transforma la presión en velocidad | por completo en la tobera, antes del rodete | en parte dentro del rodete |
| Presión en el rodete | atmosférica | variable, mayor a la entrada |
| Admisión | parcial | total |
| Ejemplos | Pelton | Francis, Kaplan |

## Turbina Pelton

![Rodete Pelton con un chorro de agua que incide sobre las cucharas](/media/electromecanica/pelton.svg)

*Turbina Pelton: el chorro se divide en la arista de la cuchara y sale casi en sentido contrario.*

Uno o varios **inyectores** lanzan chorros a gran velocidad sobre las cucharas del rodete. La velocidad del chorro es

$$c_1 = C_v\sqrt{2gH}$$

con $C_v\approx 0{,}98$. La cuchara desvía el chorro casi 180° y el máximo rendimiento se obtiene cuando la velocidad periférica es aproximadamente la mitad de la del chorro:

$$u\approx 0{,}46\,c_1$$

La aguja del inyector regula el caudal manteniendo buen rendimiento con cargas parciales. Se usa en **grandes saltos** con poco caudal, típicos de la cordillera.

## Turbina Francis

El agua entra radialmente por una **cámara espiral**, pasa por el **distribuidor** de álabes orientables, que regula el caudal, y sale axialmente hacia el **tubo de aspiración**. Este recupera parte de la energía cinética de salida y permite aprovechar el desnivel hasta el nivel de restitución. Es la más versátil y la más usada en saltos medios.

## Turbina Kaplan

Es una hélice de eje vertical cuyas palas cambian de inclinación en marcha, igual que los álabes del distribuidor. Esa **doble regulación** mantiene alto el rendimiento en un amplio rango de caudales. Se usa en **saltos bajos** con caudales muy grandes. La variante **bulbo**, de eje horizontal, se instala en ríos de llanura.

## Selección

| Turbina | Salto | Caudal | Velocidad específica |
|---|---|---|---|
| Pelton | 150 a 1800 m | bajo | baja |
| Francis | 30 a 600 m | medio | media |
| Kaplan | 5 a 70 m | alto | alta |

La **velocidad específica** resume en un número la forma de rodete más conveniente:

$$n_s = \frac{n\sqrt P}{H^{5/4}}$$

con $n$ en rpm, $P$ en kW (históricamente en CV) y $H$ en m.

## Triángulos de velocidades

En cualquier turbomáquina, la velocidad absoluta del fluido $\vec c$ es la suma de la velocidad periférica del rodete $\vec u$ y la velocidad relativa al álabe $\vec w$:

$$\vec c = \vec u + \vec w$$

La ecuación de Euler da la altura aprovechada:

$$H_u = \frac{u_1\,c_{u1} - u_2\,c_{u2}}{g}$$

El mejor diseño hace que el agua salga sin componente tangencial ($c_{u2} = 0$), para no desperdiciar energía en giro.

## Cavitación

En las turbinas de reacción, la presión a la salida del rodete es inferior a la atmosférica. Si baja demasiado, el agua cavita y erosiona los álabes. Se controla con la altura de instalación respecto del nivel de aguas abajo: las Kaplan suelen quedar por debajo de ese nivel.

## Regulación

La red eléctrica exige que el generador gire a velocidad constante. Un **regulador** ajusta el caudal según la carga actuando sobre la aguja, el distribuidor o las palas. Cerrar rápido provocaría un golpe de ariete en la tubería forzada; por eso las Pelton tienen un **deflector** que desvía el chorro y las centrales con conducciones largas llevan una **chimenea de equilibrio**.

## Tipos de central

| Tipo | Característica |
|---|---|
| De pasada | usa el caudal del río tal como llega, sin embalse |
| De embalse | almacena agua y regula la generación |
| De bombeo | bombea a un embalse superior en horas de baja demanda y turbina en las de punta |

Las centrales de bombeo son hoy la principal forma de almacenar energía eléctrica a gran escala. En Argentina, grandes aprovechamientos como Yacyretá y Salto Grande usan turbinas Kaplan; El Chocón y Piedra del Águila, Francis.
