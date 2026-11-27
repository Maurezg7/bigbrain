+++
title = "Procesos, hilos y sincronización"
summary = "Estados de un proceso, cambio de contexto, algoritmos de planificación, hilos, secciones críticas, semáforos, monitores e interbloqueos."
branch = "Sistemas operativos"
order = 2
level = "Intermedio"
tags = ["procesos", "hilos", "planificación", "semáforos", "interbloqueo"]
+++

## Proceso

Un **proceso** es un programa en ejecución. El sistema operativo guarda, para cada uno, un **bloque de control de proceso** (PCB):

| Contenido del PCB | |
|---|---|
| Identificador (PID) y proceso padre | |
| Estado | listo, en ejecución, bloqueado... |
| Contexto de CPU | registros, contador de programa |
| Memoria | tablas de páginas |
| Recursos | archivos abiertos, sockets |
| Contabilidad | tiempo de CPU usado, prioridad |

Cada proceso tiene su **espacio de direcciones** propio: no puede leer la memoria de otro.

```text
direcciones altas
┌────────────────┐
│ pila           │ ↓ variables locales, llamadas
│                │
│ montón (heap)  │ ↑ memoria dinámica
├────────────────┤
│ datos          │   variables globales
├────────────────┤
│ código         │   instrucciones (solo lectura)
└────────────────┘
direcciones bajas
```

## Estados

```text
            admitido             despachado
  nuevo ─────────────▶ listo ───────────────▶ en ejecución ─────▶ terminado
                         ▲  ◀─────────────────────┘    │
                         │      se acabó su turno       │ pide E/S
                         │                              ▼
                         └───────────────────────── bloqueado
                              terminó la E/S
```

| Estado | Significado |
|---|---|
| Listo | puede ejecutarse; espera una CPU |
| En ejecución | está usando una CPU |
| Bloqueado | espera un evento (entrada y salida, un cerrojo, un temporizador) |

## Cambio de contexto

Para pasar la CPU de un proceso a otro, el sistema operativo:

1. Guarda los registros del proceso saliente en su PCB.
2. Elige el próximo proceso.
3. Cambia el espacio de direcciones.
4. Restaura los registros del entrante.

Cuesta microsegundos, más el efecto indirecto de las cachés y el TLB «fríos». Demasiados cambios degradan el rendimiento.

## Crear procesos

En Unix, `fork` duplica el proceso y `exec` reemplaza su programa:

```c
pid_t pid = fork();

if (pid == 0) {
    execlp("ls", "ls", "-l", NULL);     // hijo: ejecuta otro programa
} else {
    wait(NULL);                         // padre: espera al hijo
}
```

`fork` devuelve 0 en el hijo y el PID del hijo en el padre. La memoria no se copia de inmediato: se comparte y se duplica solo al modificarla (*copy-on-write*).

## Planificación

El **planificador** decide qué proceso listo recibe la CPU.

| Criterio | Se busca |
|---|---|
| Uso de CPU | máximo |
| Rendimiento (procesos terminados por unidad de tiempo) | máximo |
| Tiempo de retorno (desde que llega hasta que termina) | mínimo |
| Tiempo de espera (en la cola de listos) | mínimo |
| Tiempo de respuesta (hasta la primera respuesta) | mínimo |

Una planificación es **apropiativa** si el sistema puede quitarle la CPU a un proceso; **no apropiativa** si este la conserva hasta bloquearse o terminar.

### Algoritmos

| Algoritmo | Regla | Observación |
|---|---|---|
| FCFS | por orden de llegada | simple; un proceso largo demora a todos (efecto convoy) |
| SJF | primero el más corto | mínimo tiempo de espera medio; exige conocer la duración |
| SRTF | SJF apropiativo | |
| *Round robin* | turnos de duración fija (*quantum*) | buen tiempo de respuesta |
| Por prioridades | primero el de mayor prioridad | riesgo de inanición; se corrige con envejecimiento |
| Colas multinivel con realimentación | varias colas; los procesos cambian de cola según su conducta | lo que usan los sistemas reales |

> [!NOTE]
> **Ejemplo.** Tres procesos llegan juntos, con ráfagas de 24, 3 y 3 ms.
>
> | Orden | Esperas | Espera media |
> |---|---|---|
> | FCFS (24, 3, 3) | 0, 24, 27 | 17 ms |
> | SJF (3, 3, 24) | 0, 3, 6 | 3 ms |

En *round robin*, un quantum muy largo equivale a FCFS; uno muy corto desperdicia tiempo en cambios de contexto. Valores típicos: 1 a 100 ms.

Los procesos **interactivos** (limitados por entrada y salida) usan ráfagas cortas de CPU; los de **cálculo**, largas. Los planificadores favorecen a los primeros para que el sistema responda.

## Hilos

Un **hilo** es una secuencia de ejecución dentro de un proceso.

