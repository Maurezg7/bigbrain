+++
title = "SQL avanzado"
summary = "Subconsultas, expresiones de tabla comunes y recursivas, funciones de ventana, tipos de JOIN, vistas, valores nulos y consultas de agregación complejas."
branch = "Bases de datos"
order = 3
level = "Intermedio"
tags = ["SQL", "subconsultas", "CTE", "funciones de ventana", "JOIN", "vistas"]
+++

## Orden lógico de una consulta

SQL se escribe en un orden y se evalúa en otro:

| Orden de evaluación | Cláusula | Qué hace |
|---|---|---|
| 1 | `FROM`, `JOIN` | arma el conjunto de filas |
| 2 | `WHERE` | filtra filas |
| 3 | `GROUP BY` | forma grupos |
| 4 | `HAVING` | filtra grupos |
| 5 | `SELECT` | calcula las columnas; funciones de ventana |
| 6 | `DISTINCT` | quita duplicados |
| 7 | `ORDER BY` | ordena |
| 8 | `LIMIT`, `OFFSET` | recorta |

> [!IMPORTANT]
> Por eso un alias definido en `SELECT` no puede usarse en `WHERE`, y un agregado como `COUNT(*)` se filtra en `HAVING`, no en `WHERE`.

Los ejemplos usan estas tablas:

```sql
estudiante(legajo, nombre, carrera)
materia(id, nombre, departamento)
inscripcion(legajo, materia_id, anio, nota)
```

## Tipos de JOIN

| Tipo | Devuelve |
|---|---|
| `INNER JOIN` | solo las filas con coincidencia en ambas tablas |
| `LEFT JOIN` | todas las de la izquierda; `NULL` donde no hay coincidencia |
| `RIGHT JOIN` | todas las de la derecha |
| `FULL JOIN` | todas las de ambas |
| `CROSS JOIN` | todas las combinaciones |

```sql
-- Estudiantes sin ninguna inscripción
SELECT e.nombre
FROM estudiante e
LEFT JOIN inscripcion i ON i.legajo = e.legajo
WHERE i.legajo IS NULL;
```

> [!WARNING]
> En un `LEFT JOIN`, una condición sobre la tabla derecha puesta en `WHERE` descarta las filas sin coincidencia y lo convierte en `INNER JOIN`. Esa condición va en el `ON`.

Una tabla puede combinarse **consigo misma** usando dos alias (por ejemplo, empleados y sus jefes).

## Subconsultas

Una consulta dentro de otra.

```sql
-- Con IN: la subconsulta devuelve una lista de valores
SELECT nombre
FROM estudiante
WHERE legajo IN (SELECT legajo FROM inscripcion WHERE nota = 10);

-- Escalar: la subconsulta devuelve un solo valor (el promedio general)
SELECT legajo, nota
FROM inscripcion
WHERE nota > (SELECT AVG(nota) FROM inscripcion);
```

Una subconsulta **correlacionada** usa valores de la consulta exterior y se evalúa para cada fila:

```sql
-- Notas mayores que el promedio de su propia materia
SELECT i.legajo, i.materia_id, i.nota
FROM inscripcion i
WHERE i.nota > (
    SELECT AVG(nota) FROM inscripcion WHERE materia_id = i.materia_id
);
```

| Operador | Verdadero si |
|---|---|
| `IN (subconsulta)` | el valor está en el resultado |
| `EXISTS (subconsulta)` | la subconsulta devuelve alguna fila |
| `NOT EXISTS` | no devuelve ninguna |
| `> ALL`, `> ANY` | la comparación vale para todos; para alguno |

```sql
-- Estudiantes que aprobaron al menos una materia
SELECT e.nombre
FROM estudiante e
WHERE EXISTS (
    SELECT 1 FROM inscripcion i WHERE i.legajo = e.legajo AND i.nota >= 6
);
```

## Expresiones de tabla comunes (CTE)

`WITH` da nombre a una consulta para usarla después. Hace legibles las consultas largas.

```sql
WITH promedios AS (
    SELECT legajo, AVG(nota) AS promedio, COUNT(*) AS materias
    FROM inscripcion
    GROUP BY legajo
)
SELECT e.nombre, p.promedio, p.materias
FROM promedios p
JOIN estudiante e ON e.legajo = p.legajo
WHERE p.materias >= 5
ORDER BY p.promedio DESC;
```

### CTE recursivas

Recorren estructuras jerárquicas: organigramas, categorías, grafos.

```sql
-- empleado(id, nombre, jefe_id)
WITH RECURSIVE subordinados AS (
    SELECT id, nombre, 0 AS nivel
    FROM empleado
    WHERE id = 1                              -- caso base

    UNION ALL

    SELECT e.id, e.nombre, s.nivel + 1        -- paso recursivo
    FROM empleado e
    JOIN subordinados s ON e.jefe_id = s.id
)
SELECT * FROM subordinados ORDER BY nivel;
```

## Agregación

| Función | Calcula |
|---|---|
| `COUNT(*)` | cantidad de filas |
| `COUNT(columna)` | cantidad de valores no nulos |
| `COUNT(DISTINCT columna)` | cantidad de valores distintos |
| `SUM`, `AVG`, `MIN`, `MAX` | suma, promedio, extremos |
| `STRING_AGG(texto, ', ')` | concatena |

```sql
SELECT m.departamento,
       COUNT(*) AS inscripciones,
       ROUND(AVG(i.nota), 2) AS promedio,
       COUNT(*) FILTER (WHERE i.nota >= 6) AS aprobados
FROM inscripcion i
JOIN materia m ON m.id = i.materia_id
WHERE i.anio = 2025
GROUP BY m.departamento
HAVING COUNT(*) >= 30
ORDER BY promedio DESC;
```

