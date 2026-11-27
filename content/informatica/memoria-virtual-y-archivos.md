+++
title = "Memoria virtual y sistemas de archivos"
summary = "Paginación, tablas de páginas, fallos de página, algoritmos de reemplazo, hiperpaginación, y organización de archivos, directorios, inodos y journaling."
branch = "Sistemas operativos"
order = 3
level = "Intermedio"
tags = ["memoria virtual", "paginación", "sistema de archivos", "inodo", "swap"]
+++

## Por qué memoria virtual

Cada proceso ve un **espacio de direcciones virtual** propio, grande y contiguo. El sistema operativo y el hardware lo traducen a memoria física.

| Beneficio | |
|---|---|
| Aislamiento | un proceso no puede tocar la memoria de otro |
| Simplicidad | cada programa cree tener toda la memoria |
| Más memoria que la física | lo que no entra en RAM se guarda en disco |
| Compartir | varias copias de un programa comparten el mismo código |

## Paginación

La memoria virtual se divide en **páginas** y la física en **marcos**, del mismo tamaño (normalmente **4 kB**).

Una dirección virtual tiene dos partes:

```text
┌──────────────────────┬────────────────┐
│   número de página   │ desplazamiento │
└──────────────────────┴────────────────┘
          │                    │
   tabla de páginas            │
          ▼                    ▼
┌──────────────────────┬────────────────┐
│   número de marco    │ desplazamiento │
└──────────────────────┴────────────────┘
```

El desplazamiento no cambia; solo se traduce el número de página.

> [!NOTE]
> **Ejemplo.** Con páginas de 4 kB ($2^{12}$ bytes), el desplazamiento ocupa 12 bits. La dirección virtual `0x00003A7F` está en la página `0x3`, desplazamiento `0xA7F`. Si la página 3 está en el marco `0x1B`, la dirección física es `0x0001BA7F`.

### Entrada de la tabla de páginas

| Campo | Significado |
|---|---|
| Número de marco | dónde está la página |
| Presente | si está en RAM |
| Protección | lectura, escritura, ejecución |
| Modificada (sucia) | si se escribió desde que se cargó |
| Referenciada | si se usó recientemente |
| Usuario / núcleo | quién puede accederla |

### Tablas multinivel

Una tabla plana para un espacio de 48 bits sería enorme. Se usa una tabla **en varios niveles** (cuatro en x86-64): solo se crean las partes que el proceso realmente usa.

El **TLB** guarda las traducciones recientes para no recorrer los niveles en cada acceso.

La paginación no produce fragmentación externa; sí una pequeña **fragmentación interna** (la última página de cada región queda parcialmente usada).

## Paginación por demanda

Las páginas se cargan **solo cuando se usan**. Si el proceso accede a una página que no está en RAM, ocurre un **fallo de página**:

1. El procesador genera una excepción.
2. El sistema operativo ubica la página en el disco.
3. Busca un marco libre; si no hay, elige una página víctima y, si está sucia, la guarda.
4. Carga la página, actualiza la tabla y reanuda la instrucción.

Un fallo que requiere disco cuesta milisegundos: millones de veces más que un acceso normal.

El espacio de disco usado para páginas desalojadas es el área de **intercambio** (*swap*).

## Algoritmos de reemplazo

| Algoritmo | Desaloja | Observación |
|---|---|---|
| Óptimo | la página que más tardará en usarse | imposible de implementar; sirve de referencia |
| FIFO | la más antigua | simple; puede empeorar con más marcos (anomalía de Belady) |
| LRU | la menos recientemente usada | muy bueno; costoso de implementar exactamente |
| Reloj (segunda oportunidad) | recorre circularmente; perdona a las páginas con el bit de referencia en 1 | aproxima LRU; el usado en la práctica |

> [!NOTE]
> **Ejemplo.** Con 3 marcos y la secuencia de páginas 7, 0, 1, 2, 0, 3, 0, 4:
>
> | Referencia | 7 | 0 | 1 | 2 | 0 | 3 | 0 | 4 |
> |---|---|---|---|---|---|---|---|---|
> | FIFO | F | F | F | F | — | F | F | F |
> | LRU | F | F | F | F | — | F | — | F |
>
> FIFO produce 7 fallos y LRU, 6: al llegar el 3, FIFO desaloja la página 0 (la más antigua) aunque se acababa de usar.

## Hiperpaginación

Si los procesos necesitan más memoria que la disponible, el sistema pasa el tiempo trayendo y llevando páginas del disco: es la **hiperpaginación** (*thrashing*). El uso de CPU se desploma.

El **conjunto de trabajo** de un proceso son las páginas que usó en un intervalo reciente. Para evitar la hiperpaginación, la suma de los conjuntos de trabajo debe entrar en la RAM.

## Asignación de memoria dinámica

Dentro de cada proceso, `malloc` y `free` administran el montón:

| Estrategia | Elige el hueco |
|---|---|
| Primer ajuste | primero que alcance |
| Mejor ajuste | más pequeño que alcance |
| Sistema de compañeros (*buddy*) | bloques de tamaño potencia de 2 que se dividen y se fusionan |

