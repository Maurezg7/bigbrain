+++
title = "Hidrostática, continuidad y Bernoulli"
summary = "Propiedades de los fluidos, presión, empuje, ecuación de continuidad, ecuación de Bernoulli y medición de caudal."
branch = "Mecánica de los fluidos"
order = 1
level = "Básico"
tags = ["presión", "Bernoulli", "continuidad", "caudal", "Venturi"]
+++

## Propiedades de los fluidos

Un fluido es una sustancia que se deforma continuamente ante un esfuerzo de corte, por pequeño que sea. Líquidos y gases lo son.

| Propiedad | Símbolo | Agua a 20 °C | Aire a 20 °C |
|---|---|---|---|
| Densidad | $\rho$ | $998\ \text{kg/m}^3$ | $1{,}2\ \text{kg/m}^3$ |
| Viscosidad dinámica | $\mu$ | $1{,}0\cdot10^{-3}$ Pa·s | $1{,}8\cdot10^{-5}$ Pa·s |
| Viscosidad cinemática | $\nu = \mu/\rho$ | $1{,}0\cdot10^{-6}\ \text{m}^2$/s | $1{,}5\cdot10^{-5}\ \text{m}^2$/s |

La **viscosidad** es la resistencia interna a fluir. En un fluido newtoniano, el esfuerzo de corte es proporcional al gradiente de velocidad: $\tau = \mu\,\dfrac{du}{dy}$.

## Presión

La presión es fuerza por unidad de área y en un fluido en reposo actúa igual en todas las direcciones. Aumenta con la profundidad:

$$p = p_0 + \rho\,g\,h$$

Cada 10 m de agua agregan aproximadamente 1 bar.

| Unidad | Equivalencia |
|---|---|
| 1 bar | $10^5$ Pa |
| 1 atm | 101 325 Pa |
| 1 kgf/cm² | 98 067 Pa |
| 1 psi | 6895 Pa |
| 1 m de columna de agua | 9807 Pa |

La presión **absoluta** se mide desde el vacío; la **manométrica**, desde la atmosférica: $p_{abs} = p_{man} + p_{atm}$.

## Principios de Pascal y de Arquímedes

- **Pascal**: la presión aplicada a un fluido encerrado se transmite íntegra a todos sus puntos. En una prensa hidráulica, $\dfrac{F_1}{A_1} = \dfrac{F_2}{A_2}$: un pistón con 100 veces más área ejerce 100 veces más fuerza.
- **Arquímedes**: un cuerpo sumergido recibe un empuje igual al peso del fluido que desplaza, $E = \rho\,g\,V$.

La fuerza sobre una pared plana sumergida es la presión en su centroide por el área, $F = \rho g\,h_G\,A$, y actúa en el **centro de presión**, siempre más abajo que el centroide.

## Ecuación de continuidad

La masa se conserva. Para un fluido incompresible, el caudal volumétrico es el mismo en todas las secciones:

$$Q = A_1 v_1 = A_2 v_2$$

Si el caño se angosta a la mitad del diámetro, la velocidad se cuadruplica.

## Ecuación de Bernoulli

![Tubo de Venturi con la sección que se estrecha y la presión que baja en la garganta](/media/electromecanica/venturi.svg)

*En el estrechamiento la velocidad aumenta y la presión disminuye.*

Para un fluido ideal (sin rozamiento) e incompresible en régimen permanente, la energía por unidad de peso se conserva a lo largo de una línea de corriente:

$$\frac{p}{\rho g} + \frac{v^2}{2g} + z = \text{constante}$$

Los tres términos son alturas: de **presión**, de **velocidad** y de **posición**. Su suma es la altura total.

> [!NOTE]
> Un tanque abierto descarga por un orificio ubicado 5 m por debajo de la superficie. Aplicando Bernoulli entre la superficie y la salida (teorema de Torricelli):
>
> $v = \sqrt{2gh} = \sqrt{2\cdot 9{,}81\cdot 5}\approx 9{,}9$ m/s.

En fluidos reales hay pérdidas por rozamiento y máquinas que agregan o extraen energía. La ecuación se generaliza:

$$\frac{p_1}{\rho g} + \frac{v_1^2}{2g} + z_1 + H_{bomba} = \frac{p_2}{\rho g} + \frac{v_2^2}{2g} + z_2 + H_{turbina} + h_{pérdidas}$$

## Medición de caudal

| Medidor | Principio |
|---|---|
| Tubo de Venturi | caída de presión en un estrechamiento suave |
| Placa orificio | caída de presión en un orificio; simple, con más pérdida |
| Tubo de Pitot | diferencia entre presión total y estática: $v = \sqrt{2\Delta p/\rho}$ |
| Rotámetro | flotador en un tubo cónico |
| Electromagnético | tensión inducida en un líquido conductor |
| Ultrasónico | tiempo de tránsito de una onda |
| Coriolis | mide directamente el caudal másico |

Para un Venturi con secciones $A_1$ y $A_2$ y coeficiente de descarga $C_d\approx 0{,}98$:

$$Q = C_d\,A_2\sqrt{\frac{2\,(p_1 - p_2)}{\rho\left[1 - (A_2/A_1)^2\right]}}$$

## Cantidad de movimiento

Un chorro que cambia de dirección o de velocidad ejerce una fuerza:

$$\vec F = \rho\,Q\,(\vec v_2 - \vec v_1)$$

Es la base del cálculo de fuerzas en codos y toberas y del funcionamiento de las turbomáquinas.

> [!WARNING]
> Si la presión local cae por debajo de la presión de vapor del líquido, se forman burbujas que colapsan con violencia al llegar a zonas de mayor presión: es la **cavitación**, que erosiona rodetes de bombas, hélices y válvulas.