Toda columna de `SELECT` que no esté dentro de un agregado debe figurar en `GROUP BY`.

## Funciones de ventana

Calculan un valor para cada fila mirando un **conjunto de filas relacionadas**, sin colapsarlas como hace `GROUP BY`.

```sql
función() OVER (PARTITION BY ... ORDER BY ...)
```

| Función | Devuelve |
|---|---|
| `ROW_NUMBER()` | 1, 2, 3, ... |
| `RANK()` | posición, con saltos en los empates (1, 1, 3) |
| `DENSE_RANK()` | posición, sin saltos (1, 1, 2) |
| `LAG(col)`, `LEAD(col)` | el valor de la fila anterior; de la siguiente |
| `SUM(col) OVER (...)` | suma sobre la ventana |
| `AVG(col) OVER (...)` | promedio sobre la ventana |
| `FIRST_VALUE`, `LAST_VALUE` | primero y último de la ventana |
| `NTILE(n)` | reparte en $n$ grupos |

```sql
-- Posición de cada estudiante dentro de su materia
SELECT materia_id, legajo, nota,
       RANK() OVER (PARTITION BY materia_id ORDER BY nota DESC) AS puesto
FROM inscripcion;
```

```sql
-- Los 3 mejores de cada materia
WITH ordenado AS (
    SELECT materia_id, legajo, nota,
           ROW_NUMBER() OVER (PARTITION BY materia_id ORDER BY nota DESC) AS n
    FROM inscripcion
)
SELECT * FROM ordenado WHERE n <= 3;
```

```sql
-- Total acumulado y variación respecto del mes anterior
SELECT mes, ventas,
       SUM(ventas) OVER (ORDER BY mes) AS acumulado,
       ventas - LAG(ventas) OVER (ORDER BY mes) AS variacion
FROM ventas_mensuales;
```

> [!TIP]
> «Los $n$ primeros de cada grupo», «acumulados» y «comparar con la fila anterior» se resuelven con funciones de ventana.

## Valores nulos

`NULL` significa «desconocido». La lógica pasa a tener **tres valores**: verdadero, falso y desconocido.

| Expresión | Resultado |
|---|---|
| `NULL = NULL` | desconocido (¡no verdadero!) |
| `5 + NULL` | `NULL` |
| `NULL AND FALSE` | falso |
| `NULL OR TRUE` | verdadero |
| `x IS NULL` | verdadero o falso |

`WHERE` solo conserva las filas donde la condición es **verdadera**.

| Función | Devuelve |
|---|---|
| `COALESCE(a, b, c)` | el primer valor no nulo |
| `NULLIF(a, b)` | `NULL` si son iguales; si no, `a` |

> [!WARNING]
> `NOT IN` con una subconsulta que devuelve algún `NULL` no devuelve **ninguna** fila. Usar `NOT EXISTS`.

Los agregados ignoran los nulos: `AVG(nota)` promedia solo las notas cargadas.

## Expresiones condicionales

```sql
SELECT legajo, nota,
       CASE
           WHEN nota >= 8 THEN 'promocionado'
           WHEN nota >= 6 THEN 'aprobado'
           ELSE 'desaprobado'
       END AS condicion
FROM inscripcion;
```

## Operaciones de conjuntos

| Operador | Resultado |
|---|---|
| `UNION` | filas de ambas consultas, sin duplicados |
| `UNION ALL` | con duplicados (más rápido) |
| `INTERSECT` | las comunes |
| `EXCEPT` | las de la primera que no están en la segunda |

Las consultas deben tener la misma cantidad de columnas, de tipos compatibles.

## Vistas

Una **vista** es una consulta guardada con nombre; se usa como una tabla.

```sql
CREATE VIEW promedio_por_estudiante AS
SELECT legajo, AVG(nota) AS promedio
FROM inscripcion
GROUP BY legajo;
```

Una **vista materializada** guarda además el resultado: es rápida de leer, pero hay que refrescarla.

## Modificar con consultas

```sql
INSERT INTO egresado (legajo, nombre)
SELECT legajo, nombre FROM estudiante WHERE carrera = 'Física';

UPDATE inscripcion
SET nota = nota + 1
WHERE materia_id = 12 AND nota < 10;

-- Insertar, o actualizar si ya existe (PostgreSQL)
INSERT INTO inscripcion (legajo, materia_id, anio, nota)
VALUES (101, 12, 2025, 8)
ON CONFLICT (legajo, materia_id, anio) DO UPDATE SET nota = EXCLUDED.nota;
```

> [!WARNING]
> Un `UPDATE` o `DELETE` **sin `WHERE`** afecta a toda la tabla. Probar primero la condición con un `SELECT`, y trabajar dentro de una transacción.

## Seguridad: inyección SQL

Nunca armar una consulta concatenando texto del usuario:

```python
# Vulnerable
cursor.execute(f"SELECT * FROM usuario WHERE email = '{email}'")

# Correcto: consulta parametrizada
cursor.execute("SELECT * FROM usuario WHERE email = %s", (email,))
```

Con la entrada `' OR '1'='1`, la primera versión devuelve todos los usuarios. Las consultas **parametrizadas** separan el código de los datos.

> [!TIP]
> Escribir las consultas complejas de a pasos: primero el `FROM` con sus `JOIN`, comprobar las filas, y recién después agregar filtros, grupos y ventanas. Las CTE permiten probar cada etapa por separado.
