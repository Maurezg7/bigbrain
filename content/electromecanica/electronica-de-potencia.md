+++
title = "Electrónica de potencia"
summary = "Tiristores, IGBT, rectificadores controlados, convertidores CC-CC, inversores y variadores de frecuencia."
branch = "Electrónica"
order = 5
level = "Avanzado"
tags = ["tiristor", "IGBT", "PWM", "inversor", "variador de frecuencia"]
+++

## Convertir energía conmutando

La electrónica industrial controla el flujo de energía eléctrica con semiconductores que trabajan como **llaves**: abiertos o cerrados, nunca a medias. Así casi no disipan y se logran rendimientos superiores al 95 %.

| Conversión | Nombre | Aplicación |
|---|---|---|
| CA → CC | rectificador | cargadores, accionamientos de CC, electrólisis |
| CC → CC | chopper o convertidor | fuentes, vehículos eléctricos, paneles solares |
| CC → CA | inversor u ondulador | variadores, UPS, energía solar |
| CA → CA | regulador de alterna, cicloconvertidor | arrancadores suaves, atenuadores |

## Dispositivos

| Dispositivo | Encendido | Apagado | Uso |
|---|---|---|---|
| Diodo | automático | automático | rectificación no controlada |
| Tiristor (SCR) | pulso en la compuerta | cuando la corriente se anula | rectificadores de gran potencia |
| TRIAC | pulso, en ambos sentidos | al anularse la corriente | control de alterna de baja potencia |
| MOSFET | tensión de compuerta | tensión de compuerta | baja tensión y alta frecuencia |
| IGBT | tensión de compuerta | tensión de compuerta | variadores e inversores de media y alta potencia |

## Rectificador controlado

Reemplazando diodos por tiristores se puede retrasar el instante de encendido un **ángulo de disparo** $\alpha$ y regular la tensión continua. Para el puente trifásico totalmente controlado:

$$V_{cc} = 1{,}35\,V_L\cos\alpha$$

Con $\alpha > 90^\circ$ la tensión media es negativa y la energía vuelve a la red: el convertidor trabaja como **inversor**, lo que permite frenar un motor de continua recuperando energía.

## Convertidores CC-CC

Un interruptor se abre y se cierra a alta frecuencia; la fracción del período que permanece cerrado es el **ciclo de trabajo** $D$. Un inductor y un capacitor alisan la salida.

| Topología | Relación ideal | Función |
|---|---|---|
| Reductor (buck) | $V_o = D\,V_i$ | baja la tensión |
| Elevador (boost) | $V_o = \dfrac{V_i}{1 - D}$ | sube la tensión |
| Reductor-elevador | $V_o = -\dfrac{D}{1 - D}\,V_i$ | ambas, invierte la polaridad |

> [!NOTE]
> Un reductor alimentado con 24 V debe entregar 5 V: $D = 5/24 \approx 0{,}21$.
>
> Un regulador lineal haría lo mismo disipando $(24 - 5)\cdot I$; con 2 A serían 38 W de calor para entregar 10 W útiles.

## Inversores y PWM

![Señal PWM cuyo ancho de pulso varía siguiendo una senoide de referencia](/media/electromecanica/pwm.svg)

*Modulación por ancho de pulso: el valor medio de los pulsos reproduce la senoide.*

Un inversor trifásico tiene seis IGBT que conectan cada fase al positivo o al negativo de una barra de continua. Con **modulación por ancho de pulso** (PWM), los pulsos son anchos donde la senoide deseada es grande y estrechos donde es pequeña. La inductancia del motor filtra la corriente, que resulta casi senoidal.

## Variador de frecuencia

Un variador combina rectificador, barra de continua con capacitores e inversor. Como la velocidad de un motor de inducción depende de la frecuencia, el variador la regula de forma continua. Para no saturar el motor mantiene constante el flujo:

$$\frac{V}{f} = \text{constante}$$

Por encima de la frecuencia nominal la tensión ya no puede subir y el motor trabaja con flujo debilitado y menos par. En bombas y ventiladores, donde la potencia varía con el cubo de la velocidad, bajar la velocidad un 20 % reduce el consumo casi a la mitad.

El **arrancador suave** usa tiristores en antiparalelo para aumentar la tensión de a poco durante el arranque, sin variar la frecuencia.

> [!WARNING]
> Estos equipos generan armónicos de corriente y flancos de tensión muy abruptos. Requieren filtros o reactores de línea, cables apantallados y una buena puesta a tierra. Los capacitores de la barra de continua quedan cargados varios minutos después de desconectar.
