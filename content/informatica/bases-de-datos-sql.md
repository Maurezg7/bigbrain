+++
title = "Bases de datos relacionales y SQL"
summary = "Tablas, claves y relaciones; consultas con SELECT, JOIN y GROUP BY; normalización y transacciones."
branch = "Bases de datos"
order = 1
level = "Intermedio"
tags = ["SQL", "tablas", "JOIN", "normalización", "transacciones"]
+++

## El modelo relacional

Una **base de datos relacional** organiza la información en **tablas**. Cada tabla tiene **columnas** (atributos, con un tipo de dato) y **filas** (registros).

- La **clave primaria** (PK) identifica de forma única cada fila.
- Una **clave foránea** (FK) es una columna que referencia la clave primaria de otra tabla. Así se relacionan los datos sin duplicarlos.

![Tablas alumnos, inscripciones y materias relacionadas por claves foráneas](/media/informatica/tablas.svg)

*Un alumno puede cursar muchas materias y una materia tener muchos alumnos: la relación muchos a muchos se resuelve con la tabla intermedia `inscripciones`.*

## Definir tablas

```sql
CREATE TABLE alumnos (
    id      INTEGER PRIMARY KEY,
    nombre  TEXT NOT NULL,
    carrera TEXT NOT NULL
);

CREATE TABLE materias (
    id     INTEGER PRIMARY KEY,
    nombre TEXT NOT NULL
);

CREATE TABLE inscripciones (
    alumno_id  INTEGER REFERENCES alumnos(id),
    materia_id INTEGER REFERENCES materias(id),
    nota       INTEGER,
    PRIMARY KEY (alumno_id, materia_id)
);
```

## Consultar

`SELECT` elige columnas, `WHERE` filtra filas y `ORDER BY` ordena:

```sql
SELECT nombre, carrera
FROM alumnos
WHERE carrera = 'Ingeniería Electromecánica'
ORDER BY nombre;
```

## Combinar tablas: JOIN

Un `JOIN` une filas de dos tablas cuando se cumple una condición, normalmente la igualdad entre una clave foránea y su clave primaria:

```sql
SELECT a.nombre, m.nombre AS materia, i.nota
FROM inscripciones AS i
JOIN alumnos  AS a ON a.id = i.alumno_id
JOIN materias AS m ON m.id = i.materia_id
WHERE i.nota >= 7;
```

| Tipo | Devuelve |
|---|---|
| `INNER JOIN` | solo las filas con coincidencia en ambas tablas |
| `LEFT JOIN` | todas las de la tabla izquierda; `NULL` donde no hay coincidencia |
| `FULL JOIN` | todas las de ambas tablas |

## Agrupar y resumir

Las funciones de agregación (`COUNT`, `SUM`, `AVG`, `MIN`, `MAX`) resumen varias filas en una. `GROUP BY` forma los grupos y `HAVING` filtra grupos ya calculados.

> [!NOTE]
> Promedio de cada materia, solo de las que tienen al menos 5 inscriptos, de mayor a menor:
>
> ```sql
> SELECT m.nombre, COUNT(*) AS inscriptos, AVG(i.nota) AS promedio
> FROM inscripciones AS i
> JOIN materias AS m ON m.id = i.materia_id
> GROUP BY m.nombre
> HAVING COUNT(*) >= 5
> ORDER BY promedio DESC;
> ```
>
> `WHERE` filtra filas antes de agrupar; `HAVING` filtra los grupos después.

## Modificar datos

```sql
INSERT INTO alumnos (id, nombre, carrera) VALUES (1, 'Lucía Paz', 'Física');
UPDATE inscripciones SET nota = 9 WHERE alumno_id = 1 AND materia_id = 3;
DELETE FROM inscripciones WHERE nota IS NULL;
```

> [!WARNING]
> Un `UPDATE` o `DELETE` sin `WHERE` afecta a **todas** las filas de la tabla. Probá primero la condición con un `SELECT`.

## Normalización

Normalizar es organizar las tablas para **no repetir información**, porque los datos duplicados terminan contradiciéndose.

- **1FN**: cada celda contiene un único valor (no listas).
- **2FN**: cada columna depende de toda la clave primaria, no de una parte.
- **3FN**: ninguna columna depende de otra que no sea clave.

En la práctica: cada dato se guarda en un solo lugar, y lo demás lo referencia.

## Transacciones

Una **transacción** agrupa varias operaciones que deben ocurrir todas o ninguna, como una transferencia bancaria. Garantiza las propiedades **ACID**:

| Propiedad | Garantiza que… |
|---|---|
| Atomicidad | la transacción se completa entera o no se aplica nada |
| Consistencia | la base pasa de un estado válido a otro válido |
| Aislamiento (*isolation*) | las transacciones simultáneas no interfieren |
| Durabilidad | lo confirmado sobrevive a un corte de energía |

```sql
BEGIN;
UPDATE cuentas SET saldo = saldo - 1000 WHERE id = 1;
UPDATE cuentas SET saldo = saldo + 1000 WHERE id = 2;
COMMIT;   -- o ROLLBACK para deshacer todo
```

> [!TIP]
> Los **índices** aceleran las búsquedas de $O(n)$ a $O(\log n)$, a cambio de espacio y de escrituras algo más lentas. Conviene crearlos en las columnas que se usan para filtrar y unir.
