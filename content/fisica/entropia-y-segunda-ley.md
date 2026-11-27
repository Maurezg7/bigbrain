+++
title = "Entropía y segunda ley"
summary = "Procesos reversibles e irreversibles, máquinas térmicas, ciclo de Carnot, refrigeradores y entropía."
branch = "Termodinámica"
order = 3
level = "Avanzado"
tags = ["entropía", "segunda ley", "Carnot", "máquinas térmicas", "rendimiento"]
+++

## Lo que la primera ley no dice

La primera ley exige que la energía se conserve, pero no indica **en qué sentido** ocurren los procesos. Un café se enfría solo; jamás se calienta tomando calor del aire, aunque eso no violaría la conservación de la energía.

La **segunda ley** establece la dirección de los procesos naturales.

> [!IMPORTANT]
> - **Clausius**: el calor no pasa espontáneamente de un cuerpo frío a uno caliente.
> - **Kelvin-Planck**: ninguna máquina cíclica puede convertir **todo** el calor que recibe en trabajo.

Ambos enunciados son equivalentes.

### Procesos reversibles e irreversibles

- **Reversible**: puede invertirse sin dejar rastro. Es un ideal: debe ser infinitamente lento y sin rozamiento.
- **Irreversible**: todos los reales. Causas: rozamiento, transferencia de calor con diferencia finita de temperatura, expansión libre, mezcla.

## Máquinas térmicas

Una máquina térmica toma calor $Q_c$ de una fuente caliente, entrega trabajo $W$ y cede calor $Q_f$ a una fuente fría.

$$W = Q_c - Q_f \qquad \eta = \frac{W}{Q_c} = 1 - \frac{Q_f}{Q_c}$$

El **rendimiento** $\eta$ siempre es menor que 1: parte del calor debe descartarse.

### Ciclo de Carnot

Es el ciclo reversible entre dos temperaturas: dos isotermas y dos adiabáticas.

> [!IMPORTANT]
> $$\eta_{Carnot} = 1 - \frac{T_f}{T_c}$$
> Ninguna máquina que opere entre esas dos temperaturas puede superar este rendimiento.

Las temperaturas van en **kelvin**. Para mejorar el rendimiento hay que subir $T_c$ o bajar $T_f$.

> [!NOTE]
> Una central opera con vapor a $550$ °C ($823$ K) y condensa a $30$ °C ($303$ K).
>
> $\eta_{Carnot} = 1 - \dfrac{303}{823} = 63\,\%$. Las centrales reales rondan el 40 %.

| Máquina | Rendimiento típico |
|---|---|
| Motor naftero | 25–30 % |
| Motor diésel | 35–45 % |
| Central térmica de vapor | 35–42 % |
| Ciclo combinado | hasta 60 % |

## Refrigeradores y bombas de calor

Funcionan al revés: con trabajo $W$ extraen calor $Q_f$ de un lugar frío y entregan $Q_c = Q_f + W$ a uno caliente.

| Equipo | Coeficiente de desempeño | Máximo (Carnot) |
|---|---|---|
| Refrigerador | $COP = \dfrac{Q_f}{W}$ | $\dfrac{T_f}{T_c - T_f}$ |
| Bomba de calor | $COP = \dfrac{Q_c}{W}$ | $\dfrac{T_c}{T_c - T_f}$ |

El COP suele ser mayor que 1: una bomba de calor entrega varias veces más calor que la electricidad que consume, porque **mueve** calor en lugar de generarlo.

> [!NOTE]
> Un aire acondicionado con $COP = 3$ que consume $1$ kW extrae $3$ kW de la habitación y expulsa $4$ kW al exterior.

## Entropía

> [!IMPORTANT]
> $$\Delta S = \int\frac{dQ_{rev}}{T}$$

La **entropía** $S$ es una función de estado, medida en J/K.

| Proceso | $\Delta S$ |
|---|---|
| Isotérmico a temperatura $T$ | $\dfrac{Q}{T}$ |
| Cambio de fase | $\dfrac{mL}{T}$ |
| Calentamiento ($c$ constante) | $mc\ln\dfrac{T_2}{T_1}$ |
| Gas ideal | $nC_V\ln\dfrac{T_2}{T_1} + nR\ln\dfrac{V_2}{V_1}$ |
| Adiabático reversible | $0$ |

### La segunda ley con entropía

> [!IMPORTANT]
> $$\Delta S_{universo} \ge 0$$
> La entropía de un sistema aislado nunca disminuye: aumenta en los procesos irreversibles y se mantiene en los reversibles.

La entropía de un sistema **puede** bajar (una heladera enfría), siempre que la del entorno suba más.

> [!NOTE]
> Pasan $1000$ J de un cuerpo a $400$ K a otro a $300$ K.
>
> $\Delta S = -\dfrac{1000}{400} + \dfrac{1000}{300} = -2{,}5 + 3{,}33 = +0{,}83$ J/K.
>
> Es positiva: el proceso es posible e irreversible. El flujo inverso daría $-0{,}83$ J/K: imposible.

> [!NOTE]
> Fundir $1$ kg de hielo a $0$ °C: $\Delta S = \dfrac{334\,000}{273} = 1223$ J/K.

### Interpretación microscópica

Boltzmann relacionó la entropía con el número $\Omega$ de configuraciones microscópicas compatibles con un estado:

$$S = k_B\ln\Omega$$

Un sistema evoluciona hacia los estados **más probables**, que son los que tienen más configuraciones. Un gas se expande por todo el recipiente porque hay muchísimas más formas de estar repartido que de estar en un rincón.

### Degradación de la energía

La energía se conserva, pero su **calidad** no. En cada proceso irreversible, parte de la energía queda menos disponible para hacer trabajo. La entropía define así la «flecha del tiempo».

## Tercera ley

La entropía de un cristal perfecto tiende a cero cuando $T \to 0$ K. Además, el cero absoluto no puede alcanzarse en un número finito de pasos.

| Ley | Idea |
|---|---|
| Cero | existe la temperatura |
| Primera | la energía se conserva |
| Segunda | la entropía del universo no disminuye |
| Tercera | el cero absoluto es inalcanzable |

> [!WARNING]
> No existe el móvil perpetuo: ni el de primera especie (que crea energía) ni el de segunda (que convierte todo el calor en trabajo).

> [!TIP]
> Para saber si un proceso es posible, calcular el cambio de entropía total. Si da negativo, no ocurre.
