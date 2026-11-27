+++
title = "Bases de datos NoSQL"
summary = "Modelos clave-valor, documental, de columnas y de grafos; teorema CAP, consistencia eventual, replicación, fragmentación y criterios para elegir."
branch = "Bases de datos"
order = 5
level = "Intermedio"
tags = ["NoSQL", "MongoDB", "Redis", "CAP", "consistencia eventual", "grafos"]
+++

## Por qué existen

Las bases relacionales dominan desde los años setenta. Las **NoSQL** («no solo SQL») surgieron para casos en que el modelo relacional resulta incómodo:

| Necesidad | Dificultad en el modelo relacional |
|---|---|
| Volúmenes enormes repartidos en muchos servidores | las combinaciones y transacciones entre servidores son costosas |
| Datos de estructura variable | el esquema rígido exige migraciones |
| Latencias de microsegundos | el motor hace más trabajo del necesario |
| Datos muy conectados | recorrer relaciones exige muchos `JOIN` |

No reemplazan a las relacionales: resuelven problemas distintos.

## Los cuatro modelos

| Modelo | Estructura | Ejemplos | Uso típico |
|---|---|---|---|
| Clave-valor | un diccionario gigante | Redis, DynamoDB, etcd | caché, sesiones, contadores |
| Documental | documentos JSON anidados | MongoDB, CouchDB, Firestore | catálogos, contenido, perfiles |
| Columnas anchas | filas con columnas variables, agrupadas en familias | Cassandra, HBase, ScyllaDB | series de tiempo, registros masivos |
| Grafos | nodos y aristas con propiedades | Neo4j, Neptune | redes sociales, recomendaciones, fraude |

Otros tipos especializados: bases de **series temporales** (InfluxDB, TimescaleDB), de **búsqueda** (Elasticsearch) y **vectoriales** (para búsqueda por similitud).

## Clave-valor

La operación básica es leer o escribir un valor por su clave.

```text
SET sesion:a81f '{"usuario": 42}' EX 3600      # expira en una hora
GET sesion:a81f
INCR visitas:pagina:17
LPUSH cola:tareas "enviar-correo"
ZADD ranking 1500 "ana"                        # conjunto ordenado
```

**Redis** guarda los datos en memoria y ofrece estructuras: cadenas, listas, conjuntos, conjuntos ordenados, tablas hash.

| Uso | Estructura |
|---|---|
| Caché | cadena con vencimiento |
| Cola de trabajos | lista |
| Tabla de posiciones | conjunto ordenado |
| Límite de peticiones | contador con vencimiento |

### Patrón de caché

```python
def obtener_producto(id):
    clave = f"producto:{id}"
    dato = cache.get(clave)
    if dato is None:                           # fallo de caché
        dato = db.consultar_producto(id)
        cache.set(clave, dato, ex=300)
    return dato
```

Al modificar el producto, hay que **invalidar** la clave. Decidir cuándo hacerlo es la parte difícil.

## Documental

Cada registro es un **documento** autocontenido. Los documentos de una colección no necesitan tener los mismos campos.

```json
{
  "_id": "p-1001",
  "cliente": { "nombre": "Ana Ruiz", "email": "ana@ejemplo.com" },
  "fecha": "2025-03-14",
  "renglones": [
    { "producto": "Teclado", "cantidad": 1, "precio": 45000 },
    { "producto": "Mouse", "cantidad": 2, "precio": 12000 }
  ],
  "total": 69000
}
```

En una base relacional esto ocuparía tres tablas. Aquí el pedido se lee completo en **una** operación.

```javascript
db.pedidos.find({ "cliente.email": "ana@ejemplo.com", total: { $gt: 50000 } })
          .sort({ fecha: -1 })
          .limit(10);

db.pedidos.aggregate([
  { $unwind: "$renglones" },
  { $group: { _id: "$renglones.producto", unidades: { $sum: "$renglones.cantidad" } } },
  { $sort: { unidades: -1 } }
]);
```

### Incrustar o referenciar

| Estrategia | Conviene cuando |
|---|---|
| **Incrustar** el dato dentro del documento | se lee siempre junto con él y le pertenece (los renglones de un pedido) |
| **Referenciar** por identificador | se comparte entre muchos documentos, cambia seguido o crece sin límite |

> [!IMPORTANT]
> En el modelo relacional se diseña a partir de **los datos**; en NoSQL, a partir de **las consultas** que hará la aplicación.

## Columnas anchas

Pensadas para escrituras masivas en muchos nodos. Cada tabla tiene una **clave de partición**, que decide en qué nodo vive la fila, y una **clave de ordenamiento** dentro de la partición.

