+++
title = "Soldadura"
summary = "Procesos de soldadura por arco y por resistencia, zona afectada por el calor, defectos, tipos de junta y control de calidad."
branch = "Mecanismos y tecnología mecánica"
order = 3
level = "Intermedio"
tags = ["soldadura", "arco", "MIG", "TIG", "electrodo"]
+++

## Unir fundiendo

La soldadura une piezas de forma permanente fundiendo localmente sus bordes, con o sin material de aporte. Al solidificar queda continuidad metálica entre ambas.

## Procesos por arco eléctrico

Un arco entre un electrodo y la pieza alcanza más de 5000 °C. El baño fundido debe protegerse del oxígeno y del nitrógeno del aire.

| Proceso | Electrodo | Protección | Características |
|---|---|---|---|
| Electrodo revestido (SMAW) | varilla consumible | gases y escoria del revestimiento | equipo simple, sirve en obra; lento |
| MIG/MAG (GMAW) | alambre continuo | gas inerte o activo ($\text{CO}_2$, mezclas con argón) | muy productivo, fácil de automatizar |
| Alambre tubular (FCAW) | alambre con fundente | fundente interior, con o sin gas | alta deposición, sirve en exteriores |
| TIG (GTAW) | tungsteno no consumible | argón | máxima calidad; inoxidable, aluminio, espesores finos |
| Arco sumergido (SAW) | alambre continuo | fundente granular | grandes espesores, automático |

Los electrodos revestidos se identifican según la AWS. En un **E7018**, "70" indica 70 000 psi (unos 490 MPa) de resistencia, "1" que sirve en todas las posiciones y "8" un revestimiento básico de bajo hidrógeno.

## Otros procesos

- **Por resistencia** (por puntos, por costura): el calor lo genera la corriente al atravesar el contacto entre chapas, $Q = I^2 R\,t$. Es la soldadura de las carrocerías.
- **Oxiacetilénica**: llama de acetileno y oxígeno; también se usa para cortar.
- **Láser y haz de electrones**: gran penetración con poco calor aportado.
- **Por fricción**: sin fusión, por deformación plástica en caliente.
- **Soldadura blanda y fuerte** (estaño, plata, bronce): solo funde el aporte.

## Aporte de calor

La energía por unidad de longitud determina la estructura que queda:

$$H = \frac{\eta\,V\,I}{v}$$

con $V$ e $I$ la tensión y corriente del arco, $v$ la velocidad de avance y $\eta$ el rendimiento del proceso (0,8 en electrodo revestido y MIG, 0,6 en TIG).

> [!NOTE]
> Con 25 V, 200 A, 5 mm/s y $\eta = 0{,}8$: $H = \dfrac{0{,}8\cdot 25\cdot 200}{5} = 800$ J/mm.

## Zona afectada por el calor

Junto al cordón queda una franja que no se fundió pero cambió su estructura: la **ZAC**. En aceros puede formarse martensita dura y frágil si el enfriamiento es rápido. El riesgo se estima con el **carbono equivalente**:

$$CE = C + \frac{Mn}{6} + \frac{Cr + Mo + V}{5} + \frac{Ni + Cu}{15}$$

| $CE$ | Soldabilidad | Precaución |
|---|---|---|
| Menos de 0,40 | buena | ninguna especial |
| 0,40 a 0,60 | regular | precalentar, electrodos de bajo hidrógeno |
| Más de 0,60 | difícil | precalentar y tratar después |

> [!WARNING]
> El hidrógeno, que proviene de la humedad del revestimiento o de la pieza, provoca **fisuras en frío** que aparecen horas después de soldar. Los electrodos básicos se guardan en estufa.

## Tensiones y deformaciones

El cordón se contrae al enfriarse, pero el material frío de alrededor se lo impide. Quedan **tensiones residuales** del orden de la fluencia y la pieza se deforma. Se controlan con una secuencia de soldadura equilibrada, embridando, predeformando o con un tratamiento de alivio de tensiones.

## Juntas y posiciones

| Junta | Preparación |
|---|---|
| A tope | bordes rectos o biselados en V, X o U según el espesor |
| En ángulo (filete) | sin preparación; el cordón queda en la esquina |
| A solape | chapas superpuestas |
| En T y en esquina | filete o penetración completa |

Las posiciones son plana, horizontal, vertical y sobre cabeza; la plana es la más fácil y productiva.

## Defectos

| Defecto | Causa habitual |
|---|---|
| Porosidad | gas atrapado, suciedad, protección insuficiente |
| Inclusiones de escoria | limpieza deficiente entre pasadas |
| Falta de fusión o de penetración | poca corriente, técnica o preparación incorrectas |
| Socavado | corriente o velocidad excesivas |
| Fisuras | hidrógeno, enfriamiento rápido, embridamiento |

## Calidad

La calidad se asegura **antes** de soldar: una especificación de procedimiento (WPS) calificada con ensayos y soldadores calificados. Después se inspecciona con ensayos visuales, líquidos penetrantes, partículas magnéticas, ultrasonido o radiografía. Los códigos de referencia son AWS D1.1 para estructuras y ASME IX para recipientes y cañerías.

> [!WARNING]
> El arco emite radiación ultravioleta que quema la piel y los ojos, y los humos son tóxicos. Careta, guantes, ropa de cuero, ventilación y pantallas para proteger a terceros son obligatorios.
