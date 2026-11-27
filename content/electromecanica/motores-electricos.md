+++
title = "Motores eléctricos de inducción"
summary = "Campo giratorio, velocidad sincrónica, deslizamiento y arranque del motor más usado en la industria."
branch = "Máquinas eléctricas"
order = 2
level = "Intermedio"
tags = ["motor asincrónico", "deslizamiento", "par", "arranque"]
+++

## El motor más usado

El **motor trifásico de inducción** (o asincrónico) mueve la mayor parte de la industria: bombas, ventiladores, compresores, cintas. Es robusto, barato y casi no requiere mantenimiento, porque su rotor **jaula de ardilla** no tiene escobillas ni conexiones eléctricas.

## Campo magnético giratorio

El **estátor** tiene tres bobinados separados 120° en el espacio y alimentados por tres corrientes desfasadas 120° en el tiempo. La suma de sus campos es un campo de módulo constante que **gira** a la **velocidad sincrónica**:

$$n_s = \frac{120\,f}{p}$$

con $n_s$ en rpm, $f$ la frecuencia en Hz y $p$ el número de polos.

| Polos | $n_s$ a 50 Hz | $n_s$ a 60 Hz |
|---|---|---|
| 2 | 3000 rpm | 3600 rpm |
| 4 | 1500 rpm | 1800 rpm |
| 6 | 1000 rpm | 1200 rpm |

## Por qué gira el rotor

El campo giratorio corta las barras del rotor e induce en ellas tensiones (Faraday) y, como están en cortocircuito, corrientes. Esas corrientes dentro de un campo magnético experimentan fuerzas que arrastran al rotor en el sentido del campo.

El rotor **nunca alcanza** la velocidad sincrónica: si lo hiciera, no habría movimiento relativo, ni corriente inducida, ni par. Esa diferencia se mide con el **deslizamiento**:

$$s = \frac{n_s - n}{n_s}$$

A plena carga $s$ suele estar entre 2 % y 5 %.

> [!NOTE]
> Un motor de 4 polos conectado a 50 Hz gira a 1440 rpm. Su velocidad sincrónica es $n_s = \dfrac{120 \cdot 50}{4} = 1500$ rpm y su deslizamiento
> $s = \dfrac{1500 - 1440}{1500} = 0{,}04 = 4\ \%$.

## Potencia y par

La potencia mecánica en el eje y el par se relacionan por

$$P = T\,\omega, \qquad \omega = \frac{2\pi n}{60}$$

La potencia eléctrica absorbida es $P_{el} = \sqrt3\,V_L I_L \cos\varphi$, y el rendimiento $\eta = P_{eje}/P_{el}$.

> [!NOTE]
> Un motor entrega 7,5 kW a 1440 rpm. Su par es $T = \dfrac{7500}{2\pi \cdot 1440/60} \approx 49{,}7$ N·m.

## Arranque y control de velocidad

En el arranque directo el motor toma de 5 a 7 veces su corriente nominal. Para limitarla se usa:

- **Arranque estrella-triángulo**: reduce la corriente y el par de arranque a un tercio.
- **Arrancador suave**: aumenta la tensión gradualmente con tiristores.
- **Variador de frecuencia**: modifica $f$ (y con ella $n_s$) manteniendo constante la relación $V/f$. Además de arrancar suavemente, permite regular la velocidad y ahorrar energía.

> [!TIP]
> Para invertir el sentido de giro de un motor trifásico basta con intercambiar dos de las tres fases: eso invierte el sentido del campo giratorio.
