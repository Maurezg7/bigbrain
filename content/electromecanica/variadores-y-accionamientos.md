+++
title = "Variadores de velocidad y accionamientos"
summary = "Accionamiento eléctrico, cuadrantes, variador de frecuencia, control escalar V/f y vectorial, arranque suave, frenado, selección del motor y servomotores."
branch = "Máquinas eléctricas"
order = 5
level = "Avanzado"
tags = ["variador de frecuencia", "accionamientos", "control vectorial", "arrancador suave", "servomotor"]
+++

## Accionamiento eléctrico

Es el sistema que convierte energía eléctrica en movimiento **controlado**:

```text
red ──▶ convertidor ──▶ motor ──▶ transmisión ──▶ carga
            ▲                                       │
            └────────── control ◀── sensores ◀──────┘
```

Los motores consumen cerca de la mitad de la energía eléctrica mundial. Controlar su velocidad en lugar de estrangular el flujo o frenar mecánicamente ahorra gran parte de ella.

## Dinámica del accionamiento

$$T_m - T_c = J \frac{d\omega}{dt}$$

| Símbolo | Significado |
|---|---|
| $T_m$ | par del motor |
| $T_c$ | par resistente de la carga |
| $J$ | momento de inercia total, referido al eje del motor |
| $\omega$ | velocidad angular |

Si $T_m > T_c$, acelera; si son iguales, la velocidad es constante.

Con un reductor de relación $i = \omega_m / \omega_c$, la carga se refiere al eje del motor:

$$T_c' = \frac{T_c}{i \, \eta} \qquad J_c' = \frac{J_c}{i^2}$$

### Tipos de carga

| Carga | Par | Potencia | Ejemplos |
|---|---|---|---|
| Par constante | $T = \text{cte}$ | $\propto n$ | cintas transportadoras, elevadores, compresores de pistón |
| Par cuadrático | $T \propto n^2$ | $\propto n^3$ | bombas centrífugas, ventiladores |
| Potencia constante | $T \propto 1/n$ | cte | bobinadoras, husillos de máquinas herramienta |
| Par lineal | $T \propto n$ | $\propto n^2$ | calandras, frenos viscosos |

> [!IMPORTANT]
> En bombas y ventiladores, la potencia varía con el **cubo** de la velocidad. Reducir la velocidad un 20 % baja la potencia a $0{,}8^3 = 51\ \%$.

### Cuadrantes de operación

| Cuadrante | Velocidad | Par | Funcionamiento |
|---|---|---|---|
| I | + | + | motor, hacia adelante |
| II | + | − | freno (generador), hacia adelante |
| III | − | − | motor, hacia atrás |
| IV | − | + | freno, hacia atrás |

Una bomba trabaja en un cuadrante; un ascensor o una grúa, en los cuatro.

## Velocidad del motor de inducción

$$n_s = \frac{120 f}{p} \qquad n = n_s (1 - s)$$

con $f$ la frecuencia, $p$ el número de polos y $s$ el deslizamiento. Para variar la velocidad de forma continua y eficiente hay que variar la **frecuencia**.

## Variador de frecuencia

```text
red trifásica ──▶ [ rectificador ] ──▶ [ bus de continua ] ──▶ [ inversor ] ──▶ motor
   50 Hz              CA → CC            capacitores            CC → CA      f y V variables
```

| Etapa | Función |
|---|---|
| Rectificador | convierte la alterna de la red en continua (puente de diodos) |
| Bus de continua | capacitores que filtran y almacenan energía; unos 540 V con red de 400 V |
| Inversor | seis IGBT que conmutan para sintetizar una alterna de frecuencia y tensión variables |

### Modulación por ancho de pulso

El inversor no entrega una senoide, sino **pulsos** de amplitud fija y ancho variable (PWM). La inductancia del motor los filtra y la corriente resulta casi senoidal. La **frecuencia de conmutación** típica es de 2 a 16 kHz.

## Control escalar V/f

El flujo del motor es proporcional a $V/f$. Si se bajara la frecuencia sin bajar la tensión, el núcleo se **saturaría**. Por eso se mantiene la relación constante:

