+++
title = "Introducción a la física moderna"
summary = "Relatividad especial y los primeros pasos de la cuántica: lo que cambia a altas velocidades y a escala atómica."
branch = "Física moderna"
order = 1
level = "Avanzado"
tags = ["relatividad", "cuántica", "fotón", "dualidad"]
+++

## Por qué hizo falta

A fines del siglo XIX la mecánica de Newton y el electromagnetismo de Maxwell parecían explicarlo todo. Pero fallaban en dos extremos: a velocidades cercanas a la de la luz y a escala atómica. De ahí nacieron la **relatividad** y la **mecánica cuántica**.

## Relatividad especial

Einstein (1905) partió de dos postulados:

1. Las leyes de la física son las mismas en todos los sistemas de referencia inerciales.
2. La velocidad de la luz en el vacío, $c$, es la misma para todos los observadores.

Las consecuencias se expresan con el **factor de Lorentz**:

$$\gamma = \frac{1}{\sqrt{1 - v^2/c^2}}$$

| Efecto | Fórmula | Significado |
|---|---|---|
| Dilatación del tiempo | $\Delta t = \gamma\,\Delta t_0$ | un reloj en movimiento marcha más lento |
| Contracción de longitudes | $L = L_0/\gamma$ | un objeto en movimiento se acorta en la dirección del movimiento |
| Energía | $E = \gamma m c^2$ | la masa en reposo equivale a una energía $E_0 = mc^2$ |

> [!NOTE]
> Una nave viaja a $v = 0{,}8c$. Entonces $\gamma = \dfrac{1}{\sqrt{1 - 0{,}64}} = \dfrac{1}{0{,}6} \approx 1{,}67$.
> Por cada hora que pasa a bordo, en la Tierra pasan 1 h 40 min.

A velocidades cotidianas $\gamma \approx 1$ y se recupera la física de Newton.

## Cuantización de la energía

Planck (1900) y Einstein (1905) mostraron que la luz intercambia energía en paquetes, los **fotones**, de energía

$$E = hf, \qquad h = 6{,}626 \times 10^{-34}\ \text{J·s}$$

Esto explica el **efecto fotoeléctrico**: la luz arranca electrones de un metal solo si su frecuencia supera un umbral, sin importar su intensidad. La energía cinética máxima de los electrones es $E_c = hf - \phi$, donde $\phi$ es el trabajo de extracción del metal.

## Dualidad onda-partícula

De Broglie propuso que, así como la luz se comporta como partícula, la materia se comporta como onda de longitud

$$\lambda = \frac{h}{p} = \frac{h}{mv}$$

Para objetos macroscópicos $\lambda$ es absurdamente pequeña; para electrones es del tamaño de los átomos, y por eso se difractan en los cristales.

## Principio de incertidumbre

No se pueden conocer a la vez con precisión arbitraria la posición y la cantidad de movimiento de una partícula:

$$\Delta x\,\Delta p \ge \frac{\hbar}{2}, \qquad \hbar = \frac{h}{2\pi}$$

No es un límite de los instrumentos sino una propiedad de la naturaleza. En lugar de trayectorias, la cuántica describe cada partícula con una **función de onda** $\psi$, cuyo módulo al cuadrado $|\psi|^2$ da la probabilidad de encontrarla en cada lugar.
