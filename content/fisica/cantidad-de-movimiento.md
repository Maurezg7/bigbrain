+++
title = "Cantidad de movimiento y choques"
summary = "Impulso, conservación de la cantidad de movimiento, choques elásticos e inelásticos y centro de masa."
branch = "Mecánica"
order = 5
level = "Intermedio"
tags = ["cantidad de movimiento", "impulso", "choques", "centro de masa"]
+++

## Cantidad de movimiento

> [!IMPORTANT]
> $$\vec{p} = m\vec{v}$$

Es un **vector** con la dirección de la velocidad. Su unidad es kg·m/s. Mide cuánto cuesta detener un cuerpo: un camión lento puede tener más cantidad de movimiento que una bala.

La segunda ley de Newton en su forma general:

$$\vec{F}_{neta} = \frac{d\vec{p}}{dt}$$

## Impulso

$$\vec{J} = \int\vec{F}\,dt = \vec{F}_{media}\,\Delta t = \Delta\vec{p}$$

El **teorema del impulso** dice que el impulso de la fuerza neta es igual al cambio de cantidad de movimiento. Es el área bajo la curva fuerza-tiempo.

Para un mismo $\Delta p$, **alargar el tiempo reduce la fuerza**. Por eso funcionan los airbags, los cascos y flexionar las rodillas al caer.

> [!NOTE]
> Una pelota de $0{,}15$ kg llega a $40$ m/s y sale bateada a $50$ m/s en sentido opuesto. El contacto dura $2$ ms.
>
> $\Delta p = 0{,}15(50 - (-40)) = 13{,}5$ kg·m/s. $F_{media} = \dfrac{13{,}5}{0{,}002} = 6750$ N.

## Conservación

> [!IMPORTANT]
> Si la fuerza externa neta sobre un sistema es cero, su cantidad de movimiento total se conserva:
> $$\vec{p}_{antes} = \vec{p}_{después}$$

Sale de la tercera ley de Newton: las fuerzas internas son pares de acción y reacción y se cancelan.

Vale en choques y explosiones aunque haya fuerzas externas, porque durante el brevísimo instante de la interacción las fuerzas internas son enormes en comparación.

> [!NOTE]
> **Retroceso**: un fusil de $4$ kg dispara una bala de $10$ g a $800$ m/s.
>
> $0 = 0{,}01(800) + 4v \Rightarrow v = -2$ m/s. El fusil retrocede a $2$ m/s.

Es el principio de la **propulsión a reacción**: el cohete expulsa gases hacia atrás y gana cantidad de movimiento hacia adelante.

## Choques

En todo choque se conserva la cantidad de movimiento. La energía cinética, no siempre.

| Tipo | Cantidad de movimiento | Energía cinética | Ejemplo |
|---|---|---|---|
| Elástico | se conserva | se conserva | bolas de billar, átomos |
| Inelástico | se conserva | se pierde en parte | choque de autos |
| Perfectamente inelástico | se conserva | pérdida máxima | los cuerpos quedan pegados |

La energía «perdida» se transforma en calor, deformación y sonido.

### Perfectamente inelástico

$$m_1v_1 + m_2v_2 = (m_1 + m_2)\,v_f$$

> [!NOTE]
> Un vagón de $2000$ kg a $3$ m/s se acopla a otro de $1000$ kg en reposo.
>
> $v_f = \dfrac{2000 \cdot 3}{3000} = 2$ m/s.
>
> Energía inicial: $9000$ J. Final: $6000$ J. Se perdieron $3000$ J (un tercio).

### Elástico en una dimensión

Se conservan ambas magnitudes. Con el cuerpo 2 inicialmente en reposo:

$$v_1' = \frac{m_1 - m_2}{m_1 + m_2}\,v_1 \qquad v_2' = \frac{2m_1}{m_1 + m_2}\,v_1$$

| Caso | Resultado |
|---|---|
| $m_1 = m_2$ | intercambian velocidades: el primero se detiene |
| $m_1 \gg m_2$ | el liviano sale a casi $2v_1$ |
| $m_1 \ll m_2$ | el liviano rebota con casi la misma rapidez |

### Coeficiente de restitución

$$e = \frac{v_2' - v_1'}{v_1 - v_2}$$

$e = 1$ para un choque elástico y $e = 0$ para uno perfectamente inelástico.

### Choques en dos dimensiones

La cantidad de movimiento se conserva **en cada eje por separado**:

$$\sum p_x \text{ antes} = \sum p_x \text{ después} \qquad \sum p_y \text{ antes} = \sum p_y \text{ después}$$

> [!WARNING]
> La cantidad de movimiento es un vector: hay que cuidar los signos. Una pelota que rebota en una pared cambia su $p$ en $2mv$, no en cero.

## Péndulo balístico

Una bala de masa $m$ se incrusta en un bloque de masa $M$ colgado, que sube una altura $h$.

1. **Choque** (se conserva $p$, no la energía): $mv = (m + M)V$.
2. **Subida** (se conserva la energía, no $p$): $\tfrac{1}{2}(m + M)V^2 = (m + M)gh$.

$$v = \frac{m + M}{m}\sqrt{2gh}$$

El problema muestra cuándo usar cada ley de conservación.

## Centro de masa

$$\vec{r}_{cm} = \frac{\sum m_i\vec{r}_i}{\sum m_i}$$

Es el punto donde puede considerarse concentrada toda la masa del sistema.

> [!IMPORTANT]
> $$\vec{F}_{ext} = M\vec{a}_{cm}$$
> El centro de masa se mueve como una partícula con toda la masa, sometida solo a las fuerzas externas.

Si no hay fuerzas externas, el centro de masa se mueve con velocidad constante, pase lo que pase dentro del sistema. Los fragmentos de un proyectil que explota en el aire se dispersan, pero su centro de masa sigue la parábola original.

> [!NOTE]
> Masas de $2$ kg en $x = 0$ y $6$ kg en $x = 4$ m: $x_{cm} = \dfrac{2(0) + 6(4)}{8} = 3$ m. Queda más cerca de la masa mayor.

> [!TIP]
> En un choque, usar siempre conservación de la cantidad de movimiento. La energía cinética solo se conserva si el enunciado dice que es elástico.
