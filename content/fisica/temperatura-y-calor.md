+++
title = "Temperatura y calor"
summary = "Escalas de temperatura, dilatación térmica, calor específico, cambios de fase, calorimetría y mecanismos de transferencia de calor."
branch = "Termodinámica"
order = 0
level = "Básico"
tags = ["temperatura", "calor específico", "calor latente", "dilatación", "calorimetría"]
+++

## Temperatura

La **temperatura** mide la energía cinética media de las moléculas. Dos cuerpos en contacto intercambian energía hasta alcanzar la misma temperatura: el **equilibrio térmico** (principio cero de la termodinámica).

| Escala | Congelación del agua | Ebullición del agua | Cero absoluto |
|---|---|---|---|
| Celsius (°C) | 0 | 100 | −273,15 |
| Kelvin (K) | 273,15 | 373,15 | 0 |
| Fahrenheit (°F) | 32 | 212 | −459,67 |

$$T_K = T_C + 273{,}15 \qquad T_F = \tfrac{9}{5}T_C + 32$$

El **cero absoluto** es la temperatura más baja posible. Una diferencia de 1 °C es igual a una de 1 K.

> [!WARNING]
> En las leyes de los gases y en toda fórmula con $T$ multiplicando o dividiendo hay que usar **kelvin**. Los grados Celsius solo sirven para diferencias $\Delta T$.

## Dilatación térmica

Casi todos los materiales se expanden al calentarse.

| Tipo | Fórmula |
|---|---|
| Lineal | $\Delta L = \alpha L_0\,\Delta T$ |
| Superficial | $\Delta A = 2\alpha A_0\,\Delta T$ |
| Volumétrica | $\Delta V = \beta V_0\,\Delta T$, con $\beta \approx 3\alpha$ |

| Material | $\alpha$ ($10^{-6}\ \text{°C}^{-1}$) |
|---|---|
| Aluminio | 23 |
| Acero | 12 |
| Hormigón | 12 |
| Vidrio común | 9 |
| Vidrio Pyrex | 3,3 |

> [!NOTE]
> Un riel de acero de $25$ m pasa de $0$ a $40$ °C: $\Delta L = 12 \times 10^{-6} \cdot 25 \cdot 40 = 0{,}012$ m. Se alarga $12$ mm; por eso se dejan juntas de dilatación.

El acero y el hormigón tienen el mismo coeficiente, lo que permite el hormigón armado.

El **agua** es anómala: entre 0 y 4 °C se contrae al calentarse. Su densidad es máxima a 4 °C, y por eso los lagos se congelan desde arriba.

## Calor

El **calor** $Q$ es energía que se transfiere por una diferencia de temperatura. Se mide en joules; $1\ \text{cal} = 4{,}186\ \text{J}$.

> [!WARNING]
> Un cuerpo no «tiene» calor: tiene energía interna. El calor es energía **en tránsito**.

### Calor específico

> [!IMPORTANT]
> $$Q = mc\,\Delta T$$

$c$ es el **calor específico**: la energía necesaria para elevar 1 K la temperatura de 1 kg.

| Sustancia | $c$ (J/kg·K) |
|---|---|
| Agua | 4186 |
| Hielo | 2090 |
| Aluminio | 900 |
| Hierro | 450 |
| Cobre | 385 |

El agua tiene un calor específico muy alto: tarda en calentarse y en enfriarse. Por eso modera el clima de las zonas costeras y se usa como refrigerante.

### Cambios de fase

Durante un cambio de fase la temperatura **no cambia**: la energía se usa para romper o formar enlaces.

$$Q = mL$$

| Cambio (agua) | Calor latente |
|---|---|
| Fusión (0 °C) | $L_f = 334$ kJ/kg |
| Vaporización (100 °C) | $L_v = 2260$ kJ/kg |

> [!NOTE]
> Energía para convertir $1$ kg de hielo a $-10$ °C en vapor a $100$ °C:
>
> | Etapa | Cálculo | Energía |
> |---|---|---|
> | Calentar el hielo | $1 \cdot 2090 \cdot 10$ | 21 kJ |
> | Fundir | $1 \cdot 334\,000$ | 334 kJ |
> | Calentar el agua | $1 \cdot 4186 \cdot 100$ | 419 kJ |
> | Vaporizar | $1 \cdot 2\,260\,000$ | 2260 kJ |
>
> Total: $3034$ kJ. Vaporizar exige más que todo lo demás junto; por eso el vapor quema tanto y transpirar enfría.

## Calorimetría

En un sistema aislado, el calor que cede un cuerpo lo absorbe otro:

$$\sum Q = 0$$

> [!NOTE]
> Se echan $200$ g de hierro a $300$ °C en $1$ kg de agua a $20$ °C.
>
> $0{,}2 \cdot 450(T - 300) + 1 \cdot 4186(T - 20) = 0 \Rightarrow T = 25{,}9$ °C.

## Transferencia de calor

| Mecanismo | Cómo ocurre | Ley |
|---|---|---|
| Conducción | por contacto, sin movimiento de materia | $\dfrac{Q}{t} = kA\dfrac{\Delta T}{L}$ |
| Convección | por el movimiento de un fluido | $\dfrac{Q}{t} = hA\,\Delta T$ |
| Radiación | por ondas electromagnéticas; no necesita medio | $\dfrac{Q}{t} = \varepsilon\sigma AT^4$ |

- $k$ es la **conductividad térmica**: alta en metales, baja en aire, madera y lana de vidrio (aislantes).
- $\sigma = 5{,}67 \times 10^{-8}\ \text{W/m}^2\text{K}^4$ (Stefan-Boltzmann). La radiación crece con la **cuarta potencia** de $T$.

Un termo combate las tres: vacío entre paredes (conducción y convección) y superficies espejadas (radiación).

> [!TIP]
> Un metal a temperatura ambiente se siente frío no por estar más frío, sino porque conduce mejor y nos quita calor más rápido.
