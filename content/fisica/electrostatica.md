+++
title = "Carga eléctrica, campo y potencial"
summary = "La ley de Coulomb, el campo eléctrico como intermediario de la fuerza y el potencial como energía por carga."
branch = "Electromagnetismo"
order = 1
level = "Intermedio"
tags = ["Coulomb", "campo eléctrico", "potencial", "Gauss"]
+++

## Carga eléctrica

La carga es una propiedad de la materia que existe en dos tipos, positiva y negativa. Cargas del mismo signo se repelen y de signo opuesto se atraen. Está **cuantizada** (viene en múltiplos de $e = 1{,}602 \times 10^{-19}$ C) y **se conserva**.

## Ley de Coulomb

La fuerza entre dos cargas puntuales separadas una distancia $r$ es

$$F = k\,\frac{|q_1 q_2|}{r^2}, \qquad k = \frac{1}{4\pi\varepsilon_0} \approx 8{,}99 \times 10^{9}\ \text{N·m}^2/\text{C}^2$$

dirigida a lo largo de la recta que las une. Como la gravedad, decae con el cuadrado de la distancia.

## Campo eléctrico

En lugar de pensar en fuerzas a distancia, se dice que cada carga crea a su alrededor un **campo eléctrico** $\vec E$, y que ese campo empuja a las demás cargas:

$$\vec F = q\,\vec E \qquad\qquad E = k\,\frac{|Q|}{r^2} \;\text{ (carga puntual)}$$

![Líneas de campo eléctrico entre una carga positiva y una negativa](/media/fisica/campo-electrico.svg)

*Las líneas de campo salen de las cargas positivas y entran en las negativas; donde están más juntas el campo es más intenso.*

Para varias cargas vale el **principio de superposición**: el campo total es la suma vectorial de los campos individuales.

## Ley de Gauss

El flujo del campo eléctrico a través de cualquier superficie cerrada es proporcional a la carga que encierra:

$$\oint \vec E \cdot d\vec A = \frac{Q_{\text{enc}}}{\varepsilon_0}$$

Con ella se calcula el campo de distribuciones simétricas (esferas, cilindros, planos) casi sin integrar.

## Potencial eléctrico

El **potencial** $V$ es la energía potencial por unidad de carga, medida en volts (1 V = 1 J/C):

$$V = k\,\frac{Q}{r} \;\text{ (carga puntual)} \qquad\qquad U = qV$$

El campo apunta hacia donde el potencial disminuye. En un campo uniforme, $E = \dfrac{\Delta V}{d}$.

> [!NOTE]
> Dos cargas de $+2\ \mu\text{C}$ y $-3\ \mu\text{C}$ están a 30 cm. La fuerza entre ellas es
>
> $$F = 8{,}99 \times 10^{9} \cdot \frac{(2 \times 10^{-6})(3 \times 10^{-6})}{0{,}3^2} \approx 0{,}6\ \text{N}$$
>
> y es atractiva, porque las cargas tienen signos opuestos.

> [!WARNING]
> La fuerza y el campo son vectores (se suman por componentes); el potencial es un escalar (se suma con su signo). Confundirlos es el error más común en estos problemas.
