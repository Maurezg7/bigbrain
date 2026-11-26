+++
title = "Sensores e instrumentación industrial"
summary = "Características de un instrumento, medición de temperatura, presión, caudal y nivel, señal de 4 a 20 mA y diagramas P&ID."
branch = "Instrumentación y control automático"
order = 1
level = "Intermedio"
tags = ["sensores", "termocupla", "Pt100", "transmisor", "4-20 mA"]
+++

## La cadena de medición

Para controlar un proceso primero hay que medirlo.

| Elemento | Función |
|---|---|
| Sensor (elemento primario) | responde a la variable física |
| Transmisor | convierte esa respuesta en una señal normalizada |
| Controlador | compara con el valor deseado y decide |
| Elemento final de control | actúa sobre el proceso: válvula, variador, resistencia |

## Características de un instrumento

| Término | Significado |
|---|---|
| Rango | valores mínimo y máximo que mide |
| Alcance (*span*) | diferencia entre ambos |
| Exactitud | cercanía al valor verdadero |
| Precisión (repetibilidad) | cercanía entre mediciones repetidas |
| Resolución | menor cambio que distingue |
| Sensibilidad | cambio de la salida por unidad de cambio de la entrada |
| Histéresis | diferencia según se llegue subiendo o bajando |
| Tiempo de respuesta | lo que tarda en seguir un cambio |

Un instrumento puede ser muy repetible y poco exacto: eso se corrige calibrando.

## Señal de 4 a 20 mA

Es el estándar industrial analógico. El transmisor regula la corriente del lazo en proporción a la variable:

$$I = 4 + 16\,\frac{x - x_{min}}{x_{max} - x_{min}}\ \text{mA}$$

- La corriente no se atenúa con la longitud del cable y es poco sensible al ruido.
- El **cero vivo** de 4 mA permite detectar un cable cortado, que daría 0 mA.
- El mismo par de cables alimenta al transmisor.

> [!NOTE]
> Transmisor de presión de 0 a 10 bar que entrega 13,6 mA: $p = 10\cdot\dfrac{13{,}6 - 4}{16} = 6$ bar.
>
> Sobre una resistencia de 250 Ω, la señal se lee como 1 a 5 V.

El protocolo **HART** superpone comunicación digital sobre el mismo lazo. Los buses de campo y Ethernet industrial (Profibus, Modbus, Profinet, EtherNet/IP) reemplazan las señales individuales por redes.

## Temperatura

| Sensor | Principio | Rango | Observaciones |
|---|---|---|---|
| Termocupla | dos metales distintos generan una tensión (efecto Seebeck) | −200 a 1700 °C | robusta y económica; requiere compensación de junta fría |
| Pt100 (RTD) | la resistencia del platino crece con la temperatura | −200 a 600 °C | la más exacta y estable |
| Termistor NTC | semiconductor cuya resistencia baja al calentarse | −50 a 150 °C | muy sensible, no lineal |
| Pirómetro infrarrojo | mide la radiación emitida | hasta 3000 °C | sin contacto; depende de la emisividad |

Una Pt100 tiene 100 Ω a 0 °C y aumenta unos 0,385 Ω por grado:

$$R\approx 100\,(1 + 0{,}00385\,T)$$

Se conecta con **tres o cuatro hilos** para descontar la resistencia de los cables. Las termocuplas más comunes son la tipo J (hierro y constantán) y la tipo K (cromel y alumel), con unos 41 µV/°C.

## Presión

| Tipo | Principio |
|---|---|
| Manómetro de Bourdon | un tubo curvo se endereza con la presión; indicación local |
| Piezorresistivo | galgas extensométricas sobre un diafragma |
| Capacitivo | el diafragma es una placa de un capacitor |
| Piezoeléctrico | para presiones que cambian rápido |

Según la referencia, se mide presión **absoluta**, **manométrica** o **diferencial**. El transmisor de presión diferencial es el instrumento más versátil de una planta, porque también sirve para medir caudal y nivel.

## Caudal

| Medidor | Principio | Observaciones |
|---|---|---|
| Placa orificio | caída de presión proporcional al cuadrado del caudal | simple; poca rangeabilidad |
| Electromagnético | ley de Faraday en un líquido conductor | sin obstrucción ni pérdida de carga |
| Vórtex | frecuencia de los remolinos tras un obstáculo | vapor, gases y líquidos |
| Turbina | velocidad de un rotor | exacto con fluidos limpios |
| Ultrasónico | tiempo de tránsito | puede ser externo al caño |
| Coriolis | deformación de un tubo vibrante | mide masa directamente; el más exacto |

Con placa orificio, $Q = k\sqrt{\Delta p}$: hay que extraer la raíz cuadrada de la señal.

## Nivel

| Método | Principio |
|---|---|
| Presión hidrostática | $h = \dfrac{p}{\rho g}$ |
| Flotante o desplazador | empuje |
| Ultrasonido y radar | tiempo de ida y vuelta de una onda |
| Capacitivo | cambio de la constante dieléctrica |
| Interruptores de nivel | detectan un punto: horquilla vibrante, boya, conductivo |

## Posición, velocidad y fuerza

| Variable | Sensores |
|---|---|
| Presencia | inductivos para metales, capacitivos, fotoeléctricos, finales de carrera |
| Posición | potenciómetro, LVDT, *encoder* incremental o absoluto |
| Velocidad | *encoder*, tacogenerador |
| Fuerza y peso | celda de carga con galgas extensométricas |
| Vibración | acelerómetro piezoeléctrico |

Una **galga extensométrica** cambia su resistencia al deformarse; cuatro de ellas en puente de Wheatstone dan una señal proporcional a la fuerza y compensada en temperatura.

## Elementos finales de control

La **válvula de control** es el más común: un actuador neumático mueve el obturador según la señal, con un **posicionador** que asegura la posición. Su tamaño se define con el coeficiente $K_v$, el caudal de agua en m³/h que deja pasar con 1 bar de caída:

$$Q = K_v\sqrt{\frac{\Delta p}{\rho/\rho_{agua}}}$$

Se especifica además qué hace ante una falla de aire: abrir o cerrar, según lo que deje seguro al proceso.

## Diagramas P&ID

Los diagramas de cañerías e instrumentos identifican cada instrumento con letras según la norma ISA 5.1: la primera indica la variable y las siguientes la función.

| Código | Significado |
|---|---|
| TT | transmisor de temperatura |
| PI | indicador de presión |
| FIC | controlador indicador de caudal |
| LT | transmisor de nivel |
| LSH | interruptor de nivel alto |
| PSV | válvula de seguridad |
| FCV | válvula de control de caudal |

## Calibración

Consiste en comparar el instrumento con un patrón trazable en varios puntos del rango y ajustar el **cero** y el **alcance**. Se registra el estado "como se encontró" y "como se dejó", con una frecuencia acorde a la criticidad de la medición.
