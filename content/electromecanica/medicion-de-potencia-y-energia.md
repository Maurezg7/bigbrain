+++
title = "Medición de potencia y energía"
summary = "Vatímetro, método de los dos vatímetros, medidores de energía, factor de potencia y calidad de energía."
branch = "Mediciones eléctricas"
order = 2
level = "Intermedio"
tags = ["vatímetro", "Aron", "energía", "kWh", "calidad de energía"]
+++

## El vatímetro

Un vatímetro tiene dos circuitos: uno de **corriente**, en serie con la carga, y uno de **tensión**, en paralelo. Indica la potencia activa

$$P = V I \cos\varphi$$

En corriente alterna no alcanza con multiplicar las lecturas de un voltímetro y un amperímetro: eso da la potencia aparente $S$.

## Potencia trifásica

| Sistema | Método | Potencia total |
|---|---|---|
| Cuatro hilos (con neutro) | tres vatímetros, uno por fase | $P = P_1 + P_2 + P_3$ |
| Tres hilos | dos vatímetros (Aron) | $P = W_1 + W_2$ |
| Equilibrado | un vatímetro | $P = 3\,P_1$ |

En el **método de los dos vatímetros**, las bobinas de corriente van en dos líneas y las de tensión entre cada una de ellas y la tercera. Vale para cargas equilibradas o no, siempre que no haya neutro. En carga equilibrada, además:

$$Q = \sqrt3\,(W_1 - W_2),\qquad \tan\varphi = \sqrt3\,\frac{W_1 - W_2}{W_1 + W_2}$$

> [!NOTE]
> Se miden $W_1 = 8$ kW y $W_2 = 4$ kW en un motor trifásico.
>
> $P = 12$ kW, $Q = \sqrt3\cdot 4 \approx 6{,}93$ kVAr, $\tan\varphi = 0{,}577$, $\varphi = 30^\circ$ y $\cos\varphi = 0{,}866$.

> [!TIP]
> Si las dos lecturas son iguales, la carga es resistiva. Si una es cero, $\cos\varphi = 0{,}5$. Si una es negativa, $\cos\varphi < 0{,}5$.

## Energía

La energía es la potencia acumulada en el tiempo y se factura en kWh:

$$E = \int P\,dt,\qquad 1\ \text{kWh} = 3{,}6\ \text{MJ}$$

Los medidores electrónicos actuales registran energía activa y reactiva, la demanda máxima (potencia media en 15 minutos) y los consumos por franja horaria. A los grandes usuarios se les factura también la potencia contratada y se les penaliza un factor de potencia bajo.

> [!NOTE]
> Un motor que absorbe 7,5 kW y funciona 8 horas por día durante 22 días consume $7{,}5\cdot 8\cdot 22 = 1320$ kWh por mes.

## Transformadores de medida

En media y alta tensión, o con corrientes grandes, los instrumentos se conectan a través de transformadores:

- **de corriente (TI)**: relación como 200/5 A. Su secundario **nunca** debe quedar abierto con el primario en servicio, porque aparecen tensiones peligrosas.
- **de tensión (TV)**: relación como 13 200/110 V. Su secundario nunca debe ponerse en cortocircuito.

La lectura se multiplica por las relaciones de ambos. Con un TI de 200/5 y un TV de 13 200/110, la constante es $40\cdot 120 = 4800$.

## Calidad de energía

Un analizador de redes registra, además de potencias y energías:

- tensión y sus variaciones (huecos, sobretensiones, cortes);
- desequilibrio entre fases;
- armónicos y distorsión total (THD);
- parpadeo (flicker) y frecuencia.

Con estos datos se diagnostican disparos intempestivos, calentamiento de neutros y transformadores, o fallas en bancos de capacitores.
