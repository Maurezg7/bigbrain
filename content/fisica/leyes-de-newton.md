+++
title = "Leyes de Newton"
summary = "Las tres leyes que explican por qué se mueven los cuerpos, y cómo usarlas con diagramas de cuerpo libre."
branch = "Mecánica"
order = 3
level = "Básico"
tags = ["fuerza", "dinámica", "rozamiento", "plano inclinado"]
+++

## Las tres leyes

1. **Inercia.** Un cuerpo sobre el que la fuerza neta es cero mantiene su velocidad: sigue en reposo o en movimiento rectilíneo uniforme.
2. **Fuerza y aceleración.** La fuerza neta sobre un cuerpo es igual a su masa por su aceleración:
   $$\sum \vec{F} = m\,\vec{a}$$
3. **Acción y reacción.** Si $A$ ejerce una fuerza sobre $B$, entonces $B$ ejerce sobre $A$ una fuerza igual y opuesta. Las dos fuerzas actúan sobre cuerpos **distintos**, así que no se cancelan.

La unidad de fuerza es el **newton**: $1\ \text{N} = 1\ \text{kg·m/s}^2$.

## Fuerzas habituales

| Fuerza | Expresión | Dirección |
|---|---|---|
| Peso | $P = mg$ | hacia el centro de la Tierra |
| Normal | $N$ (se despeja) | perpendicular a la superficie |
| Rozamiento estático | $f_e \le \mu_e N$ | se opone al deslizamiento inminente |
| Rozamiento dinámico | $f_d = \mu_d N$ | opuesta al deslizamiento |
| Elástica (Hooke) | $F = -k\,x$ | hacia la posición de equilibrio |
| Tensión | $T$ (se despeja) | a lo largo de la cuerda |

## El método: diagrama de cuerpo libre

1. Aislá el cuerpo y dibujá **todas** las fuerzas que actúan sobre él.
2. Elegí ejes convenientes (uno en la dirección del movimiento).
3. Descomponé las fuerzas y planteá $\sum F_x = m a_x$ y $\sum F_y = m a_y$.
4. Resolvé el sistema.

![Bloque sobre un plano inclinado con peso, normal y rozamiento](/media/fisica/plano-inclinado.svg)

*Diagrama de cuerpo libre de un bloque en un plano inclinado, con el peso descompuesto según los ejes del plano.*

> [!NOTE]
> Un bloque baja por un plano inclinado $30°$ con $\mu_d = 0{,}2$. ¿Con qué aceleración?
>
> Eje perpendicular al plano: $N = mg\cos\theta$. Eje paralelo: $mg\sin\theta - \mu_d N = ma$. Entonces
>
> $$a = g\,(\sin\theta - \mu_d\cos\theta) = 9{,}8\,(0{,}5 - 0{,}2 \cdot 0{,}866) \approx 3{,}2\ \text{m/s}^2$$
>
> La masa se cancela: todos los bloques bajan con la misma aceleración.

> [!WARNING]
> La normal **no** siempre vale $mg$. En un plano inclinado vale $mg\cos\theta$, y en un ascensor que acelera cambia con la aceleración.