| | Comparten los hilos de un proceso | Propio de cada hilo |
|---|---|---|
| | código, datos globales, montón | pila |
| | archivos abiertos | registros y contador de programa |

| | Proceso | Hilo |
|---|---|---|
| Crear y cambiar de contexto | costoso | barato |
| Comunicación | mediante el sistema operativo | por memoria compartida |
| Aislamiento | total | ninguno: un error afecta a todos |

Los hilos permiten aprovechar varios núcleos y mantener una aplicación atenta mientras hace otro trabajo. A cambio, introducen los problemas de **concurrencia**.

## Condición de carrera

Dos hilos que ejecutan `contador++` sobre una variable compartida:

```text
Hilo A                    Hilo B
lee contador (5)
                          lee contador (5)
suma 1 → 6
                          suma 1 → 6
escribe 6
                          escribe 6          ← se perdió un incremento
```

El resultado depende del orden exacto de ejecución: es una **condición de carrera**.

Una **sección crítica** es el fragmento que accede a datos compartidos. La solución debe garantizar:

| Requisito | Significado |
|---|---|
| Exclusión mutua | a lo sumo un hilo dentro de la sección crítica |
| Progreso | si nadie está dentro, alguno de los que quieren entrar lo logra |
| Espera acotada | ningún hilo espera para siempre |

## Mecanismos de sincronización

### Mutex

Un cerrojo con dos operaciones: tomar y liberar.

```c
pthread_mutex_lock(&cerrojo);
contador++;                          // sección crítica
pthread_mutex_unlock(&cerrojo);
```

### Semáforo

Un contador entero con dos operaciones **atómicas**:

| Operación | Efecto |
|---|---|
| `wait` (P) | si el valor es 0, bloquea al hilo; si no, lo decrementa |
| `signal` (V) | incrementa el valor y despierta a un hilo en espera |

Un semáforo iniciado en 1 funciona como mutex; iniciado en $n$, permite $n$ accesos simultáneos.

### Variable de condición

Permite a un hilo **esperar** hasta que se cumpla una condición, liberando el cerrojo mientras tanto.

```c
pthread_mutex_lock(&m);
while (cola_vacia()) {
    pthread_cond_wait(&hay_datos, &m);     // libera m y duerme
}
dato = sacar();
pthread_mutex_unlock(&m);
```

> [!WARNING]
> La condición se comprueba siempre con `while`, no con `if`: el hilo puede despertarse sin que la condición se cumpla.

Un **monitor** reúne datos, cerrojo y variables de condición en una sola construcción (los bloques `synchronized` de Java).

### Problemas clásicos

| Problema | Modela |
|---|---|
| Productor y consumidor | una cola de tamaño limitado entre hilos |
| Lectores y escritores | muchos lectores simultáneos, o un solo escritor |
| Filósofos comensales | varios procesos que necesitan dos recursos compartidos |

## Interbloqueo

Un **interbloqueo** (*deadlock*) ocurre cuando un conjunto de procesos espera, cada uno, un recurso que tiene otro del conjunto.

```text
Hilo A: tiene R1, espera R2
Hilo B: tiene R2, espera R1
```

Deben darse a la vez **cuatro condiciones** (de Coffman):

| Condición | Significado |
|---|---|
| Exclusión mutua | el recurso no se puede compartir |
| Retención y espera | se conserva un recurso mientras se pide otro |
| No apropiación | no se le puede quitar el recurso a quien lo tiene |
| Espera circular | existe un ciclo de procesos que se esperan |

> [!IMPORTANT]
> Basta con romper **una** condición para evitar el interbloqueo. La más práctica: imponer un **orden global** para tomar los recursos, que elimina la espera circular.

| Estrategia | Idea |
|---|---|
| Prevención | impedir alguna de las cuatro condiciones |
| Evitación | conceder un recurso solo si el estado queda seguro (algoritmo del banquero) |
| Detección y recuperación | buscar ciclos en el grafo de espera y abortar un proceso |
| Ignorarlo | lo que hacen la mayoría de los sistemas de uso general |

Problemas relacionados: **inanición** (un proceso nunca recibe el recurso) y **bloqueo activo** (*livelock*: los procesos cambian de estado sin avanzar).

## Comunicación entre procesos

| Mecanismo | Característica |
|---|---|
| Tubería (*pipe*) | flujo de bytes en un sentido: `ls \| grep txt` |
| Cola de mensajes | mensajes delimitados |
| Memoria compartida | la más rápida; exige sincronización |
| Señales | notificaciones asíncronas (`SIGINT`, `SIGTERM`, `SIGKILL`) |
| *Sockets* | también entre máquinas distintas |

> [!TIP]
> Reglas prácticas de concurrencia: compartir lo menos posible, preferir datos inmutables o paso de mensajes, mantener las secciones críticas cortas y tomar los cerrojos siempre en el mismo orden.
