+++
title = "Transferencia de calor"
summary = "Conducción, convección y radiación, resistencias térmicas, aislación, aletas e intercambiadores de calor."
branch = "Termodinámica y transmisión de calor"
order = 3
level = "Intermedio"
tags = ["conducción", "convección", "radiación", "intercambiador", "aislación"]
+++

## Tres mecanismos

El calor fluye siempre de lo caliente a lo frío, por tres mecanismos que suelen actuar juntos.

| Mecanismo | Cómo ocurre | Ley |
|---|---|---|
| Conducción | de molécula a molécula, sin movimiento del material | Fourier |
| Convección | un fluido en movimiento transporta el calor | Newton |
| Radiación | ondas electromagnéticas; no necesita medio | Stefan-Boltzmann |

## Conducción

Para una pared plana de espesor $e$, área $A$ y conductividad $k$:

$$\dot Q = k\,A\,\frac{T_1 - T_2}{e}$$

| Material | $k$ en W/(m·K) |
|---|---|
| Cobre | 385 |
| Aluminio | 205 |
| Acero al carbono | 50 |
| Acero inoxidable | 15 |
| Ladrillo | 0,7 |
| Agua | 0,6 |
| Lana mineral, poliuretano | 0,025 a 0,04 |
| Aire quieto | 0,026 |

Los aislantes funcionan atrapando aire en celdas pequeñas.

## Convección

$$\dot Q = h\,A\,(T_s - T_\infty)$$

El coeficiente $h$ no es una propiedad del material: depende del fluido, de su velocidad y de la geometría.

| Situación | $h$ en W/(m²·K) |
|---|---|
| Convección natural en aire | 5 a 25 |
| Convección forzada en aire | 25 a 250 |
| Convección forzada en agua | 500 a 10 000 |
| Ebullición y condensación | 2500 a 100 000 |

Se calcula con correlaciones entre números adimensionales:

| Número | Definición | Significado |
|---|---|---|
| Nusselt | $Nu = hL/k$ | convección frente a conducción |
| Reynolds | $Re = \rho vL/\mu$ | régimen del flujo forzado |
| Prandtl | $Pr = c_p\mu/k$ | propiedades del fluido |
| Grashof | — | empuje en convección natural |

Para flujo turbulento dentro de un tubo se usa la correlación de Dittus-Boelter: $Nu = 0{,}023\,Re^{0{,}8}Pr^{n}$, con $n = 0{,}4$ si el fluido se calienta y $0{,}3$ si se enfría.

## Radiación

Todo cuerpo emite radiación según la cuarta potencia de su temperatura absoluta. Para una superficie de emisividad $\varepsilon$ rodeada por un recinto a $T_{alr}$:

$$\dot Q = \varepsilon\,\sigma\,A\,(T_s^4 - T_{alr}^4),\qquad \sigma = 5{,}67\cdot10^{-8}\ \text{W/(m}^2\text{K}^4)$$

Las superficies pulidas emiten poco ($\varepsilon < 0{,}1$); las pintadas u oxidadas, mucho ($\varepsilon\approx 0{,}9$). A alta temperatura la radiación domina, como en hornos y hogares de calderas.

## Resistencias térmicas

El calor se comporta como la corriente eléctrica, con la diferencia de temperatura haciendo de tensión:

$$\dot Q = \frac{\Delta T}{R_{total}}$$

| Elemento | Resistencia |
|---|---|
| Pared plana | $\dfrac{e}{kA}$ |
| Pared cilíndrica | $\dfrac{\ln(r_2/r_1)}{2\pi kL}$ |
| Convección | $\dfrac{1}{hA}$ |

Las capas sucesivas están en serie y sus resistencias se suman. El **coeficiente global** $U$ las reúne: $\dot Q = U A\,\Delta T$.

> [!NOTE]
> Pared de 20 cm de ladrillo ($k = 0{,}7$) con $h_i = 8$ y $h_e = 25$ W/(m²·K), entre 20 °C y 0 °C.
>
> Por m²: $R = \dfrac{1}{8} + \dfrac{0{,}2}{0{,}7} + \dfrac{1}{25} = 0{,}451$ y $\dot Q = \dfrac{20}{0{,}451}\approx 44$ W/m².
>
> Agregando 5 cm de aislante ($k = 0{,}035$): $R = 0{,}451 + 1{,}429 = 1{,}88$ y $\dot Q\approx 10{,}6$ W/m². Las pérdidas bajan a la cuarta parte.

## Aletas

Cuando $h$ es bajo, como en el aire, se aumenta la superficie con **aletas**: radiadores, disipadores de electrónica, cilindros de motores refrigerados por aire. Conviene que sean delgadas, de material muy conductor y estén próximas entre sí.

## Intercambiadores de calor

Transfieren calor entre dos fluidos sin mezclarlos.

| Tipo | Uso |
|---|---|
| De doble tubo | caudales pequeños |
| De casco y tubos | el más usado en la industria; altas presiones |
| De placas | compacto, fácil de limpiar |
| De tubos aletados | cuando uno de los fluidos es un gas |

El calor intercambiado cumple a la vez el balance de energía de cada fluido y la ecuación de transferencia:

$$\dot Q = \dot m_c\,c_{p,c}\,(T_{c,e} - T_{c,s}) = \dot m_f\,c_{p,f}\,(T_{f,s} - T_{f,e}) = U\,A\,\Delta T_{ml}$$

donde $\Delta T_{ml}$ es la **diferencia de temperatura media logarítmica**, con $\Delta T_1$ y $\Delta T_2$ las diferencias en cada extremo:

$$\Delta T_{ml} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1/\Delta T_2)}$$

La disposición en **contracorriente** aprovecha mejor el salto térmico que la de corrientes paralelas y permite que el fluido frío salga más caliente que la salida del caliente.

> [!NOTE]
> Aceite que se enfría de 90 a 50 °C con agua que entra a 20 °C y sale a 40 °C, en contracorriente.
>
> $\Delta T_1 = 90 - 40 = 50$ y $\Delta T_2 = 50 - 20 = 30$ °C, entonces $\Delta T_{ml} = \dfrac{20}{\ln(50/30)}\approx 39{,}2$ °C.
>
> Para 50 kW con $U = 300$ W/(m²·K): $A = \dfrac{50\,000}{300\cdot 39{,}2}\approx 4{,}3\ \text{m}^2$.

Con el uso, las superficies se cubren de depósitos (**ensuciamiento**) que agregan resistencia; el diseño lo prevé con un factor de ensuciamiento y un área mayor.
