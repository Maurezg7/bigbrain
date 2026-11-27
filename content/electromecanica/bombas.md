+++
title = "Bombas"
summary = "Bombas centrífugas y de desplazamiento positivo: curvas características, punto de funcionamiento, cavitación, NPSH y leyes de semejanza."
branch = "Máquinas térmicas e hidráulicas"
order = 1
level = "Intermedio"
tags = ["bombas", "centrífuga", "NPSH", "cavitación", "curva característica"]
+++

## Dos familias

Una bomba entrega energía a un líquido para elevarlo, darle presión o hacerlo circular.

| | Rotodinámicas (centrífugas, axiales) | De desplazamiento positivo |
|---|---|---|
| Principio | un rodete acelera el líquido | encierran un volumen y lo empujan |
| Caudal | depende de la presión del sistema | casi independiente de la presión |
| Presiones | bajas y medias | altas |
| Fluidos | poco viscosos | también muy viscosos |
| Ejemplos | centrífuga, de hélice | pistón, engranajes, paletas, tornillo, diafragma |

> [!WARNING]
> Una bomba de desplazamiento positivo nunca debe funcionar contra una válvula cerrada: la presión sube hasta que algo se rompe. Siempre lleva válvula de alivio.

## Bomba centrífuga

El líquido entra por el centro del **rodete**, los álabes lo impulsan hacia afuera y la **voluta** convierte parte de la velocidad en presión. La energía que el rodete entrega idealmente la da la ecuación de Euler de las turbomáquinas:

$$H_t = \frac{u_2\,c_{u2} - u_1\,c_{u1}}{g}$$

con $u$ la velocidad periférica del rodete y $c_u$ la componente tangencial de la velocidad del fluido.

## Altura y potencia

La **altura manométrica** $H$ es la energía por unidad de peso que la bomba entrega, en metros de columna del líquido. La potencia útil y la del eje son

$$P_h = \rho\,g\,Q\,H,\qquad P_{eje} = \frac{\rho\,g\,Q\,H}{\eta}$$

> [!NOTE]
> Elevar 20 L/s de agua con una altura total de 35 m y rendimiento 0,72:
>
> $P_{eje} = \dfrac{1000\cdot 9{,}81\cdot 0{,}020\cdot 35}{0{,}72}\approx 9{,}5$ kW. Se elige un motor de 11 kW.

## Curvas y punto de funcionamiento

![Curva característica de una bomba centrífuga y curva del sistema que se cruzan en el punto de funcionamiento](/media/electromecanica/curva-bomba.svg)

*La bomba trabaja donde su curva corta a la del sistema.*

El fabricante entrega, en función del caudal, la altura $H$, la potencia $P$, el rendimiento $\eta$ y el NPSH requerido. La bomba no "elige" su caudal: funciona en la intersección de su curva con la del sistema, $H_{sistema} = H_{est} + k\,Q^2$.

Conviene que ese punto quede cerca del de **máximo rendimiento**. Lejos de él aumentan las vibraciones, el desgaste y el consumo.

| Para regular el caudal | Efecto |
|---|---|
| Estrangular con una válvula | simple, pero desperdicia energía |
| Variar la velocidad | lo más eficiente |
| Recircular (bypass) | desperdicia energía |

## Leyes de semejanza

Al cambiar la velocidad de giro $n$ de una misma bomba:

$$\frac{Q_2}{Q_1} = \frac{n_2}{n_1},\qquad \frac{H_2}{H_1} = \left(\frac{n_2}{n_1}\right)^2,\qquad \frac{P_2}{P_1} = \left(\frac{n_2}{n_1}\right)^3$$

Reducir la velocidad un 20 % baja la potencia casi a la mitad. De ahí el ahorro que logran los **variadores de frecuencia** en bombas y ventiladores.

## Cavitación y NPSH

Si la presión en la entrada del rodete baja hasta la presión de vapor, el líquido hierve; las burbujas colapsan más adelante, con ruido de grava, vibraciones y erosión del rodete.

Para evitarlo, la altura neta positiva de aspiración **disponible** en la instalación debe superar a la **requerida** por la bomba:

$$NPSH_d = \frac{p_{atm} - p_v}{\rho g} - h_{asp} - h_{pérd,asp}\ >\ NPSH_r$$

con $h_{asp}$ el desnivel entre el nivel del líquido y la bomba (positivo si la bomba está arriba).

> [!NOTE]
> Agua a 20 °C ($p_v = 2{,}3$ kPa) al nivel del mar, bomba 4 m por encima del nivel y 1 m de pérdidas en la aspiración:
>
> $NPSH_d = \dfrac{101{,}3 - 2{,}3}{9{,}81} - 4 - 1\approx 5{,}1$ m. Sirve una bomba con $NPSH_r$ de 3 m; no una de 6 m.

En Salta capital, a unos 1200 m de altitud, la presión atmosférica ronda los 88 kPa y se pierden casi 1,4 m de NPSH disponible. Con agua caliente el margen se reduce todavía más.

## Bombas en serie y en paralelo

| Disposición | Resultado |
|---|---|
| En serie | se suman las alturas para el mismo caudal |
| En paralelo | se suman los caudales para la misma altura |

En paralelo, el caudal total no llega a duplicarse, porque al crecer el caudal crecen las pérdidas del sistema.

## Velocidad específica

Clasifica la forma del rodete más adecuada para cada servicio:

$$n_s = \frac{n\sqrt Q}{H^{3/4}}$$

Valores bajos corresponden a rodetes **radiales** (mucha altura, poco caudal); intermedios, a los de flujo **mixto**; altos, a los **axiales** (mucho caudal, poca altura).

## Instalación

- La cañería de aspiración debe ser corta, de diámetro generoso y sin puntos altos donde se acumule aire.
- La bomba centrífuga común no es autocebante: hay que **cebarla** (llenarla de líquido) antes de arrancar.
- Se arranca con la válvula de impulsión cerrada, porque así la potencia absorbida es mínima.
