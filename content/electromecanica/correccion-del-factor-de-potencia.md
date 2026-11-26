+++
title = "Corrección del factor de potencia"
summary = "Por qué conviene compensar la potencia reactiva, cómo calcular el banco de capacitores y qué cuidados requiere."
branch = "Instalaciones y sistemas eléctricos"
order = 4
level = "Intermedio"
tags = ["factor de potencia", "capacitores", "reactiva", "banco automático"]
+++

## El problema

Motores, transformadores y lámparas de descarga necesitan potencia **reactiva** para crear sus campos magnéticos. Esa potencia no produce trabajo, pero su corriente sí circula por cables y transformadores:

$$I = \frac{P}{\sqrt3\,V_L\cos\varphi}$$

Para la misma potencia útil, pasar de $\cos\varphi = 1$ a $0{,}7$ aumenta la corriente un 43 % y **duplica** las pérdidas en los conductores, que dependen de $I^2$.

Consecuencias de un factor de potencia bajo:

- recargos en la factura (las distribuidoras suelen exigir 0,85 a 0,95 como mínimo);
- cables, interruptores y transformadores más grandes;
- mayores pérdidas y caídas de tensión.

## La solución

Un capacitor absorbe una corriente adelantada 90°, opuesta a la componente reactiva de las cargas inductivas. Conectado cerca de la carga, le entrega la reactiva localmente y descarga a la red.

Para llevar una carga de potencia activa $P$ desde $\cos\varphi_1$ hasta $\cos\varphi_2$:

$$Q_C = P\,(\tan\varphi_1 - \tan\varphi_2)$$

En un banco trifásico en triángulo, la capacidad por fase es

$$C = \frac{Q_C}{3\cdot 2\pi f\,V_L^2}$$

> [!NOTE]
> Una planta consume 200 kW con $\cos\varphi_1 = 0{,}75$ y se quiere llegar a $0{,}95$, en 380 V.
>
> $\tan\varphi_1 = 0{,}882$ y $\tan\varphi_2 = 0{,}329$, entonces $Q_C = 200\,(0{,}882 - 0{,}329) \approx 111$ kVAr.
>
> La corriente baja de $\dfrac{200\,000}{\sqrt3\cdot 380\cdot 0{,}75}\approx 405$ A a $\dfrac{200\,000}{\sqrt3\cdot 380\cdot 0{,}95}\approx 320$ A: un 21 % menos de corriente y un 38 % menos de pérdidas en la línea.

## Dónde compensar

| Tipo | Ubicación | Ventaja | Desventaja |
|---|---|---|---|
| Individual | en bornes de cada motor grande | descarga toda la instalación | muchos capacitores, poco aprovechados |
| Por grupos | en cada tablero seccional | equilibrio entre costo y efecto | no descarga los circuitos terminales |
| Central | en el tablero general | la más económica y fácil de mantener | solo descarga la acometida |

## Banco automático

Como la carga cambia a lo largo del día, un banco fijo quedaría corto o sobrado. El **banco automático** tiene varios escalones que un regulador conecta y desconecta con contactores según la reactiva medida.

> [!WARNING]
> Sobrecompensar es tan malo como no compensar: el factor de potencia pasa a ser capacitivo, sube la tensión y la distribuidora también lo penaliza.

## Cuidados

- **Armónicos**: los capacitores ofrecen baja impedancia a las frecuencias altas y pueden entrar en resonancia con la inductancia de la red. En plantas con variadores se usan capacitores reforzados y **reactores de desintonía** en serie.
- **Conexión**: al energizar aparece un pico de corriente de hasta cien veces la nominal; se usan contactores especiales con resistencias de preinserción.
- **Descarga**: cada capacitor lleva resistencias que lo descargan en pocos minutos. Antes de tocarlo hay que esperar, verificar y ponerlo en cortocircuito.
- **Motores**: al compensar en bornes no debe superarse la reactiva de vacío del motor, para evitar la autoexcitación al desconectar.
