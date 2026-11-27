+++
title = "Memoria y caché"
summary = "Jerarquía de memoria, principio de localidad, organización de la caché, fallos, políticas de escritura y cómo escribir código que la aproveche."
branch = "Arquitectura de computadoras"
order = 3
level = "Intermedio"
tags = ["memoria", "caché", "localidad", "RAM", "rendimiento"]
+++

## El problema

El procesador es mucho más rápido que la memoria principal: un acceso a RAM cuesta lo mismo que cientos de instrucciones. Y no existe una memoria que sea a la vez rápida, grande y barata.

La solución es una **jerarquía**: memorias pequeñas y rápidas cerca del procesador, y grandes y lentas lejos.

| Nivel | Capacidad típica | Tiempo de acceso |
|---|---|---|
| Registros | cientos de bytes | menos de 1 ns |
| Caché L1 | 32 a 64 kB | 1 ns |
| Caché L2 | 256 kB a 2 MB | 4 ns |
| Caché L3 | 8 a 64 MB | 15 a 40 ns |
| Memoria principal (RAM) | 8 a 128 GB | 100 ns |
| SSD | 0,5 a 4 TB | 100 μs |
| Disco rígido | 1 a 20 TB | 10 ms |

> [!NOTE]
> Si un acceso a L1 durara 1 segundo, uno a RAM duraría casi 2 minutos, uno a SSD más de un día y uno a disco rígido varios meses.

## Principio de localidad

La jerarquía funciona porque los programas no acceden a la memoria al azar:

| Localidad | Significado | Ejemplo |
|---|---|---|
| **Temporal** | lo usado hace poco probablemente se vuelva a usar | el contador de un bucle |
| **Espacial** | lo cercano a lo usado probablemente se use pronto | recorrer un arreglo |

La caché guarda lo usado recientemente y sus vecinos.

## Tipos de memoria

| Tipo | Característica | Uso |
|---|---|---|
| SRAM | rápida, 6 transistores por bit, cara | cachés |
| DRAM | 1 transistor y 1 capacitor por bit; hay que refrescarla | memoria principal |
| Flash | no volátil; escrituras limitadas | SSD, memorias USB |
| ROM / EEPROM | no volátil | firmware |

SRAM y DRAM son **volátiles**: pierden su contenido sin alimentación.

## Cómo funciona la caché

La caché se divide en **bloques** (o líneas), normalmente de **64 bytes**. Al pedir una dirección:

- **Acierto** (*hit*): el bloque está en la caché. Respuesta inmediata.
- **Fallo** (*miss*): hay que traer el bloque completo del nivel inferior.

### Dividir la dirección

```text
┌────────────────┬──────────┬──────────────┐
│   etiqueta     │  índice  │ desplazamiento│
└────────────────┴──────────┴──────────────┘
```

| Campo | Indica | Bits |
|---|---|---|
| Desplazamiento | el byte dentro del bloque | $\log_2(\text{tamaño de bloque})$ |
| Índice | el conjunto de la caché | $\log_2(\text{cantidad de conjuntos})$ |
| Etiqueta | qué bloque de memoria ocupa esa posición | el resto |

Cada entrada guarda además un **bit de validez**.

> [!NOTE]
> **Ejemplo.** Caché de correspondencia directa de 32 kB con bloques de 64 bytes y direcciones de 32 bits.
> - Bloques: $32\,768 / 64 = 512$ → índice de 9 bits.
> - Desplazamiento: $\log_2 64 = 6$ bits.
> - Etiqueta: $32 - 9 - 6 = 17$ bits.

### Organización

| Organización | Un bloque puede ir en | Ventaja | Desventaja |
|---|---|---|---|
| Correspondencia directa | una sola posición | simple y rápida | conflictos entre bloques con el mismo índice |
| Asociativa por conjuntos de $n$ vías | cualquiera de las $n$ posiciones de un conjunto | buen equilibrio | $n$ comparaciones |
| Totalmente asociativa | cualquier posición | sin conflictos | muy costosa |

Las cachés reales son asociativas de 4 a 16 vías.

### Reemplazo

Cuando el conjunto está lleno, hay que desalojar un bloque. La política habitual es **LRU** (*least recently used*): se va el que hace más tiempo que no se usa, o una aproximación.

## Tipos de fallos

