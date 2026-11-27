+++
title = "Calderas y turbinas de vapor"
summary = "Generadores de vapor humotubulares y acuotubulares, combustión, rendimiento, seguridad y turbinas de acción y de reacción."
branch = "Máquinas térmicas e hidráulicas"
order = 4
level = "Intermedio"
tags = ["calderas", "vapor", "combustión", "turbina de vapor", "rendimiento"]
+++

## Para qué se genera vapor

El vapor de agua transporta mucha energía, se distribuye fácilmente y la entrega a temperatura constante al condensar. Se usa para calentar procesos, esterilizar y mover turbinas.

## Tipos de calderas

| | Humotubular (pirotubular) | Acuotubular |
|---|---|---|
| Por dentro de los tubos circulan | los gases | el agua y el vapor |
| Presión | hasta unos 25 bar | hasta más de 250 bar |
| Producción | hasta unas 25 t/h | sin límite práctico |
| Respuesta | lenta, con gran reserva de agua | rápida |
| Uso | industria, servicios | centrales eléctricas, grandes industrias |

## Partes de un generador de vapor

| Componente | Función |
|---|---|
| Hogar y quemador | donde se quema el combustible |
| Evaporador | el agua hierve |
| Domo | separa el vapor del agua |
| Sobrecalentador | eleva la temperatura del vapor por encima de la de saturación |
| Economizador | precalienta el agua de alimentación con los gases de salida |
| Precalentador de aire | calienta el aire de combustión con los gases |
| Chimenea y ventiladores | tiro de los gases |

Economizador y precalentador recuperan calor que de otro modo se iría por la chimenea.

## Combustión

Es la oxidación rápida del combustible. Para el metano:

$$\text{CH}_4 + 2\,\text{O}_2\rightarrow\text{CO}_2 + 2\,\text{H}_2\text{O}$$

| Combustible | Poder calorífico inferior |
|---|---|
| Gas natural | unos 36 MJ/m³ (48 MJ/kg) |
| Gasoil | 42,5 MJ/kg |
| Fueloil | 40 MJ/kg |
| Carbón | 20 a 30 MJ/kg |
| Bagazo húmedo | 7 a 8 MJ/kg |
| Leña seca | 15 a 18 MJ/kg |

El poder calorífico **inferior** no cuenta el calor de condensación del vapor de agua de los humos; el **superior**, sí.

Para quemar bien hace falta más aire que el estequiométrico. Ese **exceso de aire** se controla midiendo el oxígeno en los humos:

| Combustible | Exceso de aire recomendado |
|---|---|
| Gas natural | 5 a 15 % |
| Líquidos | 15 a 25 % |
| Sólidos en parrilla | 30 a 60 % |

Poco aire deja combustible sin quemar y produce monóxido de carbono y hollín; demasiado aire enfría los gases y se lleva calor por la chimenea.

## Rendimiento

$$\eta = \frac{\dot m_v\,(h_v - h_a)}{\dot m_c\,PCI}$$

con $h_v$ la entalpía del vapor, $h_a$ la del agua de alimentación y $\dot m_c$ el consumo de combustible.

> [!NOTE]
> Caldera que produce 2000 kg/h de vapor saturado a 10 bar ($h_v = 2778$ kJ/kg) con agua a 80 °C ($h_a = 335$ kJ/kg) y rendimiento 0,88, a gas natural (36 MJ/m³).
>
> Calor útil: $\dfrac{2000}{3600}\cdot(2778 - 335)\approx 1357$ kW.
>
> Gas: $\dfrac{1357}{0{,}88\cdot 36\,000}\approx 0{,}0428$ m³/s, unos 154 m³/h.

La mayor pérdida es el calor sensible de los humos: cada 20 °C menos en la chimenea mejoran el rendimiento alrededor de 1 %. Siguen las purgas, la radiación de la envolvente y los inquemados.

## Agua de alimentación

El agua sin tratar deja **incrustaciones** que aíslan los tubos y los sobrecalientan, y el oxígeno disuelto los **corroe**. Por eso se ablanda o desmineraliza, se desgasifica y se dosifican productos químicos. Las **purgas** extraen agua concentrada en sales.

## Seguridad

> [!WARNING]
> Una caldera almacena una energía enorme. Si se queda sin agua con el quemador encendido, los tubos se sobrecalientan y puede explotar.

Elementos obligatorios: **válvulas de seguridad**, dos indicadores de nivel, corte por bajo nivel de agua, presostatos, control de llama con barrido previo del hogar y manómetro. La operación requiere personal habilitado e inspecciones periódicas.

## Trampas de vapor

En la red, el vapor que cede calor se condensa. Las **trampas** dejan salir el condensado y retienen el vapor. Recuperar ese condensado caliente ahorra combustible, agua y tratamiento.

## Turbinas de vapor

Convierten la energía del vapor en trabajo en dos pasos: una **tobera** lo expande y lo acelera, y los **álabes** del rotor aprovechan su cantidad de movimiento.

| Tipo | Dónde se expande el vapor |
|---|---|
| De acción (impulso) | solo en las toberas fijas; en el rotor la presión no cambia |
| De reacción | en los álabes fijos y en los móviles |

Una sola etapa obligaría a velocidades imposibles: expandir de 100 bar al vacío da chorros de más de 1500 m/s. Por eso se **escalona** en muchas etapas, de presión o de velocidad.

El trabajo ideal es el salto isoentrópico de entalpía, y el real se obtiene con el rendimiento interno:

$$w = \eta_i\,(h_1 - h_{2s})$$

Según la salida del vapor:

| Tipo | Descarga | Uso |
|---|---|---|
| De condensación | a un condensador en vacío | máxima generación eléctrica |
| De contrapresión | a la presión que necesita el proceso | cogeneración |
| De extracción | parte del vapor se saca a presión intermedia | electricidad y proceso a la vez |

Los ingenios azucareros del noroeste argentino son un ejemplo clásico de cogeneración: queman bagazo en calderas, generan electricidad con turbinas de contrapresión y usan el vapor de escape en el proceso.

En las últimas etapas el vapor llega húmedo y las gotas erosionan los álabes; el título a la salida no debe bajar de 0,88 aproximadamente.
