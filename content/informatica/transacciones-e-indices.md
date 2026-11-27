+++
title = "Transacciones, concurrencia e índices"
summary = "Propiedades ACID, niveles de aislamiento, bloqueos y MVCC, índices de árbol B y hash, planes de ejecución y optimización de consultas."
branch = "Bases de datos"
order = 4
level = "Avanzado"
tags = ["transacciones", "ACID", "aislamiento", "índices", "árbol B", "EXPLAIN"]
+++

## Transacciones

Una **transacción** es un conjunto de operaciones que se ejecuta como una unidad: o se aplican todas, o ninguna.

```sql
BEGIN;

UPDATE cuenta SET saldo = saldo - 500 WHERE id = 1;
UPDATE cuenta SET saldo = saldo + 500 WHERE id = 2;

COMMIT;          -- confirma; ROLLBACK deshace todo
```

Sin transacción, una falla entre las dos sentencias haría desaparecer dinero.

## ACID

| Propiedad | Garantiza | Mecanismo |
|---|---|---|
| **Atomicidad** | todo o nada | registro de deshacer |
| **Consistencia** | se respetan las restricciones | claves, `CHECK`, disparadores |
| **Aislamiento** (*isolation*) | las transacciones concurrentes no se interfieren | bloqueos, MVCC |
| **Durabilidad** | lo confirmado sobrevive a una caída | registro de escritura anticipada |

### Registro de escritura anticipada (WAL)

Antes de modificar los datos, el motor escribe el cambio en un **registro** secuencial y lo fuerza al disco. Tras una caída, relee el registro: rehace lo confirmado y deshace lo incompleto.

## Problemas de concurrencia

| Anomalía | Qué ocurre |
|---|---|
| Lectura sucia | se lee un dato que otra transacción aún no confirmó (y puede deshacer) |
| Lectura no repetible | la misma fila, leída dos veces, da valores distintos |
| Lectura fantasma | la misma consulta, repetida, devuelve filas nuevas |
| Actualización perdida | dos transacciones leen el mismo valor y la segunda escritura pisa a la primera |
| Sesgo de escritura | cada una lee datos que la otra modifica, y juntas rompen una regla |

## Niveles de aislamiento

| Nivel | Lectura sucia | No repetible | Fantasma |
|---|---|---|---|
| *Read uncommitted* | posible | posible | posible |
| *Read committed* | no | posible | posible |
| *Repeatable read* | no | no | posible |
| *Serializable* | no | no | no |

- **Serializable**: el resultado equivale a ejecutar las transacciones una tras otra. Es el más seguro y el más costoso.
- *Read committed* es el predeterminado en PostgreSQL y Oracle; *repeatable read*, en MySQL (InnoDB).

```sql
BEGIN ISOLATION LEVEL SERIALIZABLE;
```

> [!IMPORTANT]
> Más aislamiento significa menos anomalías, pero también menos concurrencia y más transacciones abortadas que hay que **reintentar**.

## Control de concurrencia

### Bloqueos

| Bloqueo | Permite a otros |
|---|---|
| Compartido (lectura) | leer, no escribir |
| Exclusivo (escritura) | nada |

En el **bloqueo en dos fases**, una transacción primero adquiere bloqueos y, desde que libera el primero, ya no adquiere más. Garantiza serializabilidad, pero puede producir **interbloqueos**: el motor los detecta y aborta una de las transacciones.

```sql
SELECT saldo FROM cuenta WHERE id = 1 FOR UPDATE;      -- bloquea la fila
```

### MVCC

Con **control de concurrencia multiversión**, cada escritura crea una **versión nueva** de la fila. Cada transacción ve una instantánea coherente.

> [!IMPORTANT]
> Con MVCC, **los lectores no bloquean a los escritores ni los escritores a los lectores**. Lo usan PostgreSQL, MySQL (InnoDB) y Oracle.

Las versiones viejas se eliminan después (en PostgreSQL, con `VACUUM`).

### Optimista o pesimista

| Enfoque | Idea | Conviene cuando |
|---|---|---|
| Pesimista | bloquear antes de modificar | hay muchos conflictos |
| Optimista | no bloquear; verificar al confirmar que nadie cambió el dato | los conflictos son raros |

El control optimista suele implementarse con una columna de **versión**:

```sql
UPDATE producto
SET precio = 120, version = version + 1
WHERE id = 7 AND version = 3;          -- si afecta 0 filas, alguien lo cambió antes
```

## Índices

Sin índice, buscar una fila exige leer toda la tabla (**recorrido secuencial**, $O(n)$). Un **índice** es una estructura auxiliar que permite encontrarla rápido.

```sql
CREATE INDEX idx_inscripcion_legajo ON inscripcion (legajo);
```

### Árbol B+

Es el índice por defecto.

```text
                    [ 30 | 60 ]
                   /     |     \
          [10 | 20]   [40 | 50]   [70 | 80]
           hojas enlazadas entre sí ──▶ ──▶
```

