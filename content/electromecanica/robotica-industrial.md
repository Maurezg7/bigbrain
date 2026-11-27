+++
title = "Robótica industrial"
summary = "Tipos de robots, grados de libertad, cinemática directa e inversa, matrices de transformación, jacobiano, trayectorias, actuadores, programación y seguridad."
branch = "Instrumentación y control automático"
order = 6
level = "Avanzado"
tags = ["robótica", "cinemática", "grados de libertad", "Denavit-Hartenberg", "jacobiano", "robots colaborativos"]
+++

## Qué es un robot industrial

Según la norma ISO 8373, es un **manipulador multipropósito, reprogramable y controlado automáticamente**, con tres o más ejes.

| Componente | Función |
|---|---|
| Manipulador | la estructura mecánica: eslabones unidos por articulaciones |
| Actuadores | mueven cada articulación (servomotores con reductor) |
| Sensores | miden la posición de cada eje (codificadores) y el entorno |
| Controlador | calcula las trayectorias y cierra los lazos de control |
| Efector final | la herramienta: pinza, ventosa, pistola de soldar |
| Consola de programación | para enseñar y programar |

## Articulaciones y grados de libertad

| Articulación | Movimiento | Símbolo |
|---|---|---|
| De rotación | giro alrededor de un eje | R |
| Prismática | desplazamiento a lo largo de un eje | P |

Los **grados de libertad** son los movimientos independientes. Ubicar y orientar libremente un objeto en el espacio exige **seis**: tres de posición y tres de orientación.

| Grados de libertad | Capacidad |
|---|---|
| menos de 6 | no alcanza todas las posiciones y orientaciones |
| 6 | posición y orientación arbitrarias |
| más de 6 | **redundante**: infinitas configuraciones para la misma pose; esquiva obstáculos |

## Configuraciones

| Tipo | Ejes | Espacio de trabajo | Usos |
|---|---|---|---|
| **Cartesiano** (pórtico) | PPP | prisma rectangular | paletizado, control numérico, impresión 3D |
| Cilíndrico | RPP | cilindro | carga de máquinas |
| Esférico (polar) | RRP | porción de esfera | históricos |
| **SCARA** | RRP, con ejes verticales | cilindro | ensamblaje rápido, *pick and place* |
| **Articulado** (antropomórfico) | 6 R | casi esférico | soldadura, pintura, manipulación: el más común |
| **Paralelo** (delta) | cadenas cerradas | reducido | envasado a muy alta velocidad |

Un robot articulado de seis ejes tiene tres para posicionar (cintura, hombro, codo) y tres en la **muñeca** para orientar.

### Especificaciones

| Parámetro | Significado | Valores típicos |
|---|---|---|
| Carga útil | masa máxima en la muñeca, con la herramienta | 3 a 1000 kg |
| Alcance | distancia máxima del centro de la muñeca | 0,5 a 3,5 m |
| **Repetibilidad** | dispersión al volver muchas veces al mismo punto | ±0,02 a ±0,1 mm |
| Exactitud | diferencia entre el punto ordenado y el alcanzado | peor que la repetibilidad |
| Velocidad | de cada eje y de la herramienta | hasta varios m/s |

> [!IMPORTANT]
> **Repetibilidad** no es **exactitud**. Un robot puede volver siempre al mismo punto (muy repetible) y que ese punto no sea el que se calculó. Por eso los puntos enseñados a mano son más confiables que los calculados fuera de línea, salvo que se calibre.

## Descripción de la posición

### Matriz de rotación

Describe la orientación de un sistema respecto de otro. Para un giro $\theta$ alrededor del eje $z$:

$$R_z(\theta) = \begin{pmatrix} \cos\theta & -\sin\theta & 0 \\ \sin\theta & \cos\theta & 0 \\ 0 & 0 & 1 \end{pmatrix}$$

Es **ortogonal**: $R^{-1} = R^T$ y $\det R = 1$. Las rotaciones se componen multiplicando matrices, y el orden importa.