Aquí sí aparece la **fragmentación externa**: memoria libre repartida en huecos demasiado chicos.

## Archivos

Un **archivo** es una secuencia de bytes con nombre, más sus **metadatos**: tamaño, propietario, permisos, fechas.

| Operación | Llamada (Unix) |
|---|---|
| Abrir | `open` → devuelve un **descriptor** de archivo |
| Leer, escribir | `read`, `write` |
| Mover la posición | `lseek` |
| Cerrar | `close` |
| Metadatos | `stat` |

Los **directorios** forman un árbol. Una ruta es **absoluta** (`/home/ana/tesis.pdf`) o **relativa** al directorio actual (`../datos/a.csv`).

## Inodos

En los sistemas de archivos de Unix, cada archivo tiene un **inodo** con sus metadatos y la ubicación de sus bloques de datos. El **nombre** no está en el inodo: un directorio es una tabla de pares (nombre, número de inodo).

```text
directorio /home/ana            inodo 5231                 bloques de datos
┌──────────────┬───────┐      ┌─────────────────┐
│ tesis.pdf    │ 5231  │ ───▶ │ tamaño, permisos │
│ notas.txt    │ 5240  │      │ punteros ────────┼──▶ [bloque] [bloque] ...
└──────────────┴───────┘      └─────────────────┘
```

| Enlace | Qué es | |
|---|---|---|
| Duro | otro nombre para el mismo inodo | el archivo existe mientras quede algún nombre |
| Simbólico | un archivo que contiene una ruta | puede quedar roto si se borra el destino |

### Ubicación de los bloques

| Método | Idea | Usado en |
|---|---|---|
| Contigua | bloques consecutivos | CD-ROM |
| Enlazada con tabla (FAT) | una tabla indica el bloque siguiente | FAT32, exFAT |
| Indexada | el inodo lista los bloques; con bloques indirectos para archivos grandes | ext2, ext3 |
| Por extensiones | rangos (inicio, longitud) | ext4, NTFS, XFS |

## Consistencia

Una operación como crear un archivo modifica varias estructuras. Si se corta la energía a la mitad, el sistema de archivos queda inconsistente.

| Técnica | Idea |
|---|---|
| *Journaling* | anotar primero la operación en un registro; tras una caída, se rehace o se descarta |
| *Copy-on-write* | nunca sobrescribir: escribir en bloques nuevos y cambiar un puntero al final |

> [!WARNING]
> Las escrituras pasan primero por la caché de disco en RAM. Por eso hay que **expulsar** las unidades extraíbles antes de desconectarlas, y un programa que necesita durabilidad debe llamar a `fsync`.

| Sistema de archivos | Sistema operativo | Rasgos |
|---|---|---|
| ext4 | Linux | journaling, extensiones |
| XFS, Btrfs, ZFS | Linux, BSD | archivos enormes; instantáneas y sumas de verificación (Btrfs, ZFS) |
| NTFS | Windows | journaling, permisos por listas de control |
| APFS | macOS | copy-on-write, instantáneas |
| FAT32, exFAT | memorias extraíbles | simples y compatibles; FAT32 limita los archivos a 4 GB |

## Permisos en Unix

Cada archivo tiene un propietario, un grupo y tres juegos de permisos:

```text
-rwxr-x---   ana   docentes   programa.sh
 └┬┘└┬┘└┬┘
  │  │  └── otros:  ninguno
  │  └───── grupo:  leer y ejecutar
  └──────── dueño:  leer, escribir y ejecutar
```

| Permiso | En un archivo | En un directorio |
|---|---|---|
| r (4) | leer el contenido | listar los nombres |
| w (2) | modificar | crear y borrar entradas |
| x (1) | ejecutar | entrar y acceder a su contenido |

En octal, `rwxr-x---` es **750**.

## Entrada y salida

| Técnica | Idea |
|---|---|
| Sondeo (*polling*) | consultar al dispositivo repetidamente |
| Interrupciones | el dispositivo avisa cuando terminó |
| DMA | el dispositivo transfiere directo a memoria, sin pasar por la CPU |

Los **controladores** (*drivers*) ocultan los detalles de cada dispositivo detrás de una interfaz común. En Unix, casi todo se presenta como un archivo: discos (`/dev/sda`), terminales, y hasta información del núcleo (`/proc`).

## RAID

Combina varios discos para ganar velocidad, tolerancia a fallos o ambas:

| Nivel | Técnica | Tolera | Capacidad útil ($n$ discos) |
|---|---|---|---|
| 0 | reparto en franjas | ningún fallo | $n$ |
| 1 | espejo | $n - 1$ fallos | 1 |
| 5 | franjas con paridad distribuida | 1 fallo | $n - 1$ |
| 6 | doble paridad | 2 fallos | $n - 2$ |
| 10 | espejo y franjas | 1 por espejo | $n/2$ |

> [!IMPORTANT]
> RAID protege contra la **rotura de un disco**, no contra el borrado accidental, un programa malicioso o un incendio. No reemplaza a las copias de seguridad.
