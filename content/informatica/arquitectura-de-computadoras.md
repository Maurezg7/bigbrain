+++
title = "Arquitectura de von Neumann"
summary = "CPU, memoria y entrada/salida: cómo ejecuta un programa una computadora, instrucción por instrucción."
branch = "Arquitectura de computadoras"
order = 1
level = "Intermedio"
tags = ["CPU", "memoria", "ciclo de instrucción", "caché"]
+++

## La idea de programa almacenado

Casi todas las computadoras siguen el modelo propuesto por John von Neumann en 1945: las **instrucciones** del programa se guardan en la misma memoria que los **datos**, y un procesador las lee y ejecuta una tras otra. Cambiar de programa es solo cambiar el contenido de la memoria.

![Diagrama de la arquitectura de von Neumann con CPU, memoria y entrada/salida unidas por buses](/media/informatica/von-neumann.svg)

*CPU, memoria y dispositivos de entrada/salida se comunican a través de los buses.*

## Componentes

### CPU

La unidad central de procesamiento tiene tres partes:

- **Unidad de control**: interpreta cada instrucción y genera las señales que coordinan al resto.
- **ALU** (unidad aritmético-lógica): hace las sumas, restas, comparaciones y operaciones lógicas.
- **Registros**: memorias muy pequeñas y rapidísimas dentro de la CPU. Dos son esenciales: el **contador de programa** (PC), con la dirección de la próxima instrucción, y el **registro de instrucción** (IR), con la que se está ejecutando.

### Memoria principal

La **RAM** es una sucesión de celdas, cada una con una **dirección**. Es rápida y volátil: se borra al cortar la energía.

### Entrada/salida y buses

Teclado, pantalla, disco y red se conectan mediante controladores. Los **buses** son los conductores que transportan direcciones, datos y señales de control entre los componentes.

## El ciclo de instrucción

La CPU repite sin parar tres pasos:

1. **Búsqueda** (*fetch*): lee de la memoria la instrucción cuya dirección está en el PC, y lo incrementa.
2. **Decodificación** (*decode*): la unidad de control determina qué operación es y con qué operandos.
3. **Ejecución** (*execute*): la ALU opera, se accede a memoria o se modifica el PC (en un salto).

> [!NOTE]
> La instrucción de alto nivel `c = a + b` se traduce en varias instrucciones de máquina:
>
> ```asm
> LOAD  R1, a      ; copia a de la memoria al registro R1
> LOAD  R2, b      ; copia b al registro R2
> ADD   R3, R1, R2 ; la ALU suma y deja el resultado en R3
> STORE R3, c      ; guarda R3 en la posición de memoria de c
> ```
>
> Cada una pasa por el ciclo completo de búsqueda, decodificación y ejecución.

Un procesador de 3 GHz ejecuta del orden de tres mil millones de ciclos de reloj por segundo. Para ganar velocidad se usa **segmentación** (*pipeline*: varias instrucciones en distintas etapas a la vez) y varios **núcleos** que trabajan en paralelo.

## Jerarquía de memoria

La memoria rápida es cara y la barata es lenta. La solución es combinarlas en niveles:

| Nivel | Capacidad típica | Tiempo de acceso | Volátil |
|---|---|---|---|
| Registros | cientos de bytes | < 1 ns | sí |
| Caché (L1–L3) | KB a decenas de MB | 1–20 ns | sí |
| RAM | 8–64 GB | ~100 ns | sí |
| SSD | 256 GB – 2 TB | ~100 µs | no |
| Disco rígido | 1–20 TB | ~10 ms | no |

La **memoria caché** guarda copias de los datos usados recientemente. Funciona gracias al **principio de localidad**: los programas tienden a reutilizar los mismos datos (localidad temporal) y a acceder a datos vecinos (localidad espacial).

> [!TIP]
> Por eso recorrer un arreglo en orden es mucho más rápido que saltar por posiciones al azar, aunque la cantidad de operaciones sea la misma: los elementos vecinos ya están en la caché.

## Conjunto de instrucciones

El **ISA** (*instruction set architecture*) es el repertorio de instrucciones que entiende un procesador. Los más difundidos son **x86-64** (PC y servidores), de tipo CISC, con muchas instrucciones complejas, y **ARM** (celulares, Apple Silicon) y **RISC-V**, de tipo RISC, con pocas instrucciones simples y de bajo consumo.
