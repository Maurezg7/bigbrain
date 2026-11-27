+++
title = "Trigonometría y la circunferencia unitaria"
summary = "Seno, coseno y tangente definidos sobre un círculo de radio 1, con sus identidades fundamentales."
branch = "Geometría y trigonometría"
order = 1
level = "Básico"
tags = ["seno", "coseno", "ángulos", "radianes"]
+++

## Razones en el triángulo rectángulo

Para un ángulo agudo $\theta$ de un triángulo rectángulo:

$$\sin\theta = \frac{\text{opuesto}}{\text{hipotenusa}}, \qquad \cos\theta = \frac{\text{adyacente}}{\text{hipotenusa}}, \qquad \tan\theta = \frac{\text{opuesto}}{\text{adyacente}}$$

## La circunferencia unitaria

Para extender estas razones a cualquier ángulo se usa una circunferencia de radio 1 centrada en el origen. Si giramos un ángulo $\theta$ desde el eje $x$ positivo (en sentido antihorario), el punto donde llegamos tiene coordenadas $(\cos\theta, \sin\theta)$.

![Circunferencia unitaria con un ángulo, su seno y su coseno](/media/matematicas/circulo-unitario.svg)

*El coseno es la coordenada horizontal del punto $P$ y el seno la vertical.*

De ahí salen directamente los signos por cuadrante y la **identidad pitagórica**:

$$\sin^2\theta + \cos^2\theta = 1$$

## Radianes

Un **radián** es el ángulo que abarca un arco de longitud igual al radio. Una vuelta completa mide $2\pi$ radianes, así que $180° = \pi$ rad. En cálculo siempre se trabaja en radianes.

| Grados | 0° | 30° | 45° | 60° | 90° |
|---|---|---|---|---|---|
| Radianes | $0$ | $\pi/6$ | $\pi/4$ | $\pi/3$ | $\pi/2$ |
| $\sin$ | $0$ | $1/2$ | $\sqrt2/2$ | $\sqrt3/2$ | $1$ |
| $\cos$ | $1$ | $\sqrt3/2$ | $\sqrt2/2$ | $1/2$ | $0$ |

## Identidades que más se usan

- Suma de ángulos: $\sin(\alpha \pm \beta) = \sin\alpha\cos\beta \pm \cos\alpha\sin\beta$
- $\cos(\alpha \pm \beta) = \cos\alpha\cos\beta \mp \sin\alpha\sin\beta$
- Ángulo doble: $\sin 2\theta = 2\sin\theta\cos\theta$ y $\cos 2\theta = \cos^2\theta - \sin^2\theta$

## Triángulos cualesquiera

Para un triángulo de lados $a, b, c$ opuestos a los ángulos $A, B, C$:

$$\frac{a}{\sin A} = \frac{b}{\sin B} = \frac{c}{\sin C} \qquad \text{(teorema del seno)}$$

$$c^2 = a^2 + b^2 - 2ab\cos C \qquad \text{(teorema del coseno)}$$

> [!NOTE]
> Una escalera de 5 m se apoya en una pared formando 60° con el piso. ¿A qué altura llega?
>
> La altura es el cateto opuesto: $h = 5 \sin 60° = 5 \cdot \dfrac{\sqrt3}{2} \approx 4{,}33$ m.

> [!WARNING]
> Revisá el modo de la calculadora (DEG o RAD). $\sin 30$ vale $0{,}5$ en grados y $-0{,}988$ en radianes.
