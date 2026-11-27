+++
title = "Máquinas de corriente continua"
summary = "Principio, colector, fuerza electromotriz y par, tipos de excitación, curvas características y control de velocidad."
branch = "Máquinas eléctricas"
order = 3
level = "Intermedio"
tags = ["motor de CC", "colector", "excitación", "par", "control de velocidad"]
+++

## Constitución

- **Estator (inductor)**: polos con bobinas de excitación o imanes permanentes que crean el flujo $\Phi$.
- **Rotor (inducido)**: bobinado alojado en ranuras, por donde circula la corriente de armadura $I_a$.
- **Colector y escobillas**: un rectificador mecánico que invierte la corriente de cada bobina cuando pasa por la zona neutra, de modo que el par siempre empuja en el mismo sentido.

La misma máquina funciona como motor o como generador (dínamo).

## Ecuaciones fundamentales

$$E = k\,\Phi\,\omega,\qquad T = k\,\Phi\,I_a$$

$E$ es la fuerza electromotriz inducida en el rotor y $T$ el par. En un motor, con resistencia de armadura $R_a$:

$$V = E + I_a R_a \quad\Longrightarrow\quad \omega = \frac{V - I_a R_a}{k\,\Phi}$$

De aquí salen las tres formas de variar la velocidad: cambiar la tensión de armadura, cambiar el flujo o agregar resistencia en serie.

## Tipos de excitación

| Conexión | Característica | Aplicación |
|---|---|---|
| Independiente o imanes | velocidad casi constante, muy controlable | accionamientos regulados, servos |
| Derivación (shunt) | velocidad casi constante con la carga | máquinas herramienta |
| Serie | par de arranque enorme; la velocidad cae con la carga | tracción, grúas, motor de arranque |
| Compuesta | intermedia | laminadores, prensas |

En el motor **serie** el flujo depende de la corriente, por lo que $T \propto I_a^2$ mientras no sature.

> [!WARNING]
> Un motor serie sin carga se embala hasta destruirse, porque al bajar la corriente baja el flujo y la velocidad crece sin límite. Nunca se acopla con correas que puedan salirse. Algo parecido ocurre en un motor derivación si se corta el circuito de excitación.

## Arranque

En el arranque $\omega = 0$ y por lo tanto $E = 0$: la corriente solo está limitada por $R_a$, que es muy pequeña.

> [!NOTE]
> Motor de 220 V, $R_a = 0{,}5\ \Omega$, corriente nominal 20 A.
>
> Corriente de arranque directo: $\dfrac{220}{0{,}5} = 440$ A, 22 veces la nominal.
>
> A plena carga: $E = 220 - 20\cdot 0{,}5 = 210$ V. La potencia convertida en mecánica es $E\,I_a = 4200$ W y las pérdidas en el cobre del inducido, $I_a^2R_a = 200$ W.

Por eso se arranca con resistencias que se van eliminando o, hoy, con un convertidor electrónico que limita la corriente.

## Control de velocidad

- **Por tensión de armadura**: de cero a la velocidad nominal, con par constante.
- **Por debilitamiento de campo**: por encima de la nominal, con potencia constante y menos par.

Esta respuesta lineal y sencilla hizo del motor de continua el estándar de los accionamientos de velocidad variable durante décadas. Hoy lo reemplazan el motor de inducción con variador y los motores sin escobillas, que no requieren mantenimiento del colector.

## Reacción de inducido y conmutación

La corriente del rotor crea su propio campo, que deforma el del estator y desplaza la zona neutra: aparecen chispas en las escobillas. Se corrige con **polos auxiliares** (de conmutación) y, en máquinas grandes, con bobinados de compensación.

## Pérdidas y rendimiento

$$\eta = \frac{P_{útil}}{P_{absorbida}}$$

Las pérdidas son en el cobre (inducido y excitación), en el hierro, mecánicas (rozamiento y ventilación) y en las escobillas. El rendimiento va del 75 % en máquinas chicas a más del 93 % en las grandes.
