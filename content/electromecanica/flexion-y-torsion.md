+++
title = "Flexión, corte y torsión"
summary = "Diagramas de esfuerzo de corte y momento flector, tensiones de flexión, flecha de vigas y torsión de ejes."
branch = "Estabilidad y resistencia de materiales"
order = 3
level = "Intermedio"
tags = ["flexión", "momento flector", "torsión", "viga", "flecha"]
+++

## Esfuerzos internos

Al cortar imaginariamente una barra aparecen las fuerzas con que una parte sostiene a la otra:

| Esfuerzo | Símbolo | Efecto |
|---|---|---|
| Normal | $N$ | estira o comprime |
| Corte | $V$ | tiende a deslizar una sección sobre otra |
| Momento flector | $M$ | curva la barra |
| Momento torsor | $T$ | la retuerce |

## Diagramas de corte y momento

![Diagramas de esfuerzo de corte y de momento flector de una viga simplemente apoyada con carga puntual](/media/electromecanica/diagramas-viga.svg)

*El momento flector es máximo donde el esfuerzo de corte cambia de signo.*

Los diagramas muestran cómo varían $V$ y $M$ a lo largo de la viga. Están ligados por

$$\frac{dV}{dx} = -q,\qquad \frac{dM}{dx} = V$$

así que el momento es máximo donde el corte se anula. Casos de referencia:

| Viga | Momento máximo | Flecha máxima |
|---|---|---|
| Apoyada, carga $P$ en el centro | $\dfrac{PL}{4}$ | $\dfrac{PL^3}{48EI}$ |
| Apoyada, carga uniforme $q$ | $\dfrac{qL^2}{8}$ | $\dfrac{5qL^4}{384EI}$ |
| En voladizo, carga $P$ en el extremo | $PL$ | $\dfrac{PL^3}{3EI}$ |
| En voladizo, carga uniforme $q$ | $\dfrac{qL^2}{2}$ | $\dfrac{qL^4}{8EI}$ |

## Tensiones de flexión

Al flexionarse, las fibras de un lado se alargan y las del otro se acortan; entre ambas queda la **línea neutra**, que pasa por el centroide. La tensión crece linealmente con la distancia $y$ a ella (fórmula de Navier):

$$\sigma = \frac{M\,y}{I},\qquad \sigma_{max} = \frac{M}{W},\quad W = \frac{I}{y_{max}}$$

$W$ es el **módulo resistente**. Para un rectángulo, $W = bh^2/6$: la altura cuenta al cuadrado, por eso las vigas se ponen "de canto" y los perfiles doble T concentran el material lejos del eje neutro.

> [!NOTE]
> Viga de acero apoyada de 4 m con 20 kN en el centro; tensión admisible 140 MPa.
>
> $M_{max} = \dfrac{20\cdot 4}{4} = 20$ kN·m.
>
> $W \ge \dfrac{20\cdot10^{6}\ \text{N·mm}}{140\ \text{N/mm}^2}\approx 143\ \text{cm}^3$. Un perfil IPN 180 ($W = 161\ \text{cm}^3$) alcanza.

## Tensiones de corte

La tensión tangencial por corte se distribuye según la fórmula de Jourawski, $\tau = \dfrac{V S}{I b}$. En una sección rectangular es parabólica, con máximo en el eje neutro igual a $1{,}5\,V/A$. En vigas esbeltas suele ser secundaria frente a la flexión.

## Deformación

La curvatura de la viga es proporcional al momento:

$$EI\,\frac{d^2y}{dx^2} = M(x)$$

Integrando dos veces se obtiene la **elástica**. Muchas veces el diseño lo decide la flecha admisible (del orden de $L/300$ a $L/500$) y no la tensión.

## Torsión de ejes circulares

Un par torsor $T$ produce tensiones tangenciales que crecen desde cero en el centro hasta el máximo en la superficie:

$$\tau_{max} = \frac{T\,r}{J} = \frac{16\,T}{\pi d^3},\qquad J = \frac{\pi d^4}{32}$$

y un ángulo de torsión

$$\theta = \frac{T\,L}{G\,J}$$

con $G$ el módulo de elasticidad transversal (unos 80 GPa en acero). Un eje que transmite una potencia $P$ a velocidad angular $\omega$ soporta $T = P/\omega$.

> [!NOTE]
> Eje que transmite 30 kW a 1500 rpm: $\omega = 157$ rad/s y $T = 191$ N·m.
>
> Con $\tau_{adm} = 40$ MPa: $d \ge \sqrt[3]{\dfrac{16\cdot 191\,000}{\pi\cdot 40}}\approx 29$ mm.

Como el material del centro casi no trabaja, un eje **hueco** resiste casi lo mismo con mucho menos peso.

## Esfuerzos combinados

Los ejes de máquinas soportan flexión y torsión a la vez. Las tensiones se combinan con el círculo de Mohr o con una teoría de falla; para materiales dúctiles se usa la tensión equivalente de von Mises:

$$\sigma_{eq} = \sqrt{\sigma^2 + 3\tau^2}$$
