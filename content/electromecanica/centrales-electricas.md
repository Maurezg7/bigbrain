+++
title = "Centrales eléctricas y sistema interconectado"
summary = "Tipos de centrales, curva de carga, factores característicos, despacho, regulación de frecuencia y estaciones transformadoras."
branch = "Instalaciones y sistemas eléctricos"
order = 6
level = "Intermedio"
tags = ["central", "generación", "curva de carga", "frecuencia", "despacho"]
+++

## Del recurso al enchufe

Un sistema eléctrico tiene cuatro eslabones: **generación**, **transporte**, **distribución** y **consumo**. Como la energía eléctrica casi no se almacena, la generación debe igualar a la demanda en todo instante.

## Tipos de centrales

| Central | Fuente | Máquina motriz | Rasgos |
|---|---|---|---|
| Térmica de vapor | gas, carbón, fueloil, biomasa | turbina de vapor (ciclo Rankine) | grande, lenta para arrancar |
| Turbina de gas | gas natural, gasoil | turbina de gas (ciclo Brayton) | arranque rápido, rendimiento 30 a 40 % |
| Ciclo combinado | gas natural | turbina de gas más turbina de vapor | rendimiento de 55 a 60 % |
| Motores | gasoil, gas | motor de combustión interna | potencias chicas, sistemas aislados |
| Nuclear | fisión de uranio | turbina de vapor | base, costo de combustible bajo |
| Hidráulica | salto de agua | turbina Pelton, Francis o Kaplan | muy flexible, sin combustible |
| Eólica | viento | aerogenerador | variable, sin combustible |
| Solar fotovoltaica | radiación solar | sin partes móviles | variable, solo diurna |
| Geotérmica | calor del subsuelo | turbina de vapor | base, depende del sitio |

## Curva de carga

La demanda cambia hora a hora, con un valle de madrugada y picos al mediodía y al anochecer. Se cubre apilando centrales:

- **de base**: funcionan siempre (nuclear, hidráulica de pasada, ciclos combinados);
- **de semibase**: siguen las variaciones diarias;
- **de punta**: solo en las horas de mayor demanda (turbinas de gas, hidráulicas de embalse).

## Factores característicos

| Factor | Definición | Indica |
|---|---|---|
| De carga | demanda media / demanda máxima | cuán pareja es la demanda |
| De planta (de capacidad) | energía generada / (potencia instalada × horas) | cuánto se aprovecha la central |
| De disponibilidad | horas disponible / horas del período | confiabilidad |

> [!NOTE]
> Un parque eólico de 100 MW genera 350 GWh en un año.
>
> Factor de planta: $\dfrac{350\,000}{100\cdot 8760}\approx 40\ \%$. Equivale a 3500 horas a plena potencia.

## Costo de la energía

El costo de cada kWh suma una parte fija (la inversión, repartida entre la energía generada) y una variable (combustible, operación y mantenimiento). Las centrales de alta inversión y combustible barato o gratuito conviene usarlas muchas horas; las de baja inversión y combustible caro, solo en las puntas.

El **despacho económico** ordena las máquinas de menor a mayor costo variable hasta cubrir la demanda. En Argentina lo realiza CAMMESA sobre el Sistema Argentino de Interconexión (SADI).

## Regulación de frecuencia y tensión

La frecuencia refleja el equilibrio entre generación y consumo: si la demanda supera a la generación, las máquinas se frenan y la frecuencia baja.

- **Regulación primaria**: en segundos, el regulador de velocidad de cada turbina aumenta su potencia de forma automática.
- **Regulación secundaria**: en minutos, se restituyen los 50 Hz y los intercambios programados.
- **Reserva rotante**: potencia disponible en máquinas ya sincronizadas.

La **tensión** se regula con la excitación de los generadores, los cambiadores de tomas de los transformadores, capacitores y reactores.

## Estaciones transformadoras

Elevan la tensión a la salida de las centrales y la reducen cerca del consumo. Sus equipos principales:

| Equipo | Función |
|---|---|
| Transformador de potencia | cambia el nivel de tensión |
| Interruptor | abre y cierra con carga y bajo falla |
| Seccionador | aísla de forma visible; solo se opera sin carga |
| Transformadores de medida | alimentan mediciones y protecciones |
| Descargador | deriva a tierra las sobretensiones |
| Barras | reparten la energía; su esquema fija la flexibilidad |

> [!WARNING]
> Un seccionador nunca se abre con corriente: no tiene capacidad para extinguir el arco. La secuencia siempre es abrir el interruptor, luego los seccionadores y por último poner a tierra.
