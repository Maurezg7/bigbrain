+++
title = "Aplicaciones de la integral"
summary = "Áreas entre curvas, volúmenes de revolución, longitud de arco, valor medio, trabajo y centro de masa."
branch = "Cálculo"
order = 6
level = "Intermedio"
tags = ["integrales", "área", "volumen de revolución", "centro de masa"]
+++

## La idea común

Todas las aplicaciones siguen el mismo esquema: se corta la magnitud en **pedazos infinitesimales**, se calcula cada pedazo y se **suman** con una integral.

$$\text{Total} = \int_a^b (\text{pedazo})$$

## Área entre curvas

Si $f(x) \ge g(x)$ en $[a, b]$:

> [!IMPORTANT]
> $$A = \int_a^b [f(x) - g(x)]\,dx$$

Los límites suelen ser los puntos de intersección. Si las curvas se cruzan, se parte la integral.

> [!NOTE]
> Área entre $y = x$ e $y = x^2$.
>
> Se cortan en $x = 0$ y $x = 1$, y allí $x \ge x^2$.
>
> $$A = \int_0^1 (x - x^2)\,dx = \left[\frac{x^2}{2} - \frac{x^3}{3}\right]_0^1 = \frac{1}{6}$$

> [!WARNING]
> La integral da área **con signo**. Para el área geométrica de una función que cambia de signo hay que integrar $|f(x)|$, es decir, separar en tramos.

## Volúmenes de revolución

### Método de discos

Al girar la región bajo $y = f(x)$ alrededor del eje $x$, cada corte es un disco de radio $f(x)$:

$$V = \pi\int_a^b [f(x)]^2\,dx$$

Si la región está entre dos curvas, cada corte es una **arandela**: $V = \pi\displaystyle\int_a^b \left([R(x)]^2 - [r(x)]^2\right)dx$.

> [!NOTE]
> Volumen de la esfera: se gira $y = \sqrt{r^2 - x^2}$ alrededor del eje $x$.
>
> $$V = \pi\int_{-r}^{r} (r^2 - x^2)\,dx = \pi\left[r^2 x - \frac{x^3}{3}\right]_{-r}^{r} = \frac{4}{3}\pi r^3$$

### Método de capas cilíndricas

Al girar alrededor del eje $y$, conviene sumar cilindros huecos de radio $x$ y altura $f(x)$:

$$V = 2\pi\int_a^b x\,f(x)\,dx$$

### Secciones conocidas

En general, si el área de la sección transversal es $A(x)$: $V = \displaystyle\int_a^b A(x)\,dx$.

## Longitud de arco

Un tramo pequeño de curva mide $ds = \sqrt{dx^2 + dy^2}$:

$$L = \int_a^b \sqrt{1 + [f'(x)]^2}\,dx$$

Para una curva paramétrica: $L = \displaystyle\int_{t_1}^{t_2} \sqrt{\left(\frac{dx}{dt}\right)^2 + \left(\frac{dy}{dt}\right)^2}\,dt$.

### Área de una superficie de revolución

$$S = 2\pi\int_a^b f(x)\sqrt{1 + [f'(x)]^2}\,dx$$

## Valor medio

> [!IMPORTANT]
> $$\bar{f} = \frac{1}{b - a}\int_a^b f(x)\,dx$$

Es la altura del rectángulo con la misma área que la función.

> [!NOTE]
> Valor medio de $\sin x$ en $[0, \pi]$: $\dfrac{1}{\pi}\displaystyle\int_0^{\pi} \sin x\,dx = \dfrac{2}{\pi} \approx 0{,}637$.

El **valor eficaz** (RMS) de una señal periódica, usado en corriente alterna, es $f_{ef} = \sqrt{\dfrac{1}{T}\displaystyle\int_0^T f^2(t)\,dt}$. Para una senoide de amplitud $A$ vale $A/\sqrt{2}$.

## Aplicaciones físicas

| Magnitud | Integral |
|---|---|
| Desplazamiento | $\displaystyle\int_{t_1}^{t_2} v(t)\,dt$ |
| Trabajo de una fuerza variable | $\displaystyle\int_a^b F(x)\,dx$ |
| Masa de una barra de densidad $\rho(x)$ | $\displaystyle\int_a^b \rho(x)\,dx$ |
| Carga eléctrica | $\displaystyle\int i(t)\,dt$ |
| Fuerza hidrostática sobre una pared | $\displaystyle\int \rho g\,h\,w(h)\,dh$ |

> [!NOTE]
> Trabajo para estirar 0,2 m un resorte de $k = 500$ N/m:
>
> $$W = \int_0^{0{,}2} 500x\,dx = 250x^2\Big|_0^{0{,}2} = 10\ \text{J}$$

### Centro de masa

Para una placa plana homogénea bajo $y = f(x)$, de área $A$:

$$\bar{x} = \frac{1}{A}\int_a^b x\,f(x)\,dx \qquad \bar{y} = \frac{1}{A}\int_a^b \frac{[f(x)]^2}{2}\,dx$$

**Teorema de Pappus**: el volumen generado al girar una región plana es su área por la distancia que recorre su centroide: $V = 2\pi\bar{r}A$.

### Momento de inercia

Mide la resistencia a girar: $I = \displaystyle\int r^2\,dm$. Para una barra de masa $M$ y largo $L$ que gira por su centro, $I = \tfrac{1}{12}ML^2$.

## Probabilidad

Si $f(x)$ es una densidad de probabilidad, $P(a \le X \le b) = \displaystyle\int_a^b f(x)\,dx$, y el área total vale 1.

> [!TIP]
> Antes de integrar, dibujar la región y un pedazo típico. La fórmula sale de preguntarse: ¿cuánto vale ese pedazo?
