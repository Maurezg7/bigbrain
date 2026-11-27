+++
title = "Gases y teoría cinética"
summary = "Ecuación de estado del gas ideal, leyes de los gases, teoría cinética, energía interna y procesos termodinámicos."
branch = "Termodinámica"
order = 2
level = "Intermedio"
tags = ["gas ideal", "teoría cinética", "procesos termodinámicos", "calor específico molar"]
+++

## El gas ideal

Un **gas ideal** está formado por moléculas puntuales que no interactúan salvo al chocar. Los gases reales se le parecen a baja presión y alta temperatura.

> [!IMPORTANT]
> $$pV = nRT = Nk_BT$$

| Símbolo | Significado | Valor |
|---|---|---|
| $n$ | cantidad de moles | |
| $N$ | cantidad de moléculas | $N = nN_A$ |
| $R$ | constante de los gases | $8{,}314$ J/mol·K |
| $k_B$ | constante de Boltzmann | $1{,}38 \times 10^{-23}$ J/K |
| $N_A$ | número de Avogadro | $6{,}022 \times 10^{23}$ |

### Leyes particulares

| Ley | Constante | Relación |
|---|---|---|
| Boyle | $T$ | $p_1V_1 = p_2V_2$ |
| Charles | $p$ | $V_1/T_1 = V_2/T_2$ |
| Gay-Lussac | $V$ | $p_1/T_1 = p_2/T_2$ |
| General | $n$ | $p_1V_1/T_1 = p_2V_2/T_2$ |

> [!NOTE]
> Un neumático a $20$ °C tiene $200$ kPa manométricos ($301$ kPa absolutos). Al rodar llega a $50$ °C, a volumen constante.
>
> $p_2 = 301 \cdot \dfrac{323}{293} = 332$ kPa absolutos, o $231$ kPa manométricos.

> [!WARNING]
> Usar siempre temperatura en **kelvin** y presión **absoluta**.

En condiciones normales (0 °C y 1 atm), un mol de gas ideal ocupa $22{,}4$ litros.

## Teoría cinética

Explica las propiedades macroscópicas a partir del movimiento de las moléculas.

- La **presión** se debe a los choques de las moléculas contra las paredes.
- La **temperatura** mide la energía cinética media de traslación:

> [!IMPORTANT]
> $$\langle E_c\rangle = \tfrac{3}{2}k_BT$$

**Velocidad cuadrática media**:

$$v_{rms} = \sqrt{\frac{3k_BT}{m}} = \sqrt{\frac{3RT}{M}}$$

A igual temperatura, las moléculas livianas son más rápidas.

> [!NOTE]
> Nitrógeno ($M = 0{,}028$ kg/mol) a $300$ K: $v_{rms} = \sqrt{\dfrac{3 \cdot 8{,}314 \cdot 300}{0{,}028}} = 517$ m/s.

Las velocidades no son todas iguales: siguen la **distribución de Maxwell-Boltzmann**, que se ensancha y se desplaza al aumentar $T$.

### Energía interna

**Equipartición**: cada grado de libertad aporta $\tfrac{1}{2}k_BT$ por molécula.

$$U = \frac{f}{2}nRT$$

| Gas | Grados de libertad $f$ | $C_V$ | $C_p$ | $\gamma = C_p/C_V$ |
|---|---|---|---|---|
| Monoatómico (He, Ar) | 3 | $\tfrac{3}{2}R$ | $\tfrac{5}{2}R$ | 1,67 |
| Diatómico (N₂, O₂) | 5 | $\tfrac{5}{2}R$ | $\tfrac{7}{2}R$ | 1,40 |

La energía interna de un gas ideal depende **solo de la temperatura**. Además $C_p - C_V = R$.

## Primer principio

$$\Delta U = Q - W$$

con $Q$ el calor que **recibe** el gas y $W$ el trabajo que **realiza**:

$$W = \int p\,dV$$

Es el área bajo la curva en el diagrama $p$-$V$.

## Procesos

| Proceso | Constante | Trabajo $W$ | Calor $Q$ | $\Delta U$ |
|---|---|---|---|---|
| Isocórico | $V$ | $0$ | $nC_V\Delta T$ | $nC_V\Delta T$ |
| Isobárico | $p$ | $p\,\Delta V$ | $nC_p\Delta T$ | $nC_V\Delta T$ |
| Isotérmico | $T$ | $nRT\ln\dfrac{V_2}{V_1}$ | $= W$ | $0$ |
| Adiabático | $Q = 0$ | $-\Delta U$ | $0$ | $nC_V\Delta T$ |

En todo proceso de un gas ideal, $\Delta U = nC_V\Delta T$.

### Proceso adiabático

Sin intercambio de calor (aislado o muy rápido):

$$pV^\gamma = \text{cte} \qquad TV^{\gamma - 1} = \text{cte}$$

Al comprimirse, el gas **se calienta**; al expandirse, **se enfría**.

> [!NOTE]
> En un motor diésel el aire a $300$ K se comprime adiabáticamente a $\tfrac{1}{18}$ de su volumen ($\gamma = 1{,}4$).
>
> $T_2 = 300 \cdot 18^{0{,}4} = 953$ K. Suficiente para encender el combustible sin bujía.

> [!NOTE]
> Dos moles de gas ideal se expanden isotérmicamente a $300$ K hasta duplicar su volumen.
>
> $W = 2 \cdot 8{,}314 \cdot 300 \cdot \ln 2 = 3458$ J. Como $\Delta U = 0$, absorbe $Q = 3458$ J.

## Gases reales

A presiones altas o temperaturas bajas importan el volumen de las moléculas y sus atracciones. La **ecuación de van der Waals** lo corrige:

$$\left(p + \frac{an^2}{V^2}\right)(V - nb) = nRT$$

Por debajo de la **temperatura crítica**, un gas puede licuarse comprimiéndolo.

> [!TIP]
> Dibujar el proceso en un diagrama $p$-$V$: el área bajo la curva es el trabajo, y un ciclo cerrado encierra el trabajo neto.