La orientación también se expresa con tres ángulos (de Euler; alabeo, cabeceo y guiñada) o con **cuaterniones**, que evitan las singularidades.

### Transformación homogénea

Reúne rotación y traslación en una matriz de $4 \times 4$:

$$T = \begin{pmatrix} R & \mathbf{p} \\ \mathbf{0}^T & 1 \end{pmatrix}$$

Las transformaciones sucesivas se **encadenan** multiplicando:

$$T_0^n = T_0^1 \, T_1^2 \cdots T_{n-1}^n$$

## Cinemática directa

Dados los ángulos de las articulaciones, hallar la **pose** (posición y orientación) de la herramienta. Siempre tiene solución única.

> [!NOTE]
> **Brazo plano de dos eslabones**, de longitudes $l_1$ y $l_2$, con ángulos $\theta_1$ y $\theta_2$:
> $$x = l_1 \cos\theta_1 + l_2 \cos(\theta_1 + \theta_2)$$
> $$y = l_1 \sin\theta_1 + l_2 \sin(\theta_1 + \theta_2)$$
>
> Con $l_1 = 0{,}4$ m, $l_2 = 0{,}3$ m, $\theta_1 = 30°$ y $\theta_2 = 60°$:
> $$x = 0{,}4 \cos 30° + 0{,}3 \cos 90° = 0{,}346\ \text{m} \qquad y = 0{,}4 \sin 30° + 0{,}3 \sin 90° = 0{,}500\ \text{m}$$

### Parámetros de Denavit-Hartenberg

Un método sistemático: a cada eslabón se le fija un sistema de coordenadas, y la relación entre dos consecutivos queda descrita por **cuatro** parámetros.

| Parámetro | Significado |
|---|---|
| $\theta_i$ | ángulo de giro alrededor de $z$ (la variable, en una articulación de rotación) |
| $d_i$ | desplazamiento a lo largo de $z$ (la variable, en una prismática) |
| $a_i$ | longitud del eslabón, a lo largo de $x$ |
| $\alpha_i$ | torsión del eslabón, alrededor de $x$ |

## Cinemática inversa

Dada la pose deseada, hallar los ángulos de las articulaciones. Es el problema que se resuelve en cada movimiento, y es más difícil:

| Dificultad | |
|---|---|
| Puede no haber solución | el punto está fuera del espacio de trabajo |
| Puede haber varias | «codo arriba» y «codo abajo»; hasta 8 en un robot de seis ejes |
| Ecuaciones no lineales | solución analítica solo para ciertas geometrías |

> [!NOTE]
> **Brazo plano de dos eslabones.** Por el teorema del coseno:
> $$\cos\theta_2 = \frac{x^2 + y^2 - l_1^2 - l_2^2}{2\, l_1 l_2}$$
> $$\theta_1 = \operatorname{atan2}(y, x) - \operatorname{atan2}(l_2 \sin\theta_2,\ l_1 + l_2 \cos\theta_2)$$
>
> Los dos signos posibles de $\sin\theta_2$ dan las dos soluciones.
>
> Para $(x, y) = (0{,}346;\ 0{,}500)$, el punto del ejemplo anterior: $\cos\theta_2 = 0{,}5$. Con $\theta_2 = 60°$ resulta $\theta_1 = 55{,}3° - 25{,}3° = 30°$, los ángulos de partida. La otra solución es $\theta_2 = -60°$ con $\theta_1 = 55{,}3° + 25{,}3° = 80{,}6°$.

Los robots industriales de seis ejes tienen **muñeca esférica** (los tres últimos ejes se cortan en un punto), lo que permite separar el problema en posición y orientación y resolverlo analíticamente. En los demás casos se usan métodos numéricos iterativos.

## Jacobiano

Relaciona las **velocidades** de las articulaciones con la velocidad de la herramienta:

$$\mathbf{v} = J(\mathbf{q}) \, \dot{\mathbf{q}}$$

Para el brazo plano, con $s_1 = \sin\theta_1$, $c_{12} = \cos(\theta_1 + \theta_2)$, etc.:

