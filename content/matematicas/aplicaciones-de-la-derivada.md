+++
title = "Aplicaciones de la derivada"
summary = "Estudio completo de funciones, optimización, razones de cambio relacionadas, regla de L'Hôpital y aproximación lineal."
branch = "Cálculo"
order = 3
level = "Intermedio"
tags = ["derivada", "optimización", "L'Hôpital", "estudio de funciones"]
+++

## Derivación implícita y logarítmica

Cuando $y$ no está despejada, se derivan ambos miembros respecto de $x$ recordando que $y$ depende de $x$ (regla de la cadena).

> [!NOTE]
> Pendiente de la circunferencia $x^2 + y^2 = 25$ en $(3, 4)$.
>
> $2x + 2y\,y' = 0 \Rightarrow y' = -\dfrac{x}{y} = -\dfrac{3}{4}$.

Para funciones como $y = x^x$ se toma logaritmo antes de derivar: $\ln y = x\ln x \Rightarrow \dfrac{y'}{y} = \ln x + 1 \Rightarrow y' = x^x(\ln x + 1)$.

## Teoremas del valor medio

Si $f$ es continua en $[a, b]$ y derivable en $(a, b)$:

- **Rolle**: si $f(a) = f(b)$, existe $c$ con $f'(c) = 0$.
- **Lagrange**: existe $c \in (a, b)$ con

$$f'(c) = \frac{f(b) - f(a)}{b - a}$$

En algún punto la pendiente instantánea iguala a la pendiente media. Si un auto recorre 100 km en una hora, en algún instante fue exactamente a 100 km/h.

## Estudio de funciones

| Información | Qué mirar |
|---|---|
| Crecimiento | $f' > 0$ crece; $f' < 0$ decrece |
| Extremos locales | $f' = 0$ o no existe, con cambio de signo de $f'$ |
| Concavidad | $f'' > 0$ hacia arriba; $f'' < 0$ hacia abajo |
| Puntos de inflexión | $f''$ cambia de signo |
| Asíntotas verticales | $\lim_{x \to a} f(x) = \pm\infty$ |
| Asíntotas horizontales | $\lim_{x \to \pm\infty} f(x) = L$ |

> [!NOTE]
> Estudiar $f(x) = x^3 - 3x$.
>
> $f'(x) = 3x^2 - 3 = 0 \Rightarrow x = \pm 1$. $f''(x) = 6x$.
>
> - $f''(-1) = -6 < 0$: **máximo** local en $(-1, 2)$.
> - $f''(1) = 6 > 0$: **mínimo** local en $(1, -2)$.
> - $f''(0) = 0$ con cambio de signo: **inflexión** en $(0, 0)$.

### Extremos absolutos

En un intervalo cerrado $[a, b]$, una función continua alcanza máximo y mínimo absolutos. Se buscan entre los **puntos críticos** y los **extremos del intervalo**.

## Optimización

1. Identificar la magnitud a maximizar o minimizar.
2. Escribirla como función de **una** variable usando las restricciones.
3. Determinar el dominio.
4. Derivar, igualar a cero y resolver.
5. Verificar que sea el extremo buscado y responder con unidades.

> [!NOTE]
> Una lata cilíndrica debe contener $V = 1000\ \text{cm}^3$. ¿Qué dimensiones minimizan el material?
>
> Superficie: $S = 2\pi r^2 + 2\pi r h$. De $V = \pi r^2 h$: $h = \dfrac{1000}{\pi r^2}$.
>
> $$S(r) = 2\pi r^2 + \frac{2000}{r} \qquad S'(r) = 4\pi r - \frac{2000}{r^2} = 0 \Rightarrow r = \sqrt[3]{\frac{500}{\pi}} \approx 5{,}42\ \text{cm}$$
>
> Entonces $h \approx 10{,}84$ cm: la altura es igual al diámetro.

## Razones de cambio relacionadas

Si dos magnitudes están ligadas por una ecuación, sus velocidades de cambio también. Se deriva la relación **respecto del tiempo**.

> [!NOTE]
> Una escalera de 5 m apoyada en una pared resbala: su pie se aleja a $0{,}5$ m/s. ¿A qué velocidad baja el extremo superior cuando el pie está a 3 m?
>
> $x^2 + y^2 = 25 \Rightarrow 2x\dfrac{dx}{dt} + 2y\dfrac{dy}{dt} = 0$. Con $x = 3$, $y = 4$:
>
> $\dfrac{dy}{dt} = -\dfrac{x}{y}\dfrac{dx}{dt} = -\dfrac{3}{4}(0{,}5) = -0{,}375$ m/s.

> [!WARNING]
> Reemplazar los valores numéricos **después** de derivar, nunca antes: si se pone $x = 3$ antes, se lo trata como constante.

## Regla de L'Hôpital

> [!IMPORTANT]
> Si $\lim \dfrac{f(x)}{g(x)}$ da $\dfrac{0}{0}$ o $\dfrac{\infty}{\infty}$, entonces
> $$\lim \frac{f(x)}{g(x)} = \lim \frac{f'(x)}{g'(x)}$$
> siempre que este último exista.

$$\lim_{x \to 0} \frac{1 - \cos x}{x^2} = \lim_{x \to 0} \frac{\sin x}{2x} = \lim_{x \to 0} \frac{\cos x}{2} = \frac{1}{2}$$

Otras indeterminaciones se llevan a un cociente: $0 \cdot \infty$ reescribiendo como fracción; $1^\infty$, $0^0$, $\infty^0$ tomando logaritmo.

> [!WARNING]
> Se deriva numerador y denominador **por separado**; no es la regla del cociente. Y solo vale si hay indeterminación.

## Aproximación lineal y diferenciales

Cerca de $a$, la función se parece a su recta tangente:

$$f(x) \approx f(a) + f'(a)(x - a)$$

El **diferencial** $dy = f'(x)\,dx$ estima el cambio de $f$ y sirve para propagar errores.

> [!NOTE]
> Estimar $\sqrt{26}$. Con $f(x) = \sqrt{x}$ y $a = 25$: $f'(25) = \tfrac{1}{10}$.
>
> $\sqrt{26} \approx 5 + \tfrac{1}{10}(1) = 5{,}1$. El valor real es $5{,}099$.

## Método de Newton-Raphson

Para resolver $f(x) = 0$ numéricamente, se sigue la tangente hasta el eje:

$$x_{n+1} = x_n - \frac{f(x_n)}{f'(x_n)}$$

Cada paso suele duplicar las cifras correctas.

> [!TIP]
> $f'$ dice si la función sube o baja; $f''$ dice cómo se curva. Con el signo de ambas se dibuja cualquier gráfico.
