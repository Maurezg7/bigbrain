+++
title = "Diodos y rectificadores"
summary = "Semiconductores, unión PN, diodo real e ideal, rectificación de media onda y onda completa, filtrado y Zener."
branch = "Electrónica"
order = 1
level = "Básico"
tags = ["diodo", "unión PN", "rectificador", "Zener", "LED"]
+++

## Semiconductores

El silicio puro conduce poco. Al **doparlo** con impurezas se obtienen dos tipos de material:

- **tipo N**: con átomos de fósforo, sobran electrones libres;
- **tipo P**: con átomos de boro, faltan electrones y quedan *huecos* que se comportan como cargas positivas.

Al unir ambos se forma la **unión PN**, con una zona sin portadores y una barrera de potencial de unos 0,7 V en silicio.

## El diodo

Un diodo deja pasar corriente en un solo sentido, del ánodo (P) al cátodo (N).

| Polarización | Estado | Comportamiento |
|---|---|---|
| Directa, $V > 0{,}7$ V | conduce | cae una tensión casi constante de 0,6 a 0,7 V |
| Inversa | bloquea | solo circula una corriente de fuga minúscula |
| Inversa excesiva | ruptura | conduce en avalancha; destruye un diodo común |

La curva real sigue la ecuación de Shockley, con $V_T \approx 26$ mV a temperatura ambiente:

$$I = I_S\left(e^{V/nV_T} - 1\right)$$

Para calcular a mano se usa el modelo de **caída constante**: un interruptor cerrado con 0,7 V cuando conduce y abierto cuando no.

## Rectificadores

![Tensión senoidal de entrada y salida rectificada de onda completa con el rizado del capacitor de filtro](/media/electromecanica/rectificador.svg)

*Rectificación de onda completa: el capacitor mantiene la tensión entre los picos.*

Rectificar es convertir alterna en continua. Con $V_m$ el valor pico de la alterna:

| Circuito | Diodos | Tensión media | Frecuencia del rizado |
|---|---|---|---|
| Media onda | 1 | $V_m/\pi \approx 0{,}318\,V_m$ | $f$ |
| Onda completa con punto medio | 2 | $2V_m/\pi \approx 0{,}637\,V_m$ | $2f$ |
| Puente de Graetz | 4 | $2V_m/\pi$ | $2f$ |
| Trifásico en puente | 6 | $1{,}35\,V_L$ | $6f$ |

## Filtro con capacitor

Un capacitor en paralelo con la carga se carga hasta el pico y entrega corriente mientras los diodos no conducen. El rizado pico a pico es aproximadamente

$$\Delta V \approx \frac{I}{f_r\,C}$$

con $f_r$ la frecuencia del rizado.

> [!NOTE]
> Fuente con transformador de 12 V eficaces, puente y carga de 1 A a 50 Hz.
>
> Pico: $12\sqrt2 - 2\cdot 0{,}7 \approx 15{,}6$ V (conducen dos diodos a la vez).
>
> Para un rizado de 1 V: $C = \dfrac{1}{100\cdot 1} = 10\,000\ \mu$F.

> [!WARNING]
> Cada diodo debe soportar la tensión inversa de pico (en el puente, $V_m$) y la corriente de carga inicial del capacitor, que es varias veces la nominal.

## Diodos especiales

| Diodo | Característica | Uso |
|---|---|---|
| Zener | trabaja en ruptura a una tensión fija | referencia y regulación de tensión |
| LED | emite luz; cae de 1,8 a 3,3 V según el color | señalización e iluminación |
| Schottky | caída de 0,3 V y conmutación muy rápida | fuentes conmutadas |
| Fotodiodo | la luz genera corriente | sensores, celdas solares |
| Varistor / TVS | conduce al superar un umbral | protección contra sobretensiones |

Un LED siempre necesita limitar su corriente. Para alimentarlo desde 12 V a 20 mA con caída de 2 V:

$$R = \frac{12 - 2}{0{,}02} = 500\ \Omega$$
