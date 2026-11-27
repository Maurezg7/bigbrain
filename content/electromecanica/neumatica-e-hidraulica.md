+++
title = "Neumática e hidráulica"
summary = "Cilindros, válvulas distribuidoras, simbología, fuerza y velocidad de actuadores, circuitos básicos y centrales oleohidráulicas."
branch = "Instrumentación y control automático"
order = 5
level = "Intermedio"
tags = ["neumática", "oleohidráulica", "cilindros", "válvulas", "actuadores"]
+++

## Potencia mediante fluidos

Ambas técnicas transmiten fuerza y movimiento con un fluido a presión: aire comprimido en la **neumática** y aceite en la **oleohidráulica**.

| | Neumática | Hidráulica |
|---|---|---|
| Fluido | aire, compresible | aceite, casi incompresible |
| Presión de trabajo | 6 a 8 bar | 100 a 350 bar |
| Fuerzas | hasta unos 30 kN | prácticamente sin límite |
| Velocidad | alta | baja y muy controlable |
| Precisión de posición | baja | alta |
| Retorno del fluido | se descarga a la atmósfera | vuelve al tanque |
| Limpieza | limpia | riesgo de fugas de aceite |
| Uso típico | manipulación, embalaje, sujeción | prensas, máquinas viales, inyectoras |

## Fuerza de un cilindro

Por el principio de Pascal, la fuerza es presión por área. Con émbolo de diámetro $D$ y vástago de diámetro $d$:

$$F_{avance} = p\,\frac{\pi D^2}{4},\qquad F_{retroceso} = p\,\frac{\pi (D^2 - d^2)}{4}$$

En el retroceso el vástago ocupa parte del área, de modo que la fuerza es menor. En la práctica se descuenta entre 10 y 15 % por rozamiento.

> [!NOTE]
> Cilindro neumático de 63 mm con vástago de 20 mm a 6 bar.
>
> Avance: $600\,000\cdot\dfrac{\pi\cdot 0{,}063^2}{4}\approx 1870$ N. Retroceso: $600\,000\cdot\dfrac{\pi\,(0{,}063^2 - 0{,}020^2)}{4}\approx 1682$ N.
>
> El mismo cilindro con aceite a 160 bar empujaría unos 49,9 kN, casi 27 veces más.

## Velocidad y caudal

La velocidad del vástago depende del caudal que entra:

$$v = \frac{Q}{A}$$

En hidráulica, la potencia es el producto de presión y caudal:

$$P = p\,Q$$

> [!NOTE]
> Una bomba que entrega 30 L/min a 200 bar: $P = 200\cdot10^5\cdot\dfrac{0{,}030}{60} = 10$ kW hidráulicos; con rendimiento 0,85 pide unos 11,8 kW al motor.

## Actuadores

| Actuador | Movimiento |
|---|---|
| Cilindro de simple efecto | una carrera con fluido; vuelve por resorte |
| Cilindro de doble efecto | las dos carreras con fluido |
| Cilindro sin vástago | carreras largas en poco espacio |
| Actuador giratorio | giro limitado |
| Motor neumático o hidráulico | giro continuo |

## Válvulas distribuidoras

Dirigen el fluido hacia uno u otro lado del actuador. Se nombran con dos números: **vías** (conexiones) y **posiciones**.

| Válvula | Uso típico |
|---|---|
| 2/2 | abrir o cerrar un paso |
| 3/2 | mandar un cilindro de simple efecto; emitir señales |
| 5/2 | mandar un cilindro de doble efecto |
| 5/3 | igual, con posición central de reposo |
| 4/3 | la habitual en hidráulica |

![Cilindro de doble efecto mandado por una válvula distribuidora 5/2](/media/electromecanica/neumatica.svg)

*Circuito básico: la válvula 5/2 conecta una cámara del cilindro a presión y la otra a escape.*

En el símbolo, cada posición es un cuadrado con flechas que indican los caminos; las conexiones se dibujan sobre la posición de reposo. A los costados va el **accionamiento**: manual, mecánico, neumático o por solenoide. Una válvula **monoestable** vuelve por resorte al quitar la señal; una **biestable** conserva la última posición.

| Conexión | Numeración | Letras antiguas |
|---|---|---|
| Presión | 1 | P |
| Utilizaciones | 2 y 4 | A y B |
| Escapes | 3 y 5 | R y S |

## Otras válvulas

| Válvula | Función |
|---|---|
| Antirretorno | deja pasar en un solo sentido |
| Reguladora de caudal | ajusta la velocidad del actuador |
| Reguladora de presión | mantiene una presión constante aguas abajo |
| Limitadora (de alivio) | protege al circuito de sobrepresiones |
| Selectora (función O) | da salida si hay señal en una u otra entrada |
| De simultaneidad (función Y) | da salida solo con señal en ambas entradas |
| Escape rápido | acelera el cilindro descargando junto a él |

En neumática, la velocidad se regula estrangulando el aire que **sale** del cilindro: así el émbolo queda sujeto entre dos colchones de aire y avanza de forma pareja.

## Circuitos típicos

- **Mando directo**: la válvula que acciona el operario alimenta al cilindro. Para cilindros chicos.
- **Mando indirecto**: una válvula pequeña pilotea a la distribuidora principal.
- **Ciclo automático**: finales de carrera al final de cada recorrido dan la señal para el movimiento siguiente.
- **Mando bimanual**: exige las dos manos del operario en los pulsadores, para alejarlas de la zona peligrosa.

Las secuencias se anotan con letras: $A+\ B+\ A-\ B-$ indica que avanza A, avanza B, retrocede A y retrocede B. Hoy las electroválvulas se gobiernan con un PLC (**electroneumática**).

## Central hidráulica

| Componente | Función |
|---|---|
| Tanque | reserva, enfría y decanta el aceite |
| Bomba | de engranajes, de paletas o de pistones |
| Válvula limitadora | fija la presión máxima |
| Filtros | retienen partículas |
| Acumulador | almacena energía y absorbe golpes |
| Intercambiador | mantiene la temperatura del aceite |

La bomba entrega **caudal**; la presión aparece por la resistencia que opone la carga.

> [!WARNING]
> Un chorro fino de aceite a alta presión atraviesa la piel y causa lesiones graves. Nunca se busca una fuga con la mano, y antes de intervenir hay que despresurizar el circuito, incluidos los acumuladores.

La contaminación del aceite es la causa de la mayoría de las fallas hidráulicas: la filtración y la limpieza al reparar son esenciales.

## Hidráulica proporcional

Las válvulas **proporcionales** y las **servoválvulas** ajustan caudal o presión de forma continua según una señal eléctrica. Con un sensor de posición y un controlador forman lazos cerrados capaces de mover grandes fuerzas con precisión de centésimas de milímetro.
