+++
title = "Límites y continuidad"
summary = "La idea de acercarse tanto como se quiera a un valor: la base de todo el cálculo."
branch = "Cálculo"
order = 1
level = "Intermedio"
tags = ["límite", "continuidad", "indeterminaciones"]
+++

## La idea

Decir que $\lim_{x \to a} f(x) = L$ significa que $f(x)$ se acerca a $L$ tanto como queramos, con tal de tomar $x$ suficientemente cerca de $a$ (sin importar qué pasa exactamente en $a$).

> [!IMPORTANT]
> $\displaystyle\lim_{x \to a} f(x) = L$ si para todo $\varepsilon > 0$ existe $\delta > 0$ tal que
> $0 < |x - a| < \delta$ implica $|f(x) - L| < \varepsilon$.

El límite existe solo si los **límites laterales** (por izquierda y por derecha) existen y coinciden.

## Cómo calcularlos

1. **Sustitución directa.** Si $f$ es continua en $a$ (polinomios, exponenciales, senos…), el límite es $f(a)$.
2. **Indeterminación $0/0$.** Factorizá, racionalizá o simplificá para cancelar el factor que se anula.
3. **Indeterminación $\infty/\infty$.** Dividí numerador y denominador por la mayor potencia de $x$.

> [!NOTE]
> Calcular $\displaystyle\lim_{x \to 2} \frac{x^2 - 4}{x - 2}$.
>
> Sustituir da $0/0$. Factorizando: $\dfrac{(x-2)(x+2)}{x-2} = x + 2$ para $x \neq 2$, así que el límite es $4$. La función no está definida en $x=2$, pero el límite existe igual.

## Límites notables

$$\lim_{x \to 0} \frac{\sin x}{x} = 1, \qquad \lim_{x \to 0} \frac{e^x - 1}{x} = 1, \qquad \lim_{n \to \infty}\left(1 + \frac{1}{n}\right)^n = e$$

## Continuidad

Una función es **continua en $a$** si se cumplen tres cosas: $f(a)$ existe, $\lim_{x\to a} f(x)$ existe, y ambos son iguales. Intuitivamente, su gráfico se puede dibujar sin levantar el lápiz.

Tipos de discontinuidad:

- **Evitable**: el límite existe pero no coincide con $f(a)$ (un "agujero").
- **De salto**: los límites laterales existen pero son distintos.
- **Esencial o infinita**: algún límite lateral no existe o es infinito (asíntota vertical).

> [!TIP]
> Dos teoremas que dependen de la continuidad: el **de Bolzano** (si $f$ es continua en $[a,b]$ y cambia de signo, tiene una raíz adentro) y el **de Weierstrass** (una función continua en un intervalo cerrado alcanza máximo y mínimo).
