+++
title = "Autómatas programables (PLC)"
summary = "Arquitectura y ciclo de barrido, entradas y salidas, lenguajes IEC 61131-3, temporizadores, contadores, Grafcet y SCADA."
branch = "Instrumentación y control automático"
order = 4
level = "Intermedio"
tags = ["PLC", "ladder", "Grafcet", "automatización", "SCADA"]
+++

## De los relés al PLC

Un autómata programable (PLC) es una computadora industrial que reemplaza a los tableros de relés. La lógica deja de estar en el cableado y pasa a un programa, que se modifica sin tocar un cable.

| Parte | Función |
|---|---|
| Fuente | alimenta al equipo, normalmente con 24 V de corriente continua |
| CPU | ejecuta el programa |
| Entradas digitales | pulsadores, finales de carrera, sensores |
| Salidas digitales | contactores, electroválvulas, pilotos |
| Entradas y salidas analógicas | 4 a 20 mA, 0 a 10 V, Pt100, termocuplas |
| Comunicaciones | Ethernet, Modbus, Profibus, Profinet |

Las salidas pueden ser a **relé** (cualquier carga, lentas), a **transistor** (corriente continua, rápidas) o a **triac** (corriente alterna).

## Ciclo de barrido

El PLC repite sin parar:

1. lee todas las entradas y las guarda en la **imagen de entradas**;
2. ejecuta el programa, de arriba hacia abajo, usando esa imagen;
3. escribe las salidas;
4. atiende comunicaciones y diagnósticos.

Un ciclo dura unos milisegundos. Una señal más breve que el ciclo puede perderse; para esos casos existen entradas rápidas e interrupciones.

## Lenguajes

La norma **IEC 61131-3** define cinco:

| Lenguaje | Sigla | Carácter |
|---|---|---|
| Diagrama de contactos (*ladder*) | LD | gráfico; imita un esquema de relés |
| Diagrama de bloques de función | FBD | gráfico; bloques conectados |
| Texto estructurado | ST | textual, parecido a Pascal |
| Lista de instrucciones | IL | textual de bajo nivel; en desuso |
| Diagrama funcional secuencial | SFC | gráfico; secuencias, derivado del Grafcet |

## Diagrama de contactos

![Diagrama ladder de un arranque con retención: pulsador de marcha, pulsador de parada y bobina del motor](/media/electromecanica/ladder.svg)

*Marcha y parada con retención: el contacto de la propia salida mantiene el circuito cerrado.*

Cada **escalón** conecta contactos (condiciones) con una bobina (resultado). Contactos en serie equivalen a AND y en paralelo a OR.

| Símbolo | Significado |
|---|---|
| Contacto abierto | deja pasar si la variable vale 1 |
| Contacto cerrado | deja pasar si la variable vale 0 |
| Bobina | la variable toma el valor del escalón |
| Bobinas *set* y *reset* | enclavan o desenclavan la variable |

El ejemplo clásico es el arranque de un motor con **retención**:

$$\text{Motor} = (\text{Marcha} + \text{Motor})\cdot\overline{\text{Parada}}\cdot\overline{\text{Térmico}}$$

El mismo circuito en texto estructurado:

```iecst
Motor := (Marcha OR Motor) AND NOT Parada AND NOT Termico;
```

> [!WARNING]
> El pulsador de parada y los dispositivos de seguridad se cablean con contacto **normalmente cerrado**: si el cable se corta, la máquina se detiene. Las paradas de emergencia actúan además por un circuito de seguridad independiente del programa.

## Temporizadores y contadores

| Bloque | Comportamiento |
|---|---|
| TON (retardo a la conexión) | la salida se activa cuando la entrada lleva activa el tiempo fijado |
| TOF (retardo a la desconexión) | la salida se mantiene un tiempo después de caer la entrada |
| TP (pulso) | genera un pulso de duración fija |
| CTU y CTD | cuentan flancos hacia arriba o hacia abajo |

```iecst
(* Arranque estrella-triángulo: 5 s en estrella *)
Temporizador(IN := Linea, PT := T#5s);
Linea     := (Marcha OR Linea) AND NOT Parada;
Estrella  := Linea AND NOT Temporizador.Q;
Triangulo := Linea AND Temporizador.Q AND NOT Estrella;
```

El enclavamiento entre estrella y triángulo se repite en el cableado de los contactores, porque cerrar ambos a la vez es un cortocircuito.

## Grafcet

Los procesos secuenciales se describen mejor con un **Grafcet** (SFC): una sucesión de **etapas**, cada una con sus acciones, separadas por **transiciones** con la condición para avanzar.

| Etapa | Acción | Transición a la siguiente |
|---|---|---|
| 0 | reposo | pulsador de inicio y pieza presente |
| 1 | avanza el cilindro de sujeción | final de carrera de sujeción |
| 2 | baja el taladro | final de carrera inferior |
| 3 | sube el taladro | final de carrera superior |
| 4 | retrocede la sujeción | final de carrera de reposo; vuelve a la etapa 0 |

Solo las etapas activas ejecutan sus acciones. Admite ramas alternativas y secuencias simultáneas.

## Señales analógicas

El PLC convierte la señal en un número entero y el programa lo **escala** a unidades de ingeniería:

$$x = x_{min} + (x_{max} - x_{min})\,\frac{N - N_{min}}{N_{max} - N_{min}}$$

> [!NOTE]
> Entrada de 4 a 20 mA que el módulo convierte en el rango de 0 a 27 648, conectada a un transmisor de 0 a 200 °C. Si se lee 13 824: $T = 200\cdot\dfrac{13\,824}{27\,648} = 100$ °C.

Con esos valores se programan lazos PID con bloques ya incluidos.

## Supervisión

| Nivel | Equipos |
|---|---|
| Campo | sensores, actuadores, variadores |
| Control | PLC |
| Supervisión | paneles HMI y sistemas SCADA |
| Gestión | sistemas de planta y de empresa |

Un **SCADA** muestra el proceso en pantallas, registra históricos, gestiona alarmas y permite operar a distancia.

## Buenas prácticas

- Nombres simbólicos y comentarios en todas las variables.
- Programa dividido en bloques por máquina o función.
- Modos manual y automático bien separados, con condiciones iniciales claras.
- Prever qué pasa al volver la tensión: las salidas no deben activarse solas.
- Respaldo del programa y control de versiones.
