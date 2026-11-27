+++
title = "Transistores BJT y MOSFET"
summary = "Cómo funcionan los transistores bipolares y de efecto de campo, como llave y como amplificador."
branch = "Electrónica"
order = 2
level = "Intermedio"
tags = ["BJT", "MOSFET", "amplificador", "conmutación", "polarización"]
+++

## Para qué sirve un transistor

Un transistor permite que una señal pequeña controle una corriente grande. Tiene dos usos:

- como **llave** (corte o saturación): electrónica digital y de potencia;
- como **amplificador** (zona activa): señales analógicas.

## Transistor bipolar (BJT)

Tiene tres terminales: base, colector y emisor, en versiones NPN y PNP. En zona activa, la corriente de colector es proporcional a la de base:

$$I_C = \beta\,I_B,\qquad I_E = I_C + I_B,\qquad V_{BE}\approx 0{,}7\ \text{V}$$

con $\beta$ típicamente entre 50 y 300.

| Zona | Condición | Comportamiento |
|---|---|---|
| Corte | $I_B = 0$ | llave abierta |
| Activa | $I_C = \beta I_B$ | amplifica |
| Saturación | $I_B > I_C/\beta$ | llave cerrada, $V_{CE}\approx 0{,}2$ V |

> [!NOTE]
> Se quiere accionar un relé de 12 V y 100 mA desde una salida de 5 V con un NPN de $\beta = 100$.
>
> Hace falta $I_B > 1$ mA; se elige 3 mA para asegurar la saturación.
>
> $R_B = \dfrac{5 - 0{,}7}{0{,}003} \approx 1{,}4\ \text{k}\Omega$. En paralelo con la bobina va un diodo que absorbe la sobretensión al cortar.

## MOSFET

En el transistor de efecto de campo, la **tensión** en la compuerta (gate) controla la corriente entre drenador y fuente. La compuerta está aislada: no consume corriente en régimen.

- Conduce cuando $V_{GS}$ supera la tensión umbral $V_{th}$.
- Totalmente encendido se comporta como una resistencia $R_{DS(on)}$ de miliohmios.
- En saturación (uso como amplificador): $I_D = \dfrac{k}{2}\,(V_{GS} - V_{th})^2$.

| | BJT | MOSFET |
|---|---|---|
| Se controla con | corriente de base | tensión de compuerta |
| Pérdida en conducción | $V_{CE(sat)}\,I_C$ | $R_{DS(on)}\,I_D^2$ |
| Velocidad | media | alta |
| Uso típico | señal, baja potencia | conmutación, fuentes, inversores |

## El transistor como amplificador

Para amplificar hay que **polarizarlo**: fijar un punto de trabajo en continua (punto Q) en mitad de la zona activa. El circuito usual es el divisor de tensión en la base con resistencia de emisor, que estabiliza el punto frente a cambios de $\beta$ y de temperatura.

Sobre ese punto se superpone la señal. En pequeña señal el transistor se reemplaza por un modelo lineal con transconductancia

$$g_m = \frac{I_C}{V_T}\approx 40\,I_C\ \ (\text{en A/V, con } I_C \text{ en A})$$

| Configuración | Ganancia de tensión | Impedancia de entrada | Uso |
|---|---|---|---|
| Emisor común | alta, invierte la fase | media | etapa de ganancia |
| Colector común (seguidor) | ≈ 1 | alta | adaptar impedancias |
| Base común | alta | baja | alta frecuencia |

La ganancia del emisor común con el emisor desacoplado es $A_v \approx -g_m R_C$.

## Disipación

La potencia que el transistor convierte en calor es $P = V_{CE} I_C$. Como llave casi no disipa, porque o la tensión o la corriente son casi nulas; en zona activa puede disipar mucho y necesita **disipador**. La temperatura de juntura se estima con

$$T_j = T_{amb} + P\,(R_{\theta jc} + R_{\theta cs} + R_{\theta sa})$$

> [!WARNING]
> Las compuertas de los MOSFET se perforan con descargas electrostáticas. Se manipulan con pulsera antiestática y nunca se dejan flotando en el circuito.
