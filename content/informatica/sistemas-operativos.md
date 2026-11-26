+++
title = "Sistemas operativos: procesos y memoria"
summary = "Qué hace el sistema operativo: repartir la CPU entre procesos, administrar la memoria y los archivos."
branch = "Sistemas operativos"
order = 1
level = "Intermedio"
tags = ["procesos", "hilos", "planificación", "memoria virtual"]
+++

## Qué es un sistema operativo

El **sistema operativo** (Linux, Windows, macOS, Android) es el programa que administra el hardware y ofrece servicios a los demás programas. Tiene dos funciones:

- **Abstraer** el hardware: un programa pide "abrir un archivo" sin saber qué tipo de disco hay.
- **Administrar recursos**: reparte CPU, memoria y dispositivos entre programas que compiten por ellos.

Su núcleo, el **kernel**, corre en modo privilegiado. Los programas le piden servicios mediante **llamadas al sistema** (*system calls*) como `open`, `read`, `write` o `fork`.

## Procesos e hilos

Un **proceso** es un programa en ejecución: su código, sus datos, su pila y el estado de la CPU. Cada proceso tiene su propio espacio de memoria, aislado de los demás.

Un **hilo** (*thread*) es una línea de ejecución dentro de un proceso. Los hilos de un mismo proceso **comparten memoria**, lo que hace barata la comunicación pero exige cuidado para no corromper datos compartidos.

![Diagrama de estados de un proceso: nuevo, listo, en ejecución, bloqueado y terminado](/media/informatica/estados-proceso.svg)

*Estados de un proceso y transiciones entre ellos.*

Un proceso **en ejecución** pasa a **bloqueado** cuando espera algo (leer del disco, un dato de la red) y vuelve a **listo** cuando eso ocurre.

## Planificación de la CPU

Con más procesos listos que núcleos, el **planificador** decide quién ejecuta. Alterna tan rápido entre ellos (**cambio de contexto**) que parecen correr a la vez.

| Algoritmo | Idea | Problema |
|---|---|---|
| FCFS | por orden de llegada | un proceso largo demora a todos |
| SJF | primero el más corto | hay que conocer la duración; los largos pueden esperar indefinidamente |
| Round Robin | turnos de duración fija (*quantum*) | muchos cambios de contexto si el quantum es chico |
| Por prioridades | primero el más prioritario | los de baja prioridad pueden no ejecutar nunca |

> [!NOTE]
> Tres procesos llegan juntos con ráfagas de 24, 3 y 3 ms.
>
> Con **FCFS** en ese orden, esperan 0, 24 y 27 ms: promedio 17 ms. Con **SJF** (3, 3, 24) esperan 0, 3 y 6 ms: promedio 3 ms. El orden cambia mucho el tiempo de espera.

## Concurrencia

Cuando dos hilos modifican el mismo dato sin coordinarse, el resultado depende de quién llegue primero: es una **condición de carrera**. Se evita protegiendo la **sección crítica** con un **mutex**, que deja pasar de a un hilo por vez.

El riesgo opuesto es el **interbloqueo** (*deadlock*): dos hilos se quedan esperando, cada uno, un recurso que tiene el otro. Una prevención habitual es que todos pidan los recursos en el mismo orden.

## Memoria virtual

Cada proceso cree tener para sí un espacio de direcciones grande y continuo. En realidad, el sistema operativo y el hardware (la **MMU**) traducen esas **direcciones virtuales** a direcciones físicas.

La memoria se divide en **páginas** de tamaño fijo (típicamente 4 KiB) y una **tabla de páginas** por proceso indica dónde está cada una. Esto permite:

- **Aislar** los procesos: ninguno puede leer la memoria de otro.
- **Usar más memoria que la RAM**: las páginas poco usadas se guardan en disco (*swap*). Acceder a una de ellas produce un **fallo de página**, y el sistema la trae de vuelta.

> [!WARNING]
> El disco es miles de veces más lento que la RAM. Si la memoria no alcanza y el sistema pasa el tiempo intercambiando páginas (*thrashing*), todo se vuelve extremadamente lento.

## Sistema de archivos

Organiza los datos del disco en **archivos** y **directorios**, y guarda para cada archivo sus metadatos: tamaño, fechas, dueño y **permisos**. En Unix los permisos son de lectura (`r`), escritura (`w`) y ejecución (`x`) para el dueño, el grupo y el resto: `rwxr-xr--` equivale a `754`.