$$\frac{V}{f} = \text{cte}$$

| Zona | Frecuencia | Tensión | Flujo | Par disponible |
|---|---|---|---|---|
| Par constante | 0 a $f_n$ | proporcional a $f$ | nominal | nominal |
| Debilitamiento de campo | mayor que $f_n$ | fija en $V_n$ | $\propto 1/f$ | $\propto 1/f$ (potencia constante) |

> [!NOTE]
> **Ejemplo.** Motor de 400 V y 50 Hz: $V/f = 8$ V/Hz. A 25 Hz se aplican 200 V; a 10 Hz, 80 V.

A frecuencias muy bajas, la caída en la resistencia del estator pesa demasiado y el par cae: se agrega un **refuerzo de tensión** (*boost*).

El control escalar es simple y no requiere sensor. Es adecuado para bombas, ventiladores y cintas, donde no se exige respuesta dinámica rápida.

## Control vectorial

El **control por orientación de campo** descompone la corriente del estator en dos componentes, en un sistema de referencia que gira con el flujo del rotor:

| Componente | Controla | Equivale, en un motor de continua, a |
|---|---|---|
| $i_d$ | el flujo | la corriente de excitación |
| $i_q$ | el par | la corriente de armadura |

$$T = k \, \psi_r \, i_q$$

Así el motor de inducción se controla como uno de corriente continua: par y flujo por separado.

| | Escalar V/f | Vectorial sin sensor | Vectorial con codificador |
|---|---|---|---|
| Precisión de velocidad | 1 a 3 % | 0,2 a 0,5 % | 0,01 % |
| Par a velocidad cero | no | limitado | sí, nominal |
| Respuesta dinámica | lenta | buena | muy buena |
| Aplicaciones | bombas, ventiladores | cintas, extrusoras, mezcladoras | grúas, ascensores, bobinadoras |

El **control directo de par** (DTC) es una alternativa: elige directamente el estado del inversor según los errores de par y de flujo.

## Parámetros de un variador

| Parámetro | Función |
|---|---|
| Datos de placa del motor | tensión, corriente, frecuencia, velocidad, $\cos\varphi$ |
| Rampas de aceleración y desaceleración | tiempo para ir de 0 a la frecuencia máxima |
| Frecuencias mínima y máxima | límites de velocidad |
| Límite de corriente | protección del motor y del variador |
| Fuente de la consigna | teclado, entrada analógica (0–10 V, 4–20 mA), bus de campo |
| Frecuencias de salto | evitar resonancias mecánicas |
| Regulador PID integrado | mantener presión, caudal o nivel |

La **identificación automática** mide los parámetros del motor, necesarios para el control vectorial.

## Frenado

Al desacelerar una carga con inercia, el motor funciona como **generador** y devuelve energía al bus de continua, cuya tensión sube.

| Método | Qué pasa con la energía | Uso |
|---|---|---|
| Rampa larga | la absorben las pérdidas | cargas de poca inercia |
| **Resistencia de frenado** | un transistor la disipa en una resistencia | frenados frecuentes |
| **Regenerativo** | un rectificador activo la devuelve a la red | grúas, ascensores, centrífugas |
| Inyección de continua | se disipa en el rotor | detención final; no es un freno de seguridad |
| Bus de continua compartido | la usan otros variadores | líneas con varios motores |

## Ahorro en bombas y ventiladores

Las **leyes de afinidad** para una máquina centrífuga:

$$\frac{Q_2}{Q_1} = \frac{n_2}{n_1} \qquad \frac{H_2}{H_1} = \left( \frac{n_2}{n_1} \right)^2 \qquad \frac{P_2}{P_1} = \left( \frac{n_2}{n_1} \right)^3$$

> [!NOTE]
> **Ejemplo.** Un ventilador de 30 kW trabaja 6000 h al año al 70 % del caudal.
>
> - Con persiana de estrangulación: consume cerca del 90 % de la potencia, 27 kW.
> - Con variador al 70 % de velocidad: $30 \times 0{,}7^3 = 10{,}3$ kW (unos 11 kW con las pérdidas del variador).
>
> Ahorro: $16 \times 6000 = 96\,000$ kWh por año.

