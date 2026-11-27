+++
title = "Ecuaciones de Maxwell"
summary = "Las cuatro leyes que resumen todo el electromagnetismo, sus consecuencias y las ondas electromagnéticas."
branch = "Electromagnetismo"
order = 1
level = "Avanzado"
tags = ["Maxwell", "Gauss", "Faraday", "Ampère", "ondas"]
+++

## Campos y fuentes

El electromagnetismo describe dos campos vectoriales, el eléctrico $\vec E$ y el magnético $\vec B$, creados por cargas y corrientes. Una carga $q$ que se mueve con velocidad $\vec v$ siente la **fuerza de Lorentz**:

$$\vec F = q\left(\vec E + \vec v \times \vec B\right)$$

En medios materiales se usan además $\vec D = \varepsilon\vec E$ y $\vec H = \vec B/\mu$, con $\varepsilon = \varepsilon_r\varepsilon_0$ y $\mu = \mu_r\mu_0$.

## Las cuatro ecuaciones

| Ley | Forma integral | Qué dice |
|---|---|---|
| Gauss eléctrica | $\displaystyle\oint \vec D\cdot d\vec S = Q_{enc}$ | las cargas son fuentes del campo eléctrico |
| Gauss magnética | $\displaystyle\oint \vec B\cdot d\vec S = 0$ | no existen polos magnéticos aislados |
| Faraday | $\displaystyle\oint \vec E\cdot d\vec l = -\frac{d\Phi_B}{dt}$ | un flujo magnético variable induce tensión |
| Ampère-Maxwell | $\displaystyle\oint \vec H\cdot d\vec l = I_{enc} + \frac{d\Phi_D}{dt}$ | corrientes y campos eléctricos variables crean campo magnético |

En forma diferencial:

$$\nabla\cdot\vec D = \rho,\qquad \nabla\cdot\vec B = 0,\qquad \nabla\times\vec E = -\frac{\partial\vec B}{\partial t},\qquad \nabla\times\vec H = \vec J + \frac{\partial\vec D}{\partial t}$$

## De las ecuaciones a las máquinas

Cada ley sostiene una parte de la ingeniería eléctrica:

- **Gauss** permite calcular el campo y la capacidad de cables y aisladores, y fija cuándo un aislante se perfora (rigidez dieléctrica).
- **Ampère** da el campo dentro de una bobina o un núcleo: $H\,l = N I$.
- **Faraday** es el principio del transformador y del generador: $e = -N\,\dfrac{d\Phi}{dt}$.
- **Lorentz**, aplicada a un conductor de longitud $l$ con corriente $I$, da la fuerza que mueve a los motores: $F = B\,I\,l$.

> [!NOTE]
> Campo a 5 cm de un conductor recto largo con 100 A. Por la ley de Ampère, sobre una circunferencia de radio $r$:
>
> $B = \dfrac{\mu_0 I}{2\pi r} = \dfrac{4\pi\cdot10^{-7}\cdot 100}{2\pi\cdot 0{,}05} = 4\cdot10^{-4}$ T, unas ocho veces el campo terrestre.

## Ley de Lenz

El signo menos de la ley de Faraday indica que la tensión inducida se opone a la causa que la produce. De ahí salen:

- las **corrientes parásitas** (de Foucault), que calientan los núcleos macizos y obligan a construirlos con chapas aisladas;
- el frenado magnético y el calentamiento por inducción, que las aprovechan.

## Ondas electromagnéticas

El término $\partial\vec D/\partial t$ que añadió Maxwell hace que un campo eléctrico variable genere uno magnético y viceversa: la perturbación se propaga como onda a la velocidad

$$c = \frac{1}{\sqrt{\mu_0\varepsilon_0}} \approx 3\cdot10^{8}\ \text{m/s}$$

La energía viaja en la dirección del **vector de Poynting** $\vec S = \vec E\times\vec H$, en W/m².

## Efecto pelicular

En corriente alterna la corriente se concentra cerca de la superficie del conductor. La **profundidad de penetración** es

$$\delta = \sqrt{\frac{2}{\omega\mu\sigma}}$$

En cobre a 50 Hz vale unos 9 mm, por lo que las barras de gran sección se hacen huecas o planas, y los conductores de alta frecuencia, de muchos hilos finos aislados entre sí.
