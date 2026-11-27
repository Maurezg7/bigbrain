+++
title = "Refrigeración y aire acondicionado"
summary = "Ciclo de compresión de vapor, COP, refrigerantes, componentes, carga térmica, psicrometría y bombas de calor."
branch = "Máquinas térmicas e hidráulicas"
order = 7
level = "Intermedio"
tags = ["refrigeración", "COP", "refrigerantes", "aire acondicionado", "psicrometría"]
+++

## Mover calor de frío a caliente

El calor no pasa por sí solo de un lugar frío a uno caliente. Una máquina frigorífica lo consigue consumiendo trabajo, aprovechando que un líquido absorbe calor al evaporarse y lo cede al condensarse.

## Ciclo de compresión de vapor

![Esquema del ciclo frigorífico con compresor, condensador, válvula de expansión y evaporador](/media/electromecanica/ciclo-frigorifico.svg)

*El refrigerante absorbe calor en el evaporador a baja presión y lo cede en el condensador a alta presión.*

| Etapa | Componente | Qué pasa con el refrigerante |
|---|---|---|
| 1 → 2 | compresor | vapor a baja presión pasa a vapor caliente a alta presión |
| 2 → 3 | condensador | cede calor al ambiente y se vuelve líquido |
| 3 → 4 | dispositivo de expansión | baja la presión y se enfría; queda una mezcla |
| 4 → 1 | evaporador | absorbe calor del recinto y se evapora |

La temperatura de evaporación depende de la presión: al mantener baja la presión del evaporador, el refrigerante hierve a temperaturas bajo cero.

## Balance y COP

$$q_f = h_1 - h_4,\qquad w = h_2 - h_1,\qquad q_c = h_2 - h_3,\qquad h_4 = h_3$$

El **coeficiente de desempeño** indica cuánto frío se obtiene por cada unidad de trabajo:

$$COP = \frac{q_f}{w} = \frac{h_1 - h_4}{h_2 - h_1}$$

y el máximo teórico entre las temperaturas absolutas de evaporación y de condensación es

$$COP_{Carnot} = \frac{T_f}{T_c - T_f}$$

> [!NOTE]
> Ciclo con R-134a entre −10 °C y 40 °C: $h_1 = 392$, $h_2 = 425$ y $h_3 = h_4 = 256$ kJ/kg.
>
> $q_f = 136$ kJ/kg, $w = 33$ kJ/kg y $COP\approx 4{,}1$. El de Carnot sería $\dfrac{263}{50} = 5{,}3$.
>
> Para 10 kW de frío hacen falta $\dfrac{10}{136} = 0{,}0735$ kg/s de refrigerante y 2,4 kW en el compresor.

El COP mejora al **acercar** las dos temperaturas: cada grado menos de condensación o más de evaporación ahorra entre 2 y 4 % de energía. Por eso importa mantener limpios el condensador y el evaporador.

## Componentes

| Componente | Variantes |
|---|---|
| Compresor | hermético, semihermético o abierto; alternativo, scroll, de tornillo o centrífugo |
| Condensador | por aire, por agua con torre de enfriamiento o evaporativo |
| Expansión | tubo capilar, válvula termostática o válvula electrónica |
| Evaporador | de expansión directa o inundado; con aire forzado o enfriando un líquido |

La válvula de expansión regula el caudal para que el vapor salga del evaporador con algunos grados de **sobrecalentamiento**, de modo que nunca llegue líquido al compresor, que se dañaría.

## Refrigerantes

| Refrigerante | Tipo | Uso | Observación |
|---|---|---|---|
| R-22 | HCFC | equipos antiguos | daña la capa de ozono; en retirada |
| R-134a | HFC | heladeras, automóviles | alto efecto invernadero |
| R-410A | HFC | aire acondicionado | alto efecto invernadero |
| R-32 | HFC | aire acondicionado actual | menor efecto invernadero; levemente inflamable |
| R-600a, R-290 | hidrocarburos | heladeras, equipos chicos | inflamables |
| R-717 (amoníaco) | natural | frío industrial | muy eficiente; tóxico |
| R-744 ($\text{CO}_2$) | natural | supermercados | presiones muy altas |

Dos índices los califican: el potencial de agotamiento del ozono (ODP) y el de calentamiento global (GWP). El Protocolo de Montreal y la Enmienda de Kigali ordenan su reemplazo progresivo.

> [!WARNING]
> Liberar refrigerante a la atmósfera está prohibido. Se recupera con equipos específicos.

## Unidades y carga térmica

| Unidad | Equivalencia |
|---|---|
| 1 tonelada de refrigeración (TR) | 3,517 kW |
| 1 frigoría/h | 1,163 W |
| 1 BTU/h | 0,293 W |

Un equipo de "3000 frigorías" extrae unos 3,5 kW, es decir 1 TR.

La **carga térmica** es el calor que hay que extraer: el que entra por paredes y vidrios, la radiación solar, las personas, la iluminación, los equipos y el aire exterior. En cámaras frigoríficas se agregan el enfriamiento del producto y la apertura de puertas.

## Psicrometría

El aire acondicionado controla la temperatura y también la humedad. El **diagrama psicrométrico** relaciona las propiedades del aire húmedo:

| Propiedad | Significado |
|---|---|
| Temperatura de bulbo seco | la que marca un termómetro común |
| Temperatura de bulbo húmedo | la de un termómetro con el bulbo mojado y ventilado |
| Humedad relativa | vapor presente respecto del máximo posible a esa temperatura |
| Humedad absoluta | gramos de vapor por kilogramo de aire seco |
| Punto de rocío | temperatura a la que empieza a condensar |
| Entalpía | energía total del aire húmedo |

Al enfriar el aire por debajo de su punto de rocío, condensa agua: así deshumidifica un acondicionador. La potencia se obtiene del salto de entalpía:

$$\dot Q = \dot m_{aire}\,(h_{entrada} - h_{salida})$$

Para confort se busca entre 22 y 26 °C con 40 a 60 % de humedad relativa.

## Bomba de calor

La misma máquina, invertida con una válvula de cuatro vías, calefacciona: toma calor del exterior y lo entrega adentro.

$$COP_{calor} = COP_{frío} + 1$$

Con un COP de 3 a 4, entrega tres o cuatro veces más calor que la electricidad que consume, frente a 1 de una estufa eléctrica.

## Otros sistemas

- **Absorción**: reemplaza el compresor por un circuito térmico (agua y bromuro de litio, o amoníaco y agua); aprovecha calor residual o solar.
- **Enfriamiento evaporativo**: el aire se enfría al evaporar agua. Funciona muy bien en climas secos, como el del noroeste argentino.
- **Torres de enfriamiento**: enfrían el agua de condensación evaporando una pequeña parte.