| Característica | |
|---|---|
| Nodos | del tamaño de una página de disco, con cientos de claves |
| Altura | 3 o 4 niveles alcanzan para millones de filas |
| Búsqueda, inserción | $O(\log n)$ |
| Hojas enlazadas | recorridos por **rango** y en orden |

Sirve para igualdad (`=`), rangos (`<`, `BETWEEN`), prefijos (`LIKE 'abc%'`) y `ORDER BY`.

### Otros índices

| Índice | Sirve para |
|---|---|
| Hash | solo igualdad |
| GIN | búsqueda de texto completo, arreglos, JSON |
| GiST, R-tree | datos geográficos, rangos |
| BRIN | tablas enormes ordenadas naturalmente (por fecha) |

### Índices compuestos

```sql
CREATE INDEX idx_insc ON inscripcion (materia_id, anio);
```

> [!IMPORTANT]
> Un índice sobre `(a, b)` sirve para filtrar por `a`, o por `a` y `b`, pero **no** por `b` solo. El orden de las columnas importa: como una guía telefónica ordenada por apellido y luego por nombre.

| Tipo | Idea |
|---|---|
| Único | además garantiza que no haya repetidos |
| Parcial | indexa solo las filas que cumplen una condición |
| De expresión | indexa el resultado de una función: `lower(email)` |
| De cobertura | contiene todas las columnas que pide la consulta; no hace falta leer la tabla |

### El costo

| Ventaja | Costo |
|---|---|
| lecturas mucho más rápidas | cada `INSERT`, `UPDATE` y `DELETE` debe actualizar los índices |
| | ocupan espacio en disco y en memoria |

Conviene indexar las columnas de claves foráneas, de filtros frecuentes y de ordenamiento. No conviene indexar todo.

### Cuándo no se usa un índice

| Situación | Motivo |
|---|---|
| `WHERE lower(email) = '...'` | la función oculta la columna (salvo índice de expresión) |
| `LIKE '%texto'` | el comodín inicial impide usar el orden |
| La condición selecciona gran parte de la tabla | es más barato leerla entera |
| Tabla muy pequeña | idem |
| Tipos distintos en la comparación | la conversión implícita lo impide |

## Planes de ejecución

El **optimizador** elige cómo ejecutar cada consulta, estimando costos con estadísticas de las tablas. `EXPLAIN` muestra su decisión:

```sql
EXPLAIN ANALYZE
SELECT * FROM inscripcion WHERE legajo = 101;
```

```text
Index Scan using idx_inscripcion_legajo on inscripcion
  Index Cond: (legajo = 101)
  actual time=0.031..0.045 rows=12
```

| Operación | Significado |
|---|---|
| *Seq Scan* | lee toda la tabla |
| *Index Scan* | usa un índice y luego lee la tabla |
| *Index Only Scan* | le alcanza con el índice |
| *Nested Loop* | para cada fila de una tabla, busca en la otra |
| *Hash Join* | arma una tabla hash con una y la recorre con la otra |
| *Merge Join* | combina dos entradas ordenadas |
| *Sort*, *Aggregate* | ordena; agrupa |

## Problemas frecuentes

| Problema | Solución |
|---|---|
| **N + 1 consultas**: una consulta por cada fila de un listado | un `JOIN`, o una sola consulta con `IN` |
| `SELECT *` | pedir solo las columnas necesarias |
| Paginación con `OFFSET` grande | paginar por clave: `WHERE id > ultimo ORDER BY id LIMIT 20` |
| Falta un índice en una clave foránea | crearlo |
| Transacciones largas | mantenerlas cortas; no esperar al usuario dentro de una |
| Abrir una conexión por petición | usar un **grupo de conexiones** (*pool*) |

> [!NOTE]
> **El problema N + 1.** Listar 100 pedidos y, para cada uno, consultar su cliente genera 101 consultas. Una sola consulta con `JOIN` hace lo mismo en una ida y vuelta.

## Escalar

| Técnica | Idea | Costo |
|---|---|---|
| Escalado vertical | un servidor más potente | tiene un techo |
| Réplicas de lectura | copias que atienden consultas | pueden estar levemente desactualizadas |
| Particionado | dividir una tabla grande por rangos (por fecha) | |
| Fragmentación (*sharding*) | repartir los datos en varios servidores | complica las consultas y las transacciones |
| Caché | guardar resultados en memoria (Redis) | hay que invalidarla |

## Copias de seguridad

| Tipo | |
|---|---|
| Volcado lógico (`pg_dump`) | sentencias SQL; portable; lento con bases grandes |
| Copia física | los archivos de datos |
| Recuperación a un punto en el tiempo | copia base más el registro WAL |

> [!WARNING]
> Una copia de seguridad que nunca se probó restaurar no es una copia de seguridad. Ensayar la restauración periódicamente.

> [!TIP]
> Ante una consulta lenta: mirar el plan con `EXPLAIN ANALYZE`, buscar recorridos secuenciales sobre tablas grandes y estimaciones de filas muy alejadas de la realidad, y recién entonces decidir si falta un índice.
