+++
title = "Máquinas síncronas"
summary = "Alternadores y motores síncronos: velocidad de sincronismo, circuito equivalente, ángulo de carga y control de reactiva."
branch = "Máquinas eléctricas"
order = 4
level = "Avanzado"
tags = ["alternador", "sincronismo", "excitación", "ángulo de carga", "reactiva"]
+++

## La máquina que genera casi toda la electricidad

El generador síncrono o **alternador** produce prácticamente toda la energía de las centrales. Su rotor gira exactamente a la **velocidad de sincronismo**, fijada por la frecuencia y el número de pares de polos $p$:

$$n_s = \frac{60 f}{p}\ \text{rpm}$$

| Polos | 2 | 4 | 6 | 12 | 40 |
|---|---|---|---|---|---|
| rpm a 50 Hz | 3000 | 1500 | 1000 | 500 | 150 |

## Constitución

- **Estator (inducido)**: bobinado trifásico igual al de un motor de inducción.
- **Rotor (inductor)**: electroimán alimentado con corriente continua de excitación, o imanes permanentes.

| Rotor | Polos | Máquina motriz |
|---|---|---|
| Liso (cilíndrico) | 2 o 4 | turbinas de vapor y de gas, alta velocidad |
| De polos salientes | muchos | turbinas hidráulicas, motores diésel |

## Circuito equivalente

Por fase, la máquina se modela como una fuente $E_0$ (proporcional a la excitación y a la velocidad) en serie con la **reactancia síncrona** $X_s$:

$$\mathbf{E}_0 = \mathbf{V} + jX_s\,\mathbf{I}$$

## Potencia y ángulo de carga

Despreciando la resistencia, la potencia activa trifásica que entrega a una red de tensión $V$ por fase es

$$P = \frac{3\,V E_0}{X_s}\,\sin\delta$$

$\delta$ es el **ángulo de carga**, el desfase entre $E_0$ y $V$. Crece cuando la turbina empuja más. El máximo teórico está en $\delta = 90^\circ$: más allá la máquina **pierde el sincronismo**.

> [!NOTE]
> Alternador con $V = 6{,}35$ kV por fase (11 kV de línea), $E_0 = 8$ kV y $X_s = 4\ \Omega$.
>
> $P_{max} = \dfrac{3\cdot 6350\cdot 8000}{4} \approx 38{,}1$ MW. Entregando 19 MW trabaja con $\sin\delta = 0{,}5$, es decir $\delta = 30^\circ$, con buen margen de estabilidad.

## Dos mandos independientes

Conectada a una red grande (barra infinita), la tensión y la frecuencia están impuestas. Quedan dos controles:

- el **combustible, vapor o agua** de la máquina motriz regula la potencia **activa**;
- la **corriente de excitación** regula la potencia **reactiva**.

| Excitación | Reactiva | La máquina se comporta como |
|---|---|---|
| Sobreexcitada | entrega | un capacitor |
| Normal | cero | factor de potencia unitario |
| Subexcitada | absorbe | un inductor |

## Puesta en paralelo

Antes de cerrar el interruptor que une el alternador con la red deben coincidir:

1. la secuencia de fases;
2. la tensión;
3. la frecuencia;
4. la fase (lo indica el sincronoscopio).

> [!WARNING]
> Acoplar fuera de sincronismo equivale a un cortocircuito con un golpe de par enorme sobre el eje: puede destruir el acoplamiento y los bobinados.

## Motor síncrono

Alimentado desde la red, gira a velocidad rigurosamente constante con cualquier carga. No arranca por sí solo: lleva una jaula amortiguadora para arrancar como motor de inducción, o se alimenta con un variador. Sobreexcitado y sin carga mecánica funciona como **compensador síncrono**, corrigiendo el factor de potencia de la red.

Los motores de **imanes permanentes** y los *brushless* son máquinas síncronas alimentadas por un inversor que conoce la posición del rotor. Dominan en vehículos eléctricos, servomotores y aerogeneradores por su alto rendimiento y densidad de potencia.
