+++
title = "Búsqueda y tablas hash"
summary = "Búsqueda lineal y binaria, funciones hash, resolución de colisiones, y conjuntos y diccionarios."
branch = "Algoritmos y estructuras de datos"
order = 4
level = "Intermedio"
tags = ["búsqueda binaria", "hash", "diccionario", "colisiones"]
+++

## Búsqueda lineal

Recorre los elementos uno por uno hasta encontrar el buscado. Funciona con cualquier secuencia, ordenada o no.

```python
def busqueda_lineal(lista, objetivo):
    for i, valor in enumerate(lista):
        if valor == objetivo:
            return i
    return -1
```

Costo: $O(n)$ en el peor caso.

## Búsqueda binaria

Si la secuencia está **ordenada**, se puede descartar la mitad en cada paso: se compara con el elemento central y se sigue por la mitad que corresponde.

```python
def busqueda_binaria(lista, objetivo):
    izq, der = 0, len(lista) - 1
    while izq <= der:
        medio = (izq + der) // 2
        if lista[medio] == objetivo:
            return medio
        if lista[medio] < objetivo:
            izq = medio + 1
        else:
            der = medio - 1
    return -1
```

Costo: $O(\log n)$. En un millón de elementos, a lo sumo 20 comparaciones.

> [!NOTE]
> Buscar 23 en `[2, 5, 8, 12, 16, 23, 38, 56, 72, 91]`:
>
> | Paso | izq | der | medio | Valor | Acción |
> |---|---|---|---|---|---|
> | 1 | 0 | 9 | 4 | 16 | 16 < 23: ir a la derecha |
> | 2 | 5 | 9 | 7 | 56 | 56 > 23: ir a la izquierda |
> | 3 | 5 | 6 | 5 | 23 | encontrado |

> [!WARNING]
> La búsqueda binaria **exige** datos ordenados. Sobre una lista desordenada da resultados incorrectos sin avisar.

Errores clásicos: usar `izq < der` en lugar de `<=`, y no mover los límites más allá de `medio` (bucle infinito). En lenguajes con enteros de tamaño fijo, `(izq + der)` puede desbordar: se usa `izq + (der - izq) / 2`.

La misma idea sirve para **buscar una respuesta** en un rango: si una condición es falsa hasta cierto valor y verdadera después, la búsqueda binaria encuentra la frontera.

| Método | Requisito | Costo |
|---|---|---|
| Lineal | ninguno | $O(n)$ |
| Binaria | datos ordenados y acceso por índice | $O(\log n)$ |
| Tabla hash | una función hash | $O(1)$ promedio |
| Árbol de búsqueda balanceado | orden entre claves | $O(\log n)$ |

## Tablas hash

Una **tabla hash** guarda pares clave-valor y permite buscar, insertar y borrar en tiempo **constante** en promedio.

1. Una **función hash** convierte la clave en un número entero.
2. Ese número, módulo el tamaño de la tabla, da el **índice** de la casilla.

$$\text{índice} = \text{hash}(\text{clave}) \bmod m$$

### Función hash

Una buena función hash es:

- **Determinista**: la misma clave da siempre el mismo valor.
- **Uniforme**: distribuye las claves parejo entre las casillas.
- **Rápida** de calcular.

```python
def hash_texto(texto, m):
    h = 0
    for c in texto:
        h = (h * 31 + ord(c)) % m
    return h
```

### Colisiones

Dos claves distintas pueden caer en la misma casilla: es una **colisión**. Son inevitables, porque hay más claves posibles que casillas.

| Estrategia | Cómo funciona |
|---|---|
| **Encadenamiento** | cada casilla guarda una lista con todos los pares que cayeron allí |
| **Direccionamiento abierto** | si la casilla está ocupada, se prueba otra según una regla (la siguiente, saltos cuadráticos, doble hash) |

### Factor de carga

$$\alpha = \frac{n}{m} = \frac{\text{elementos}}{\text{casillas}}$$

Cuando $\alpha$ supera un umbral (típicamente 0,75), la tabla se **redimensiona**: se crea una más grande y se reubican todos los elementos. Esa operación cuesta $O(n)$, pero ocurre tan pocas veces que el costo **amortizado** de insertar sigue siendo $O(1)$.

| Operación | Promedio | Peor caso |
|---|---|---|
| Buscar | $O(1)$ | $O(n)$ |
| Insertar | $O(1)$ | $O(n)$ |
| Borrar | $O(1)$ | $O(n)$ |

El peor caso ocurre si muchas claves colisionan en la misma casilla.

## Diccionarios y conjuntos

| Estructura | Guarda | Python | Java | Rust | JavaScript |
|---|---|---|---|---|---|
| Diccionario (mapa) | pares clave-valor | `dict` | `HashMap` | `HashMap` | `Map` |
| Conjunto | claves sin repetir | `set` | `HashSet` | `HashSet` | `Set` |

```python
edades = {"Ana": 30, "Luis": 25}
edades["Eva"] = 41
edades.get("Juan", 0)          # 0: valor por defecto si no está
"Ana" in edades                # True, en O(1)

vistos = set()
vistos.add(3)
3 in vistos                    # True
```

> [!IMPORTANT]
> Las claves deben ser **inmutables** (o no cambiar mientras estén en la tabla): si una clave cambia, su hash cambia y ya no se la encuentra.

Las tablas hash no mantienen las claves **ordenadas**. Para recorrer en orden o buscar por rangos se usa un árbol balanceado (`TreeMap` en Java, `BTreeMap` en Rust).

## Problemas típicos

**Contar frecuencias**:

```python
def frecuencias(palabras):
    conteo = {}
    for p in palabras:
        conteo[p] = conteo.get(p, 0) + 1
    return conteo
```

**Detectar duplicados** en $O(n)$:

```python
def tiene_duplicados(lista):
    vistos = set()
    for x in lista:
        if x in vistos:
            return True
        vistos.add(x)
    return False
```

**Dos números que suman un objetivo**, en una sola pasada:

```python
def dos_que_suman(numeros, objetivo):
    posiciones = {}
    for i, n in enumerate(numeros):
        if objetivo - n in posiciones:
            return posiciones[objetivo - n], i
        posiciones[n] = i
    return None
```

La versión ingenua con dos bucles anidados cuesta $O(n^2)$; con un diccionario, $O(n)$.

## Hash más allá de las tablas

| Uso | Idea |
|---|---|
| Verificar integridad | el hash de un archivo cambia si se altera un solo bit |
| Guardar contraseñas | se guarda el hash (con sal), no la contraseña |
| Caché | la clave identifica un resultado ya calculado |
| Git | cada commit se identifica por el hash de su contenido |
| Deduplicación | contenidos iguales tienen el mismo hash |
| Filtros de Bloom | prueba de pertenencia aproximada con muy poca memoria |

Los **hash criptográficos** (SHA-256) son además resistentes a colisiones intencionales e imposibles de invertir en la práctica. Para contraseñas se usan funciones deliberadamente lentas (Argon2, bcrypt).

> [!TIP]
> Si un algoritmo pregunta muchas veces «¿ya vi este elemento?» o «¿cuánto vale para esta clave?», la respuesta casi siempre es un conjunto o un diccionario.