| Fallo | Causa | Se reduce con |
|---|---|---|
| Obligatorio (en frío) | primer acceso al bloque | bloques más grandes, precarga |
| De capacidad | la caché no alcanza para los datos en uso | una caché más grande |
| De conflicto | varios bloques compiten por el mismo conjunto | más asociatividad |

La **precarga** (*prefetching*) detecta patrones de acceso secuenciales y trae los bloques antes de que se pidan.

## Rendimiento

$$t_{\text{medio}} = t_{\text{acierto}} + \text{tasa de fallos} \times \text{penalización}$$

> [!NOTE]
> **Ejemplo.** Acierto en 1 ns, penalización de 100 ns, tasa de fallos del 3 %:
> $$t = 1 + 0{,}03 \times 100 = 4\ \text{ns}$$
> Un 3 % de fallos cuadruplica el tiempo medio de acceso.

Con varios niveles, la penalización de L1 es el tiempo medio de L2, y así sucesivamente.

## Escrituras

| Política | Cuándo se escribe en memoria | |
|---|---|---|
| *Write-through* | de inmediato, en caché y en memoria | simple, lenta |
| *Write-back* | al desalojar el bloque (marcado con un bit de «sucio») | rápida; es la usual |

Con varios núcleos, cada uno tiene sus cachés L1 y L2, y un protocolo de coherencia (MESI) mantiene las copias consistentes.

## Código amigable con la caché

### Recorrer en el orden de almacenamiento

Una matriz en C o Rust se guarda **por filas**.

```c
// Rápido: recorre la memoria en orden
for (int i = 0; i < N; i++)
    for (int j = 0; j < N; j++)
        suma += m[i][j];

// Lento: salta N posiciones en cada acceso
for (int j = 0; j < N; j++)
    for (int i = 0; i < N; i++)
        suma += m[i][j];
```

Con matrices grandes, la segunda versión puede ser entre 5 y 20 veces más lenta: cada acceso cae en un bloque distinto.

### Estructuras contiguas

| Estructura | Localidad | Motivo |
|---|---|---|
| Arreglo, `Vec` | excelente | elementos contiguos |
| Lista enlazada | mala | cada nodo puede estar en cualquier parte |
| Tabla hash con direccionamiento abierto | buena | un solo arreglo |
| Árbol con punteros | mala | saltos entre nodos |
| Árbol B | buena | muchos datos por nodo |

> [!IMPORTANT]
> Por eso un arreglo suele ganarle a una lista enlazada incluso en operaciones donde la lista es mejor en teoría: la notación $O$ no cuenta los fallos de caché.

### Otras técnicas

- **Datos compactos**: tipos más chicos y estructuras sin relleno hacen que entren más elementos por bloque.
- **Arreglo de estructuras o estructura de arreglos**: si un bucle solo usa un campo, conviene tener cada campo en su propio arreglo.
- **Procesamiento por bloques**: dividir un problema grande en partes que entren en la caché (clave en la multiplicación de matrices).
- **Falsa compartición**: dos hilos que escriben variables distintas en el **mismo bloque** se invalidan la caché mutuamente. Se evita separando esos datos.

## Memoria virtual y TLB

Cada proceso usa direcciones **virtuales**, que la unidad de gestión de memoria (MMU) traduce a direcciones físicas mediante **tablas de páginas** (páginas de 4 kB).

Consultar esas tablas en cada acceso sería lentísimo. El **TLB** es una caché pequeña de traducciones recientes. Un fallo de TLB obliga a recorrer las tablas.

| Evento | Costo aproximado |
|---|---|
| Acierto de TLB | menos de 1 ns |
| Fallo de TLB | 10 a 100 ns |
| Fallo de página (traer del disco) | milisegundos |

## Almacenamiento

| | Disco rígido (HDD) | Estado sólido (SSD) |
|---|---|---|
| Mecanismo | platos magnéticos giratorios | memoria flash |
| Acceso aleatorio | lento (mueve un cabezal) | rápido |
| Acceso secuencial | aceptable | muy rápido |
| Desgaste | mecánico | número limitado de escrituras |
| Costo por GB | menor | mayor |

El sistema operativo usa la RAM libre como **caché de disco** (*page cache*): por eso la segunda lectura de un archivo es mucho más rápida.

> [!TIP]
> Antes de optimizar, medir. En Linux, `perf stat -e cache-misses,cache-references ./programa` muestra los fallos de caché reales.
