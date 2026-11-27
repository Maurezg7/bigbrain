+++
title = "Modelo relacional y normalización"
summary = "Modelo entidad-relación, claves, paso al modelo relacional, dependencias funcionales, formas normales 1FN a FNBC y álgebra relacional."
branch = "Bases de datos"
order = 2
level = "Intermedio"
tags = ["modelo relacional", "entidad-relación", "normalización", "claves", "álgebra relacional"]
+++

## Diseñar antes de crear tablas

Un mal diseño produce datos duplicados, inconsistencias y consultas difíciles. El diseño sigue tres etapas:

| Etapa | Resultado |
|---|---|
| Conceptual | diagrama entidad-relación |
| Lógico | tablas, claves y restricciones |
| Físico | tipos de datos, índices, particiones |

## Modelo entidad-relación

| Elemento | Qué es | Ejemplo |
|---|---|---|
| **Entidad** | una clase de objetos del dominio | Estudiante, Materia |
| **Atributo** | una propiedad | nombre, legajo |
| **Relación** | una asociación entre entidades | un estudiante *cursa* materias |
| **Cardinalidad** | cuántas instancias participan | 1:1, 1:N, N:M |

```text
┌────────────┐  N        M  ┌──────────┐  N        1  ┌──────────────┐
│ Estudiante │──── cursa ───│ Materia  │── pertenece ─│ Departamento │
└────────────┘              └──────────┘              └──────────────┘
```

| Cardinalidad | Ejemplo |
|---|---|
| 1:1 | una persona tiene un pasaporte |
| 1:N | un departamento tiene muchas materias |
| N:M | un estudiante cursa muchas materias, y cada materia tiene muchos estudiantes |

Una **entidad débil** no se identifica por sí sola: depende de otra (los renglones de una factura).

## Modelo relacional

Los datos se organizan en **relaciones** (tablas). Cada fila es una **tupla** y cada columna, un **atributo** con un dominio.

### Claves

| Clave | Definición |
|---|---|
| Superclave | conjunto de atributos que identifica cada fila |
| Clave candidata | superclave mínima (no le sobra ningún atributo) |
| **Clave primaria** | la clave candidata elegida; no admite nulos |
| Clave alternativa | las demás candidatas (restricción `UNIQUE`) |
| **Clave foránea** | atributo que referencia la clave primaria de otra tabla |
| Clave sustituta | identificador artificial (un entero autoincremental o un UUID) |

La **integridad referencial** exige que todo valor de una clave foránea exista en la tabla referenciada.

### Del diagrama a las tablas

| En el diagrama | En el modelo relacional |
|---|---|
| Entidad | una tabla |
| Relación 1:N | clave foránea en el lado «N» |
| Relación N:M | una **tabla intermedia** con las dos claves foráneas |
| Relación 1:1 | clave foránea con restricción de unicidad |
| Atributo multivaluado | una tabla aparte |

```sql
CREATE TABLE departamento (
    id      INTEGER PRIMARY KEY,
    nombre  TEXT NOT NULL UNIQUE
);

CREATE TABLE materia (
    id               INTEGER PRIMARY KEY,
    nombre           TEXT NOT NULL,
    departamento_id  INTEGER NOT NULL REFERENCES departamento(id)
);

CREATE TABLE estudiante (
    legajo  INTEGER PRIMARY KEY,
    nombre  TEXT NOT NULL,
    email   TEXT NOT NULL UNIQUE
);

CREATE TABLE inscripcion (
    legajo      INTEGER REFERENCES estudiante(legajo),
    materia_id  INTEGER REFERENCES materia(id),
    anio        INTEGER NOT NULL,
    nota        NUMERIC(4, 2) CHECK (nota BETWEEN 0 AND 10),
    PRIMARY KEY (legajo, materia_id, anio)
);
```

## Anomalías

Una tabla mal diseñada:

| legajo | estudiante | materia | docente | oficina_docente |
|---|---|---|---|---|
| 101 | Ana | Física I | Ruiz | B-12 |
| 101 | Ana | Álgebra | Soto | A-03 |
| 102 | Luis | Física I | Ruiz | B-12 |

| Anomalía | Problema |
|---|---|
| De actualización | si Ruiz cambia de oficina, hay que modificar muchas filas; si se olvida una, hay inconsistencia |
| De inserción | no se puede registrar un docente que todavía no tiene estudiantes |
| De borrado | al borrar la última inscripción de una materia, se pierden los datos del docente |

La causa es la **redundancia**. La **normalización** la elimina dividiendo las tablas.

## Dependencias funcionales

$X \to Y$ («$X$ determina a $Y$») significa que dos filas con el mismo valor de $X$ tienen siempre el mismo valor de $Y$.

En el ejemplo:

- legajo → estudiante
- materia → docente
- docente → oficina_docente

