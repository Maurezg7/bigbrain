+++
title = "Mecanizado por arranque de viruta"
summary = "Torneado, fresado, taladrado y rectificado: parámetros de corte, herramientas, vida de la herramienta y control numérico."
branch = "Mecanismos y tecnología mecánica"
order = 2
level = "Intermedio"
tags = ["mecanizado", "torneado", "fresado", "velocidad de corte", "CNC"]
+++

## Fabricar quitando material

En el mecanizado, una herramienta más dura que la pieza arranca material en forma de **viruta** hasta lograr la forma, la medida y la terminación deseadas. Es el proceso que da las mejores tolerancias.

| Proceso | Movimiento de corte | Formas que produce |
|---|---|---|
| Torneado | gira la pieza | cilindros, conos, roscas, caras planas |
| Fresado | gira la herramienta, con varios filos | planos, ranuras, cavidades, perfiles |
| Taladrado | gira la broca | agujeros |
| Rectificado | muela abrasiva a alta velocidad | terminación fina y medidas precisas |
| Limado, brochado | movimiento rectilíneo | ranuras, chaveteros, perfiles interiores |

## Parámetros de corte

| Parámetro | Símbolo | Unidad |
|---|---|---|
| Velocidad de corte | $v_c$ | m/min |
| Avance | $f$ | mm por vuelta |
| Profundidad de pasada | $a_p$ | mm |

La velocidad de corte es la velocidad relativa entre filo y pieza. Para un diámetro $D$ en mm:

$$v_c = \frac{\pi D\,n}{1000}\qquad\Longrightarrow\qquad n = \frac{1000\,v_c}{\pi D}$$

| Material de la pieza | $v_c$ con acero rápido | $v_c$ con metal duro |
|---|---|---|
| Acero al carbono | 20 a 30 m/min | 150 a 300 m/min |
| Fundición gris | 15 a 25 m/min | 100 a 200 m/min |
| Aluminio | 80 a 150 m/min | 300 a 1000 m/min |

> [!NOTE]
> Tornear un eje de acero de 50 mm con metal duro a $v_c = 200$ m/min, $f = 0{,}25$ mm/vuelta y $a_p = 3$ mm.
>
> $n = \dfrac{1000\cdot 200}{\pi\cdot 50}\approx 1273$ rpm.
>
> Caudal de viruta: $Q = v_c f a_p = 200\cdot 0{,}25\cdot 3 = 150\ \text{cm}^3$/min. Con una fuerza específica de corte de $2000\ \text{N/mm}^2$: $F_c = 2000\cdot 0{,}25\cdot 3 = 1500$ N y $P = F_c v_c = 1500\cdot\dfrac{200}{60} = 5$ kW.

## Fuerza y potencia

La fuerza de corte es proporcional a la sección de viruta:

$$F_c = k_c\,a_p\,f,\qquad P = \frac{F_c\,v_c}{60\,000\,\eta}\ [\text{kW}]$$

con $F_c$ en N, $v_c$ en m/min y $k_c$ la fuerza específica de corte del material.

## Herramientas

| Material | Característica |
|---|---|
| Acero rápido (HSS) | tenaz y afilable; velocidades bajas |
| Metal duro (carburo de tungsteno) | el estándar actual, en plaquitas intercambiables |
| Cerámica y CBN | materiales templados, altas velocidades |
| Diamante policristalino | aluminio y no ferrosos |

Los ángulos de la herramienta (de desprendimiento, de incidencia y de posición) determinan cómo se forma la viruta y cuánto resiste el filo.

## Vida de la herramienta

El filo se desgasta por abrasión, adhesión y difusión. La velocidad de corte es lo que más influye, según la **ecuación de Taylor**:

$$v_c\,T^{\,n} = C$$

con $T$ la vida en minutos y $n$ del orden de 0,125 para acero rápido y 0,25 para metal duro. Con $n = 0{,}25$, aumentar la velocidad un 20 % reduce la vida a menos de la mitad. Existe una velocidad de mínimo costo y otra, mayor, de máxima producción.

El **fluido de corte** refrigera, lubrica y arrastra la viruta.

## Terminación superficial

La rugosidad se mide con $R_a$, el promedio de las desviaciones del perfil en micrómetros. En torneado, con una herramienta de radio de punta $r$:

$$R_a\approx\frac{f^2}{32\,r}$$

Para mejorar la terminación se reduce el avance o se aumenta el radio.

| Proceso | $R_a$ típico (µm) |
|---|---|
| Desbaste | 6,3 a 25 |
| Torneado y fresado de terminación | 0,8 a 3,2 |
| Rectificado | 0,1 a 0,8 |
| Lapidado y pulido | menos de 0,1 |

## Control numérico

En una máquina **CNC**, un programa gobierna los movimientos de los ejes. El lenguaje básico es el código G:

```gcode
G21 G90        ; milímetros, coordenadas absolutas
S1200 M03      ; husillo a 1200 rpm, giro horario
G00 X52 Z2     ; aproximación rápida
G01 Z-40 F0.25 ; cilindrado con avance de 0,25 mm/vuelta
G00 X100 Z50   ; retirada
M30            ; fin del programa
```

Los programas complejos se generan con software **CAM** a partir del modelo 3D. Los centros de mecanizado de cinco ejes terminan piezas complejas en una sola sujeción.

## Procesos no convencionales

| Proceso | Principio | Uso |
|---|---|---|
| Electroerosión (EDM) | descargas eléctricas | matrices, materiales templados |
| Corte por láser y por plasma | fusión localizada | chapas |
| Corte por chorro de agua | abrasión a muy alta presión | cualquier material, sin calor |