En bombas con gran altura estática el ahorro es menor: las leyes de afinidad valen para la bomba, pero el punto de trabajo lo fija también la instalación.

## Arrancador suave

Usa **tiristores** en antiparalelo para aumentar gradualmente la **tensión** durante el arranque. No varía la frecuencia.

| | Arranque directo | Estrella-triángulo | Arrancador suave | Variador |
|---|---|---|---|---|
| Corriente de arranque | 6 a 8 $I_n$ | 2 a 2,7 $I_n$ | 2 a 4 $I_n$ | 1 a 1,5 $I_n$ |
| Par de arranque | pleno | un tercio | reducido ($\propto V^2$) | hasta 150 % |
| Control de velocidad | no | no | no | sí |
| Parada controlada | no | no | sí | sí |
| Costo | bajo | bajo | medio | alto |

Conviene para arranques y paradas suaves (bombas, para evitar el golpe de ariete) cuando no hace falta variar la velocidad.

## Efectos sobre el motor y la red

| Efecto | Causa | Solución |
|---|---|---|
| **Armónicos** en la red | el rectificador toma corriente no senoidal | reactancias de línea, filtros, rectificador activo |
| Picos de tensión en los bornes | flancos rápidos y cables largos | filtros $dV/dt$ o senoidales; motores con aislación reforzada |
| Corrientes por los rodamientos | tensión de modo común | rodamiento aislado, escobilla de puesta a tierra del eje |
| Calentamiento a baja velocidad | el ventilador propio gira lento | ventilación forzada o desclasificación |
| Interferencia electromagnética | la conmutación rápida | cable apantallado, filtros, buena puesta a tierra |

> [!WARNING]
> Un motor autoventilado que trabaja mucho tiempo a baja velocidad con par alto **se sobrecalienta**, aunque la corriente no supere la nominal. Usar ventilación independiente y protección por sondas de temperatura (PTC).

## Selección del motor

1. Determinar el **par** y la **velocidad** de la carga, y su ciclo.
2. Calcular la potencia: $P = T \omega$.
3. Considerar el **régimen de servicio**.
4. Verificar el **par de arranque** y el tiempo de aceleración.
5. Elegir el grado de protección, la forma constructiva y la clase de eficiencia.

| Servicio | Descripción |
|---|---|
| S1 | continuo, a carga constante |
| S2 | de corta duración |
| S3 | intermitente periódico |
| S6 | continuo con carga intermitente |

Para cargas variables se calcula el **par eficaz**:

$$T_{ef} = \sqrt{\frac{\sum T_i^2 \, t_i}{\sum t_i}}$$

y se elige un motor con par nominal mayor, verificando que el par máximo del ciclo no supere su capacidad.

| Clase de eficiencia | Denominación |
|---|---|
| IE1 | estándar |
| IE2 | alta |
| IE3 | *premium* |
| IE4 | *super premium* |

| Grado de protección | Significado |
|---|---|
| IP55 | protegido contra polvo y chorros de agua; el habitual |
| IP65 | hermético al polvo |

## Servomotores

Para control preciso de **posición**, velocidad y par.

| Característica | |
|---|---|
| Motor | síncrono de imanes permanentes; baja inercia |
| Realimentación | codificador o resolver de alta resolución |
| Sobrecarga | 3 veces el par nominal por instantes |
| Control | lazos en cascada: posición, velocidad, corriente |

| Motor | Uso |
|---|---|
| Servomotor | robots, control numérico, envasado |
| Paso a paso | posicionado sin realimentación, de baja potencia (impresoras 3D) |
| Sin escobillas (BLDC) | ventiladores, herramientas, vehículos |
| De reluctancia | accionamientos de alta eficiencia |

El **codificador incremental** entrega pulsos por vuelta; el **absoluto**, la posición exacta aun después de un corte de energía.

> [!TIP]
> Antes de instalar un variador, verificar que el motor lo admita (aislación), la longitud del cable, y configurar correctamente los datos de placa: un variador mal parametrizado protege mal y controla peor.