$$J = \begin{pmatrix} -l_1 s_1 - l_2 s_{12} & -l_2 s_{12} \\ l_1 c_1 + l_2 c_{12} & l_2 c_{12} \end{pmatrix}$$

También relaciona fuerzas: los pares necesarios en las articulaciones para ejercer una fuerza $\mathbf{F}$ con la herramienta son

$$\boldsymbol{\tau} = J^T \mathbf{F}$$

### Singularidades

Donde $\det J = 0$, el robot pierde la capacidad de moverse en alguna dirección, y cerca de allí se necesitan velocidades articulares enormes.

Para el brazo plano, $\det J = l_1 l_2 \sin\theta_2$: es singular con el brazo totalmente **extendido** o **plegado**.

| Singularidad en un robot de seis ejes | Ocurre cuando |
|---|---|
| De muñeca | los ejes 4 y 6 quedan alineados |
| De codo | el brazo está completamente extendido |
| De hombro | el centro de la muñeca pasa sobre el eje 1 |

> [!WARNING]
> Al cruzar una singularidad en movimiento lineal, el controlador se detiene con error o algún eje gira a gran velocidad. Se evita cambiando la trayectoria o la ubicación de la pieza.

## Dinámica

Los pares necesarios para un movimiento dado:

$$\boldsymbol{\tau} = M(\mathbf{q}) \, \ddot{\mathbf{q}} + C(\mathbf{q}, \dot{\mathbf{q}}) \, \dot{\mathbf{q}} + \mathbf{g}(\mathbf{q})$$

| Término | Representa |
|---|---|
| $M(\mathbf{q}) \, \ddot{\mathbf{q}}$ | inercia, que depende de la configuración |
| $C \, \dot{\mathbf{q}}$ | fuerzas centrífugas y de Coriolis |
| $\mathbf{g}(\mathbf{q})$ | gravedad |

El controlador usa este modelo para anticipar los pares (**par calculado**), sumado a lazos de realimentación en cascada de posición, velocidad y corriente en cada eje.

## Generación de trayectorias

| Movimiento | Trayectoria de la herramienta | Uso |
|---|---|---|
| **Articular** (punto a punto) | no es recta; cada eje va por el camino más corto | el más rápido; traslados libres |
| **Lineal** | una recta | aproximaciones, soldadura, pegado |
| **Circular** | un arco definido por tres puntos | contornos |
| Por *spline* | una curva suave por varios puntos | pintura, mecanizado |

Para que el movimiento sea suave, cada eje sigue un **perfil de velocidad**:

| Perfil | Característica |
|---|---|
| Trapezoidal | aceleración constante, velocidad constante, desaceleración; la aceleración salta |
| En S | limita la **sobreaceleración** (*jerk*): menos vibraciones |
| Polinomio de quinto grado | posición, velocidad y aceleración continuas |

En los puntos intermedios se puede programar una **zona de aproximación**: el robot pasa cerca sin detenerse.

## Actuadores y transmisiones

| Elemento | Detalle |
|---|---|
| Motor | servomotor síncrono de imanes permanentes, con freno |
| Reductor armónico | relación alta (50 a 160), sin juego, compacto; para las muñecas |
| Reductor cicloidal | robusto y rígido; para los ejes principales |
| Codificador absoluto | conserva la posición sin energía |

### Efectores finales

| Tipo | Uso |
|---|---|
| Pinza neumática o eléctrica | piezas rígidas |
| Ventosas | superficies lisas: chapas, cajas, vidrio |
| Pinza magnética | piezas ferrosas |
| Herramienta de proceso | soldadura, pintura, atornillado, pegado |
| Cambiador automático | intercambia herramientas |

Hay que declarar el **punto central de la herramienta** (TCP) y su masa: el controlador refiere a ese punto los movimientos.

## Sistemas de coordenadas

