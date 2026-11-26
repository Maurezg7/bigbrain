+++
title = "Puesta a tierra y seguridad eléctrica"
summary = "Efectos de la corriente en el cuerpo, contactos directos e indirectos, esquemas TT, TN e IT, y diseño de la toma de tierra."
branch = "Instalaciones y sistemas eléctricos"
order = 3
level = "Intermedio"
tags = ["puesta a tierra", "electrocución", "TT", "TN", "jabalina"]
+++

## La corriente en el cuerpo humano

Lo que daña es la **corriente** y el tiempo que circula, no la tensión por sí sola. A 50 Hz y en recorrido mano-pies:

| Corriente | Efecto |
|---|---|
| 0,5 a 1 mA | umbral de percepción |
| 10 mA | contracción muscular: cuesta soltar |
| 30 mA | dificultad respiratoria; límite de los diferenciales |
| 50 a 100 mA | fibrilación ventricular si dura más de una fracción de segundo |
| más de 1 A | paro cardíaco y quemaduras graves |

La resistencia del cuerpo varía entre unos $1000\ \Omega$ (piel húmeda) y varios miles. De ahí las **tensiones de seguridad**: 24 V en ambientes húmedos y 50 V en secos.

## Contacto directo e indirecto

- **Directo**: tocar una parte normalmente con tensión. Se previene con aislación, envolventes y distancias; el diferencial de 30 mA es una protección complementaria.
- **Indirecto**: tocar una masa metálica que quedó con tensión por una falla de aislación. Se previene con la **puesta a tierra** de las masas y el corte automático de la alimentación.

## Para qué sirve la puesta a tierra

1. Limita la tensión que toman las masas durante una falla.
2. Ofrece a la corriente de falla un camino de baja impedancia para que la protección actúe.
3. Fija el potencial de referencia de la instalación.
4. Descarga rayos y cargas electrostáticas.

## Esquemas de conexión a tierra

La primera letra indica cómo está el neutro de la fuente; la segunda, cómo están las masas.

| Esquema | Neutro | Masas | Protección típica |
|---|---|---|---|
| TT | a tierra | a una tierra propia e independiente | diferencial obligatorio |
| TN-S | a tierra | al neutro mediante un conductor de protección separado | termomagnética o fusible |
| TN-C | a tierra | al neutro, que hace también de protección (PEN) | sobrecorriente; no admite diferencial |
| IT | aislado o con impedancia | a tierra | vigilancia de aislación; la primera falla no corta |

El esquema **TT** es el de las instalaciones domiciliarias en Argentina. El **IT** se usa donde no puede interrumpirse el servicio: quirófanos y procesos críticos.

## Condición de protección en TT

La tensión de contacto no debe superar el límite $U_L$ (24 V) con la corriente que hace actuar al diferencial:

$$R_A\cdot I_{\Delta n}\le U_L$$

> [!NOTE]
> Con un diferencial de 30 mA: $R_A \le \dfrac{24}{0{,}03} = 800\ \Omega$. Con uno de 300 mA: $R_A \le 80\ \Omega$.
>
> La reglamentación exige valores bastante menores (del orden de $40\ \Omega$ en viviendas, y menos en industrias) para tener margen frente a suelos secos y al envejecimiento.

## La toma de tierra

La resistencia depende sobre todo de la **resistividad del suelo** $\rho$, que va de $10\ \Omega\cdot$m en terrenos húmedos y arcillosos a más de $1000\ \Omega\cdot$m en roca o arena seca. Para una jabalina vertical de longitud $L$ y diámetro $d$:

$$R\approx\frac{\rho}{2\pi L}\,\ln\frac{4L}{d}$$

Una jabalina de 3 m y 16 mm en un suelo de $100\ \Omega\cdot$m da unos $35\ \Omega$. Para bajar el valor se hincan varias jabalinas separadas al menos su longitud, se usan conductores enterrados o mallas, o se mejora el terreno.

En subestaciones se instala una **malla** que además limita las tensiones de **paso** (entre los pies) y de **contacto** durante una falla.

## Equipotencialidad

Todas las masas y los elementos conductores ajenos a la instalación (cañerías, estructuras) se unen entre sí y a la barra de tierra. Si todo lo que una persona puede tocar está al mismo potencial, no circula corriente por ella.

> [!WARNING]
> Las cinco reglas de oro antes de trabajar en una instalación: 1) cortar todas las fuentes de tensión, 2) bloquear y señalizar los aparatos de corte, 3) verificar la ausencia de tensión, 4) poner a tierra y en cortocircuito, 5) delimitar la zona de trabajo.