La clave es (legajo, materia).

## Formas normales

### Primera forma normal (1FN)

Todos los atributos son **atómicos**: sin listas ni grupos repetidos.

| No cumple | Cumple |
|---|---|
| `telefonos = "387-111, 387-222"` | una tabla `telefono` con una fila por número |

### Segunda forma normal (2FN)

Está en 1FN y ningún atributo no clave depende de **una parte** de la clave.

En el ejemplo, `estudiante` depende solo de `legajo`, y `docente` solo de `materia`: dependencias **parciales**. Se separan:

- estudiante(<u>legajo</u>, nombre)
- materia(<u>materia</u>, docente, oficina_docente)
- inscripcion(<u>legajo, materia</u>)

### Tercera forma normal (3FN)

Está en 2FN y ningún atributo no clave depende de **otro atributo no clave** (dependencia transitiva).

En `materia`, la oficina depende del docente, no de la materia. Se separa:

- materia(<u>materia</u>, docente)
- docente(<u>docente</u>, oficina)

### Forma normal de Boyce-Codd (FNBC)

Para toda dependencia $X \to Y$ no trivial, $X$ es una superclave. Es algo más estricta que la 3FN; difieren solo cuando hay varias claves candidatas compuestas que se solapan.

> [!IMPORTANT]
> Regla mnemotécnica: cada atributo no clave debe depender de **la clave, toda la clave y nada más que la clave**.

| Forma | Elimina |
|---|---|
| 1FN | atributos no atómicos |
| 2FN | dependencias parciales de la clave |
| 3FN | dependencias transitivas |
| FNBC | toda dependencia cuyo determinante no sea superclave |

Una buena descomposición es **sin pérdida**: uniendo las tablas resultantes se recupera exactamente la original.

## Desnormalización

Normalizar reduce la redundancia, pero obliga a más combinaciones (`JOIN`). A veces se **desnormaliza** a propósito:

| Caso | Técnica |
|---|---|
| Consultas de lectura muy frecuentes | guardar un valor calculado (un total, un contador) |
| Informes y analítica | tablas resumidas, vistas materializadas |
| Historial | copiar el dato vigente (el precio al momento de la venta) |

> [!WARNING]
> Desnormalizar tiene un costo: hay que mantener sincronizadas las copias. Normalizar primero, y desnormalizar solo cuando una medición lo justifique.

Guardar el precio en el renglón de una factura **no** es redundancia: es un hecho distinto (el precio de ese día), que no debe cambiar si luego cambia el del producto.

## Álgebra relacional

Es el fundamento teórico de SQL: operaciones que toman relaciones y devuelven relaciones.

| Operación | Símbolo | Qué hace | En SQL |
|---|---|---|---|
| Selección | $\sigma_{\text{condición}}(R)$ | filtra filas | `WHERE` |
| Proyección | $\pi_{\text{atributos}}(R)$ | elige columnas | `SELECT` |
| Producto cartesiano | $R \times S$ | todas las combinaciones | `CROSS JOIN` |
| Reunión | $R \bowtie S$ | combina filas relacionadas | `JOIN` |
| Unión | $R \cup S$ | filas de una u otra | `UNION` |
| Diferencia | $R - S$ | filas de $R$ que no están en $S$ | `EXCEPT` |
| Intersección | $R \cap S$ | filas comunes | `INTERSECT` |
| Renombrado | $\rho$ | cambia nombres | `AS` |

> [!NOTE]
> **Ejemplo.** Nombres de los estudiantes con nota mayor que 8:
> $$\pi_{\text{nombre}}\big(\sigma_{\text{nota} > 8}(\text{estudiante} \bowtie \text{inscripcion})\big)$$
>
> ```sql
> SELECT e.nombre
> FROM estudiante e
> JOIN inscripcion i ON i.legajo = e.legajo
> WHERE i.nota > 8;
> ```

El motor traduce cada consulta SQL a un árbol de estas operaciones y lo **optimiza**: por ejemplo, aplica las selecciones lo antes posible para combinar menos filas.

## Restricciones

| Restricción | Garantiza |
|---|---|
| `PRIMARY KEY` | identificación única, sin nulos |
| `FOREIGN KEY` | integridad referencial |
| `UNIQUE` | valores no repetidos |
| `NOT NULL` | el dato es obligatorio |
| `CHECK` | una condición sobre los valores |
| `DEFAULT` | un valor por omisión |

Las acciones de una clave foránea definen qué pasa al borrar la fila referenciada: `RESTRICT` (lo impide), `CASCADE` (borra las dependientes), `SET NULL`.

> [!TIP]
> Dejar que la base de datos haga cumplir las reglas. Una restricción declarada no se puede saltear; una validación que vive solo en el código de la aplicación, sí.