| Sistema | Origen | Uso |
|---|---|---|
| Articular | cada eje | mover un eje por vez |
| De la base | la base del robot | movimientos cartesianos |
| Del mundo | fijo en la celda | varios robots |
| De la herramienta | el TCP | acercar o retirar en la dirección de la herramienta |
| Del usuario (de la pieza) | la mesa o el dispositivo | si la mesa se mueve, se redefine el sistema y el programa sigue valiendo |

## Programación

| Método | Descripción | Ventaja | Inconveniente |
|---|---|---|---|
| **Por enseñanza** | se lleva el robot a cada punto con la consola y se lo memoriza | simple y exacto | detiene la producción |
| **Guiado manual** | se mueve el brazo con la mano | muy intuitivo | poca precisión |
| **Fuera de línea** | se programa en un simulador 3D | no detiene la celda; detecta colisiones | requiere calibrar |

Cada fabricante tiene su lenguaje (RAPID, KRL, Karel, URScript), con una estructura similar:

```text
PROC principal()
    MoveJ inicio, v1000, z50, pinza;        ! articular, rápido
    MoveL sobre_pieza, v500, z10, pinza;    ! lineal
    MoveL toma, v100, fine, pinza;          ! lineal, lento, punto exacto
    SetDO cerrar_pinza, 1;
    WaitTime 0.3;
    MoveL sobre_pieza, v500, z10, pinza;
    MoveJ deposito, v1000, fine, pinza;
    SetDO cerrar_pinza, 0;
ENDPROC
```

Cada instrucción indica el tipo de movimiento, el punto, la velocidad, la zona de aproximación y la herramienta. El robot intercambia señales con el PLC de la celda mediante entradas y salidas digitales o buses de campo.

## Aplicaciones

| Aplicación | Requisito principal |
|---|---|
| Soldadura por puntos | carga útil, repetibilidad |
| Soldadura por arco | seguimiento de trayectorias |
| Pintura | movimientos suaves; protección antiexplosiva |
| Paletizado | alcance, velocidad |
| Carga de máquinas | fiabilidad |
| Ensamblaje | precisión; control de fuerza |
| *Pick and place* | velocidad; visión artificial |
| Mecanizado y desbarbado | rigidez |

La **visión artificial** permite ubicar piezas desordenadas e inspeccionar; los **sensores de fuerza**, ensamblar con contacto controlado.

## Seguridad

Un robot industrial es rápido y fuerte, y **no percibe** a las personas. Normas: ISO 10218 e ISO/TS 15066.

| Medida | Función |
|---|---|
| Cerramiento perimetral | impedir el acceso a la zona de trabajo |
| Puertas con enclavamiento | detener el robot al abrir |
| Barreras y escáneres láser | detectar la entrada de una persona |
| Paradas de emergencia | en la consola, el controlador y la celda |
| Dispositivo de habilitación de tres posiciones | en la consola: el robot solo se mueve con el pulsador a medio recorrido |
| Velocidad reducida en modo manual | 250 mm/s como máximo |
| Límites de ejes por software y mecánicos | restringir el espacio |

> [!WARNING]
> Un robot detenido **no está apagado**: puede estar esperando una señal y arrancar sin aviso. Antes de entrar a la celda hay que aplicar el procedimiento de bloqueo y llevar consigo la consola.

### Robots colaborativos

Los **cobots** están diseñados para compartir el espacio con personas, sin cerramiento.

| Modo de colaboración | Principio |
|---|---|
| Parada monitoreada | el robot se detiene cuando la persona entra |
| Guiado manual | la persona lo mueve con la mano |
| Control de velocidad y separación | reduce la velocidad según la distancia |
| Limitación de potencia y fuerza | detecta el contacto y se detiene antes de lastimar |

Son más lentos y de menor carga que los industriales. Que el robot sea colaborativo no vuelve segura a la aplicación: una herramienta filosa o una pieza caliente siguen siendo peligrosas, y la **evaluación de riesgos** es obligatoria.

> [!TIP]
> Al diseñar una celda, simular primero: verificar alcances, tiempos de ciclo, colisiones y singularidades en el programa de simulación evita rehacer la instalación.