```sql
CREATE TABLE lectura (
    sensor_id  uuid,
    momento    timestamp,
    valor      double,
    PRIMARY KEY (sensor_id, momento)
) WITH CLUSTERING ORDER BY (momento DESC);

SELECT * FROM lectura WHERE sensor_id = ? AND momento > ? LIMIT 100;
```

Las consultas eficientes son solo las que indican la clave de partición. No hay `JOIN`: se crea una tabla por cada tipo de consulta, duplicando datos.

## Grafos

Los datos son **nodos** y **aristas**. Recorrer relaciones es directo, sin combinaciones.

```cypher
// Amigos de amigos de Ana que ella todavía no sigue
MATCH (ana:Persona {nombre: "Ana"})-[:SIGUE]->()-[:SIGUE]->(sugerido)
WHERE NOT (ana)-[:SIGUE]->(sugerido) AND sugerido <> ana
RETURN sugerido.nombre, count(*) AS en_comun
ORDER BY en_comun DESC
LIMIT 5;
```

Convienen cuando las consultas siguen caminos de longitud variable: redes sociales, rutas, dependencias, detección de fraude.

## Distribución

### Replicación

Mantener **copias** de los datos en varios nodos, para tolerar fallas y repartir lecturas.

| Esquema | Funcionamiento |
|---|---|
| Líder y seguidores | las escrituras van al líder, que las propaga |
| Varios líderes | varios nodos aceptan escrituras; hay que resolver conflictos |
| Sin líder | cualquier nodo acepta; se usa un **cuórum** |

Con $N$ réplicas, si una escritura se confirma en $W$ y una lectura consulta $R$, la lectura ve el último dato cuando

$$R + W > N$$

### Fragmentación

Repartir los datos entre nodos según una clave (*sharding*).

| Método | Ventaja | Inconveniente |
|---|---|---|
| Por rango | consultas por rango eficientes | puede concentrar la carga en un nodo |
| Por hash | reparto uniforme | las consultas por rango tocan todos los nodos |

El **hash consistente** permite agregar o quitar nodos moviendo solo una fracción pequeña de las claves.

## Teorema CAP

En un sistema distribuido, cuando ocurre una **partición de red** (nodos que no pueden comunicarse), hay que elegir:

| Opción | Se mantiene | Se sacrifica |
|---|---|---|
| **CP** | consistencia: todos ven el mismo dato | disponibilidad: algunos nodos rechazan peticiones |
| **AP** | disponibilidad: todos responden | consistencia: pueden devolver datos viejos |

> [!NOTE]
> No se trata de «elegir dos de tres»: las particiones ocurren siempre, así que la decisión real es entre **consistencia** y **disponibilidad** mientras dura la partición.

### Consistencia eventual

Si dejan de llegar escrituras, todas las réplicas **terminan** convergiendo al mismo valor. Mientras tanto, una lectura puede devolver un dato viejo.

| Modelo | Garantía |
|---|---|
| Fuerte (linealizable) | toda lectura ve la última escritura confirmada |
| Lectura de las propias escrituras | cada usuario ve lo que él mismo escribió |
| Eventual | las réplicas convergen con el tiempo |

### ACID y BASE

| ACID | BASE |
|---|---|
| consistencia fuerte | consistencia eventual |
| prioriza la corrección | prioriza la disponibilidad |
| bases relacionales | muchas NoSQL |

La frontera se borró: MongoDB ofrece transacciones ACID entre documentos, y las bases **NewSQL** (CockroachDB, Spanner, TiDB) dan SQL y transacciones en sistemas distribuidos.

## Cómo elegir

| Si se necesita | Opción |
|---|---|
| Datos relacionados, transacciones, consultas variadas | relacional (PostgreSQL) |
| Caché, sesiones, colas | clave-valor (Redis) |
| Documentos de estructura variable que se leen enteros | documental |
| Escrituras masivas con consultas conocidas | columnas anchas |
| Recorrer relaciones | grafos |
| Búsqueda de texto | motor de búsqueda |

Es habitual combinar varias (**persistencia políglota**): PostgreSQL como fuente de verdad, Redis como caché y un motor de búsqueda para el texto.

> [!WARNING]
> «Sin esquema» no significa que no haya esquema: significa que **lo hace cumplir la aplicación**. Los documentos viejos con otra forma siguen ahí, y el código debe saber leerlos.

> [!TIP]
> Ante la duda, empezar con una base relacional. PostgreSQL maneja además JSON (`jsonb`), texto completo y datos geográficos, y cubre la gran mayoría de las aplicaciones. Pasar a NoSQL cuando haya un requisito concreto que lo pida.
